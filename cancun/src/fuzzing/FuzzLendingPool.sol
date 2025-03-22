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
}
