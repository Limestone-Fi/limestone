// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Ownable} from "@solidstate/access/ownable/Ownable.sol";
import {SolidStateERC20} from "@solidstate/token/ERC20/SolidStateERC20.sol";
import {Initializable} from "@solidstate/security/initializable/Initializable.sol";
import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";
import {LendingPoolLib} from "./lib/LendingPoolLib.sol";
import {Cast} from "./lib/Cast.sol";
import {Errors, _require} from "./lib/Errors.sol";
import {IWorker} from "./interfaces/IWorker.sol";
import {
    LendingPoolStorage,
    ILendingPool,
    IWarchest,
    Market,
    Position,
    LendingPoolConfig,
    InterestRateModel,
    AuthType
} from "./LendingPoolStorage.sol";

/// @title Limestone Lending Pool Facet
/// @author Chainvisions
/// @notice Lending pool contract for Limestone.

contract LendingPool is ILendingPool, Initializable, Ownable {
    using LendingPoolLib for uint256;
    using SafeTransferLib for address;
    using Cast for uint256;

    /// @notice Context for `manageAuthList` calls.
    struct AuthUpdate {
        /// @notice Address who's authorization is being update for.
        address authority;
        /// @notice Type of authorization to give to the authority.
        AuthType authType;
        /// @notice Whether or not the authority is authorized to hold the auth role.
        bool authorized;
    }

    /// @dev Min amount of shares that must be minted. Prevents potential share inflation exploits.
    uint112 internal constant MIN_SHARES = 10 ** 3;

    /// @notice Initializes the lending pool facet.
    function initialize() external initializer {
        LendingPoolStorage.layout().nextPositionID = 1;
        _setOwner(msg.sender);
    }

    /// @notice Reinitialization function for migrating between upgrades.
    function scrubStorage() external {
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        if ($.nextPositionID == 0) $.nextPositionID = 1;
    }

    /// @notice Deposits tokens into a lending pool.
    /// @param _poolId ID of the lending pool to deposit into.
    /// @param _amount Amount of tokens to deposit into the lending pool.
    function deposit(uint256 _poolId, uint256 _amount) external override {
        _poolId._accrue();
        _deposit(_poolId, msg.sender, _amount);
    }

    /// @notice Withdraws tokens from a lending pool.
    /// @param _poolId ID of the lending pool to withdraw from.
    /// @param _shares Amount of lending pool shares to burn.
    function withdraw(uint256 _poolId, uint256 _shares) external override {
        _poolId._accrue();
        Market storage pool = LendingPoolStorage.layout().pools[_poolId];
        uint256 amount = ((_shares * totalTokens(_poolId)) / uint256(pool.totalShares));
        pool.warchest.burn(msg.sender, _shares);
        unchecked {
            // Overflow is literally impossible as total shares will always be MIN_SHARES at minimum.
            pool.totalShares -= _shares.u112();
        }
        pool.warchest.withdrawReserves(msg.sender, amount);
        emit Withdraw(_poolId, msg.sender, _shares, amount);
    }

    /// @notice Adds a new market to the lending pool.
    /// @param _config Configuration for the new lending market.
    /// @param _underlying Underlying token of the new lending market.
    /// @param _warchest Warchest contract that holds custody of the market's assets.
    /// @param _seedLiquidity Initial liquidity used for seeding the lending market. Can be `0` to avoid seeding.
    function addLendingPool(
        LendingPoolConfig calldata _config,
        address _underlying,
        address _warchest,
        uint256 _seedLiquidity
    ) external onlyOwner {
        // Push lending pool to storage.
        LendingPoolStorage.Layout storage l = LendingPoolStorage.layout();
        uint256 poolId = l.totalLendingPools;
        l.pools[poolId] = Market({
            underlying: _underlying,
            lastAccrueTime: uint32(block.timestamp),
            minDebtSize: _config.minDebtSize,
            reservePoolBps: _config.reservePoolBps,
            liquidateBps: _config.liquidateBps,
            interestRateModel: _config.interestRateModel,
            warchest: IWarchest(_warchest),
            totalShares: 0,
            delegatedDebtAvailable: type(uint88).max,
            globalDebtValue: 0,
            globalDebtShare: 0,
            reservePool: 0
        });
        l.totalLendingPools++;
        emit MarketCreated(poolId, _underlying, _warchest, _config);

        // Add seed liquidity to the pool if needed.
        if (_seedLiquidity > 0) {
            poolId._accrue();
            _deposit(poolId, msg.sender, _seedLiquidity);
        }
    }

    /// @notice Sets the total amount of delegated debt available for a specific market.
    /// @param _poolId ID of the lending pool to set the delegated debt limit for.
    /// @param _debt Total delegated debt amount to allocate.
    function setDelegatedDebt(uint256 _poolId, uint88 _debt) external onlyOwner {
        Market storage pool = LendingPoolStorage.layout().pools[_poolId];
        pool.delegatedDebtAvailable = _debt;
    }

    /// @notice Sets whether liquidations are permissioned or not.
    /// @param _permissionedLiquidation Whether or not liquidatons are restricted to `authorizedLiquidators`.
    function setPermissionedLiquidation(bool _permissionedLiquidation) external onlyOwner {
        LendingPoolStorage.layout().permissionedLiquidation = _permissionedLiquidation;
    }

    /// @notice Adds new workers to the lending pool. Can also be used to configure existing ones.
    /// @param _workers Workers to add to the lending pool.
    /// @param _params Parameters used to configure each of the workers.
    function addWorkers(address[] calldata _workers, LendingPoolStorage.WorkerDebtParams[] calldata _params)
        external
        onlyOwner
    {
        LendingPoolStorage.Layout storage l = LendingPoolStorage.layout();
        for (uint256 i; i < _workers.length;) {
            l.workerDebtParams[_workers[i]] = _params[i];
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }
    }

    /// @notice Manages the status of a list of addresses holding specific authorization related roles.
    /// @param _ctx Context containing each update to be made to a list of authorized addresses.
    function manageAuthList(AuthUpdate[] calldata _ctx) external onlyOwner {
        LendingPoolStorage.Layout storage l = LendingPoolStorage.layout();
        for (uint256 i; i < _ctx.length;) {
            AuthUpdate memory ctx = _ctx[i];
            if (ctx.authType == AuthType.Keeper) {
                l.authorizedKeepers[ctx.authority] = ctx.authorized;
            } else if (ctx.authType == AuthType.Liquidator) {
                l.authorizedLiquidators[ctx.authority] = ctx.authorized;
            } else {
                l.authorizedContractBorrowers[ctx.authority] = ctx.authorized;
            }

            // forgefmt: disable-next-line
            unchecked {++i;}
        }
    }

    /// @notice Collects tokens generated from the lending pool reserves (protocol fees).
    /// @param _destination Address to send tokens from the reserves to.
    /// @param _pools Pools to collect reserves from.
    /// @param _amounts Amounts of tokens to collect from each of the reserves in `_pools`.
    function collectReserves(address _destination, uint256[] calldata _pools, uint112[] calldata _amounts)
        external
        onlyOwner
    {
        for (uint256 i; i < _pools.length;) {
            Market storage pool = LendingPoolStorage.layout().pools[i];
            pool.reservePool -= _amounts[i];
            pool.warchest.withdrawReserves(_destination, _amounts[i]);
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }
    }

    /// @notice Distributes pending lending pool reserves (collected protocol fees) to existing lenders in the pool.
    /// @param _pools Pools to distribute the reserves of.
    /// @param _amounts Amounts of tokens from each reserve in `_pools` to distribute to lenders.
    function distributeReserves(uint256[] calldata _pools, uint112[] calldata _amounts) external onlyOwner {
        for (uint256 i; i < _pools.length;) {
            LendingPoolStorage.layout().pools[_pools[i]].reservePool -= _amounts[i];
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }
    }

    /// @notice Salvages any tokens stuck in the contract.
    /// @param _token Token to salvage.
    /// @param _to Address to send salvaged tokens to.
    /// @param _amount Amount of tokens to salvage.
    function salvage(address _token, address _to, uint256 _amount) external onlyOwner {
        _token.safeTransfer(_to, _amount);
    }

    /// @notice Fetches `permissionedLiquidation` from storage.
    /// @return Whether or not liquidations are permissioned.
    function permissionedLiquidation() external view override returns (bool) {
        return LendingPoolStorage.layout().permissionedLiquidation;
    }

    /// @notice Fetches `authorizedLiquidators[_liquidator]` from storage.
    /// @param _liquidator The liquidator to fetch the authorization for.
    /// @return Whether or not `_liquidator` can liquidate positions.
    function authorizedLiquidators(address _liquidator) external view override returns (bool) {
        return LendingPoolStorage.layout().authorizedLiquidators[_liquidator];
    }

    /// @notice Fetches `authorizedKeepers[_keeper]` from storage.
    /// @param _keeper The keeper to fetch the authorization for.
    /// @return Whether or not `_keeper` can reinvest on workers.
    function authorizedKeepers(address _keeper) external view override returns (bool) {
        return LendingPoolStorage.layout().authorizedKeepers[_keeper];
    }

    /// @notice Fetches the stored data on a position.
    /// @param _posId ID of the position to fetch.
    /// @return The info about the position.
    function positions(uint256 _posId) external view override returns (Position memory) {
        return LendingPoolStorage.layout().positions[_posId];
    }

    /// @notice Fetches all of the stored lending pools.
    /// @return An array of all lending pools on the contract.
    function pools() external view override returns (Market[] memory) {
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        Market[] memory all = new Market[]($.totalLendingPools);
        for (uint256 i; i < $.totalLendingPools;) {
            all[i] = $.pools[i];
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }
        return all;
    }

    /// @notice Calculates the amount of tokens a specific amount of debt shares are worth.
    /// @param _poolId ID of the pool to calculate share values for.
    /// @param _debtShare The amount of debt shares to calculate the value of.
    /// @return The amount of `underlying` tokens `_debtShares` is worth.
    function debtShareToVal(uint256 _poolId, uint112 _debtShare) external view returns (uint112) {
        return LendingPoolLib._debtShareToVal(_poolId, _debtShare);
    }

    /// @notice Calculates the amount of pool debt shares a specific amount of `underlying` tokens are worth.
    /// @param _poolId ID of the pool to calculate shares from.
    /// @param _debtVal The amount of `underlying` tokens to calculate the shares value of.
    /// @return The amount of shares that `_debtVal` tokens are worth.
    function debtValToShare(uint256 _poolId, uint112 _debtVal) external view returns (uint112) {
        return LendingPoolLib._debtValToShare(_poolId, _debtVal);
    }

    /// @notice Calculates the pending amount of interest that will be accrued for a specific pool.
    /// @param _poolId Pool ID to calculate the pending interest for.
    /// @return The pending amount of interest that will be accrued by the pool.
    function pendingInterest(uint256 _poolId) external view returns (uint256) {
        return _poolId._pendingInterest();
    }

    /// @notice Calculates the current value of a specific position.
    /// @param _id ID of the position to calculate the value of.
    /// @return The total value of the position and the total amount of debt held by the position.
    function positionInfo(uint256 _id) public view returns (uint256, uint112) {
        Position storage pos = LendingPoolStorage.layout().positions[_id];
        return (IWorker(pos.worker).health(_id), LendingPoolLib._debtShareToVal(pos.poolId, pos.debtShare));
    }

    /// @notice Calculates the total amount of tokens held by a specific lending pool.
    /// @param _poolId ID of the pool to calculate the total tokens of.
    /// @return How many tokens are held by the lending pool, including pool debts.
    function totalTokens(uint256 _poolId) public view returns (uint256) {
        Market storage lendingPool = LendingPoolStorage.layout().pools[_poolId];
        return (
            (lendingPool.warchest.underlyingBalanceWithInvestment() + uint256(lendingPool.globalDebtValue))
                - uint256(lendingPool.reservePool)
        );
    }

    function _deposit(uint256 _poolId, address _depositor, uint256 _amount) internal {
        LendingPoolStorage.Layout storage l = LendingPoolStorage.layout();
        Market storage pool = l.pools[_poolId];
        pool.underlying.safeTransferFrom(_depositor, address(pool.warchest), _amount);
        uint256 total = (totalTokens(_poolId) - _amount);
        uint256 share = total == 0 ? _amount - MIN_SHARES : ((_amount * uint256(pool.totalShares)) / total);
        if (total == 0) {
            unchecked {
                // We reserve a small amount (10 ** 3) of shares to prevent inflation attacks.
                pool.warchest.mint(address(pool.warchest), MIN_SHARES);
                // @dev type(uint112).max is VERY big so the overflow risk really just isn't there.
                pool.totalShares += MIN_SHARES;
            }
        }
        pool.warchest.mint(msg.sender, share);
        // forgefmt: disable-next-line
        unchecked {pool.totalShares += share.u112();}// @dev same as the other unchecked increase.
    }
}
