// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {CommonBase} from "forge-std/Base.sol";
import {StdCheats} from "forge-std/StdCheats.sol";
import {StdUtils} from "forge-std/StdUtils.sol";
import {LendingPool} from "../../src/LendingPool.sol";

contract LendingPoolHandler is CommonBase, StdCheats, StdUtils {
    LendingPool public pool;

    constructor(LendingPool _pool) {
        pool = _pool;
    }

    function deposit(uint256 _amount) public {}

    function withdraw(uint256 _amount) public {}
}
