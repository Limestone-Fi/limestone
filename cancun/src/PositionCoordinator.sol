// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";
import {
    IPositionCoordinator,
    V2LikePositionInvestmentContext,
    V2LikePositionDivestmentContext,
    V2LikePositionLiquidationContext,
    ClaimWorkerRewardsContext
} from "./interfaces/IPositionCoordinator.sol";
import {IMultiModalWorker, MultiModalPosition} from "./interfaces/IMultiModalWorker.sol";
import {ILendingPool} from "./interfaces/ILendingPool.sol";
import {_require, Errors} from "./lib/Errors.sol";
import {LendingPoolLib} from "./lib/LendingPoolLib.sol";
import {Cast} from "./lib/Cast.sol";
import {LendingPoolStorage, Market} from "./LendingPoolStorage.sol";

/// @title Position Coordinator Facet
/// @author Chainvisions
/// @notice A periphery facet used for managing multimodal LYF positions.

contract PositionCoordinator is IPositionCoordinator {
    using SafeTransferLib for address;
    using LendingPoolLib for *;
    using Cast for uint256;

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
        // Validate current borrowing parameters, increase worker debt, and transfer assets to the worker.
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        (Market storage pool0, Market storage pool1) = ($.pools[_ctx.token0PoolId], $.pools[_ctx.token1PoolId]);
        InvestmentStack memory stack;
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
        _require(
            stack.positionEquity > stack.positionDebt
                && (stack.positionEquity - stack.positionDebt) * stack.userMaxLeverage >= stack.positionEquity * 100,
            Errors.UNHEALTHY_POSITION
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
    function divestFromV2LikePosition(V2LikePositionDivestmentContext calldata _ctx) external override {
        // Accrue any pending interest for debt accounting.
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        MultiModalPosition memory pos = IMultiModalWorker(_ctx.worker).getPosition(_ctx.positionId);
        DivestmentStack memory stack;
        pos.debt0PoolId._accrue();
        pos.debt1PoolId._accrue();
        stack.userMaxLeverage = LendingPoolLib._calculateMaxWorkFactorForUser(msg.sender, _ctx.worker);

        // Call worker to divest assets from the position and repay debt.
        (stack.amount0Out, stack.amount1Out, stack.newDebt0, stack.newDebt1) =
            IMultiModalWorker(_ctx.worker).divest(_ctx);

        // Calculate debt share differences and use them to remove any accounted debt shares if needed.
        stack.debt0Delta = FixedPointMathLib.dist(pos.debtShare0, stack.newDebt0);
        stack.debt1Delta = FixedPointMathLib.dist(pos.debtShare1, stack.newDebt1);
        if (stack.debt0Delta > 0) _ctx.worker._decreaseDelegatedDebtByShares(pos.debt0PoolId, stack.debt0Delta.u112());
        if (stack.debt1Delta > 0) _ctx.worker._decreaseDelegatedDebtByShares(pos.debt1PoolId, stack.debt1Delta.u112());

        // Calculate position health to ensure it remains within the optimal debt ratio.
        if (stack.newDebt0 > 0 || stack.newDebt1 > 0) {
            (stack.positionEquity, stack.positionDebt) =
                IMultiModalWorker(_ctx.worker).calculatePositionValue(_ctx.positionId);
            _require(
                stack.positionEquity > stack.positionDebt
                    && (stack.positionEquity - stack.positionDebt) * stack.userMaxLeverage >= stack.positionEquity * 100,
                Errors.UNHEALTHY_POSITION
            );
        }

        (Market storage pool0, Market storage pool1) = ($.pools[pos.debt0PoolId], $.pools[pos.debt1PoolId]);
        pool0.underlying.safeTransfer(address(pool0.warchest), pool0.underlying.balanceOf(address(this)));
        pool1.underlying.safeTransfer(address(pool1.warchest), pool1.underlying.balanceOf(address(this)));

        emit PositionDivested(
            _ctx.positionId,
            _ctx.worker,
            0, // TODO: Replace
            stack.amount0Out,
            stack.amount1Out,
            _ctx.token0Repay,
            _ctx.token1Repay
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
    ) external override {
        // Accrue any pending interest for debt accounting.
        MultiModalPosition memory pos = IMultiModalWorker(_worker).getPosition(_positionId);
        pos.debt0PoolId._accrue();
        pos.debt1PoolId._accrue();

        // Repay assets to the worker.
        (, uint256 debtValueBefore) = IMultiModalWorker(_worker).calculatePositionValue(_positionId);
        (uint112 shares0Removed, uint112 shares1Removed) =
            IMultiModalWorker(_worker).repayDebt(_positionId, _repayToken0, _repayToken1);
        if (shares0Removed > 0) _worker._decreaseDelegatedDebtByShares(pos.debt0PoolId, shares0Removed);
        if (shares1Removed > 0) _worker._decreaseDelegatedDebtByShares(pos.debt1PoolId, shares1Removed);
        (, uint256 debtValueAfter) = IMultiModalWorker(_worker).calculatePositionValue(_positionId);

        // Check to ensure that the debt value decreased.
        _require(debtValueAfter < debtValueBefore, Errors.HEALTH_DID_NOT_INCREASE);

        // TODO: Emit event.
    }

    /// @notice Liquidates a position for a Uniswap V2 like liquidity pool.
    /// @param _ctx Context of the liquidation call, used to discern parameters related to the liquidation.
    function liquidateV2LikePosition(V2LikePositionLiquidationContext calldata _ctx) external override {
        // Accrue any pending interest related to the debt.
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        MultiModalPosition memory pos = IMultiModalWorker(_ctx.worker).getPosition(_ctx.positionId);
        pos.debt0PoolId._accrue();
        pos.debt1PoolId._accrue();

        // Check if position can be liquidated and perform a healthcheck on the worker to ensure there is no manipulation.
        (uint256 positionValue, uint256 debtValue) =
            IMultiModalWorker(_ctx.worker).calculatePositionValue(_ctx.positionId);
        _require(
            (positionValue * $.workerDebtParams[_ctx.worker].killFactor) < (debtValue * 10000), Errors.CANT_LIQUIDATE
        );

        // Call liquidation method on the worker.
        LendingPoolLib._setExecutionScope(uint32(0), _ctx.worker);
        IMultiModalWorker(_ctx.worker).liquidate(msg.sender, _ctx);

        // Check user position health to ensure it's back within our healthy range (at least 10% less than kill factor).
        (positionValue, debtValue) = IMultiModalWorker(_ctx.worker).calculatePositionValue(_ctx.positionId);
        _require(
            positionValue * ($.workerDebtParams[_ctx.worker].killFactor - 1000) >= (debtValue * 10000),
            Errors.POSITION_NEAR_LIQ_THRESHOLD
        );

        // TODO: Emit event.
    }

    function accessAssets(address _user, address[] calldata _tokens, uint256[] calldata _amounts) external override {
        LendingPoolStorage.ExecScope memory scope = LendingPoolLib._readExecutionScope();
        _require(msg.sender == scope.worker, Errors.NOT_WORKER_IN_EXEC);
        for (uint256 i; i < _tokens.length;) {
            if (_amounts[i] > 0) {
                _tokens[i].safeTransferFrom(_user, scope.worker, _amounts[i]);
            }
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }
    }

    function reinvestmentFeeNumerator() external view override returns (uint256) {
        return 800;
    }
}
