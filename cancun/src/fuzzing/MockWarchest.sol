// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";
import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";

/// @title Mock Warchest.
/// @author Chainvisions
/// @notice A mock version of the warchest contract for testing.

contract MockWarchest is ERC20("MockWarchest", "CHEST") {
    using SafeTransferLib for address;

    constructor(address _token) {
        token = _token;
    }

    /// @notice Collateral token.
    address public token;

    function mint(address _to, uint256 _amount) external {
        _mint(_to, _amount);
    }

    function burn(address _from, uint256 _amount) external {
        _burn(_from, _amount);
    }

    function withdrawReserves(address _to, uint256 _amount) external {
        token.safeTransfer(_to, _amount);
    }

    function underlyingBalanceWithInvestment() external view returns (uint256) {
        return token.balanceOf(address(this));
    }
}
