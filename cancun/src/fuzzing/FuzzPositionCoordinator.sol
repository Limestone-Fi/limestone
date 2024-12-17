// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Market} from "../interfaces/ILendingPool.sol";
import {
    V2LikePositionInvestmentContext,
    V2LikePositionDivestmentContext,
    V2LikePositionLiquidationContext
} from "../interfaces/IPositionCoordinator.sol";
import {MultiModalPosition} from "../interfaces/IMultiModalWorker.sol";
import {FuzzSetup, MockWarchest, MockGauge, MockToken, DromeMultiModalWorker} from "./FuzzSetup.sol";

/// @title Position Coordinator Fuzz Tests
/// @author Chainvisions
/// @notice Mixin containing fuzz tests for Limestone's position coordinator facet.

contract FuzzPositionCoordinator is FuzzSetup {
    /// @notice Fuzz test for the PositionCoordinator's `investInV2LikePosition` method. Creates a new position.
    /// @dev Main invariants:
    /// 1) New position shouldn't be able to overleverage itself / exceed the set max leverage.
    /// @param _amount0In Amount of token0 to supply to the position.
    /// @param _amount1In Amount of token1 to supply to the position.
    /// @param _amount0Borrow Amount of token0 to borrow for the position.
    /// @param _amount1Borrow Amount of token1 to borrow for the position.
    function investInNewPosition(uint256 _amount0In, uint256 _amount1In, uint256 _amount0Borrow, uint256 _amount1Borrow)
        public
        setCurrentActor
    {
        _amount0In = fl.clamp(_amount0In, 0, usdc.balanceOf(currentActor));
        _amount1In = fl.clamp(_amount1In, 0, aero.balanceOf(currentActor));
        _amount0Borrow = fl.clamp(_amount0Borrow, 0, totalUsdcLiquidity);
        _amount1Borrow = fl.clamp(_amount1Borrow, 0, totalAeroLiquidity);

        // Open new position.
        V2LikePositionInvestmentContext memory ctx = V2LikePositionInvestmentContext({
            positionId: 0,
            worker: worker,
            token0In: _amount0In,
            token1In: _amount1In,
            token0PoolId: 1,
            token1PoolId: 2,
            token0Borrow: _amount0Borrow,
            token1Borrow: _amount1Borrow,
            minLiquidityMinted: 0,
            skipHealthcheck: false
        });
        STDCHEATS.prank(currentActor);
        lendingPool.investInV2LikePosition(ctx);

        // Validate position leverage invariant.
        uint256 posId = nextDromePositionId++;
        (uint256 equity, uint256 debt) = DromeMultiModalWorker(worker).calculatePositionValue(posId);
        uint256 leverage = equity > debt ? (equity - debt) * 300 : 0;
        fl.gte(leverage, equity * 100, "NEWINVEST-01: Opened position is overleveraged");

        // Record debt and new position in ghost variables.
        actorDromePositions[currentActor].push(posId);
        if (_amount0Borrow > 0) totalUsdcDebt += _amount0Borrow;
        if (_amount1Borrow > 0) totalAeroDebt += _amount1Borrow;
    }

    /// @notice Fuzz test for the PositionCoordinator's `investInV2LikePosition` method. Adds/borrows to an existing position.
    /// @dev Main invariants:
    /// 1) New position shouldn't be able to overleverage itself / exceed the set max leverage.
    /// @param _posIdx Index (related to actors positions list) of the position to access.
    /// @param _amount0In Amount of token0 to supply to the position.
    /// @param _amount1In Amount of token1 to supply to the position.
    /// @param _amount0Borrow Amount of token0 to borrow for the position.
    /// @param _amount1Borrow Amount of token1 to borrow for the position.
    function addToPosition(
        uint256 _posIdx,
        uint256 _amount0In,
        uint256 _amount1In,
        uint256 _amount0Borrow,
        uint256 _amount1Borrow
    ) public setCurrentActor {
        _posIdx = fl.clamp(_posIdx, 0, actorDromePositions[currentActor].length - 1);
        _amount0In = fl.clamp(_amount0In, 0, usdc.balanceOf(currentActor));
        _amount1In = fl.clamp(_amount1In, 0, aero.balanceOf(currentActor));
        _amount0Borrow = fl.clamp(_amount0Borrow, 0, totalUsdcLiquidity);
        _amount1Borrow = fl.clamp(_amount1Borrow, 0, totalAeroLiquidity);

        // Add to the position.
        uint256 posId = actorDromePositions[currentActor][_posIdx];
        V2LikePositionInvestmentContext memory ctx = V2LikePositionInvestmentContext({
            positionId: posId,
            worker: worker,
            token0In: _amount0In,
            token1In: _amount1In,
            token0PoolId: 1,
            token1PoolId: 2,
            token0Borrow: _amount0Borrow,
            token1Borrow: _amount1Borrow,
            minLiquidityMinted: 0,
            skipHealthcheck: false
        });
        STDCHEATS.prank(currentActor);
        lendingPool.investInV2LikePosition(ctx);

        // Validate position leverage invariant.
        (uint256 equity, uint256 debt) = DromeMultiModalWorker(worker).calculatePositionValue(posId);
        uint256 leverage = equity > debt ? (equity - debt) * 300 : 0;
        fl.gte(leverage, equity * 100, "ADDINVEST-01: Opened position is overleveraged");

        if (_amount0Borrow > 0) totalUsdcDebt += _amount0Borrow;
        if (_amount1Borrow > 0) totalAeroDebt += _amount1Borrow;
    }

    /// @notice Fuzz test for the PositionCoordinator's `investFromV2LikePosition` method. Removes assets from a specific position.
    /// @dev Main invariants:
    /// 1) Position shouldn't be overleveraged after withdrawal.
    /// 2) Repaying USDC or AERO shouldn't remove excessive amounts of debt shares (more debt than the position as this breaks the lending dynamic as a whole by repaying for everyone)
    /// @param _posIdx Index (related to actors positions list) of the position to access.
    /// @param _pct Percentage of the position to remove the assets from.
    /// @param _repay0 Amount of token0 to use for repaying debt.
    /// @param _repay1 Amount of token1 to use for repaying debt.
    /// @param _side Side of the LP to receive, either 0 for token0 or 1 for token1.
    /// @param _minimal Whether the withdrawal should be minimal.
    function removeFromPosition(
        uint256 _posIdx,
        uint16 _pct,
        uint256 _repay0,
        uint256 _repay1,
        uint8 _side,
        bool _minimal
    ) public setCurrentActor {
        _posIdx = fl.clamp(_posIdx, 0, actorDromePositions[currentActor].length - 1);
        _pct = uint8(fl.clamp(_pct, 0, 10000));
        _side = uint8(fl.clamp(_side, 0, 1));

        // Snapshot current market for later comparison.
        uint256 posId = actorDromePositions[currentActor][_posIdx];
        Market memory usdcMarket = lendingPool.pools()[1];
        Market memory aeroMarket = lendingPool.pools()[2];
        MultiModalPosition memory posSnapshot = DromeMultiModalWorker(worker).getPosition(posId);

        // Remove assets from the position.
        V2LikePositionDivestmentContext memory ctx = V2LikePositionDivestmentContext({
            positionId: posId,
            worker: worker,
            positionBps: _pct,
            minToken0Out: 0,
            minToken1Out: 0,
            token0Repay: _repay0,
            token1Repay: _repay1,
            side: _side,
            minimalWithdrawal: _minimal
        });
        STDCHEATS.prank(currentActor);
        lendingPool.divestFromV2LikePosition(ctx);

        // Validate current leverage invariant.
        (uint256 equity, uint256 debt) = DromeMultiModalWorker(worker).calculatePositionValue(posId);
        uint256 leverage = equity > debt ? (equity - debt) * 300 : 0;
        fl.gte(leverage, equity * 100, "DIVEST-01: Opened position is overleveraged");

        // Check whether or not excessive debt was removed.
        uint256 share0Diff = fl.diff(usdcMarket.globalDebtShare, lendingPool.pools()[1].globalDebtShare);
        uint256 share1Diff = fl.diff(aeroMarket.globalDebtShare, lendingPool.pools()[2].globalDebtShare);
        fl.log("USDC debt share:", posSnapshot.debtShare0);
        fl.log("Debt share diff: ", share0Diff);
        fl.log("AERO debt share: ", posSnapshot.debtShare1);
        fl.log("Debt share diff: ", share1Diff);
        fl.lte(share0Diff, posSnapshot.debtShare0, "DIVEST-02: USDC repayment removed too many debt shares");
        fl.lte(share1Diff, posSnapshot.debtShare1, "DIVEST-02: AERO repayment removed too many debt shares");

        // Update current ghost variables.
        if (_repay0 > 0) totalUsdcDebt -= _repay0;
        if (_repay1 > 0) totalAeroDebt -= _repay1;
    }

    /// @notice Fuzz test for the PositionCoordinator's `repayV2LikeLiquidityPositionDebt` method. Repays position debt.
    /// @dev Main invariants:
    /// 1) Repaying USDC or AERO shouldn't remove excessive amounts of debt shares (more debt than the position as this breaks the lending dynamic as a whole by repaying for everyone)
    /// 2) AERO/USDC totalTokens shouldn't change at all, as debt is already accounted for. This makes a change potentially weird and should be investigated if it occurs.
    /// @param _posIdx Index (related to actors positions list) of the position to access.
    /// @param _repay0 Amount of token0 to repay.
    /// @param _repay1 Amount of token1 to repay.
    function repayForPosition(uint256 _posIdx, uint256 _repay0, uint256 _repay1) public setCurrentActor {
        _posIdx = fl.clamp(_posIdx, 0, actorDromePositions[currentActor].length - 1);
        (uint256 balance0, uint256 balance1) = (usdc.balanceOf(currentActor), aero.balanceOf(currentActor));
        _repay0 = fl.clamp(_repay0, 0, balance0);
        _repay1 = fl.clamp(_repay1, 0, balance1);

        uint256 posId = actorDromePositions[currentActor][_posIdx];
        Market memory usdcMarket = lendingPool.pools()[1];
        Market memory aeroMarket = lendingPool.pools()[2];
        MultiModalPosition memory posSnapshot = DromeMultiModalWorker(worker).getPosition(posId);
        uint256 startingUsdcTotal = lendingPool.totalTokens(1);
        uint256 startingAeroTotal = lendingPool.totalTokens(2);

        // Perform repayment.
        STDCHEATS.prank(currentActor);
        lendingPool.repayV2LikeLiquidityPositionDebt(posId, worker, _repay0, _repay1);

        // Check whether or not excessive debt was removed.
        uint256 share0Diff = fl.diff(usdcMarket.globalDebtShare, lendingPool.pools()[1].globalDebtShare);
        uint256 share1Diff = fl.diff(aeroMarket.globalDebtShare, lendingPool.pools()[2].globalDebtShare);
        fl.log("USDC debt share:", posSnapshot.debtShare0);
        fl.log("Debt share diff: ", share0Diff);
        fl.log("AERO debt share: ", posSnapshot.debtShare1);
        fl.log("Debt share diff: ", share1Diff);
        fl.lte(share0Diff, posSnapshot.debtShare0, "REPAY-01: USDC repayment removed too many debt shares");
        fl.lte(share1Diff, posSnapshot.debtShare1, "REPAY-01: AERO repayment removed too many debt shares");

        // Check to ensure that the lending pool is healthier.
        fl.eq(lendingPool.totalTokens(1), startingUsdcTotal, "REPAY-02: USDC totalTokens changed");
        fl.eq(lendingPool.totalTokens(2), startingAeroTotal, "REPAY-02: AERO totalTokens changed");

        // Update current ghost variables.
        if (_repay0 > 0) totalUsdcDebt -= _repay0;
        if (_repay1 > 0) totalAeroDebt -= _repay1;
    }

    /// @notice A method for triggering reinvestment. Doesn't check invariants but is added for its use in call sequences for potential findings.
    function triggerReinvest() public setCurrentActor {
        if (block.timestamp >= lastGaugeInjection + 7 days) {
            aero.mint(address(this), AERO_NOTIFY_AMOUNT);
            aero.approve(gauge, AERO_NOTIFY_AMOUNT);
            MockGauge(gauge).notifyRewardAmount(AERO_NOTIFY_AMOUNT);
            lastGaugeInjection = block.timestamp;
        }
        STDCHEATS.warp(block.timestamp + 24 hours);
        DromeMultiModalWorker(worker).reinvest();
    }
}
