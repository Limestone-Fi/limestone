// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";
import {ReentrancyGuardTransient} from "solady/src/utils/ReentrancyGuardTransient.sol";
import {
    IPositionCoordinator,
    V2LikePositionInvestmentContext,
    V2LikePositionDivestmentContext,
    V2LikePositionLiquidationContext,
    ClaimWorkerRewardsContext
} from "./interfaces/IPositionCoordinator.sol";
import {IMultiModalWorker, MultiModalPosition} from "./interfaces/IMultiModalWorker.sol";
import {ILendingPool} from "./interfaces/ILendingPool.sol";
import {LendingPoolLib} from "./lib/LendingPoolLib.sol";
import {Cast} from "./lib/Cast.sol";
import {LendingPoolStorage, Market} from "./LendingPoolStorage.sol";

/// @title Position Coordinator Facet
/// @author Chainvisions
/// @notice A periphery facet used for managing multimodal LYF positions.

contract PositionCoordinator is IPositionCoordinator, ReentrancyGuardTransient {
    using SafeTransferLib for address;
    using LendingPoolLib for *;
    using Cast for uint256;

    /// @notice Thrown when a position has not yet met the debt ratio threshold to be liquidated.
    error CannotLiquidate();

    /// @notice Thrown when a position is unhealthy upon investing or divesting in a position.
    error UnhealthyPosition();

    /// @notice Thrown when attempting to execute a worker that is not authorized.
    error UnauthorizedWorker();

    /// @notice Thrown when the position health does not increase when repaying debt on a position.
    error HealthDidNotIncrease();

    /// @notice Thrown when attempting to access assets from a worker that is not currently in the execution scope.
    error NotWorkerInExecution();

    /// @notice Thrown when a liquidation doesn't improve a position's health enough.
    error PositionNearLiquidationThreshold();

    /// @notice Structure for investment related vars.
    /// @dev Used to avoid stack-too-deep errors when not using Yul IR (important for fuzzing).
    struct InvestmentStack {
        /// @dev The amount of token0 debt shares to add to the position.
        uint112 token0Shares;
        /// @dev The amount of token1 debt shares to add to the position.
        uint112 token1Shares;
        /// @dev The max amount of leverage that a user is permitted to open a position with.
        uint256 userMaxLeverage;
        /// @dev The ID of the position being created or invested in.
        uint256 positionId;
        /// @dev The equity of the position, used for calculating the current leverage.
        uint256 positionEquity;
        /// @dev The debt value (in token0) of the position, used in leverage calculation/validation.
        uint256 positionDebt;
    }

    /// @notice Structure for divestment related vars.
    /// @dev Used to avoid stack-too-deep errors when not using Yul IR (important for fuzzing).
    struct DivestmentStack {
        /// @dev The max amount of leverage that a user is permitted to open a position with.
        uint256 userMaxLeverage;
        /// @dev The amount of token0 received from removing liquidity.
        uint256 amount0Out;
        /// @dev The amount of token1 received from removing liquidity.
        uint256 amount1Out;
        /// @dev The position's new debt share total for token0.
        uint256 newDebt0;
        /// @dev The position's new debt share total for token1.
        uint256 newDebt1;
        /// @dev Difference in token0 debt shares after repaying, used for updating lending pool data.
        uint256 debt0Delta;
        /// @dev Difference in token1 debt shares after repaying, used for updating lending pool data.
        uint256 debt1Delta;
        /// @dev The new equity of the position, used for validating leverage.
        uint256 positionEquity;
        /// @dev The new debt of the position, used for validating leverage.
        uint256 positionDebt;
    }

    /// @notice Invests assets into a leveraged yield farming position for a Uniswap V2 like liquidity pool.
    /// @param _ctx Context for the investment call, used for discerning parameters related to the call.
    function investInV2LikePosition(V2LikePositionInvestmentContext calldata _ctx) external override {
        LendingPoolLib._enforceEOA();
        // Validate current borrowing parameters, increase worker debt, and transfer assets to the worker.
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        (Market storage pool0, Market storage pool1) = ($.pools[_ctx.token0PoolId], $.pools[_ctx.token1PoolId]);
        InvestmentStack memory stack;
        require($.authorizedContractBorrowers[_ctx.worker], UnauthorizedWorker());
        if (_ctx.token0Borrow > 0) {
            _ctx.token0PoolId._accrue();
            _ctx.worker._verifyBorrowerPermissions(_ctx.token0PoolId, _ctx.token0Borrow, false);
            stack.token0Shares = _ctx.worker._increaseDelegatedDebt(_ctx.token0PoolId, _ctx.token0Borrow);
            pool0.warchest.withdrawReserves(_ctx.worker, _ctx.token0Borrow);
        }
        if (_ctx.token1Borrow > 0) {
            _ctx.token1PoolId._accrue();
            _ctx.worker._verifyBorrowerPermissions(_ctx.token1PoolId, _ctx.token1Borrow, false);
            stack.token1Shares = _ctx.worker._increaseDelegatedDebt(_ctx.token1PoolId, _ctx.token1Borrow);
            pool1.warchest.withdrawReserves(_ctx.worker, _ctx.token1Borrow);
        }
        stack.userMaxLeverage = LendingPoolLib._calculateMaxWorkFactorForUser(msg.sender, _ctx.worker);

        // Invest into the position and gauge the position health to ensure its within the accepted debt ratio.
        LendingPoolLib._setExecutionScope(0, _ctx.worker);
        stack.positionId =
            IMultiModalWorker(_ctx.worker).invest(_ctx, msg.sender, stack.token0Shares, stack.token1Shares);
        (stack.positionEquity, stack.positionDebt) =
            IMultiModalWorker(_ctx.worker).calculatePositionValue(stack.positionId);
        require(
            stack.positionEquity > stack.positionDebt
                && (stack.positionEquity - stack.positionDebt) * stack.userMaxLeverage >= stack.positionEquity * 100,
            UnhealthyPosition()
        );

        emit PositionInvested(
            stack.positionId,
            msg.sender,
            _ctx.worker,
            _ctx.token0In,
            _ctx.token1In,
            _ctx.token0Borrow,
            _ctx.token1Borrow
        );
    }

    /// @notice Divests assets from a position for a Uniswap V2 like liquidity pool.
    /// @param _ctx Context for the divestment call, used for discerning parameters related to the call.
    function divestFromV2LikePosition(V2LikePositionDivestmentContext calldata _ctx) external override nonReentrant {
        LendingPoolLib._enforceEOA();
        // Accrue any pending interest for debt accounting.
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        MultiModalPosition memory pos = IMultiModalWorker(_ctx.worker).getPosition(_ctx.positionId);
        DivestmentStack memory stack;
        require($.authorizedContractBorrowers[_ctx.worker], UnauthorizedWorker());
        pos.debt0PoolId._accrue();
        pos.debt1PoolId._accrue();
        stack.userMaxLeverage = LendingPoolLib._calculateMaxWorkFactorForUser(msg.sender, _ctx.worker);

        // Call worker to divest assets from the position and repay debt.
        (stack.amount0Out, stack.amount1Out, stack.newDebt0, stack.newDebt1) =
            IMultiModalWorker(_ctx.worker).divest(_ctx, msg.sender);

        // Calculate debt share differences and use them to remove any accounted debt shares if needed.
        stack.debt0Delta = FixedPointMathLib.dist(pos.debtShare0, stack.newDebt0);
        stack.debt1Delta = FixedPointMathLib.dist(pos.debtShare1, stack.newDebt1);
        if (stack.debt0Delta > 0) _ctx.worker._decreaseDelegatedDebtByShares(pos.debt0PoolId, stack.debt0Delta.u112());
        if (stack.debt1Delta > 0) _ctx.worker._decreaseDelegatedDebtByShares(pos.debt1PoolId, stack.debt1Delta.u112());

        // Calculate position health to ensure it remains within the optimal debt ratio.
        if (stack.newDebt0 > 0 || stack.newDebt1 > 0) {
            (stack.positionEquity, stack.positionDebt) =
                IMultiModalWorker(_ctx.worker).calculatePositionValue(_ctx.positionId);
            require(
                stack.positionEquity > stack.positionDebt
                    && (stack.positionEquity - stack.positionDebt) * stack.userMaxLeverage >= stack.positionEquity * 100,
                UnhealthyPosition()
            );
        }

        (Market storage pool0, Market storage pool1) = ($.pools[pos.debt0PoolId], $.pools[pos.debt1PoolId]);
        pool0.underlying.safeTransfer(address(pool0.warchest), pool0.underlying.balanceOf(address(this)));
        pool1.underlying.safeTransfer(address(pool1.warchest), pool1.underlying.balanceOf(address(this)));

        // Validate state transitions to ensure that invariants weren't violated.
        _verifyPositionDebtStateTransition(
            pos, IMultiModalWorker(_ctx.worker).getPosition(_ctx.positionId), stack.debt0Delta, stack.debt1Delta
        );

        emit PositionDivested(
            _ctx.positionId, _ctx.worker, stack.amount0Out, stack.amount1Out, _ctx.token0Repay, _ctx.token1Repay
        );
    }

    /// @notice Repays debt for a position on a Uniswap V2 like liquidity pool.
    /// @param _positionId ID of the position to repay debt for.
    /// @param _worker Worker of the position to repay debt on.
    /// @param _repayToken0 Amount of token0 debt to repay on the position.
    /// @param _repayToken1 Amount of token1 debt to repay on the position.
    function repayV2LikeLiquidityPositionDebt(
        uint256 _positionId,
        address _worker,
        uint256 _repayToken0,
        uint256 _repayToken1
    ) external override nonReentrant {
        LendingPoolLib._enforceEOA();
        require(LendingPoolStorage.layout().authorizedContractBorrowers[_worker], UnauthorizedWorker());
        // Accrue any pending interest for debt accounting.
        MultiModalPosition memory pos = IMultiModalWorker(_worker).getPosition(_positionId);
        pos.debt0PoolId._accrue();
        pos.debt1PoolId._accrue();

        // Repay assets to the worker.
        (, uint256 debtValueBefore) = IMultiModalWorker(_worker).calculatePositionValue(_positionId);
        LendingPoolLib._setExecutionScope(0, _worker);
        (uint112 shares0Removed, uint112 shares1Removed) =
            IMultiModalWorker(_worker).repayDebt(msg.sender, _positionId, _repayToken0, _repayToken1);
        if (shares0Removed > 0) _worker._decreaseDelegatedDebtByShares(pos.debt0PoolId, shares0Removed);
        if (shares1Removed > 0) _worker._decreaseDelegatedDebtByShares(pos.debt1PoolId, shares1Removed);
        (, uint256 debtValueAfter) = IMultiModalWorker(_worker).calculatePositionValue(_positionId);

        // Check to ensure that the debt value decreased.
        require(debtValueAfter < debtValueBefore, HealthDidNotIncrease());

        emit DebtRepaid(
            _positionId, msg.sender, _worker, _repayToken0, _repayToken1, shares0Removed, shares1Removed, debtValueAfter
        );
    }

    /// @notice Liquidates a position for a Uniswap V2 like liquidity pool.
    /// @param _ctx Context of the liquidation call, used to discern parameters related to the liquidation.
    function liquidateV2LikePosition(V2LikePositionLiquidationContext calldata _ctx) external override nonReentrant {
        // Accrue any pending interest related to the debt.
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        MultiModalPosition memory pos = IMultiModalWorker(_ctx.worker).getPosition(_ctx.positionId);
        require($.authorizedContractBorrowers[_ctx.worker], UnauthorizedWorker());
        pos.debt0PoolId._accrue();
        pos.debt1PoolId._accrue();

        // Check if position can be liquidated and perform a healthcheck on the worker to ensure there is no manipulation.
        (uint256 positionValue, uint256 debtValue) =
            IMultiModalWorker(_ctx.worker).calculatePositionValue(_ctx.positionId);
        require((positionValue * $.workerDebtParams[_ctx.worker].killFactor) < (debtValue * 10000), CannotLiquidate());

        // Call liquidation method on the worker.
        LendingPoolLib._setExecutionScope(uint32(0), _ctx.worker);
        (uint112 newDebt0, uint112 newDebt1) = IMultiModalWorker(_ctx.worker).liquidate(msg.sender, _ctx);

        // Calculate debt share differences and use them to remove any accounted debt shares if needed.
        uint256 debt0Delta = FixedPointMathLib.dist(pos.debtShare0, newDebt0);
        uint256 debt1Delta = FixedPointMathLib.dist(pos.debtShare1, newDebt1);
        if (debt0Delta > 0) _ctx.worker._decreaseDelegatedDebtByShares(pos.debt0PoolId, debt0Delta.u112());
        if (debt1Delta > 0) _ctx.worker._decreaseDelegatedDebtByShares(pos.debt1PoolId, debt1Delta.u112());

        // Check user position health to ensure it's back within our healthy range (at least 10% less than kill factor).
        (positionValue, debtValue) = IMultiModalWorker(_ctx.worker).calculatePositionValue(_ctx.positionId);
        require(
            positionValue * ($.workerDebtParams[_ctx.worker].killFactor - 1000) >= (debtValue * 10000),
            PositionNearLiquidationThreshold()
        );

        // Validate state transitions to ensure that invariants weren't violated.
        _verifyPositionDebtStateTransition(
            pos, IMultiModalWorker(_ctx.worker).getPosition(_ctx.positionId), debt0Delta, debt1Delta
        );

        emit PositionLiquidated(
            _ctx.positionId,
            _ctx.worker,
            _ctx.token0RepayIn,
            _ctx.token1RepayIn,
            debt0Delta,
            debt1Delta,
            positionValue,
            debtValue
        );
    }

    /// @notice Used to access approved assets from a user. Called by workers for handling multi-token `transferFrom()`.
    /// @param _user User to transfer assets from.
    /// @param _tokens Assets to transfer from the user.
    /// @param _amounts Amounts of each asset to transfer from the user.
    function accessAssets(address _user, address[] calldata _tokens, uint256[] calldata _amounts) external override {
        LendingPoolStorage.ExecScope memory scope = LendingPoolLib._readExecutionScope();
        require(msg.sender == scope.worker, NotWorkerInExecution());
        for (uint256 i; i < _tokens.length;) {
            if (_amounts[i] > 0) {
                _tokens[i].safeTransferFrom(_user, scope.worker, _amounts[i]);
            }
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }
    }

    /// @notice Performance fee for Limestone workers.
    function reinvestmentFeeNumerator() external pure override returns (uint256) {
        return 800;
    }

    /// @dev An internal sanity check used to enforce an invariant that ensures that transitions in the lending pool debt state are as expected. This is designed as an assertion that should never fail.
    function _verifyMarketDebtStateTransition() internal pure {}

    /// @dev An internal sanity check used to enforce an invariant that ensures that transitions in the position debt state are as expected. This is designed as an assertion that should never fail.
    /// @param _oldSnapshot The old snapshot of the position that was originally fetched at the beginning of the function execution.
    /// @param _latestSnapshot The latest snapshot of the position freshly fetched from the worker's state used for comparing against.
    /// @param _expectedDelta0 The expected delta in debtShare0 that was calculated during the debt removal process.
    /// @param _expectedDelta1 The expected delta in debtShare1 that was calculated during the debt removal process.
    function _verifyPositionDebtStateTransition(
        MultiModalPosition memory _oldSnapshot,
        MultiModalPosition memory _latestSnapshot,
        uint256 _expectedDelta0,
        uint256 _expectedDelta1
    ) internal pure {
        uint256 trueDelta0 = FixedPointMathLib.dist(_oldSnapshot.debtShare0, _latestSnapshot.debtShare0);
        uint256 trueDelta1 = FixedPointMathLib.dist(_oldSnapshot.debtShare1, _latestSnapshot.debtShare1);
        assert(trueDelta0 == _expectedDelta0);
        assert(trueDelta1 == _expectedDelta1);
    }
}
