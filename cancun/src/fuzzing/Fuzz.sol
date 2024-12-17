// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {FuzzGlobal} from "./FuzzGlobal.sol";
import {FuzzLendingPool} from "./FuzzLendingPool.sol";
import {FuzzPositionCoordinator} from "./FuzzPositionCoordinator.sol";
import {FuzzSetup} from "./FuzzSetup.sol";

contract Fuzz is FuzzLendingPool, FuzzPositionCoordinator, FuzzGlobal {
    constructor() payable FuzzSetup() {}
}
