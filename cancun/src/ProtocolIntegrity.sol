// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Flags} from "./lib/Flags.sol";
import {LendingPoolStorage} from "./LendingPoolStorage.sol";

/// @title Limestone Protocol Integrity
/// @author Chainvisions
/// @notice A facet that provide guardrails to ensure that the Limestone system remains stable.

contract ProtocolIntegrity {
    using Flags for uint256;

    /// @notice Toggles the borrowing status of the market. Enables pausing/unpausing borrows.
    /// @param _marketId ID of the market to toggle.
    function toggleMarket(uint256 _marketId) external {
        uint256 mask = (1 << (_marketId + Flags.MARKET_OFFSET));
        LendingPoolStorage.layout().flags &= mask;
    }

    /// @notice Toggles the flag for market health checks. Checks for bad debt upon lending/borrowing/etc.
    function toggleMarketHealthcheck() external {
        uint256 mask = (1 << 0);
        LendingPoolStorage.layout().flags &= mask;
    }

    /// @notice Toggles the flag for pausing borrows on bad debt.
    function togglePauseOnBadDebt() external {
        uint256 mask = (1 << 1);
        LendingPoolStorage.layout().flags &= mask;
    }

    /// @notice Toggles the flag for permitting only protocol liquidations during bad debt.
    function toggleProtocolLiquidations() external {
        uint256 mask = (1 << 2);
        LendingPoolStorage.layout().flags &= mask;
    }

    /// @notice Toggles the flag for permissioned liquidations.
    function togglePermissionedLiquidations() external {
        uint256 mask = (1 << 3);
        LendingPoolStorage.layout().flags &= mask;
    }

    function checkMarketHealth() external view returns (bool) {
        return LendingPoolStorage.layout().flags.checkMarketHealth();
    }

    function pauseOnBadDebt() external view returns (bool) {
        return LendingPoolStorage.layout().flags.pauseOnBadDebt();
    }

    function useProtocolLiquidations() external view returns (bool) {
        return LendingPoolStorage.layout().flags.useProtocolLiquidations();
    }

    function permissionedLiquidation() external view returns (bool) {
        return LendingPoolStorage.layout().flags.permissionedLiquidation();
    }

    function pausedMarket(uint256 _marketId) external view returns (bool) {
        return LendingPoolStorage.layout().flags.pausedMarket(_marketId);
    }
}
