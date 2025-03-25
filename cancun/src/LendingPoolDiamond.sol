// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SolidStateDiamond} from "@solidstate/proxy/diamond/SolidStateDiamond.sol";
import {MosaicStorage} from "./mosaics/MosaicStorage.sol";

/// @title Lending Pool Diamond
/// @author Chainvisions
/// @notice Diamond proxy for Limestone's lending pool.

contract LendingPoolDiamond is SolidStateDiamond {
    /// @notice Diamond constructor.
    constructor() {
        // Since we assume that the deployer is a factory, we use tx.origin instead.
        _setOwner(tx.origin);
    }

    /// @notice Dummy function used for Etherscan to show all methods of the diamond.
    /// @return The current mosaic contract where all of the diamond's methods are at.
    function implementation() external view returns (address) {
        return MosaicStorage.layout().mosaic;
    }

    /// @notice Updates the current mosaic contract to a new interface.
    /// @param _mosaic New mosaic contract to act as the proxy's interface.
    function setMosaic(address _mosaic) external onlyOwner {
        MosaicStorage.layout().mosaic = _mosaic;
    }
}
