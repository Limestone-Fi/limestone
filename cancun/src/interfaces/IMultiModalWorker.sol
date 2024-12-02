// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {
    IPositionCoordinator,
    BalancerLikePositionInvestmentContext,
    V2LikePositionDivestmentContext,
    V2LikePositionLiquidationContext
} from "./IPositionCoordinator.sol";

struct MultiModalPosition {
    uint256 totalShares;
    uint112 debtShare0;
    uint112 debtShare1;
    uint32 token0PoolId;
    uint32 token1PoolId;
}

interface IMultiModalWorker {
    function invest(
        uint256 _positionId,
        address _borrower,
        uint256 _amount0,
        uint256 _amount1,
        uint256 _debtShare0,
        uint256 _debtShare1
    ) external returns (uint256);
    function divest(V2LikePositionDivestmentContext calldata) external returns (uint256, uint256, uint256, uint256);
    function repayDebt(uint256 _positionId, uint256 _repayToken0, uint256 _repayToken1) external;
    function liquidate(address _liquidator, V2LikePositionLiquidationContext calldata _ctx) external;
    function getReward(uint256 _positionId, bool _vest) external;
    function getPosition(uint256 _positionId) external view returns (MultiModalPosition memory);
    function calculatePositionValue(uint256 _positionId)
        external
        view
        returns (uint256 positionEquity, uint256 positionDebt);
    function healthcheck() external view returns (bool);
}

interface IMultiModalWorkerBalancerLike {
    function invest(BalancerLikePositionInvestmentContext calldata) external returns (uint256);
}
