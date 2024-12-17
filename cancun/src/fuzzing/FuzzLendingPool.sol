// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {LendingPool, Position, Market} from "../LendingPool.sol";
import {FuzzSetup, MockWarchest, MockToken} from "./FuzzSetup.sol";

/// @title Lending Pool Fuzz Tests
/// @author Chainvisions
/// @notice Mixin containing fuzz tests for Limestone's lending pool facet.

contract FuzzLendingPool is FuzzSetup {
    /// @notice Fuzz test for the lending pool facet's `deposit` method.
    /// @dev Main invariants:
    /// 1) Shares received from the deposit should be roughly the expected amount based on share price.
    /// @param _poolId ID of the lending pool to deposit in. Clamps to either pool 1 (USDC) or pool 2 (AERO).
    /// @param _amount Amount of tokens to deposit into the pool.
    function deposit(uint256 _poolId, uint256 _amount) public setCurrentActor {
        _poolId = fl.clamp(_poolId, 1, 2);
        uint256 balance = _poolId == 1 ? usdc.balanceOf(currentActor) : aero.balanceOf(currentActor);
        if (balance == 0) revert FuzzRequireError();
        _amount = fl.clamp(_amount, 0, balance);

        // Estimate the shares that we *should* receive.
        MockWarchest chest = _poolId == 1 ? warchestUsdc : warchestAero;
        uint256 currentTotal = lendingPool.totalTokens(_poolId) + lendingPool.pendingInterest(_poolId); // @dev Accrued interests may throw this off so we have to tack it on.
        uint256 currentSupply = chest.totalSupply();
        uint256 shouldReceive = (_amount * currentSupply) / currentTotal;
        uint256 preshares = chest.balanceOf(currentActor);

        // Deposit assets and check that we received the amount that we should.
        STDCHEATS.prank(currentActor);
        lendingPool.deposit(_poolId, _amount);
        uint256 postshares = chest.balanceOf(currentActor) - preshares;
        if (_poolId == 1) {
            totalUsdcLiquidity += _amount;
            totalUsdcShares += postshares;
        } else {
            totalAeroLiquidity += _amount;
            totalAeroShares += postshares;
        }
        fl.eq(postshares, shouldReceive, "LEND-01: Shares received from deposit != expected");
        /*
        fl.gt(
            lendingPool.totalTokens(_poolId),
            currentTotal,
            "LEND-02: Something weird happened and totalTokens didn't increase"
        );*/
    }

    /// @notice Fuzz test for the LendingPool facet's `withdraw` method.
    /// @dev Main invariants:
    /// 1) Assets received from the deposit should be roughly the expected amount based on share price.
    /// @param _poolId ID of the lending pool to withdraw from. Clamps to either pool 1 (USDC) or pool 2 (AERO).
    /// @param _amount Amount of assets to withdraw from the lending pool.
    function withdraw(uint256 _poolId, uint256 _amount) public setCurrentActor {
        _poolId = fl.clamp(_poolId, 1, 2);
        MockWarchest chest = _poolId == 1 ? warchestUsdc : warchestAero;
        uint256 shares = chest.balanceOf(currentActor);
        if (shares == 0) revert FuzzRequireError();
        _amount = fl.clamp(_amount, 0, shares);

        // Estimate what we should receive from withdrawing.
        MockToken token = _poolId == 1 ? usdc : aero;
        uint256 currentTotal = lendingPool.totalTokens(_poolId) + lendingPool.pendingInterest(_poolId);
        uint256 currentSupply = chest.totalSupply();
        uint256 shouldReceive = (_amount * currentTotal) / currentSupply;
        uint256 prebalance = token.balanceOf(currentActor);

        // Withdraw and check that we received the appropriate output.
        STDCHEATS.prank(currentActor);
        lendingPool.withdraw(_poolId, _amount);
        uint256 postbalance = token.balanceOf(currentActor) - prebalance;
        if (_poolId == 1) {
            totalUsdcShares -= _amount;
            totalUsdcLiquidity -= shouldReceive;
        } else {
            totalAeroShares -= _amount;
            totalAeroLiquidity -= shouldReceive;
        }
        fl.eq(postbalance, shouldReceive, "WITHDRAW-01: Assets received from withdrawal != expected");
        /*
        fl.lt(
            lendingPool.totalTokens(_poolId),
            currentTotal,
            "WITHDRAW-02: Something weird happened and totalTokens didn't decrease"
        );*/
    }

    /// @notice Fuzz test for the LendingPool facet's `doHardWork` method. Creates a new legacy worker position.
    /// @dev Main invariants:
    /// 1) Position owner should be the actor.
    /// 2) The debt share of the position should be around what we expected it to be based on pending interest.
    /// @param _poolId ID of the lending pool to borrow from. Clamps to either pool 1 (USDC) or pool 2 (AERO).
    /// @param _amountIn Amount of tokens to supply as the base collateral for the position.
    /// @param _amountBorrow Amount of tokens to borrow for the position.
    function createLegacyPosition(uint256 _poolId, uint256 _amountIn, uint256 _amountBorrow) public setCurrentActor {
        _poolId = fl.clamp(_poolId, 1, 2);

        uint256 balance = _poolId == 1 ? usdc.balanceOf(currentActor) : aero.balanceOf(currentActor);
        if (balance == 0) revert FuzzRequireError();
        _amountIn = fl.clamp(_amountIn, 1, balance);
        uint256 minDebt = _poolId == 1 ? USDC_MIN_DEBT : AERO_MIN_DEBT;
        uint256 maxLiquidity = _poolId == 1 ? totalUsdcLiquidity : totalAeroLiquidity;
        _amountBorrow = fl.clamp(_amountBorrow, _amountBorrow > 0 ? minDebt : 0, maxLiquidity);

        // Invest into the new position.
        lendingPool.deposit(_poolId, 0); // @dev We do this to accrue interest. Allows for precision in our calculations.
        Market memory pool = lendingPool.pools()[_poolId];
        uint256 expectedDebtShare =
            _amountBorrow > 0 ? (_amountBorrow * pool.globalDebtShare) / pool.globalDebtValue : 0;
        LendingPool.WorkContext memory ctx = LendingPool.WorkContext({
            poolId: _poolId,
            worker: _poolId == 1 ? legacyUsdcWorker : legacyAeroWorker,
            amountIn: _amountIn,
            loan: _amountBorrow,
            maxReturn: 0,
            data: abi.encode(0)
        });
        STDCHEATS.prank(currentActor);
        lendingPool.doHardWork(0, ctx);
        actorPositionIds[currentActor].push(nextPositionID);

        // Validate current parameters.
        Position memory pos = lendingPool.positions(nextPositionID);
        fl.eq(pos.owner, currentActor, "CREATE-LEGACY-01: New position owner isn't actor");
        fl.eq(pos.debtShare, expectedDebtShare, "CREATE-LEGACY-02: Position debt share did not meet expectations");
        if (_poolId == 1) {
            totalUsdcDebt += _amountBorrow;
        } else {
            totalAeroDebt += _amountBorrow;
        }

        // Validate leverage invariant (shouldn't exceed max 3x leverage).
        (uint256 positionEquity, uint256 positionDebt) = lendingPool.positionInfo(nextPositionID);
        uint256 leverage = positionEquity > positionDebt ? (positionEquity - positionDebt) * 300 : 0;
        fl.gte(leverage, positionEquity * 100, "CREATE-LEGACY-2: Newly created position is overleveraged");
        nextPositionID++;
    }

    /// @notice Fuzz test for the LendingPool facet's `increaseCollateral` method. Adds additional collateral to a legacy position.
    /// @dev Main invariants:
    /// 1) Position shouldn't somehow become overleveraged (very unlikely though).
    /// @param _positionIdx Index (related to actors positions list) of the position to access.
    /// @param _collateralIn Amount of collateral to add the position.
    function increaseLegacyCollateral(uint256 _positionIdx, uint256 _collateralIn) public setCurrentActor {
        _positionIdx = fl.clamp(_positionIdx, 0, actorPositionIds[currentActor].length - 1);
        uint256 posId = actorPositionIds[currentActor][_positionIdx];
        Position memory pos = lendingPool.positions(posId);
        MockToken collat = pos.poolId == 1 ? usdc : aero;
        uint256 balance = collat.balanceOf(currentActor);
        if (balance == 0) revert FuzzRequireError();
        _collateralIn = fl.clamp(_collateralIn, 1, collat.balanceOf(currentActor));

        // Add collateral and check leverage invariant.
        STDCHEATS.prank(currentActor);
        lendingPool.increaseCollateral(posId, _collateralIn, abi.encode(0));
        (uint256 positionEquity, uint256 positionDebt) = lendingPool.positionInfo(posId);
        uint256 leverage = positionEquity > positionDebt ? (positionEquity - positionDebt) * 300 : 0;
        fl.gte(leverage, positionEquity * 100, "INCREASE-LEGACY-01: Position has exceeded 3x leverage");
    }
}
