// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {FuzzConfig} from "./FuzzConfig.sol";

contract FuzzActor is FuzzConfig {
    address internal constant ADDRESS_ACTOR1 = address(0x10000);
    address internal constant ADDRESS_ACTOR2 = address(0x20000);
    address internal constant ADDRESS_ACTOR3 = address(0x30000);
    address internal constant ADDRESS_ACTOR4 = address(0x40000);

    address internal constant ADDRESS_OUTSIDER_SEEDER = address(0x50000);

    address[] internal ACTORS = [ADDRESS_ACTOR1, ADDRESS_ACTOR2, ADDRESS_ACTOR3, ADDRESS_ACTOR4];

    address internal currentActor;

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
