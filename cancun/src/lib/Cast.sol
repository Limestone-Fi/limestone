// SPDX-License-Identifier: Unlicense
pragma solidity 0.8.28;

/// @title Type casting library
/// @author Chainvisions
/// @notice Library for casting to different data types.

library Cast {
    /// @notice Casts a `uint256` into a `uint8`.
    /// @param _from `uint256` value to cast from.
    /// @return End `uint8` value from casting.
    function u8(uint256 _from) internal pure returns (uint8) {
        require(_from < 1 << 8);
        return uint8(_from);
    }

    /// @notice Casts a `uint256` into a `uint16`.
    /// @param _from `uint256` value to cast from.
    /// @return End `uint16` value from casting.
    function u16(uint256 _from) internal pure returns (uint16) {
        require(_from < 1 << 16);
        return uint16(_from);
    }

    /// @notice Casts a `uint256` into a `uint32`.
    /// @param _from `uint256` value to cast from.
    /// @return End `uint32` value from casting.
    function u32(uint256 _from) internal pure returns (uint32) {
        require(_from < 1 << 32);
        return uint32(_from);
    }

    /// @notice Casts a `uint256` into a `uint64`.
    /// @param _from `uint256` value to cast from.
    /// @return End `uint64` value from casting.
    function u64(uint256 _from) internal pure returns (uint64) {
        require(_from < 1 << 64);
        return uint64(_from);
    }

    /// @notice Casts a `uint256` into a `uint112`.
    /// @param _from `uint256` value to cast from.
    /// @return End `uint112` value from casting.
    function u112(uint256 _from) internal pure returns (uint112) {
        require(_from < 1 << 112);
        return uint112(_from);
    }

    /// @notice Casts a `uint256` into a `uint128`.
    /// @param _from `uint256` value to cast from.
    /// @return End `uint128` value from casting.
    function u128(uint256 _from) internal pure returns (uint128) {
        require(_from < 1 << 128);
        return uint128(_from);
    }
}
