// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

interface IEmissionsController {
    function realizeProfit(address, uint256) external returns (uint256);
}
