// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";
import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";

/// @title Mock Worker
/// @author Chainvisions
/// @notice Mock version of the legacy worker contract for testing.

contract MockWorker {
    using SafeTransferLib for address;

    constructor(address _token) {
        token = _token;
    }

    /// @notice Collateral token.
    address public token;

    function work(uint256 _id, address _user, uint256 _debt, bytes calldata _data) external pure {
        _id;
        _user;
        _debt;
        _data;
    }

    function liquidate(uint256 _id) external pure {
        _id;
    }

    function health(uint256 _id) external view returns (uint256) {
        _id;
        return token.balanceOf(address(this));
    }

    function healthcheck() external pure returns (bool) {
        return true;
    }
}
