// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {MockToken} from "./MockToken.sol";

contract MockAero is MockToken {
    function somethingAero() external {
        _mint(address(69), 100 ether);
    }
}
