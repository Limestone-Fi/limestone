// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {FuzzGlobal} from "./FuzzGlobal.sol";
import {FuzzLendingPool} from "./FuzzLendingPool.sol";
import {FuzzPositionCoordinator} from "./FuzzPositionCoordinator.sol";
import {FuzzSetup} from "./FuzzSetup.sol";

/// @title Limestone Fuzz Test
/// @author Chainvisions
/// @notice Limestone's fuzzing campaign, containing tests/invariants for Limestone's diamond facets.

contract Fuzz is
    FuzzLendingPool, // Fuzz tests for the `LendingPool` facet.
    FuzzPositionCoordinator, // Fuzz tests for the `PositionCoordinator` facet.
    FuzzGlobal // Global invariants pertaining to the system lending dynamics.
{
    constructor() payable FuzzSetup() {}
}
