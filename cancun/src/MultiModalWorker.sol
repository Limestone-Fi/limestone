// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Initializable} from "@solidstate/security/initializable/Initializable.sol";
import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";
import {Ownable} from "solady/src/auth/Ownable.sol";
import {IUniswapV2Pair} from "./interfaces/external/IUniswapV2Pair.sol";
import {ILimeDiamond} from "./interfaces/ILimeDiamond.sol";
import {
    IMultiModalWorker,
    V2LikePositionInvestmentContext,
    V2LikePositionDivestmentContext,
    V2LikePositionLiquidationContext,
    MultiModalPosition
} from "./interfaces/IMultiModalWorker.sol";
import {IEmissionsController} from "./interfaces/IEmissionsController.sol";
import {Cast} from "./lib/Cast.sol";
import {_require, Errors} from "./lib/Errors.sol";
import {MultiModalWorkerStorage} from "./MultiModalWorkerStorage.sol";

/// @title Base Multi Modal Worker
/// @author Chainvisions
/// @notice A Limestone worker capable of handling positions with multi-asset borrows.

abstract contract MultiModalWorker is IMultiModalWorker, Initializable, Ownable {
    using SafeTransferLib for address;
    using Cast for uint256;

    /// @notice Structure for worker investment related vars.
    /// @dev Used to avoid stack-too-deep errors when not using Yul IR (important for fuzzing).
    struct InvestStack {
        /// @dev ID of the worker position.
        uint256 posId;
        /// @dev The total amount of token0 to invest, including borrowed tokens.
        uint256 token0In;
        /// @dev The total amount of token1 to invest, including borrowed tokens.
        uint256 token1In;
        /// @dev The amount of liquidity minted from creating the position.
        uint256 liquidity;
        /// @dev The amount of worker shares that the position has been issued.
        uint112 positionShares;
    }

    /// @notice Structure for worker divestment related vars.
    /// @dev Used to avoid stack-too-deep errors when not using Yul IR (important for fuzzing).
    struct DivestStack {
        /// @dev Pool data.
        MultiModalWorkerStorage.LiquidityPool pool;
        /// @dev The amount of shares to remove from the position.
        uint112 sharesToRemove;
        /// @dev The amount of token0 received back from removing liquidity + repayment.
        uint256 amount0;
        /// @dev The amount of token1 received back from removing liquidity + repayment.
        uint256 amount1;
        /// @dev Whether or not token0 or token1 was borrowed. Used in their own respective cases.
        bool borrowed;
    }

    /// @notice Structure for liquidation related vars.
    /// @dev Used to avoid stack-too-deep errors when not using Yul IR (important for fuzzing).
    struct LiquidateStack {
        /// @dev The amount of debt in token0 held by the position.
        uint112 debt0;
        /// @dev The amount of debt in token1 held by the position.
        uint112 debt1;
        /// @dev The amount of token0 repaid to the lending pool.
        uint256 token0Repaid;
        /// @dev The amount of token1 repaid to the lending pool.
        uint256 token1Repaid;
        /// @dev The total amount of LP tokens to withdraw for liquidating the position.
        uint256 totalTokens;
        /// @dev The amount of worker shares to remove from the position.
        uint112 shares;
        /// @dev The amount of token0 received back after repayment and fees.
        uint256 amount0;
        /// @dev The amount of token1 received back after repayment and fees.
        uint256 amount1;
        /// @dev The fee in token0 to send to the liquidator as a reward.
        uint256 fee0;
        /// @dev The fee in token1 to send to the liquidator as a reward.
        uint256 fee1;
    }

    /// @notice Limestone diamond contract. Typically never changes so it is a constant.
    address public constant LIMESTONE_DIAMOND = 0x01000006b888030018000000D1e1AA171700fb8D;

    modifier onlyDiamond() {
        _require(msg.sender == LIMESTONE_DIAMOND, Errors.CALLER_NOT_LENDING_POOL);
        _;
    }

    /// @notice Initializes the worker.
    /// @param _pair Pair contract for the liquidity pool.
    /// @param _rewardPool Reward pool for liquidity staking.
    /// @param _router Router for performing swaps.
    /// @param _rewards Rewards generated from farming.
    function initialize(address _pair, address _rewardPool, address _router, address[] calldata _rewards)
        external
        initializer
    {
        // Write pool data to SSTORE2.
        MultiModalWorkerStorage.LiquidityPool memory pool;
        pool.pair = _pair;
        pool.rewardPool = _rewardPool;
        pool.rewardTokens = _rewards;
        pool.router = _router;

        (bool success, bytes memory result) = _pair.call(abi.encodeWithSelector(IUniswapV2Pair.tokens.selector));
        if (success) {
            (pool.token0, pool.token1) = abi.decode(result, (address, address));
        } else {
            pool.token0 = IUniswapV2Pair(_pair).token0();
            pool.token1 = IUniswapV2Pair(_pair).token1();
        }

        MultiModalWorkerStorage._writePoolData(pool);
        _setOwner(ILimeDiamond(LIMESTONE_DIAMOND).owner());
        MultiModalWorkerStorage.layout().nextPositionId = 1;
    }

    /// @notice Creates positions/invests assets into a position on the worker.
    /// @param _ctx Context parameters related to the investment call.
    /// @param _borrower Address of the borrower.
    /// @param _debtShare0 Amount of token0 debt to add to the position from borrowing.
    /// @param _debtShare1 Amount of token1 debt to add to the position from borrowing.
    function invest(
        V2LikePositionInvestmentContext calldata _ctx,
        address _borrower,
        uint112 _debtShare0,
        uint112 _debtShare1
    ) external override onlyDiamond returns (uint256) {
        MultiModalWorkerStorage.Layout storage $ = MultiModalWorkerStorage.layout();
        InvestStack memory stack;
        // Validate current position ID and update rewards in advance.
        stack.posId;
        if (_ctx.positionId == 0) {
            // forgefmt: disable-next-line
            unchecked {stack.posId = $.nextPositionId++;}
            $.positions[stack.posId].owner = _borrower;
        } else {
            _require(_ctx.positionId < $.nextPositionId, Errors.MALFORMED_POS_ID);
            _require(_borrower == $.positions[stack.posId].owner, Errors.NOT_POS_OWNER);
            stack.posId = _ctx.positionId;
        }
        MultiModalPosition storage pos = $.positions[stack.posId];

        MultiModalWorkerStorage.LiquidityPool memory pool = MultiModalWorkerStorage._readPoolData();
        address[] memory assets = new address[](2);
        uint256[] memory transferAmounts = new uint256[](2);
        (assets[0], assets[1], transferAmounts[0], transferAmounts[1]) =
            (pool.token0, pool.token1, _ctx.token0In, _ctx.token1In);
        ILimeDiamond(LIMESTONE_DIAMOND).accessAssets(pos.owner, assets, transferAmounts);
        if (_ctx.skipHealthcheck) {
            _require(_ctx.token0Borrow + _ctx.token1Borrow == 0, Errors.HEALTHCHECK_UNSKIPPABLE_IF_BORROWING);
            // Check current reserves.
            _validateReserves(pool);
        } else {
            // Perform oracle healthcheck.
            _compareSpotAndOracle(pool);
        }

        // Invest supplied assets into the liquidity pool.
        (stack.token0In, stack.token1In) = (_ctx.token0In + _ctx.token0Borrow, _ctx.token1In + _ctx.token1Borrow);
        if (stack.token0In > 0) {
            pool.token0.safeApprove(pool.router, 0);
            pool.token0.safeApprove(pool.router, stack.token0In);
        }

        if (stack.token1In > 0) {
            pool.token1.safeApprove(pool.router, 0);
            pool.token1.safeApprove(pool.router, stack.token1In);
        }
        stack.liquidity = _addLiquidity(pool, stack.token0In, stack.token1In, _ctx.minLiquidityMinted);

        // Deposit liquidity into the reward pool and update the user's position.
        stack.positionShares = _investLiquidity(stack.liquidity);
        if ($.totalPositionShares == 0) {
            $.totalPositionShares = stack.positionShares;
            pos.positionShares = stack.positionShares - 10 ** 3; // @dev Reserves 10**3 shares to avoid inflation exploits.
        } else {
            $.totalPositionShares += stack.positionShares;
            pos.positionShares += stack.positionShares;
        }
        if (_debtShare0 > 0) pos.debtShare0 += _debtShare0;
        if (_debtShare1 > 0) pos.debtShare1 += _debtShare1;

        // Store pools borrowed from if it is a new position and validate the existing if not.
        // TODO: Potential gotcha here is if they only borrow one sided. Will need to account for that esp with the latter check.
        // Update: Nah, decided it's pointless. Let them borrow using a new position.
        if (_ctx.positionId == 0) {
            pos.debt0PoolId =
                (_ctx.token0PoolId != 0 || _ctx.token0Borrow > 0) ? uint32(_ctx.token0PoolId) : type(uint32).max;
            pos.debt1PoolId =
                (_ctx.token1PoolId != 0 || _ctx.token1Borrow > 0) ? uint32(_ctx.token1PoolId) : type(uint32).max;
            // @dev ^ We check if the pool ID is zero or not and record it as type(uint32).max if it is as otherwise, there'd be an issue related to the check in the else branch.
            // Basically, since if it wasn't specified, then we'd run into a problem of being able to borrow from the lending pool at ID `0` without the contract stopping us.
            // This could pose a risk to lenders if there is an active lending pool at that index. Though, the damage that could be done is more in the line of griefing the contract
            // than an actual exploit that would enable draining from the contract (assuming the pool does not contain its underlying, else it'd just register as normal debt).
            // That said, setting it to type(uint32).max gives us peace of mind as it'll cause that check to fail and register as arbitrage. This index should also not be a valid pool (odds are its not).
        } else {
            // @dev Basically, without this check, I could theoretically arb the lending pools if there are two of the same asset.
            // Way it would work is by first borrowing a small amount from a very low interest rate pool and then performing a
            // second transaction where I borrow from a higher interest pool. This would cause my interest rate to be lower
            // due to it accruing from the smaller pool and it would also create bad debt amongst the lending pools as on repay,
            // the smaller pool would get the debt returned and not the pool that I borrowed the bulk from. This loop can also technically
            // enable borrowing higher than the max leverage in theory if done right and the conditions are proper for it due to debt share value discrepencies.
            _require(
                _ctx.token0PoolId == pos.debt0PoolId && _ctx.token1PoolId == pos.debt1PoolId,
                Errors.NO_LENDING_ARBITRAGE
            );
        }

        return stack.posId;
    }

    function divest(V2LikePositionDivestmentContext calldata _ctx, address _executor)
        external
        override
        onlyDiamond
        returns (uint256, uint256, uint256, uint256)
    {
        MultiModalWorkerStorage.Layout storage $ = MultiModalWorkerStorage.layout();
        MultiModalPosition storage pos = $.positions[_ctx.positionId];
        DivestStack memory stack;

        // Validate current position access control.
        _require(_ctx.positionId < $.nextPositionId, Errors.MALFORMED_POS_ID);
        _require(_executor == pos.owner, Errors.NOT_POS_OWNER);

        // Check current worker stability.
        stack.pool = MultiModalWorkerStorage._readPoolData();
        _compareSpotAndOracle(stack.pool);

        // Withdraw liquidity, repay debt, and swap in accordance to parameters.
        stack.sharesToRemove = (pos.positionShares * _ctx.positionBps) / 10000;
        (, stack.amount0, stack.amount1) = _divestAndRemoveLiquidity(stack.pool, _sharesToTokens(stack.sharesToRemove));
        pos.positionShares -= stack.sharesToRemove;
        $.totalPositionShares -= stack.sharesToRemove;
        _require(stack.amount0 >= _ctx.minToken0Out && stack.amount1 >= _ctx.minToken1Out, Errors.TOO_MUCH_SLIPPAGE);

        if (_ctx.token0Repay > 0) {
            stack.borrowed = pos.debtShare0 > 0;
            if (stack.borrowed) {
                uint112 totalDebtValue = ILimeDiamond(LIMESTONE_DIAMOND).debtShareToVal(pos.debt0PoolId, pos.debtShare0);
                uint112 repayAmount = FixedPointMathLib.min(uint112(_ctx.token0Repay), totalDebtValue).u112();
                uint112 repayShares = ILimeDiamond(LIMESTONE_DIAMOND).debtValToShare(pos.debt0PoolId, repayAmount);
                pos.debtShare0 -= repayShares;
                if (repayAmount > stack.amount0) {
                    uint256 needed = repayAmount - stack.amount0;
                    _swapForExactAmount(stack.pool.token1, stack.pool.token0, needed);
                    stack.amount0 = 0;
                    stack.amount1 = stack.pool.token1.balanceOf(address(this));
                } else {
                    stack.amount0 -= repayAmount;
                }
            }
        }

        if (_ctx.token1Repay > 0) {
            stack.borrowed = pos.debtShare1 > 0;
            if (stack.borrowed) {
                uint112 totalDebtValue = ILimeDiamond(LIMESTONE_DIAMOND).debtShareToVal(pos.debt1PoolId, pos.debtShare1);
                uint112 repayAmount = FixedPointMathLib.min(uint112(_ctx.token1Repay), totalDebtValue).u112();
                uint112 repayShares = ILimeDiamond(LIMESTONE_DIAMOND).debtValToShare(pos.debt1PoolId, repayAmount);
                pos.debtShare1 -= repayShares;
                if (repayAmount > stack.amount1) {
                    uint256 needed = repayAmount - stack.amount1;
                    _swapForExactAmount(stack.pool.token0, stack.pool.token1, needed);
                    stack.amount1 = 0;
                    stack.amount0 = stack.pool.token1.balanceOf(address(this));
                } else {
                    stack.amount1 -= repayAmount;
                }
            }
        }

        if (_ctx.minimalWithdrawal) {
            stack.pool.token0.safeTransfer(pos.owner, stack.amount0);
            stack.pool.token1.safeTransfer(pos.owner, stack.amount1);
        } else {
            uint256 tokensReceived =
                _swapToSide(stack.pool, _ctx.side, _ctx.side > 0 ? stack.amount0 : stack.amount1, 0);
            _ctx.side > 0
                ? stack.pool.token1.safeTransfer(pos.owner, tokensReceived)
                : stack.pool.token0.safeTransfer(pos.owner, tokensReceived);
        }

        return (stack.amount0, stack.amount1, pos.debtShare0, pos.debtShare1);
    }

    function repayDebt(address _executor, uint256 _positionId, uint256 _repayToken0, uint256 _repayToken1)
        external
        override
        onlyDiamond
        returns (uint112, uint112)
    {
        MultiModalWorkerStorage.Layout storage $ = MultiModalWorkerStorage.layout();
        MultiModalPosition storage pos = $.positions[_positionId];
        MultiModalWorkerStorage.LiquidityPool memory pool = MultiModalWorkerStorage._readPoolData();

        // Validate current position access control.
        _require(_positionId < $.nextPositionId, Errors.MALFORMED_POS_ID);
        _require(_executor == pos.owner, Errors.NOT_POS_OWNER);

        // Access user assets for repayment.
        address[] memory repayAssets = new address[](2);
        uint256[] memory repayAmounts = new uint256[](2);
        (repayAssets[0], repayAssets[1], repayAmounts[0], repayAmounts[1]) =
            (pool.token0, pool.token1, _repayToken0, _repayToken1);
        ILimeDiamond(LIMESTONE_DIAMOND).accessAssets(pos.owner, repayAssets, repayAmounts);

        // Check stability.
        _compareSpotAndOracle(pool);

        // Calculate repayment values and transfer to the lending pool.
        (uint112 repDebtShare0, uint112 repDebtShare1) = (0, 0);
        if (_repayToken0 > 0) {
            uint112 totalDebt = ILimeDiamond(LIMESTONE_DIAMOND).debtShareToVal(pos.debt0PoolId, pos.debtShare0);
            uint112 repayAmount = FixedPointMathLib.min(_repayToken0.u112(), totalDebt).u112();
            repDebtShare0 = ILimeDiamond(LIMESTONE_DIAMOND).debtValToShare(pos.debt0PoolId, repayAmount);
            pos.debtShare0 -= repDebtShare0;
            pool.token0.safeTransfer(LIMESTONE_DIAMOND, repayAmount);
        }

        if (_repayToken1 > 0) {
            uint112 totalDebt = ILimeDiamond(LIMESTONE_DIAMOND).debtShareToVal(pos.debt1PoolId, pos.debtShare1);
            uint112 repayAmount = FixedPointMathLib.min(_repayToken1.u112(), totalDebt).u112();
            repDebtShare1 = ILimeDiamond(LIMESTONE_DIAMOND).debtValToShare(pos.debt1PoolId, repayAmount);
            pos.debtShare1 -= repDebtShare1;
            pool.token1.safeTransfer(LIMESTONE_DIAMOND, repayAmount);
        }

        // Refund any remaining assets that may be left.
        (uint256 currentToken0, uint256 currentToken1) =
            (pool.token0.balanceOf(address(this)), pool.token1.balanceOf(address(this)));
        if (currentToken0 > 0) pool.token0.safeTransfer(pos.owner, currentToken0);
        if (currentToken1 > 0) pool.token1.safeTransfer(pos.owner, currentToken1);

        return (repDebtShare0, repDebtShare1);
    }

    /// @notice Partially liquidates a position on the worker.
    /// @param _liquidator Liquidator responsible for carrying out the liquidation.
    /// @param _ctx Context of the liquidation call.
    function liquidate(address _liquidator, V2LikePositionLiquidationContext calldata _ctx)
        external
        override
        onlyDiamond
        returns (uint112 newDebtShare0, uint112 newDebtShare1)
    {
        MultiModalWorkerStorage.Layout storage $ = MultiModalWorkerStorage.layout();
        MultiModalPosition storage pos = $.positions[_ctx.positionId];
        LiquidateStack memory stack;
        MultiModalWorkerStorage.LiquidityPool memory pool = MultiModalWorkerStorage._readPoolData();

        // Validate current worker stability.
        _compareSpotAndOracle(pool);

        // Transfer repayment assets in.
        address[] memory repayAssets = new address[](2);
        uint256[] memory repayAmounts = new uint256[](2);
        (repayAssets[0], repayAssets[1], repayAmounts[0], repayAmounts[1]) =
            (pool.token0, pool.token1, _ctx.token0RepayIn, _ctx.token1RepayIn);
        ILimeDiamond(LIMESTONE_DIAMOND).accessAssets(_liquidator, repayAssets, repayAmounts);

        // Calculate total position debt.
        stack.debt0 =
            pos.debtShare0 > 0 ? ILimeDiamond(LIMESTONE_DIAMOND).debtShareToVal(pos.debt0PoolId, pos.debtShare0) : 0;
        stack.debt1 =
            pos.debtShare1 > 0 ? ILimeDiamond(LIMESTONE_DIAMOND).debtShareToVal(pos.debt1PoolId, pos.debtShare1) : 0;

        // Perform repayment.
        if (stack.debt0 > 0) {
            uint112 toRepay = FixedPointMathLib.min(_ctx.token0RepayIn, stack.debt0).u112();
            uint112 sharesDeducted = ILimeDiamond(LIMESTONE_DIAMOND).debtValToShare(pos.debt0PoolId, toRepay);
            pos.debtShare0 -= sharesDeducted;
            pool.token0.safeTransfer(LIMESTONE_DIAMOND, toRepay);
            stack.token0Repaid = toRepay;
        } else {
            if (_ctx.token0RepayIn > 0) pool.token0.safeTransfer(_liquidator, _ctx.token0RepayIn);
        }

        if (stack.debt1 > 0) {
            uint112 toRepay = FixedPointMathLib.min(_ctx.token1RepayIn, stack.debt1).u112();
            uint112 sharesDeducted = ILimeDiamond(LIMESTONE_DIAMOND).debtValToShare(pos.debt1PoolId, toRepay);
            pos.debtShare1 -= sharesDeducted;
            pool.token1.safeTransfer(LIMESTONE_DIAMOND, toRepay);
            stack.token1Repaid = toRepay;
        } else {
            if (_ctx.token1RepayIn > 0) pool.token1.safeTransfer(_liquidator, _ctx.token1RepayIn);
        }

        // Reuse `stack.debt0` and `stack.debt1` to store the new total debt share of the position.
        stack.debt0 = pos.debtShare0;
        stack.debt1 = pos.debtShare1;

        // Withdraw assets from position.
        stack.totalTokens = _sharesToTokens(pos.positionShares);
        (stack.shares, stack.amount0, stack.amount1) =
            _divestAndRemoveLiquidity(pool, (stack.totalTokens * _ctx.positionBps) / 10000);
        pos.positionShares -= stack.shares;
        $.totalPositionShares -= stack.shares;

        // Calculate liquidator cut and the amount we need to refund.
        (stack.fee0, stack.fee1) = ((stack.amount0 * 800 / 10000), (stack.amount1 * 800 / 10000));
        stack.amount0 -= stack.fee0;
        stack.amount1 -= stack.fee1;
        if (_ctx.token0RepayIn > stack.token0Repaid) {
            uint256 token0Held = pool.token0.balanceOf(address(this));
            uint256 refundValue = (_ctx.token0RepayIn - stack.token0Repaid) + stack.fee0;
            if (refundValue > token0Held) {
                // Swap token0 to token1.
                _swapForExactAmount(pool.token0, pool.token1, refundValue - token0Held);
            } else {
                pool.token0.safeTransfer(_liquidator, refundValue);
            }
        }

        if (_ctx.token1RepayIn > stack.token1Repaid) {
            uint256 token1Held = pool.token1.balanceOf(address(this));
            uint256 refundValue = (_ctx.token1RepayIn - stack.token1Repaid) + stack.fee1;
            if (refundValue > token1Held) {
                // Swap token1 to token0.
                _swapForExactAmount(pool.token1, pool.token0, refundValue - token1Held);
                pool.token1.safeTransfer(_liquidator, refundValue);
            } else {
                pool.token1.safeTransfer(_liquidator, refundValue);
            }
        }

        // Transfer any remaining assets back to the user.
        pool.token0.safeTransfer(pos.owner, pool.token0.balanceOf(address(this)));
        pool.token1.safeTransfer(pos.owner, pool.token1.balanceOf(address(this)));

        return (stack.debt0, stack.debt1);
    }

    function reinvest() external virtual;

    function getPosition(uint256 _positionId) external view override returns (MultiModalPosition memory) {
        return MultiModalWorkerStorage.layout().positions[_positionId];
    }

    function calculatePositionValue(uint256 _posId) external view virtual returns (uint256, uint256);

    function _liquidateReward() internal {
        address[] memory rewards = MultiModalWorkerStorage._readPoolData().rewardTokens;
        uint256 nIndices = rewards.length;
        uint256[] memory rewardBalances = new uint256[](nIndices);
        for (uint256 i; i < nIndices;) {
            address reward = rewards[i];
            uint256 rewardBalance = reward.balanceOf(address(this));

            // Check if the reward is enough for liquidation.
            if (rewardBalance < 1e12) {
                return;
            }

            // Collect performance fees.
            _notifyProfitInRewardToken(reward, rewardBalance);

            // Push the balance after notifying fees.
            rewardBalances[i] = reward.balanceOf(address(this));

            // forgefmt: disable-next-line
            unchecked { ++i; }
        }

        _handleLiquidation(rewardBalances);
    }

    /// @dev Collects protocol fees and sends them to the Controller.
    /// @param _reward Reward token to collect fees from.
    /// @param _rewardBalance The amount of rewards generated that is to have fees taken from.
    function _notifyProfitInRewardToken(address _reward, uint256 _rewardBalance) internal {
        uint256 profitSharingNumerator = ILimeDiamond(LIMESTONE_DIAMOND).reinvestmentFeeNumerator();
        if (_rewardBalance > 0) {
            uint256 feeAmount = (_rewardBalance * profitSharingNumerator) / 10000;
            _reward.safeTransfer(owner(), feeAmount);
        }
    }

    function _handleLiquidation(uint256[] memory _balances) internal virtual;

    function _addLiquidity(
        MultiModalWorkerStorage.LiquidityPool memory _pool,
        uint256 _token0In,
        uint256 _token1In,
        uint256 _minLiquidity
    ) internal virtual returns (uint256);

    function _divestAndRemoveLiquidity(MultiModalWorkerStorage.LiquidityPool memory _pool, uint256 _liquidity)
        internal
        virtual
        returns (uint112, uint256, uint256);

    function _investLiquidity(uint256 _amount) internal virtual returns (uint112);

    function _swapToSide(
        MultiModalWorkerStorage.LiquidityPool memory _pool,
        uint8 _side,
        uint256 _amount,
        uint256 _desired
    ) internal virtual returns (uint256);

    function _swapForExactAmount(address _tokenIn, address _tokenOut, uint256 _desired) internal virtual;

    function _validateReserves(MultiModalWorkerStorage.LiquidityPool memory _pool) internal view virtual;

    function _compareSpotAndOracle(MultiModalWorkerStorage.LiquidityPool memory _pool) internal view virtual;

    function _tokensToShares(uint256 _tokens) internal view virtual returns (uint112);

    function _sharesToTokens(uint112 _shares) internal view virtual returns (uint112);
}
