// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title Fuzz Config
/// @author Chainvisions
/// @notice Mixin containing all of the config constants and ghost variables related to Limestone fuzz tests.

contract FuzzConfig {
    /// @dev Onchain USDC address (Base) that we deploy our mock token at (to mimic an actual environment)
    address internal constant ONCHAIN_USDC_ADDR = 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913;

    /// @dev Onchain AERO address that we deploy our mock token at (to mimic an actual deployment)
    address internal constant ONCHAIN_AERO_ADDR = 0x940181a94A35A4569E4529A3CDfB74e38FD98631;

    /// @dev Onchain Limestone diamond address that we deploy the lending pool at.
    address internal constant LIMESTONE_DIAMOND = 0x01000006b888030018000000D1e1AA171700fb8D;

    /// @dev Amount of WETH to use to seed the lending pool.
    uint256 internal constant SEED_WETH = 50e18;

    /// @dev Amount of USDC to use to seed the lending pool.
    uint256 internal constant SEED_USDC = 1_000_000e6;

    /// @dev Amount of AERO to use to seed the lending pool.
    uint256 internal constant SEED_AERO = 1_000_000e18;

    /// @dev The starting balance for each actor in the fuzzing environment.
    uint256 internal constant STARTING_BALANCE = 1_000_000_000e18;

    /// @dev Min debt required for borrowing USDC.
    uint256 internal constant USDC_MIN_DEBT = 50 * 1e6;

    /// @dev Min debt required for borrowing AERO.
    uint256 internal constant AERO_MIN_DEBT = 50e18;

    /// @dev Amount of AERO to notify to the gauge for simulating rewards.
    uint256 internal constant AERO_NOTIFY_AMOUNT = 2_500_000e18;

    /// @dev Snapshot of the USDC reserves for Aerodrome's AERO/USDC pair on Base.
    uint256 internal constant AERO_PAIR_USDC = 73917920908193;

    /// @dev Snapshot of the AERO reserves for Aerodrome's AERO/USDC pair on Base.
    uint256 internal constant AERO_PAIR_AERO = 34723607479672870248699627;

    /// @dev Snapshot of the LP token supply for Aerodrome's AERO/USDC pair on Base.
    uint256 internal constant AERO_PAIR_SUPPLY = 921427908861340014;

    /// @dev Timestamp of the last time that rewards were notified to a gauge.
    uint256 lastGaugeInjection = block.timestamp;

    /// @dev Ghost variable for tracking the next legacy position position ID.
    uint256 nextPositionID = 1;
    /// @dev Ghost variable for tracking the next AERO/USDC worker position ID.
    uint256 nextDromePositionId = 1;

    /// @dev Ghost variable for tracking the total amount of USDC shares minted.
    uint256 totalUsdcShares;
    /// @dev Ghost variable for tracking the total amount of USDC deposited into the lending pool.
    uint256 totalUsdcLiquidity;
    /// @dev Ghost variable for tracking the total amount of USDC Borrowed from the lending pool.
    uint256 totalUsdcDebt;
    /// @dev Ghost variable for tracking the last time that USDC debt was recorded (for calculating interest manually).
    uint256 usdcLastUpdate;

    /// @dev Ghost variable for tracking the total amount of AERO shares minted.
    uint256 totalAeroShares;
    /// @dev Ghost variable for tracking the total amount of AERO deposited into the lending pool.
    uint256 totalAeroLiquidity;
    /// @dev Ghost variable for tracking the total amount of AERO Borrowed from the lending pool.
    uint256 totalAeroDebt;
    /// @dev Ghost variable for tracking the last time that AERO debt was recorded (for calculating interest manually).
    uint256 aeroLastUpdate;

    /// @dev Ghost variable for tracking all of the legacy positions held by each actor.
    mapping(address => uint256[]) actorPositionIds;
    /// @dev Ghost variable for tracking all of the AERO/USDC worker positons held by each actor.
    mapping(address => uint256[]) actorDromePositions;
}
