// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

/// @notice Execution context for `investInV2LikePosition()`.
struct V2LikePositionInvestmentContext {
    /// @notice ID of the position to invest in. `0` for a new ID.
    uint256 positionId;
    /// @notice Worker to invest the LP assets into.
    address worker;
    /// @notice Amount of token0 to deposit into the position.
    uint256 token0In;
    /// @notice Amount of token1 to deposit into the position.
    uint256 token1In;
    /// @notice Lending pool ID for token0 to borrow from.
    uint256 token0PoolId;
    /// @notice Lending pool ID for token1 to borrow from.
    uint256 token1PoolId;
    /// @notice Amount of token0 to borrow for the position.
    uint256 token0Borrow;
    /// @notice Amount of token1 to borrow for the position.
    uint256 token1Borrow;
    /// @notice Minimum amount of liquidity to mint when investing.
    uint256 minLiquidityMinted;
}

/// @notice Execution context for `investInBalancerLikePosition()`.
struct BalancerLikePositionInvestmentContext {
    /// @notice ID of the position to invest in. `0` for a new ID.
    uint256 positionId;
    /// @notice Worker to invest the LP assets into.
    address worker;
    /// @notice Amount of each pool tokens to deposit into the position.
    uint256[] tokenAmountsIn;
    /// @notice Lending pool IDs for each token to borrow from.
    uint256[] tokenPoolIds;
    /// @notice Amounts of each token to borrow.
    uint256[] tokenBorrows;
    /// @notice Minimum amount of liquidity to mint when investing.
    uint256 minLiquidity;
}

/// @notice Execution context for `divestFromBalancerLikePosition()`.
struct BalancerLikePositionDivestmentContext {
    /// @notice ID of the position to divest from.
    uint256 positionId;
    /// @notice Address of the worker to divest from.
    address worker;
    /// @notice Amount of liquidity to burn from the position.
    uint256 liquidityToBurn;
    /// @notice Minimum amount of each token to receive from burning liquidity.
    uint256[] minTokensOut;
    /// @notice Amount of each token to use towards repaying debt.
    uint256[] tokensRepay;
    /// @notice Whether or not the withdrawal should be minimal (send all assets without swap).
    bool minimalWithdrawal;
}

/// @notice Execution context for `divestFromV2LikePosition()`.
struct V2LikePositionDivestmentContext {
    /// @notice ID of the position to divest from.
    uint256 positionId;
    /// @notice Address of the worker to divest from.
    address worker;
    /// @notice Amount of liquidity to burn from the position.
    uint256 liquidityToBurn;
    /// @notice Minimum amount of token0 to receive from burning liquidity.
    uint256 minToken0Out;
    /// @notice Minimum amount of token1 to receive from burning liquidity.
    uint256 minToken1Out;
    /// @notice Amount of token0 to use towards repaying debt.
    uint256 token0Repay;
    /// @notice Amount of token1 to use towards repaying debt.
    uint256 token1Repay;
    /// @notice Whether or not the withdrawal should be minimal (send both assets).
    bool minimalWithdrawal;
}

/// @notice Execution context for `liquidateV2LikePosition()`.
struct V2LikePositionLiquidationContext {
    /// @notice ID of the position to liquidate.
    uint256 positionId;
    /// @notice Address of the worker to liquidate on.
    address worker;
    /// @notice Percentage of the position to liquidate for repayment.
    uint16 positionBps;
    /// @notice Amount of token0 sent from the liquidator for repayment.
    uint256 token0RepayIn;
    /// @notice Amount of token1 sent from the liquidator for repayment.
    uint256 token1RepayIn;
    /// @notice Minimum amount of token0 that should be received from burning the liquidity.
    uint256 minToken0Out;
    /// @notice Minimum amount of token1 that should be received from burning the liquidity.
    uint256 minToken1Out;
}

/// @notice Execution context for `claimWorkerLimeRewards()`.
struct ClaimWorkerRewardsContext {
    /// @notice Worker to claim rewards from.
    address worker;
    /// @notice ID of the position to claim for.
    uint256 positionId;
    /// @notice Whether or not the rewards should be vested.
    bool vest;
}

/// @notice Position Coordinator Interface
/// @author Chainvisions
/// @notice Interface for the Limestone Position Coordinator facet.

interface IPositionCoordinator {
    function investInV2LikePosition(V2LikePositionInvestmentContext calldata _ctx) external;
    function divestFromV2LikePosition(V2LikePositionDivestmentContext calldata _ctx) external;
    function repayV2LikeLiquidityPositionDebt(
        uint256 _positionId,
        address _worker,
        uint256 _token0Repay,
        uint256 _token1Repay
    ) external;
    function liquidateV2LikePosition(V2LikePositionLiquidationContext calldata _ctx) external;
    //function divestAndRepayV2LikePosition(V2LikePositionDivestAndRepayContext _ctx) external;
    function claimWorkerLimeRewards(ClaimWorkerRewardsContext[] calldata _ctx) external;
}
