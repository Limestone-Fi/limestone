// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {LendingPool} from "../LendingPool.sol";
import {PositionCoordinator} from "../PositionCoordinator.sol";

/// @title Lending Pool Wrapper
/// @author Chainvisions
/// @notice Wrapper for the Limestone lending pool facets for fuzzing.

contract LendingPoolWrapper is LendingPool, PositionCoordinator {
    constructor() LendingPool() PositionCoordinator() {}
}
