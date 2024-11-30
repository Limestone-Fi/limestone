// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

/// @title Warchest Interface
/// @author Chainvisions
/// @notice Interface for the Warchest contract.

interface IWarchest {
    /// @notice Emitted when funds are invested into a strategy
    /// @param strategy Strategy that funds were invested into.
    /// @param amount Amount of tokens invested into the strategy.
    event Invested(address indexed strategy, uint256 amount);

    /// @notice Emitted when the circuit breaker is activated.
    /// @param timestamp Timestamp of the activation.
    event CircuitBreakerActivated(uint256 timestamp);

    /// @notice Mints Warchest tokens.
    /// @param _to Address to mint tokens to.
    /// @param _amount Amount of tokens to mint.
    function mint(address _to, uint256 _amount) external;

    /// @notice Burns Warchest tokens.
    /// @param _from Address to burn tokens from.
    /// @param _amount Amount of tokens to burn.
    function burn(address _from, uint256 _amount) external;

    /// @notice Withdraws tokens from the Warchest's strategies.
    /// @param _destination Address to send tokens to.
    /// @param _amountToWithdraw Amount of tokens to withdraw.
    function withdrawReserves(address _destination, uint256 _amountToWithdraw) external;

    /// @notice Calculates the total amount of assets held by the Warchest.
    /// @return Warchest's funds included ones currently invested in strategies.
    function underlyingBalanceWithInvestment() external view returns (uint256);
}
