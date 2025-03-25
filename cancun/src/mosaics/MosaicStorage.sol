// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title Mosaic Storage
/// @author Chainvisions
/// @notice Storage contract for Limestone mosaics.

library MosaicStorage {
    struct Layout {
        address mosaic;
    }

    bytes32 internal constant STORAGE_SLOT = keccak256("limestone.contracts.storage.Mosaic");

    function layout() internal pure returns (Layout storage l) {
        bytes32 slot = STORAGE_SLOT;
        assembly {
            l.slot := slot
        }
    }
}
