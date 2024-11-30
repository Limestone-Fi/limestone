// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IWorker {
    /// @notice Work on a (potentially new) position. Optionally send tokens back to the lending pool.
    function work(uint256 _id, address _user, uint256 _debt, bytes calldata _data) external;

    /// @notice Harvests and reinvests pending rewards.
    function reinvest() external;

    /// @notice Fetches the health of a position.
    /// @param _id ID of the position to fetch the health of.
    function health(uint256 _id) external view returns (uint256);

    function healthcheck() external view returns (bool);

    /// @notice Liquidates a position and converts into collateral.
    /// @param _id ID of the position liquidated.
    function liquidate(uint256 _id) external;

    /// @notice Emitted when harvested yields are reinvested on the worker.
    /// @param caller Address of the reinvestor.
    /// @param reward Rewards claimed from the harvest.
    /// @param bounty Bounty given to the reinvestor.
    event Reinvest(address indexed caller, uint256 reward, uint256 bounty);

    /// @notice Emitted when shares are added to the worker.
    /// @param id ID of the position that shares were added to.
    /// @param share Amount of shares added.
    event AddShare(uint256 indexed id, uint256 share);

    /// @notice Emitted when shares are removed from the worker.
    /// @param id ID of the position that shares were removed from.
    /// @param share Amount of shares removed.
    event RemoveShare(uint256 indexed id, uint256 share);

    /// @notice Emitted when a position is liquidated.
    /// @param id ID of the position liquidated.
    /// @param amount Amount of tokens liquidated from the position.
    event Liquidate(uint256 indexed id, uint256 amount);

    /// @notice Emitted when rewards are paid out to a vault user.
    /// @param user User that the rewards are paid out to.
    /// @param rewardToken Reward token paid out to the user.
    /// @param amount Amount of `rewardToken` paid out to the user.
    event RewardPaid(address indexed user, address indexed rewardToken, uint256 amount);

    /// @notice Emitted when rewards are injected into the vault.
    /// @param rewardToken Reward token injected into the vault.
    /// @param rewardAmount Amount of `rewardToken` injected.
    event RewardInjection(address indexed rewardToken, uint256 rewardAmount);
}
