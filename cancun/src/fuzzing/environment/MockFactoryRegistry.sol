// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

interface IFactoryRegistry {
    error FallbackFactory();
    error InvalidFactoriesToPoolFactory();
    error PathAlreadyApproved();
    error PathNotApproved();

    event Approve(address indexed poolFactory, address indexed votingRewardsFactory, address indexed gaugeFactory);
    event Unapprove(address indexed poolFactory, address indexed votingRewardsFactory, address indexed gaugeFactory);
    event SetManagedRewardsFactory(address indexed _newRewardsFactory);

    /// @notice Approve a set of factories used in Velodrome Protocol.
    ///         Router.sol is able to swap any poolFactories currently approved.
    ///         Cannot approve address(0) factories.
    ///         Cannot aprove path that is already approved.
    ///         Each poolFactory has one unique set and maintains state.  In the case a poolFactory is unapproved
    ///             and then re-approved, the same set of factories must be used.  In other words, you cannot overwrite
    ///             the factories tied to a poolFactory address.
    ///         VotingRewardsFactories and GaugeFactories may use the same address across multiple poolFactories.
    /// @dev Callable by onlyOwner
    /// @param poolFactory .
    /// @param votingRewardsFactory .
    /// @param gaugeFactory .
    function approve(address poolFactory, address votingRewardsFactory, address gaugeFactory) external;

    /// @notice Unapprove a set of factories used in Velodrome Protocol.
    ///         While a poolFactory is unapproved, Router.sol cannot swap with pools made from the corresponding factory
    ///         Can only unapprove an approved path.
    ///         Cannot unapprove the fallback path (core v2 factories).
    /// @dev Callable by onlyOwner
    /// @param poolFactory .
    function unapprove(address poolFactory) external;

    /// @notice Factory to create free and locked rewards for a managed veNFT
    function managedRewardsFactory() external view returns (address);

    /// @notice Set the rewards factory address
    /// @dev Callable by onlyOwner
    /// @param _newManagedRewardsFactory address of new managedRewardsFactory
    function setManagedRewardsFactory(address _newManagedRewardsFactory) external;

    /// @notice Get the factories correlated to a poolFactory.
    ///         Once set, this can never be modified.
    ///         Returns the correlated factories even after an approved poolFactory is unapproved.
    function factoriesToPoolFactory(address poolFactory)
        external
        view
        returns (address votingRewardsFactory, address gaugeFactory);

    /// @notice Get all PoolFactories approved by the registry
    /// @dev The same PoolFactory address cannot be used twice
    /// @return Array of PoolFactory addresses
    function poolFactories() external view returns (address[] memory);

    /// @notice Check if a PoolFactory is approved within the factory registry.  Router uses this method to
    ///         ensure a pool swapped from is approved.
    /// @param poolFactory .
    /// @return True if PoolFactory is approved, else false
    function isPoolFactoryApproved(address poolFactory) external view returns (bool);

    /// @notice Get the length of the poolFactories array
    function poolFactoriesLength() external view returns (uint256);
}

import {Ownable} from "@openzeppelin/access/Ownable.sol";
import {EnumerableSet} from "@openzeppelin/utils/structs/EnumerableSet.sol";

/// @title Velodrome V2 Factory Registry
/// @author Carter Carlson (@pegahcarter)
/// @notice Velodrome V2 Factory Registry to swap and create gauges
contract MockFactoryRegistry is IFactoryRegistry, Ownable {
    using EnumerableSet for EnumerableSet.AddressSet;

    /// @dev factory to create free and locked rewards for a managed veNFT
    address private _managedRewardsFactory;

    /// @dev Velodrome protocol will always have a usable poolFactory, votingRewardsFactory, and gaugeFactory.  The votingRewardsFactory
    // and gaugeFactory are defined to the poolFactory which can never be removed
    address public immutable fallbackPoolFactory;

    /// @dev Array of poolFactories used to create a gauge and votingRewards
    EnumerableSet.AddressSet private _poolFactories;

    struct FactoriesToPoolFactory {
        address votingRewardsFactory;
        address gaugeFactory;
    }
    /// @dev the factories linked to the poolFactory

    mapping(address => FactoriesToPoolFactory) private _factoriesToPoolsFactory;

    constructor(
        address _fallbackPoolFactory,
        address _fallbackVotingRewardsFactory,
        address _fallbackGaugeFactory,
        address _newManagedRewardsFactory
    ) Ownable(msg.sender) {
        fallbackPoolFactory = _fallbackPoolFactory;

        approve(_fallbackPoolFactory, _fallbackVotingRewardsFactory, _fallbackGaugeFactory);
        setManagedRewardsFactory(_newManagedRewardsFactory);
    }

    /// @inheritdoc IFactoryRegistry
    function approve(address poolFactory, address votingRewardsFactory, address gaugeFactory) public onlyOwner {
        if (_poolFactories.contains(poolFactory)) revert PathAlreadyApproved();

        FactoriesToPoolFactory memory usedFactories = _factoriesToPoolsFactory[poolFactory];

        // If the poolFactory *has not* been approved before, can approve any gauge/votingRewards factory
        // Only one check is sufficient
        if (usedFactories.votingRewardsFactory == address(0)) {
            _factoriesToPoolsFactory[poolFactory] = FactoriesToPoolFactory(votingRewardsFactory, gaugeFactory);
        } else {
            // If the poolFactory *has* been approved before, can only approve the same used gauge/votingRewards factory to
            //     to maintain state within Voter
            if (
                votingRewardsFactory != usedFactories.votingRewardsFactory || gaugeFactory != usedFactories.gaugeFactory
            ) revert InvalidFactoriesToPoolFactory();
        }

        _poolFactories.add(poolFactory);
        emit Approve(poolFactory, votingRewardsFactory, gaugeFactory);
    }

    /// @inheritdoc IFactoryRegistry
    function unapprove(address poolFactory) external onlyOwner {
        if (poolFactory == fallbackPoolFactory) revert FallbackFactory();
        if (!_poolFactories.contains(poolFactory)) revert PathNotApproved();
        _poolFactories.remove(poolFactory);
        (address votingRewardsFactory, address gaugeFactory) = factoriesToPoolFactory(poolFactory);
        emit Unapprove(poolFactory, votingRewardsFactory, gaugeFactory);
    }

    /// @inheritdoc IFactoryRegistry
    function setManagedRewardsFactory(address _newManagedRewardsFactory) public onlyOwner {
        _managedRewardsFactory = _newManagedRewardsFactory;
        emit SetManagedRewardsFactory(_newManagedRewardsFactory);
    }

    /// @inheritdoc IFactoryRegistry
    function managedRewardsFactory() external view returns (address) {
        return _managedRewardsFactory;
    }

    /// @inheritdoc IFactoryRegistry
    function factoriesToPoolFactory(address poolFactory)
        public
        view
        returns (address votingRewardsFactory, address gaugeFactory)
    {
        FactoriesToPoolFactory memory f = _factoriesToPoolsFactory[poolFactory];
        votingRewardsFactory = f.votingRewardsFactory;
        gaugeFactory = f.gaugeFactory;
    }

    /// @inheritdoc IFactoryRegistry
    function poolFactories() external view returns (address[] memory) {
        return _poolFactories.values();
    }

    /// @inheritdoc IFactoryRegistry
    function isPoolFactoryApproved(address poolFactory) external view returns (bool) {
        return _poolFactories.contains(poolFactory);
    }

    /// @inheritdoc IFactoryRegistry
    function poolFactoriesLength() external view returns (uint256) {
        return _poolFactories.length();
    }
}
