// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title Warchest Storage
/// @author Chainvisions
/// @notice Diamond storage contract for the Warchest contract.

library WarchestStorage {
    /// @notice Packed struct for storing strategies.
    struct Strategy {
        /// @notice Address of the strategy.
        address strategyAddress;
        /// @notice Weight of the strategy for investment.
        uint32 investmentNumerator;
    }

    struct Layout {
        /// @notice Underlying asset of the Warchest.
        address underlying;
        /// @notice Active strategies on the Warchest contract.
        Strategy[] strategies;
        /// @notice Strategy with the highest weight to save costs.
        address highestWeightStrategy;
        /// @notice Emergency circuit breaker.
        bool circuitBreakerActive;
        /// @notice Total percentage to invest into strategies.
        uint32 totalInvestmentNumerator;
    }

    bytes32 internal constant STORAGE_SLOT = keccak256("limestone.contracts.storage.Warchest");

    function layout() internal pure returns (Layout storage l) {
        bytes32 slot = STORAGE_SLOT;
        assembly {
            l.slot := slot
        }
    }
}
