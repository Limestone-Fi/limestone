// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

interface IGauge {
    error NotAlive();
    error NotAuthorized();
    error NotVoter();
    error NotTeam();
    error RewardRateTooHigh();
    error ZeroAmount();
    error ZeroRewardRate();

    event Deposit(address indexed from, address indexed to, uint256 amount);
    event Withdraw(address indexed from, uint256 amount);
    event NotifyReward(address indexed from, uint256 amount);
    event ClaimFees(address indexed from, uint256 claimed0, uint256 claimed1);
    event ClaimRewards(address indexed from, uint256 amount);

    /// @notice Address of the pool LP token which is deposited (staked) for rewards
    function stakingToken() external view returns (address);

    /// @notice Address of the token (VELO v2) rewarded to stakers
    function rewardToken() external view returns (address);

    /// @notice Address of the FeesVotingReward contract linked to the gauge
    function feesVotingReward() external view returns (address);

    /// @notice Returns if gauge is linked to a legitimate Velodrome pool
    function isPool() external view returns (bool);

    /// @notice Timestamp end of current rewards period
    function periodFinish() external view returns (uint256);

    /// @notice Current reward rate of rewardToken to distribute per second
    function rewardRate() external view returns (uint256);

    /// @notice Most recent timestamp contract has updated state
    function lastUpdateTime() external view returns (uint256);

    /// @notice Most recent stored value of rewardPerToken
    function rewardPerTokenStored() external view returns (uint256);

    /// @notice Amount of stakingToken deposited for rewards
    function totalSupply() external view returns (uint256);

    /// @notice Get the amount of stakingToken deposited by an account
    function balanceOf(address) external view returns (uint256);

    /// @notice Cached rewardPerTokenStored for an account based on their most recent action
    function userRewardPerTokenPaid(address) external view returns (uint256);

    /// @notice Cached amount of rewardToken earned for an account
    function rewards(address) external view returns (uint256);

    /// @notice View to see the rewardRate given the timestamp of the start of the epoch
    function rewardRateByEpoch(uint256) external view returns (uint256);

    /// @notice Cached amount of fees generated from the Pool linked to the Gauge of token0
    function fees0() external view returns (uint256);

    /// @notice Cached amount of fees generated from the Pool linked to the Gauge of token1
    function fees1() external view returns (uint256);

    /// @notice Get the current reward rate per unit of stakingToken deposited
    function rewardPerToken() external view returns (uint256 _rewardPerToken);

    /// @notice Returns the last time the reward was modified or periodFinish if the reward has ended
    function lastTimeRewardApplicable() external view returns (uint256 _time);

    /// @notice Returns accrued balance to date from last claim / first deposit.
    function earned(address _account) external view returns (uint256 _earned);

    /// @notice Total amount of rewardToken to distribute for the current rewards period
    function left() external view returns (uint256 _left);

    /// @notice Retrieve rewards for an address.
    /// @dev Throws if not called by same address or voter.
    /// @param _account .
    function getReward(address _account) external;

    /// @notice Deposit LP tokens into gauge for msg.sender
    /// @param _amount .
    function deposit(uint256 _amount) external;

    /// @notice Deposit LP tokens into gauge for any user
    /// @param _amount .
    /// @param _recipient Recipient to give balance to
    function deposit(uint256 _amount, address _recipient) external;

    /// @notice Withdraw LP tokens for user
    /// @param _amount .
    function withdraw(uint256 _amount) external;

    /// @dev Notifies gauge of gauge rewards. Assumes gauge reward tokens is 18 decimals.
    ///      If not 18 decimals, rewardRate may have rounding issues.
    function notifyRewardAmount(uint256 amount) external;

    /// @dev Notifies gauge of gauge rewards without distributing its fees.
    ///      Assumes gauge reward tokens is 18 decimals.
    ///      If not 18 decimals, rewardRate may have rounding issues.
    function notifyRewardWithoutClaim(uint256 amount) external;
}

library VelodromeTimeLibrary {
    uint256 internal constant WEEK = 7 days;

    /// @dev Returns start of epoch based on current timestamp
    function epochStart(uint256 timestamp) internal pure returns (uint256) {
        unchecked {
            return timestamp - (timestamp % WEEK);
        }
    }

    /// @dev Returns start of next epoch / end of current epoch
    function epochNext(uint256 timestamp) internal pure returns (uint256) {
        unchecked {
            return timestamp - (timestamp % WEEK) + WEEK;
        }
    }

    /// @dev Returns start of voting window
    function epochVoteStart(uint256 timestamp) internal pure returns (uint256) {
        unchecked {
            return timestamp - (timestamp % WEEK) + 1 hours;
        }
    }

    /// @dev Returns end of voting window / beginning of unrestricted voting window
    function epochVoteEnd(uint256 timestamp) internal pure returns (uint256) {
        unchecked {
            return timestamp - (timestamp % WEEK) + WEEK - 1 hours;
        }
    }
}

import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";
import {IDromePool} from "../../interfaces/external/IDromePool.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/utils/ReentrancyGuard.sol";

