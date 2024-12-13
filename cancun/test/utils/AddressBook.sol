// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

/// @title Address Book
/// @author Chainvisions
/// @notice An address book used in tracking onchain addresses.

abstract contract AddressBook {
    /// @dev OP stack WETH address.
    address public constant WETH = 0x4200000000000000000000000000000000000006;

    address public constant AERO_ROUTER = 0xcF77a3Ba9A5CA399B7c97c74d54e5b1Beb874E43;

    address public constant BASE_USDC = 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913;

    address public constant AERO = 0x940181a94A35A4569E4529A3CDfB74e38FD98631;

    address public constant AERO_USDC_GAUGE = 0x4F09bAb2f0E15e2A078A227FE1537665F55b8360;

    address public constant AERO_VOTER = 0x16613524e02ad97eDfeF371bC883F2F5d6C480A5;

    address public constant ONETICKWARRIOR = 0x0000000558bbA7447AdF475E3584dbEC177041Ad;
}
