// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

interface IStrategy {
    function unsalvagableTokens(address tokens) external view returns (bool);

    function governance() external view returns (address);

    function controller() external view returns (address);

    function underlying() external view returns (address);

    function vault() external view returns (address);

    function withdrawAllToVault() external;

    function withdrawToVault(uint256 amount) external;

    function investedUnderlyingBalance() external view returns (uint256); // itsNotMuch()

    function pendingYield() external view returns (uint256[] memory);

    function salvage(address recipient, address token, uint256 amount) external;

    function doHardWork() external;

    function depositArbCheck() external view returns (bool);
}
