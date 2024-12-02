// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.0;

// solhint-disable

/**
 * @dev Reverts if `condition` is false, with a revert reason containing `errorCode`. Only codes up to 999 are
 * supported.
 */
function _require(bool condition, uint256 errorCode) pure {
    if (!condition) _revert(errorCode);
}

/**
 * @dev Reverts with a revert reason containing `errorCode`. Only codes up to 999 are supported.
 */
function _revert(uint256 errorCode) pure {
    // We're going to dynamically create a revert string based on the error code, with the following format:
    // 'BEL#{errorCode}'
    // where the code is left-padded with zeroes to three digits (so they range from 000 to 999).
    //
    // We don't have revert strings embedded in the contract to save bytecode size: it takes much less space to store a
    // number (8 to 16 bits) than the individual string characters.
    //
    // The dynamic string creation algorithm that follows could be implemented in Solidity, but assembly allows for a
    // much denser implementation, again saving bytecode size. Given this function unconditionally reverts, this is a
    // safe place to rely on it without worrying about how its usage might affect e.g. memory contents.
    assembly {
        // First, we need to compute the ASCII representation of the error code. We assume that it is in the 0-999
        // range, so we only need to convert three digits. To convert the digits to ASCII, we add 0x30, the value for
        // the '0' character.

        let units := add(mod(errorCode, 10), 0x30)

        errorCode := div(errorCode, 10)
        let tenths := add(mod(errorCode, 10), 0x30)

        errorCode := div(errorCode, 10)
        let hundreds := add(mod(errorCode, 10), 0x30)

        // With the individual characters, we can now construct the full string. The "BEL#" part is a known constant
        // (0x42454C23): we simply shift this by 24 (to provide space for the 3 bytes of the error code), and add the
        // characters to it, each shifted by a multiple of 8.
        // The revert reason is then shifted left by 200 bits (256 minus the length of the string, 7 characters * 8 bits
        // per character = 56) to locate it in the most significant part of the 256 slot (the beginning of a byte
        // array).

        let revertReason := shl(200, add(0x494d0000000000, add(add(units, shl(8, tenths)), shl(16, hundreds))))

        // We can now encode the reason in memory, which can be safely overwritten as we're about to revert. The encoded
        // message will have the following layout:
        // [ revert reason identifier ] [ string location offset ] [ string length ] [ string contents ]

        // The Solidity revert reason identifier is 0x08c739a0, the function selector of the Error(string) function. We
        // also write zeroes to the next 28 bytes of memory, but those are about to be overwritten.
        mstore(0x0, 0x08c379a000000000000000000000000000000000000000000000000000000000)
        // Next is the offset to the location of the string, which will be placed immediately after (20 bytes away).
        mstore(0x04, 0x0000000000000000000000000000000000000000000000000000000000000020)
        // The string length is fixed: 7 characters.
        mstore(0x24, 7)
        // Finally, the string itself is stored.
        mstore(0x44, revertReason)

        // Even if the string is only 7 bytes long, we need to return a full 32 byte slot containing it. The length of
        // the encoded message is therefore 4 + 32 + 32 + 32 = 100.
        revert(0, 100)
    }
}

/// @title "Smolmax" Errors Library
/// @author Chainvisions
/// @author Forked and modified from Balancer (https://github.com/balancer-labs/balancer-v2-monorepo/blob/master/pkg/solidity-utils/contracts/helpers/BalancerErrors.sol)
/// @notice Library for efficiently handling errors on Beluga contracts with reduced bytecode size additions.

