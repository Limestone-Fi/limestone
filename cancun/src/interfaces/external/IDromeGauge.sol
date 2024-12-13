// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

interface IDromeGauge {
    function deposit(uint256) external;
    function deposit(uint256, address) external;
    function withdraw(uint256) external;
    function getReward(address) external;
    function notifyRewardAmount(uint256) external;
    function balanceOf(address) external view returns (uint256);
}
