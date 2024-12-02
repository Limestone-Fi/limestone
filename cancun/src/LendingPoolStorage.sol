// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {
    ILendingPool,
    IWarchest,
    Market,
    Position,
    LendingPoolConfig,
    InterestRateModel,
    AuthType
} from "./interfaces/ILendingPool.sol";
import {IWarchest} from "./interfaces/IWarchest.sol";

/// @title Lending Pool Storage
/// @author Chainvisions
/// @notice Diamond storage contract for the Limestone Lending Pool.

library LendingPoolStorage {
    /// @notice Struct for storing the execution scope.
    struct ExecScope {
        /// @notice ID of the position currently in execution.
        uint32 positionId;
        /// @notice The worker that is currently being executed.
        address worker;
    }

    struct WorkerDebtParams {
        uint16 authorizedPoolId;
        uint16 workFactor;
        uint16 killFactor;
        bool borrowable;
    }

    struct Layout {
        /// @notice Whether or not liquidation is permissioned.
        bool permissionedLiquidation;
        /// @notice ID for the next LYF position.
        uint256 nextPositionID;
        /// @notice All lending pools for the protocol.
        Market[] pools;
        /// @notice Leveraged yield farming positions.
        mapping(uint256 posId => Position) positions;
        /// @notice Debt parameters for a specific worker.
        mapping(address worker => WorkerDebtParams) workerDebtParams;
        /// @notice Keepers that are authorized to reinvest on workers.
        mapping(address keeper => bool) authorizedKeepers;
        /// @notice Liquidators that are authorized to liquidate positions (if permisioned liquidations are enabled).
        mapping(address liquidator => bool) authorizedLiquidators;
        /// @notice Contracts authorized to borrow tokens via delegated borrowing.
        mapping(address borrower => bool) authorizedContractBorrowers;
        /// @notice The amount of debt borrowed via `borrowDelegated` held by a specific contract.
        mapping(address debtHolder => mapping(uint256 poolId => uint112 debtShares)) delegatedDebt;
    }

    /// @dev Slot location for storing lending pool state variables at using the diamond storage pattern.
    bytes32 internal constant STORAGE_SLOT = keccak256("limestone.contracts.storage.LendingPool");

    /// @dev Slot location used for storing the execution scope in transient storage.
    bytes32 internal constant EXECUTION_SCOPE_SLOT = keccak256("limestone.contracts.transient.LendingPool.ExecScope");

    function layout() internal pure returns (Layout storage l) {
        bytes32 slot = STORAGE_SLOT;
        assembly {
            l.slot := slot
        }
    }
}
