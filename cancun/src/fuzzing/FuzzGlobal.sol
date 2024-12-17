// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Market} from "../interfaces/ILendingPool.sol";
import {FuzzSetup} from "./FuzzSetup.sol";

/// @title Global Invariants
/// @author Chainvisions
/// @notice Mixin for global invariants pertaining to Limestone's fuzzing tests.

contract FuzzGlobal is FuzzSetup {
    /// @notice A function that checks all of the global invariants to save search space.
    function globalInvariants() public {
        _totalSharesVsSupply();
        _warchestSupply();
        _sanityChecks();
    }

    /// @dev An invariant for comparing the total tokens to total shares.
    function _totalSharesVsSupply() internal {
        uint256 totalUsdc = lendingPool.totalTokens(1);
        uint256 totalAero = lendingPool.totalTokens(2);
        // @dev Based on our findings, there may be some very small loss as detected. It is highly negligible though, so we adjust liquidity slightly.
        fl.gte(totalUsdc, totalUsdcLiquidity, "GLOBAL-01: USDC totalTokens is less than liquidity");
        fl.gte(totalAero, totalAeroLiquidity, "GLOBAL-01: AERO totalTokens is less than liquidity");
        fl.gte(totalUsdc, totalUsdcShares, "GLOBAL-02: USDC shares are greater than liquidity");
        fl.gte(totalAero, totalAeroShares, "GLOBAL-02: AERO shares are greater than liquidity");
    }

    /// @dev An invariant for comparing the total supply of the warchests vs tokens.
    function _warchestSupply() internal {
        uint256 usdcSupply = usdc.totalSupply();
        uint256 aeroSupply = aero.totalSupply();
        uint256 usdcChestSupply = warchestUsdc.totalSupply();
        uint256 aeroChestSupply = warchestAero.totalSupply();

        fl.lte(usdcChestSupply, usdcSupply, "GLOBAL-03: USDC warchest supply exceeds USDC supply");
        fl.lte(aeroChestSupply, aeroSupply, "GLOBAL-03: AERO warchest supply exceeds AERO supply");
    }

    /// @dev Some small sanity checks to ensure we don't hit any accidental overflow/underflows (or other gotchas).
    function _sanityChecks() internal {
        Market memory usdcPool = lendingPool.pools()[1];
        Market memory aeroPool = lendingPool.pools()[2];

        // Reserve pool.
        fl.neq(usdcPool.reservePool, type(uint112).max, "GLOBAL-04: USDC reserve pool is max uint112");
        fl.neq(aeroPool.reservePool, type(uint112).max, "GLOBAL-04: AERO reserve pool is max uint112");

        // Debt value.
        fl.gte(usdcPool.globalDebtValue, totalUsdcDebt, "GLOBAL-05: USDC global debt is less than actual debt");
        fl.gte(aeroPool.globalDebtValue, totalAeroDebt, "GLOBAL-05: AERO global debt is less than actual debt");

        // Debt shares.
        if (totalUsdcDebt > 0) fl.neq(usdcPool.globalDebtShare, 0, "GLOBAL-06: USDC debt shares is 0 despite debt");
        if (totalAeroDebt > 0) fl.neq(aeroPool.globalDebtShare, 0, "GLOBAL-06: AERO debt shares is 0 despite debt");
        fl.lte(usdcPool.globalDebtShare, usdcPool.globalDebtValue, "GLOBAL-07: USDC debt shares is somehow gt debt val");
        fl.lte(aeroPool.globalDebtShare, usdcPool.globalDebtValue, "GLOBAL-07: AERO debt shares is somehow gt debt val");
    }
}
