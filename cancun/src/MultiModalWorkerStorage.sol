// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";
import {SSTORE2} from "solady/src/utils/SSTORE2.sol";
import {IMultiModalWorker, MultiModalPosition} from "./interfaces/IMultiModalWorker.sol";
import {_require, Errors} from "./lib/Errors.sol";
import {Cast} from "./lib/Cast.sol";

struct RewardToken {
    /// @notice Stored rewards per bToken for a specific reward token.
    uint256 rewardPerTokenStored;
    /// @notice Rate at which reward tokens are distributed.
    uint256 rewardRate;
    /// @notice Whether or not a reward token is vested.
    bool lockable;
    /// @notice Reward duration for a specific reward token.
    uint32 duration;
    /// @notice Time when rewards for a specific reward token ends.
    uint32 periodFinish;
    /// @notice The last time reward variables updated for a specific reward token.
    uint32 lastUpdateTime;
}

/// @title Multi Modal Worker Storage
/// @author Chainvisions
/// @notice Diamond storage contract for the Multi Modal Worker.

library MultiModalWorkerStorage {
    using Cast for uint256;

    struct LiquidityPool {
        address pair;
        address rewardPool;
        address[] rewardTokens;
        address token0;
        address token1;
        bool stableswap;
        address router;
    }

    struct Layout {
        /// @notice Storage pointer to the liquidity pool data.
        address poolPointer;
        uint112 totalPositionShares;
        uint32 nextPositionId;
        mapping(uint256 => MultiModalPosition) positions;
    }

    /// @dev Slot location for storing worker state variables at using the diamond storage pattern.
    bytes32 internal constant STORAGE_SLOT = keccak256("limestone.contracts.storage.MultiModalWorker");

    /// @dev Writes the pool metadata to SSTORE2 and stores the pointer.
    /// @param _pool `LiquidityPool` struct to be encoded for storage in SSTORE2.
    // TODO: We could replace SSTORE2 with SSTORE3 to save more gas by only storing a salt as the pointer.
    function _writePoolData(LiquidityPool memory _pool) internal {
        layout().poolPointer = SSTORE2.write(abi.encode(_pool));
    }

    /// @dev Reads the pool metadata from the current SSTORE2 pointer and decodes into a `LiquidityPool` struct.
    /// @return The pool metadata stored at `poolPointer`
    function _readPoolData() internal view returns (LiquidityPool memory) {
        return abi.decode(SSTORE2.read(layout().poolPointer), (LiquidityPool));
    }

    function layout() internal pure returns (Layout storage $) {
        bytes32 slot = STORAGE_SLOT;
        assembly {
            $.slot := slot
        }
    }
}
