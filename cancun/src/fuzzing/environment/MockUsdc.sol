// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {MockToken} from "./MockToken.sol";

contract MockUsdc is MockToken {
    function somethingDollar() external {
        _mint(address(1), 59 ether);
    }
}
