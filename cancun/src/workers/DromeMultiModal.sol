// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IDromeFactory} from "../interfaces/external/IDromeFactory.sol";
import {IDromePool} from "../interfaces/external/IDromePool.sol";
import {IDromeRouter} from "../interfaces/external/IDromeRouter.sol";
import {IDromeGauge} from "../interfaces/external/IDromeGauge.sol";
import {SwapUtils} from "../lib/SwapUtils.sol";
import {
    MultiModalWorker,
    MultiModalWorkerStorage,
    MultiModalPosition,
    ILimeDiamond,
    _require,
    Errors,
    Cast,
    FixedPointMathLib,
    SafeTransferLib
} from "../MultiModalWorker.sol";

/// @title Drome Multi Modal Worker
/// @author Chainvisions
/// @notice Multi Modal Worker for -drome liquidity pools.

contract DromeMultiModalWorker is MultiModalWorker {
    using SafeTransferLib for address;
    using Cast for uint256;

    /// @notice Structure for storing token liquidation routes.
    struct Path {
        IDromeRouter.Route[] route;
        bool swapLess;
    }

    /// @notice Routes for liquidation on Solidly.
    mapping(address => mapping(address => IDromeRouter.Route[])) public routes;

    /// @notice Claims pending rewards and reinvests them.
    function reinvest() external override {
        MultiModalWorkerStorage.LiquidityPool memory pool = MultiModalWorkerStorage._readPoolData();
        IDromeGauge(pool.rewardPool).getReward(address(this));
        _liquidateReward();
        _investLiquidity(pool.pair.balanceOf(address(this)));
    }

    /// @notice Sets the route for a specific swap.
    /// @param _from Token to swap from.
    /// @param _to Token to swap to.
    /// @param _route Route for the swap.
    function setRoute(address _from, address _to, IDromeRouter.Route[] calldata _route) external onlyOwner {
        routes[_from][_to] = _route;
    }

    /// @notice Calculates the equity and debt value for a specified position.
    /// @param _posId ID of the position to calculate the value of.
    /// @return (equityValue, totalDebtValue) Total equity value (in token0) of the position and the total debt value (in token0).
    function calculatePositionValue(uint256 _posId) external view override returns (uint256, uint256) {
        MultiModalWorkerStorage.LiquidityPool memory pool = MultiModalWorkerStorage._readPoolData();
        (uint256 token0Decimals, uint256 token1Decimals, uint256 reserve0, uint256 reserve1,,,) =
            IDromePool(pool.pair).metadata();

        // Fetch TWAP price for the assets.
        uint256 price0 = token0Decimals; // @dev Since we are only calculating the fair price in terms of token0, the price of token0 should just be one unit of token0.
        uint256 price1 = IDromePool(pool.pair).quote(pool.token1, token1Decimals, 4);

        // Normalize reserves to 1e18.
        reserve0 = (reserve0 * 1e18) / token0Decimals;
        reserve1 = (reserve1 * 1e18) / token1Decimals;

        // Calculate the fair price of the LP.
        uint256 fairPrice = SwapUtils._getFairLpPrice(
            token0Decimals,
            token1Decimals,
            reserve0,
            reserve1,
            price0,
            price1,
            IDromePool(pool.pair).totalSupply(),
            pool.stableswap
        );

        // Calculate value of the position.
        MultiModalPosition storage pos = MultiModalWorkerStorage.layout().positions[_posId];
        uint256 equityValue = (_sharesToTokens(pos.positionShares) * fairPrice) / (10 ** (18 + 6));
        (uint256 totalDebtValue, uint256 debt0, uint256 debt1) = (
            0,
            pos.debt0PoolId != type(uint32).max
                ? ILimeDiamond(LIMESTONE_DIAMOND).debtShareToVal(pos.debt0PoolId, pos.debtShare0)
                : 0,
            pos.debt1PoolId != type(uint32).max
                ? ILimeDiamond(LIMESTONE_DIAMOND).debtShareToVal(pos.debt1PoolId, pos.debtShare1)
                : 0
        );
        totalDebtValue = debt0 + (debt1 != 0 ? ((debt1 * price1) / token1Decimals) : 0);

        return (equityValue, totalDebtValue);
    }

    /// @notice Determines the optimal path for a specific swap.
    /// @param _from Token to swap from.
    /// @param _to Token to swap to.
    /// @return path Path for the swap.
    function getPath(address _from, address _to) public view returns (Path memory path) {
        if (_from == _to) path.swapLess = true;
        path.route = routes[_from][_to];
    }

    function _addLiquidity(
        MultiModalWorkerStorage.LiquidityPool memory _pool,
        uint256 _token0In,
        uint256 _token1In,
        uint256 _minLiquidity
    ) internal override returns (uint256) {
        (uint256 reserve0, uint256 reserve1,) = IDromePool(_pool.pair).getReserves();
        address swapFactory = IDromeRouter(_pool.router).defaultFactory();
        (uint256 optimalAmount, bool reversed) = SwapUtils._optimalZapAmountIn(
            _token0In, _token1In, reserve0, reserve1, IDromeFactory(swapFactory).getFee(_pool.pair, _pool.stableswap)
        );
        if (optimalAmount > 0) {
            IDromeRouter.Route[] memory route = new IDromeRouter.Route[](1);
            route[0] = reversed
                ? IDromeRouter.Route({from: _pool.token1, to: _pool.token0, stable: _pool.stableswap, factory: swapFactory})
                : IDromeRouter.Route({from: _pool.token0, to: _pool.token1, stable: _pool.stableswap, factory: swapFactory});
            if (reversed) {
                _pool.token1.safeApprove(_pool.router, 0);
                _pool.token1.safeApprove(_pool.router, optimalAmount);
            } else {
                _pool.token0.safeApprove(_pool.router, 0);
                _pool.token0.safeApprove(_pool.router, optimalAmount);
            }

            IDromeRouter(_pool.router).swapExactTokensForTokens(optimalAmount, 0, route, address(this), block.timestamp);
        }

        (uint256 amount0, uint256 amount1) =
            (_pool.token0.balanceOf(address(this)), _pool.token1.balanceOf(address(this)));
        _pool.token0.safeApprove(_pool.router, amount0);
        _pool.token1.safeApprove(_pool.router, amount1);
        (,, uint256 liquidity) = IDromeRouter(_pool.router).addLiquidity(
            _pool.token0,
            _pool.token1,
            _pool.stableswap,
            _pool.token0.balanceOf(address(this)),
            _pool.token1.balanceOf(address(this)),
            0,
            0,
            address(this),
            block.timestamp
        );
        _require(liquidity >= _minLiquidity, Errors.TOO_MUCH_SLIPPAGE);
        return liquidity;
    }

    function _divestAndRemoveLiquidity(MultiModalWorkerStorage.LiquidityPool memory _pool, uint256 _liquidity)
        internal
        override
        returns (uint112, uint256, uint256)
    {
        uint112 shareValue = _tokensToShares(_liquidity);
        IDromeGauge(_pool.rewardPool).withdraw(_liquidity);
        _pool.pair.safeApprove(_pool.router, 0);
        _pool.pair.safeApprove(_pool.router, _liquidity);
        (uint256 amount0, uint256 amount1) = IDromeRouter(_pool.router).removeLiquidity(
            _pool.token0, _pool.token1, _pool.stableswap, _liquidity, 0, 0, address(this), block.timestamp
        );

        return (shareValue, amount0, amount1);
    }

    function _investLiquidity(uint256 _amount) internal override returns (uint112 shareValue) {
        MultiModalWorkerStorage.LiquidityPool memory pool = MultiModalWorkerStorage._readPoolData();
        if (_amount > 0) {
            shareValue = _tokensToShares(_amount);

            // Deposit assets into the gauge.
            pool.pair.safeApprove(pool.rewardPool, 0);
            pool.pair.safeApprove(pool.rewardPool, _amount);
            IDromeGauge(pool.rewardPool).deposit(_amount);

            // Return the calculated share value.
            return shareValue;
        }
    }

    function _swapToSide(
        MultiModalWorkerStorage.LiquidityPool memory _pool,
        uint8 _side,
        uint256 _amount,
        uint256 _desired
    ) internal override returns (uint256) {
        IDromeRouter.Route[] memory route = new IDromeRouter.Route[](1);
        address factory = IDromeRouter(_pool.router).defaultFactory();
        if (_side == 0) {
            route[0] =
                IDromeRouter.Route({from: _pool.token1, to: _pool.token0, stable: _pool.stableswap, factory: factory});
            _pool.token1.safeApprove(_pool.router, 0);
            _pool.token1.safeApprove(_pool.router, _amount);
        } else {
            route[0] =
                IDromeRouter.Route({from: _pool.token0, to: _pool.token1, stable: _pool.stableswap, factory: factory});
            _pool.token0.safeApprove(_pool.router, 0);
            _pool.token0.safeApprove(_pool.router, _amount);
        }

        uint256[] memory amounts = IDromeRouter(_pool.router).swapExactTokensForTokens(
            _amount, _desired, route, address(this), block.timestamp
        );

        return amounts[amounts.length - 1];
    }

    function _swapForExactAmount(address _tokenIn, address _tokenOut, uint256 _desired) internal override {
        // Calculate the amount we need to swap. Spray and pray method.
        MultiModalWorkerStorage.LiquidityPool memory pool = MultiModalWorkerStorage._readPoolData();
        uint256 optimalAmount = IDromePool(pool.pair).getAmountOut(_desired, _tokenOut);

        for (uint256 i; i < 10;) {
            optimalAmount = (optimalAmount * 1010) / 1000;
            uint256 output = IDromePool(pool.pair).getAmountOut(optimalAmount, _tokenIn);

            if (output >= _desired) {
                break;
            }
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }

        IDromeRouter.Route[] memory route = new IDromeRouter.Route[](1);
        route[0] = IDromeRouter.Route({
            from: _tokenIn,
            to: _tokenOut,
            stable: pool.stableswap,
            factory: IDromeRouter(pool.router).defaultFactory()
        });
        _tokenIn.safeApprove(pool.router, 0);
        _tokenIn.safeApprove(pool.router, optimalAmount);
        IDromeRouter(pool.router).swapExactTokensForTokens(optimalAmount, 0, route, address(this), block.timestamp);
    }

    function _handleLiquidation(uint256[] memory _balances) internal override {
        MultiModalWorkerStorage.LiquidityPool memory pool = MultiModalWorkerStorage._readPoolData();
        address[] memory _rewards = pool.rewardTokens;

        for (uint256 i; i < _rewards.length;) {
            address reward = _rewards[i];
            uint256 rewardBalance = _balances[i];

            reward.safeApprove(pool.router, 0);
            reward.safeApprove(pool.router, rewardBalance);

            uint256 toSwap = rewardBalance / 2;
            Path memory token0Route = getPath(reward, pool.token0);
            Path memory token1Route = getPath(reward, pool.token1);

            (uint256 token0Amount, uint256 token1Amount) = (1, 1);
            if (token0Route.swapLess) {
                token0Amount = toSwap;
            } else {
                uint256[] memory amounts = IDromeRouter(pool.router).swapExactTokensForTokens(
                    toSwap, 0, token0Route.route, address(this), block.timestamp
                );
                token0Amount = amounts[amounts.length - 1];
            }

            if (token1Route.swapLess) {
                token1Amount = toSwap;
            } else {
                uint256[] memory amounts = IDromeRouter(pool.router).swapExactTokensForTokens(
                    toSwap, 0, token1Route.route, address(this), block.timestamp
                );
                token1Amount = amounts[amounts.length - 1];
            }

            pool.token0.safeApprove(pool.router, 0);
            pool.token0.safeApprove(pool.router, token0Amount);

            pool.token1.safeApprove(pool.router, 0);
            pool.token1.safeApprove(pool.router, token1Amount);

            IDromeRouter(pool.router).addLiquidity(
                pool.token0,
                pool.token1,
                pool.stableswap,
                token0Amount,
                token1Amount,
                0,
                0,
                address(this),
                block.timestamp
            );
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }
    }

    function _validateReserves(MultiModalWorkerStorage.LiquidityPool memory _pool) internal view override {
        (uint256 reserve0, uint256 reserve1,) = IDromePool(_pool.pair).getReserves();
        (uint256 unsyncedReserve0, uint256 unsyncedReserve1) =
            (_pool.token0.balanceOf(_pool.pair), _pool.token1.balanceOf(_pool.pair));
        _require(unsyncedReserve0 * 100 <= (reserve0 * 101), Errors.TOKEN_0_POTENTIAL_MANIPULATION);
        _require(unsyncedReserve1 * 100 <= (reserve1 * 101), Errors.TOKEN_1_POTENTIAL_MANIPULATION);
    }

    function _compareSpotAndOracle(MultiModalWorkerStorage.LiquidityPool memory _pool) internal view override {
        _validateReserves(_pool);
        (uint256 token0Decimals, uint256 token1Decimals,,,,,) = IDromePool(_pool.pair).metadata();
        (uint256 token0Spot, uint256 token1Spot, uint256 token0Twap, uint256 token1Twap) = (
            IDromePool(_pool.pair).getAmountOut(token1Decimals, _pool.token1),
            IDromePool(_pool.pair).getAmountOut(token0Decimals, _pool.token0),
            IDromePool(_pool.pair).quote(_pool.token1, token1Decimals, 4),
            IDromePool(_pool.pair).quote(_pool.token0, token0Decimals, 4)
        );

        // Compare spot and TWAP prices and see if they are within our threshold.
        _require(
            (token0Spot * 10000 <= token0Twap * 11000 && token1Spot * 10000 <= token1Twap * 11000), Errors.SPOT_TOO_HIGH
        );
        _require(
            (token0Spot * 11000 >= token0Twap * 10000 && token1Spot * 11000 >= token1Twap * 10000), Errors.SPOT_TOO_LOW
        );
    }

    function _tokensToShares(uint256 _tokens) internal view override returns (uint112) {
        uint112 _totalShares = MultiModalWorkerStorage.layout().totalPositionShares;
        if (_totalShares == 0) return _tokens.u112();
        uint256 staked = MultiModalWorkerStorage._readPoolData().rewardPool.balanceOf(address(this));
        return ((_tokens * _totalShares) / staked).u112();
    }

    function _sharesToTokens(uint112 _shares) internal view override returns (uint112) {
        uint112 _totalShares = MultiModalWorkerStorage.layout().totalPositionShares;
        if (_totalShares == 0) return _shares;
        uint256 staked = MultiModalWorkerStorage._readPoolData().rewardPool.balanceOf(address(this));
        return ((_shares * staked) / _totalShares).u112();
    }
}