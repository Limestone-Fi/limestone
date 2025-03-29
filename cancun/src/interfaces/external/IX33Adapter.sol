// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

interface Ix33Adapter {
    event NewOperator(address _oldOperator, address _newOperator);

    /// @notice submits the optimized votes for the epoch
    function submitVotes(address[] calldata _pools, uint256[] calldata _weights) external;

    /// @notice swap function using aggregators to process rewards into SHADOW
    function swapIncentiveViaAggregator(IX33.AggregatorParams calldata _params) external;

    /// @notice claims the rebase accrued to x33
    function claimRebase() external;

    /// @notice compounds any existing SHADOW within the contract
    function compound() external;

    /// @notice redeems x33 tokens in the x33 contract then deposit it in VoteModule
    function rebaseX33() external;

    /// @notice direct claim
    function claimIncentives(address[] calldata _feeDistributors, address[][] calldata _tokens) external;

    /// @notice allows the operator to unlock the contract for the current period
    function unlock() external;

    /// @notice transfers the operator via accesshub
    function transferOperator(address _newOperator) external;

    /// @notice rescues stuck tokens
    function rescue(address token) external;

    /// @notice address of the current operator
    function operator() external view returns (address);

    /// @notice accessHub address
    function accessHub() external view returns (address);

    function deposit(uint256 assets, address receiver) external returns (uint256 shares);

    function mint(uint256 shares, address receiver) external returns (uint256 assets);

    /// @dev this isn't exactly compilant with ERC4626 because we can't maniulate approvals on x33 through here
    /// so we do not allow withdrawing from another user other than the msg.sender
    function withdraw(uint256 assets, address receiver) external returns (uint256 shares);

    /// @dev this isn't exactly compilant with ERC4626 because we can't maniulate approvals on x33 through here
    /// so we do not allow withdrawing from another user other than the msg.sender
    function redeem(uint256 shares, address receiver) external returns (uint256 assets);
}
