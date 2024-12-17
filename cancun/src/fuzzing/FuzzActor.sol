// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {FuzzConfig} from "./FuzzConfig.sol";

/// @title Fuzz Actor
/// @author Chainvisions
/// @notice Mixin containing the setup for fuzz test actors.

contract FuzzActor is FuzzConfig {
    /// @dev Address for actor 1.
    address internal constant ADDRESS_ACTOR1 = address(0x10000);
    /// @dev Address for actor 2.
    address internal constant ADDRESS_ACTOR2 = address(0x20000);
    /// @dev Address for actor 3.
    address internal constant ADDRESS_ACTOR3 = address(0x30000);
    /// @dev Address for actor 4.
    address internal constant ADDRESS_ACTOR4 = address(0x40000);

    /// @dev Address responsible for seeding the lending pool.
    address internal constant ADDRESS_OUTSIDER_SEEDER = address(0x50000);

    /// @dev List of actors used for the fuzzing test.
    address[] internal ACTORS = [ADDRESS_ACTOR1, ADDRESS_ACTOR2, ADDRESS_ACTOR3, ADDRESS_ACTOR4];

    /// @dev The current active actor.
    address internal currentActor;

    /// @dev Debug toggle for disabling setting the actor.
    bool internal constant DEBUG_TOGGLE_SET_ACTOR = true;

    /// @notice Modifier storing `msg.sender` for the duration of the function call.
    modifier setCurrentActor() {
        address previousActor = currentActor;
        if (DEBUG_TOGGLE_SET_ACTOR) {
            currentActor = msg.sender;
        }

        _;

        if (DEBUG_TOGGLE_SET_ACTOR) {
            currentActor = previousActor;
        }
    }
}