library Errors {
    // Factory
    uint256 internal constant LENDING_COMPONENT_ALREADY_EXISTS = 0;
    uint256 internal constant LENDING_POOL_ALREADY_INITIALIZED = 1;
    uint256 internal constant COLLATERAL_NOT_CREATED = 2;
    uint256 internal constant BORROWABLE_ZERO_NOT_CREATED = 3;
    uint256 internal constant BORROWABLE_ONE_NOT_CREATED = 4;
    uint256 internal constant UNAUTHORIZED_CALL = 5;

    // Borrowable
    uint256 internal constant INSUFFICIENT_CASH = 6;
    uint256 internal constant INSUFFICIENT_LIQUIDITY = 7;

    // Collateral
    uint256 internal constant PRICE_CALCULATION_ERROR = 8;
    uint256 internal constant INVALID_BORROWABLE = 9;
    uint256 internal constant INSUFFICIENT_SHORTFALL = 10;
    uint256 internal constant INSUFFICIENT_REDEEM_TOKENS = 11;
    uint256 internal constant LIQUIDATING_TOO_MUCH = 12;

    // Collateral/Borrowable setter
    uint256 internal constant INVALID_SETTING = 13;

    // Borrowable Allowance
    uint256 internal constant BORROW_NOT_ALLOWED = 14;

    // Borrowable Storage
    uint256 internal constant SAFE112 = 15;

    // Smolmax ERC20
    uint256 internal constant EXPIRED = 16;
    uint256 internal constant INVALID_SIGNATURE = 17;

    // Pool token
    uint256 internal constant FACTORY_ALREADY_SET = 18;
    uint256 internal constant MINT_AMOUNT_ZERO = 19;
    uint256 internal constant REDEEM_AMOUNT_ZERO = 20;
    uint256 internal constant TRANSFER_FAILED = 21;
    uint256 internal constant REENTERED = 22;

    // Router
    uint256 internal constant NOT_WETH = 23;

    // Emissions Controller
    uint256 internal constant VAULT_NOT_EMITTABLE = 40;
    uint256 internal constant MALFORMED_VOTE = 41;
    uint256 internal constant EPOCH_NOT_COMPLETE = 42;

    // veLIME
    uint256 internal constant QUERIED_NONEXISTENT_TOKEN = 80;
    uint256 internal constant STALE = 81;
    uint256 internal constant ATTACHED = 82;
    uint256 internal constant VENFT_NOT_AUTHORIZED = 83;
    uint256 internal constant NO_LOCK_FOUND = 84;
    uint256 internal constant EXPIRED_LOCK = 85;
    uint256 internal constant CANNOT_LOCK_BACKWARDS = 86;
    uint256 internal constant FOUR_YEARS_MAX = 87;
    uint256 internal constant LOCK_NOT_EXPIRED = 88;
    uint256 internal constant TOO_MANY_DELEGATES = 89;
    uint256 internal constant DBSIG_INVALID_SIG = 90;
    uint256 internal constant DBSIG_INVALID_NONCE = 91;
    uint256 internal constant DBSIG_EXP_SIG = 92;

    // LIME
    uint256 internal constant TOKENS_NOT_APPROVED = 93;

    uint256 internal constant NICE_TRY = 69;

    // Strategies
    uint256 internal constant CALL_RESTRICTED = 27;
    uint256 internal constant STRATEGY_IN_EMERGENCY_STATE = 28;
    uint256 internal constant REWARD_POOL_UNDERLYING_MISMATCH = 29;
    uint256 internal constant UNSALVAGABLE_TOKEN = 30;

    // Strategy splitter.
    uint256 internal constant ARRAY_LENGTHS_DO_NOT_MATCH = 31;
    uint256 internal constant WEIGHTS_DO_NOT_ADD_UP = 32;
    uint256 internal constant REBALANCE_REQUIRED = 33;
    uint256 internal constant INDICE_DOES_NOT_EXIST = 34;

    // Controller
    uint256 internal constant CALLER_NOT_REFEREE = 35;
    uint256 internal constant REFERRAL_CODE_ALREADY_EXISTS = 36;
    uint256 internal constant INSUFFICIENT_ETHER_AMOUNT = 37;
    uint256 internal constant ANTI_SYBIL_ATTESTATION_EXISTS = 38;

    // Warchest
    uint256 internal constant CALLER_NOT_LENDING_POOL = 39;
    uint256 internal constant CIRCUIT_BREAKER_NOT_ACTIVE = 40;
    uint256 internal constant CIRCUIT_BREAKER_IS_ACTIVE = 41;
    uint256 internal constant VAULT_HAS_NO_SHARES = 42;
    uint256 internal constant SHARES_MUST_NOT_BE_ZERO = 43;

    // Lending Pool
    uint256 internal constant CALLER_NOT_EOA = 41;
    uint256 internal constant NOT_WORKER_IN_EXEC = 42;
    uint256 internal constant NOT_POSITION_IN_EXEC = 43;
    uint256 internal constant EXEC_IN_PROGRESS = 44;
    uint256 internal constant MALFORMED_POS_ID = 45;
    uint256 internal constant NOT_POS_WORKER = 46;
    uint256 internal constant NOT_POS_OWNER = 47;
    uint256 internal constant WORKER_NOT_AUTHORIZED = 48;
    uint256 internal constant NOT_ACCEPTING_DEBT = 49;
    uint256 internal constant INSUFFICIENT_ASSETS_LENDABLE = 50;
    uint256 internal constant DEBT_TOO_SMALL = 51;
    uint256 internal constant UNHEALTHY_POSITION = 52;
    uint256 internal constant LIQUIDATIONS_PERMISSIONED = 53;
    uint256 internal constant NO_DEBT = 54;
    uint256 internal constant CANT_LIQUIDATE = 55;
    uint256 internal constant INACTIVE_POSITION = 56;
    uint256 internal constant WORKER_HEALTHCHECK_FAILED = 57;
    uint256 internal constant CANNOT_WITHDRAW_IF_INCREASING = 58;
    uint256 internal constant HEALTH_DID_NOT_INCREASE = 59;
    uint256 internal constant POSITION_NEAR_LIQ_THRESHOLD = 60;
    uint256 internal constant NOT_PRIVILEGED_BORROWER = 61;

    // Workers // Rewards
    uint256 internal constant CALLER_NOT_GOV_OR_REWARD_DIST = 31;
    uint256 internal constant REWARD_TOKEN_ALREADY_EXIST = 32;
    uint256 internal constant DURATION_CANNOT_BE_ZERO = 33;
    uint256 internal constant REWARD_TOKEN_DOES_NOT_EXIST = 34;
    uint256 internal constant REWARD_PERIOD_HAS_NOT_ENDED = 35;
    uint256 internal constant CANNOT_REMOVE_LAST_REWARD_TOKEN = 36;
    uint256 internal constant NOTIF_AMOUNT_INVOKES_OVERFLOW = 37;
    uint256 internal constant REWARD_INDICE_NOT_FOUND = 38;

    // Workers // Farming
    uint256 internal constant CALLER_NOT_KEEPER = 47;
    uint256 internal constant TOKEN_0_POTENTIAL_MANIPULATION = 47;
    uint256 internal constant TOKEN_1_POTENTIAL_MANIPULATION = 48;
    uint256 internal constant ORACLE_PRICE_STALE = 49;
    uint256 internal constant SPOT_TOO_HIGH = 50;
    uint256 internal constant SPOT_TOO_LOW = 51;
    uint256 internal constant TOO_MUCH_SLIPPAGE = 52;

    // Position Coordinator
}
