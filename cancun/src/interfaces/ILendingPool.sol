// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {IWarchest} from "./IWarchest.sol";

/// @notice Interest rate models to calculate from.
enum InterestRateModel {
    TripleSlope
}

/// @notice Parameters used by lending pools.
struct LendingPoolConfig {
    uint16 reservePoolBps;
    uint16 liquidateBps;
    InterestRateModel interestRateModel;
    uint112 minDebtSize;
}

/// @notice Leveraged yield farming position.
struct Position {
    address worker;
    address owner;
    uint32 poolId;
    uint112 debtShare;
}

/// @notice Lending pool configuration and data.
struct Market {
    address underlying;
    uint32 lastAccrueTime;
    uint112 minDebtSize;
    uint16 reservePoolBps;
    uint16 liquidateBps;
    InterestRateModel interestRateModel;
    IWarchest warchest;
    uint112 totalShares;
    uint112 globalDebtValue;
    uint112 globalDebtShare;
    uint112 reservePool;
}

/// @notice Lending Pool Interface
/// @author Chainvisions
/// @notice Interface for the Limestone Lending Pool facet.

interface ILendingPool {
    /// @notice Emitted when tokens are deposited into the lending pool.
    /// @param poolId ID of the lending pool deposited into.
    /// @param user The user who deposited into the pool.
    /// @param amount Amount of tokens deposited by the user into the lending pool.
    event Deposit(uint256 indexed poolId, address indexed user, uint256 amount);

    /// @notice Emitted when tokens are withdrawn from the lending pool.
    /// @param poolId ID of the lending pool withdrawn from.
    /// @param user The user who withrew from the lending pool.
    /// @param totalShares The amount of shares burned.
    /// @param amountUnderlying Amount of underlying tokens withdrawn from the lending pool.
    event Withdraw(uint256 indexed poolId, address indexed user, uint256 totalShares, uint256 amountUnderlying);

    /// @notice Emitted when debt is added to a lending position.
    /// @param poolId ID of the lending pool that was borrowed from.
    /// @param posId ID of the position that borrowed the debt.
    /// @param debtShare The amount of debt shares issued to the position.
    event AddDebt(uint256 indexed poolId, uint256 indexed posId, uint256 debtShare);

    /// @notice Emitted when debt is removed from a lending position.
    /// @param poolId ID of the lending pool that was borrowed from.
    /// @param posId ID of the position that held the debt.
    /// @param debtShare The amount of debt shares removed from the position.
    event RemoveDebt(uint256 indexed poolId, uint256 indexed posId, uint256 debtShare);

    /// @notice Emitted when assets are borrowed from the lending pool to create a new position.
    /// @param poolId ID of the lending pool that the assets were borrowed from.
    /// @param posId ID of the position that borrowed the assets.
    /// @param loan The amount of assets that was lent to the position.
    event Borrow(uint256 indexed poolId, uint256 indexed posId, uint256 loan);

    /// @notice Emitted when collateral is added to a leveraged position to increase its health.
    /// @param poolId ID of the pool that collateral was added towards.
    /// @param posId ID of the position holding the collateral.
    /// @param collateralAdded The amount of collateral added to the position.
    /// @param healthBefore The health of the position before adding the collateral.
    /// @param healthAfter The health of the position after adding the collateral.
    event IncreaseCollateral(
        uint256 indexed poolId,
        uint256 indexed posId,
        uint256 collateralAdded,
        uint256 healthBefore,
        uint256 healthAfter
    );

    /// @notice Emitted when a new lending pool is created.
    /// @param poolId ID of the new lending pool.
    /// @param underlying Underlying token of the lending pool.
    /// @param warchest Warchest contract of the lending pool.
    /// @param parameters Parameters of the lending pool.
    event MarketCreated(
        uint256 indexed poolId, address indexed underlying, address warchest, LendingPoolConfig parameters
    );

    /// @notice Emitted when a position is liquidated.
    /// @param id ID of the position liquidated.
    /// @param liquidator Address of the liquidator that killed the position.
    /// @param prize Reward awarded to the liquidator for liquidating the position.
    /// @param left Amount of assets left after the liquidation.
    event Kill(uint256 indexed id, address indexed liquidator, uint256 prize, uint256 left);

    /// @notice Deposits tokens into the lending pool.
    /// @param _poolId ID of the lending pool to deposit into.
    /// @param _amount Amount of tokens to deposit into the lending pool.
    function deposit(uint256 _poolId, uint256 _amount) external;

    /// @notice Withdraws tokens from the lending pool.
    /// @param _poolId Pool ID of the lending pool to withdraw from.
    /// @param _amount Amount of tokens to withdraw from the lending pool.
    function withdraw(uint256 _poolId, uint256 _amount) external;

    /// @notice Used by workers to access any additional approved assets from a specific user. Used for two sided liquidity provision.
    /// @param _token Token to request from user.
    /// @param _requestedAmount Amount requested to transfer from the user.
    function accessUserAssets(address _token, uint256 _requestedAmount) external;

    /// @notice Fetches `permissionedLiquidation` from storage.
    /// @return Whether or not liquidations are permissioned.
    function permissionedLiquidation() external view returns (bool);

    /// @notice Fetches `authorizedLiquidators[_liquidator]` from storage.
    /// @param _liquidator The liquidator to fetch the authorization for.
    /// @return Whether or not `_liquidator` can liquidate positions.
    function authorizedLiquidators(address _liquidator) external view returns (bool);

    /// @notice Fetches `authorizedKeepers[_keeper]` from storage.
    /// @param _keeper The keeper to fetch the authorization for.
    /// @return Whether or not `_keeper` can reinvest on workers.
    function authorizedKeepers(address _keeper) external view returns (bool);

    /// @notice Fetches the stored data on a position.
    /// @param _posId ID of the position to fetch.
    /// @return The info about the position.
    function positions(uint256 _posId) external view returns (Position memory);

    /// @notice Fetches all of the stored lending pools.
    /// @return An array of all lending pools on the contract.
    function pools() external view returns (Market[] memory);
}
