// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title Flags
/// @author Chainvisions
/// @notice A helper library for parsing Limestone bitmap flags.

library Flags {
    /// @dev Bits offset used to track where global parameters end and market parameters begin.
    uint256 internal constant MARKET_OFFSET = 4;

    function checkMarketHealth(uint256 _bitmap) internal pure returns (bool) {
        return (_bitmap & (1 << 0)) > 0;
    }

    function pauseOnBadDebt(uint256 _bitmap) internal pure returns (bool) {
        return (_bitmap & (1 << 1)) > 0;
    }

    function useProtocolLiquidations(uint256 _bitmap) internal pure returns (bool) {
        return (_bitmap & (1 << 2)) > 0;
    }

    function permissionedLiquidation(uint256 _bitmap) internal pure returns (bool) {
        return (_bitmap & (1 << 3)) > 0;
    }

    function pausedMarket(uint256 _bitmap, uint256 _poolId) internal pure returns (bool) {
        return _readFromIndexWithOffset(_bitmap, _poolId);
    }

    function _readFromIndexWithOffset(uint256 _bitmap, uint256 _idx) private pure returns (bool) {
        return (_bitmap & (1 << (_idx + MARKET_OFFSET))) > 0;
    }
}
