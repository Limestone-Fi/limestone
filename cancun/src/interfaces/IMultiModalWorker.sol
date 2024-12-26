// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {
    IPositionCoordinator,
    BalancerLikePositionInvestmentContext,
    V2LikePositionInvestmentContext,
    V2LikePositionDivestmentContext,
    V2LikePositionLiquidationContext
} from "./IPositionCoordinator.sol";

struct MultiModalPosition {
    address owner;
    uint112 positionShares;
    uint32 debt0PoolId;
    uint32 debt1PoolId;
    uint112 debtShare0;
    uint112 debtShare1;
}

interface IMultiModalWorker {
    function invest(
        V2LikePositionInvestmentContext calldata _ctx,
        address _borrower,
        uint112 _debtShare0,
        uint112 _debtShare1
    ) external returns (uint256);
    function divest(V2LikePositionDivestmentContext calldata) external returns (uint256, uint256, uint256, uint256);
    function repayDebt(uint256 _positionId, uint256 _repayToken0, uint256 _repayToken1)
        external
        returns (uint112, uint112);
    function liquidate(address _liquidator, V2LikePositionLiquidationContext calldata _ctx)
        external
        returns (uint112, uint112);
    function reinvest() external;
    function getPosition(uint256 _positionId) external view returns (MultiModalPosition memory);
    function calculatePositionValue(uint256 _positionId)
        external
        view
        returns (uint256 positionEquity, uint256 positionDebt);
}

interface IMultiModalWorkerBalancerLike {
    function invest(BalancerLikePositionInvestmentContext calldata) external returns (uint256);
}