/// @title Velodrome V2 Gauge
/// @author veldorome.finance, @figs999, @pegahcarter
/// @notice Gauge contract for distribution of emissions by address
contract MockGauge is IGauge, ReentrancyGuard {
    using SafeERC20 for IERC20;
    /// @inheritdoc IGauge

    address public immutable stakingToken;
    /// @inheritdoc IGauge
    address public immutable rewardToken;
    /// @inheritdoc IGauge
    address public immutable feesVotingReward;

    /// @inheritdoc IGauge
    bool public immutable isPool;

    uint256 internal constant DURATION = 7 days; // rewards are released over 7 days
    uint256 internal constant PRECISION = 10 ** 18;

    /// @inheritdoc IGauge
    uint256 public periodFinish;
    /// @inheritdoc IGauge
    uint256 public rewardRate;
    /// @inheritdoc IGauge
    uint256 public lastUpdateTime;
    /// @inheritdoc IGauge
    uint256 public rewardPerTokenStored;
    /// @inheritdoc IGauge
    uint256 public totalSupply;
    /// @inheritdoc IGauge
    mapping(address => uint256) public balanceOf;
    /// @inheritdoc IGauge
    mapping(address => uint256) public userRewardPerTokenPaid;
    /// @inheritdoc IGauge
    mapping(address => uint256) public rewards;
    /// @inheritdoc IGauge
    mapping(uint256 => uint256) public rewardRateByEpoch;

    /// @inheritdoc IGauge
    uint256 public fees0;
    /// @inheritdoc IGauge
    uint256 public fees1;

    constructor(address _stakingToken, address _feesVotingReward, address _rewardToken, bool _isPool) {
        stakingToken = _stakingToken;
        feesVotingReward = _feesVotingReward;
        rewardToken = _rewardToken;
        isPool = _isPool;
    }

    /// @inheritdoc IGauge
    function rewardPerToken() public view returns (uint256) {
        if (totalSupply == 0) {
            return rewardPerTokenStored;
        }
        return rewardPerTokenStored
            + ((lastTimeRewardApplicable() - lastUpdateTime) * rewardRate * PRECISION) / totalSupply;
    }

    function lastTimeRewardApplicable() public view returns (uint256) {
        return FixedPointMathLib.min(block.timestamp, periodFinish);
    }

    function getReward(address _account) external nonReentrant {
        address sender = msg.sender;
        if (sender != _account) revert NotAuthorized();

        _updateRewards(_account);

        uint256 reward = rewards[_account];
        if (reward > 0) {
            rewards[_account] = 0;
            IERC20(rewardToken).safeTransfer(_account, reward);
            emit ClaimRewards(_account, reward);
        }
    }

    function mutateTotal(uint256 _newTotal) external {
        _updateRewards(address(0));
        totalSupply = _newTotal;
    }

    function earned(address _account) public view returns (uint256) {
        return (balanceOf[_account] * (rewardPerToken() - userRewardPerTokenPaid[_account])) / PRECISION
            + rewards[_account];
    }

    function deposit(uint256 _amount) external {
        _depositFor(_amount, msg.sender);
    }

    function deposit(uint256 _amount, address _recipient) external {
        _depositFor(_amount, _recipient);
    }

    function _depositFor(uint256 _amount, address _recipient) internal nonReentrant {
        if (_amount == 0) revert ZeroAmount();

        address sender = msg.sender;
        _updateRewards(_recipient);

        IERC20(stakingToken).safeTransferFrom(sender, address(this), _amount);
        totalSupply += _amount;
        balanceOf[_recipient] += _amount;

        emit Deposit(sender, _recipient, _amount);
    }

    function withdraw(uint256 _amount) external nonReentrant {
        address sender = msg.sender;

        _updateRewards(sender);

        totalSupply -= _amount;
        balanceOf[sender] -= _amount;
        IERC20(stakingToken).safeTransfer(sender, _amount);

        emit Withdraw(sender, _amount);
    }

    function _updateRewards(address _account) internal {
        rewardPerTokenStored = rewardPerToken();
        lastUpdateTime = lastTimeRewardApplicable();
        rewards[_account] = earned(_account);
        userRewardPerTokenPaid[_account] = rewardPerTokenStored;
    }

    function left() external view returns (uint256) {
        if (block.timestamp >= periodFinish) return 0;
        uint256 _remaining = periodFinish - block.timestamp;
        return _remaining * rewardRate;
    }

    function notifyRewardAmount(uint256 _amount) external nonReentrant {
        address sender = msg.sender;
        if (_amount == 0) revert ZeroAmount();
        _notifyRewardAmount(sender, _amount);
    }

    function notifyRewardWithoutClaim(uint256 _amount) external nonReentrant {
        address sender = msg.sender;
        if (_amount == 0) revert ZeroAmount();
        _notifyRewardAmount(sender, _amount);
    }

    function _notifyRewardAmount(address sender, uint256 _amount) internal {
        rewardPerTokenStored = rewardPerToken();
        uint256 timestamp = block.timestamp;
        uint256 timeUntilNext = VelodromeTimeLibrary.epochNext(timestamp) - timestamp;

        if (timestamp >= periodFinish) {
            IERC20(rewardToken).safeTransferFrom(sender, address(this), _amount);
            rewardRate = _amount / timeUntilNext;
        } else {
            uint256 _remaining = periodFinish - timestamp;
            uint256 _leftover = _remaining * rewardRate;
            IERC20(rewardToken).safeTransferFrom(sender, address(this), _amount);
            rewardRate = (_amount + _leftover) / timeUntilNext;
        }
        rewardRateByEpoch[VelodromeTimeLibrary.epochStart(timestamp)] = rewardRate;
        if (rewardRate == 0) revert ZeroRewardRate();

        // Ensure the provided reward amount is not more than the balance in the contract.
        // This keeps the reward rate in the right range, preventing overflows due to
        // very high values of rewardRate in the earned and rewardsPerToken functions;
        // Reward + leftover must be less than 2^256 / 10^18 to avoid overflow.
        uint256 balance = IERC20(rewardToken).balanceOf(address(this));
        if (rewardRate > balance / timeUntilNext) revert RewardRateTooHigh();

        lastUpdateTime = timestamp;
        periodFinish = timestamp + timeUntilNext;
        emit NotifyReward(sender, _amount);
    }
}
