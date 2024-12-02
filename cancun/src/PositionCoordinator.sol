// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";
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
import {LendingPoolStorage, Market} from "./LendingPoolStorage.sol";

/// @title Position Coordinator Facet
/// @author Chainvisions
/// @notice A periphery facet used for managing multimodal LYF positions.

contract PositionCoordinator is IPositionCoordinator {
    using SafeTransferLib for address;
    using LendingPoolLib for *;

    /// @notice Invests assets into a leveraged yield farming position for a Uniswap V2 like liquidity pool.
    /// @param _ctx Context for the investment call, used for discerning parameters related to the call.
    function investInV2LikePosition(V2LikePositionInvestmentContext calldata _ctx) external override {
        // Validate current borrowing parameters, increase worker debt, and transfer assets to the worker.
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        (Market storage pool0, Market storage pool1) = ($.pools[_ctx.token0PoolId], $.pools[_ctx.token1PoolId]);
        (uint256 token0Shares, uint256 token1Shares) = (0, 0);
        if (_ctx.token0Borrow > 0) {
            _ctx.token0PoolId._accrue();
            _ctx.worker._verifyBorrowerPermissions(_ctx.token0PoolId, _ctx.token0Borrow, false);
            token0Shares = _ctx.worker._increaseDelegatedDebt(_ctx.token0PoolId, _ctx.token0Borrow);
            pool0.warchest.withdrawReserves(_ctx.worker, _ctx.token0Borrow);
        }
        if (_ctx.token1Borrow > 0) {
            _ctx.token1PoolId._accrue();
            _ctx.worker._verifyBorrowerPermissions(_ctx.token1PoolId, _ctx.token1Borrow, false);
            token1Shares = _ctx.worker._increaseDelegatedDebt(_ctx.token1PoolId, _ctx.token1Borrow);
            pool1.warchest.withdrawReserves(_ctx.worker, _ctx.token1Borrow);
        }
        uint256 userMaxDebtRatio = LendingPoolLib._calculateMaxWorkFactorForUser(msg.sender, _ctx.worker);

        // Perform healthcheck on worker to ensure that there is no active manipulation.
        _require(IMultiModalWorker(_ctx.worker).healthcheck(), Errors.WORKER_HEALTHCHECK_FAILED);

        // Transfer user supplied assets to the worker.
        if (_ctx.token0In > 0) pool0.underlying.safeTransferFrom(msg.sender, _ctx.worker, _ctx.token0In);
        if (_ctx.token1In > 0) pool1.underlying.safeTransferFrom(msg.sender, _ctx.worker, _ctx.token1In);

        // Invest into the position and gauge the position health to ensure its within the accepted debt ratio.
        uint256 positionId = IMultiModalWorker(_ctx.worker).invest(
            _ctx.positionId, msg.sender, _ctx.token0In, _ctx.token1In, token0Shares, token1Shares
        );
        (uint256 positionEquity, uint256 positionDebt) =
            IMultiModalWorker(_ctx.worker).calculatePositionValue(positionId);
        _require(positionEquity * userMaxDebtRatio >= positionDebt * 10000, Errors.UNHEALTHY_POSITION);

        // TODO: Emit event.
    }

    /// @notice Divests assets from a position for a Uniswap V2 like liquidity pool.
    /// @param _ctx Context for the divestment call, used for discerning parameters related to the call.
    function divestFromV2LikePosition(V2LikePositionDivestmentContext calldata _ctx) external override {
        // Accrue any pending interest for debt accounting.
        //LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        MultiModalPosition memory pos = IMultiModalWorker(_ctx.worker).getPosition(_ctx.positionId);
        //(Market storage pool0, Market storage pool1) = ($.pools[pos.token0PoolId], $.pools[pos.token1PoolId]);
        pos.token0PoolId._accrue();
        pos.token1PoolId._accrue();
        if (_ctx.token0Repay > 0) _ctx.worker._decreaseDelegatedDebt(pos.token0PoolId, _ctx.token0Repay);
        if (_ctx.token1Repay > 0) _ctx.worker._decreaseDelegatedDebt(pos.token1PoolId, _ctx.token1Repay);
        uint256 userMaxDebtRatio = LendingPoolLib._calculateMaxWorkFactorForUser(msg.sender, _ctx.worker);

        // Perform healthcheck on worker to ensure that there is no active manipulation.
        _require(IMultiModalWorker(_ctx.worker).healthcheck(), Errors.WORKER_HEALTHCHECK_FAILED);

        // Call worker to divest assets from the position and repay debt.
        (uint256 amount0Out, uint256 amount1Out, uint256 newDebt0, uint256 newDebt1) =
            IMultiModalWorker(_ctx.worker).divest(_ctx);

        // Calculate position health to ensure it remains within the optimal debt ratio.
        if (newDebt0 > 0 || newDebt1 > 0) {
            (uint256 positionEquity, uint256 positionDebt) =
                IMultiModalWorker(_ctx.worker).calculatePositionValue(_ctx.positionId);
            _require(positionEquity * userMaxDebtRatio >= positionDebt * 10000, Errors.UNHEALTHY_POSITION);
        }

        // TODO: Emit event.
        amount0Out;
        amount1Out;
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
        pos.token0PoolId._accrue();
        pos.token1PoolId._accrue();

        // Perform healthcheck on worker to ensure that there is no active manipulation.
        _require(IMultiModalWorker(_worker).healthcheck(), Errors.WORKER_HEALTHCHECK_FAILED);

        // Repay assets to the worker.
        (, uint256 debtValueBefore) = IMultiModalWorker(_worker).calculatePositionValue(_positionId);
        IMultiModalWorker(_worker).repayDebt(_positionId, _repayToken0, _repayToken1);
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
        pos.token0PoolId._accrue();
        pos.token1PoolId._accrue();

        // Check if position can be liquidated and perform a healthcheck on the worker to ensure there is no manipulation.
        (uint256 positionValue, uint256 debtValue) =
            IMultiModalWorker(_ctx.worker).calculatePositionValue(_ctx.positionId);
        _require(
            (positionValue * $.workerDebtParams[_ctx.worker].killFactor) < (debtValue * 10000), Errors.CANT_LIQUIDATE
        );
        _require(IMultiModalWorker(_ctx.worker).healthcheck(), Errors.WORKER_HEALTHCHECK_FAILED);

        // Call liquidation method on the worker.
        IMultiModalWorker(_ctx.worker).liquidate(msg.sender, _ctx);

        // Check user position health to ensure it's back within our healthy range (at least 10% less than kill factor).
        (positionValue, debtValue) = IMultiModalWorker(_ctx.worker).calculatePositionValue(_ctx.positionId);
        _require(
            positionValue * ($.workerDebtParams[_ctx.worker].killFactor - 1000) >= (debtValue * 10000),
            Errors.POSITION_NEAR_LIQ_THRESHOLD
        );

        // TODO: Emit event.
    }

    /// @notice Claims pending LIME rewards from a list of worker contracts.
    /// @param _ctx Context of the claim call, containing each and every worker and how the claim should be managed.
    function claimWorkerLimeRewards(ClaimWorkerRewardsContext[] calldata _ctx) external {
        for (uint256 i; i < _ctx.length;) {
            IMultiModalWorker(_ctx[i].worker).getReward(_ctx[i].positionId, _ctx[i].vest);
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }
    }
}