// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";

contract MockToken is ERC20("MockToken", "MOCK") {
    function mint(address _to, uint256 _amount) external {
        _mint(_to, _amount);
    }
}
