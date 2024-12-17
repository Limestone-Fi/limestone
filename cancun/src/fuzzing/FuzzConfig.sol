// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

contract FuzzConfig {
    address internal constant ONCHAIN_USDC_ADDR = 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913;

    address internal constant ONCHAIN_AERO_ADDR = 0x940181a94A35A4569E4529A3CDfB74e38FD98631;

    address internal constant LIMESTONE_DIAMOND = 0x01000006b888030018000000D1e1AA171700fb8D;

    uint256 internal constant SEED_WETH = 50e18;

    uint256 internal constant SEED_USDC = 1_000_000e6;

    uint256 internal constant SEED_AERO = 1_000_000e18;

    uint256 internal constant STARTING_BALANCE = 1_000_000_000e18;

    uint256 internal constant USDC_MIN_DEBT = 50 * 1e6;

    uint256 internal constant AERO_MIN_DEBT = 50e18;

    uint256 internal constant AERO_NOTIFY_AMOUNT = 2_500_000e18;

    /// @dev Snapshot of the USDC reserves for Aerodrome's AERO/USDC pair on Base.
    uint256 internal constant AERO_PAIR_USDC = 73917920908193;

    /// @dev Snapshot of the AERO reserves for Aerodrome's AERO/USDC pair on Base.
    uint256 internal constant AERO_PAIR_AERO = 34723607479672870248699627;

    /// @dev Snapshot of the LP token supply for Aerodrome's AERO/USDC pair on Base.
    uint256 internal constant AERO_PAIR_SUPPLY = 921427908861340014;

    uint256 lastGaugeInjection = block.timestamp;

    uint256 nextPositionID = 1;
    uint256 nextDromePositionId = 1;

    uint256 totalUsdcShares;
    uint256 totalUsdcLiquidity;
    uint256 totalUsdcDebt;
    uint256 usdcLastUpdate;

    uint256 totalAeroShares;
    uint256 totalAeroLiquidity;
    uint256 totalAeroDebt;
    uint256 aeroLastUpdate;

    mapping(address => uint256[]) actorPositionIds;
    mapping(address => uint256[]) actorDromePositions;
}
