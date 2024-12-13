// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {IOwnable} from "@solidstate/access/ownable/IOwnable.sol";
import {ILendingPool} from "./ILendingPool.sol";
import {IPositionCoordinator} from "./IPositionCoordinator.sol";

/// @title Limestone Diamond Interface
/// @author Chainvisions
/// @notice An interface for Limestone's diamond proxy that includes interfaces for each facet.

interface ILimeDiamond is ILendingPool, IPositionCoordinator, IOwnable {}
