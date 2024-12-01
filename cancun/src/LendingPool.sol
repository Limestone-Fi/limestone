// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Math} from "@solidstate/utils/Math.sol";
import {ReentrancyGuard} from "@solidstate/security/reentrancy_guard/ReentrancyGuard.sol";
import {Ownable} from "@solidstate/access/ownable/Ownable.sol";
import {SolidStateERC20} from "@solidstate/token/ERC20/SolidStateERC20.sol";
import {Initializable} from "@solidstate/security/initializable/Initializable.sol";
import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";
import {LibZip} from "solady/src/utils/LibZip.sol";
import {LibTransient} from "solady/src/utils/LibTransient.sol";
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
    InterestRateModel
} from "./LendingPoolStorage.sol";

/// @title Limestone Lending Pool Facet
/// @author Chainvisions
/// @notice Lending pool contract for Limestone.

contract LendingPool is ILendingPool, Initializable, Ownable {
    using SafeTransferLib for address;
    using Cast for uint256;
    using LibTransient for LibTransient.TBytes;

    /// @notice Context for `doHardWork` calls. Provides necessary parameters.
    struct WorkContext {
        /// @notice ID of the lending pool to borrow assets from.
        /// @dev Invariant: poolId == workerDebtParams[worker].authorizedPoolId
        uint256 poolId;
        /// @notice The worker executed for the position.
        /// @dev Invariant: loan > 0 -> workerDebtParams[worker].borrowable == true
        address worker;
        /// @notice The amount of tokens to deposit into the position (or borrow against).
        uint256 amountIn;
        /// @notice The amount of tokens to borrow for the position.
        /// @dev Invariant: (amountIn + loan) * workFactor >= (loan + debt) * 10000
        uint256 loan;
        /// @notice The max amount of tokens to return back to the lending pool for paying off debt.
        uint256 maxReturn;
        /// @notice Operation data used for contextualizing worker execution.
        bytes data;
    }

    /// @dev Min amount of shares that must be minted. Prevents potential share inflation exploits.
    uint112 internal constant MIN_SHARES = 10 ** 3;

    /// @notice Used to restrict calls to EOAs only to avoid flashloan interactions.
    modifier onlyEOA() {
        _require(msg.sender == tx.origin, Errors.CALLER_NOT_EOA);
        _;
    }

    /// @notice Initializes the lending pool facet.
    function initialize() external initializer {
        LendingPoolStorage.layout().nextPositionID = 1;
        _setOwner(msg.sender);
    }

    /// @notice Deposits tokens into a lending pool.
    /// @param _poolId ID of the lending pool to deposit into.
    /// @param _amount Amount of tokens to deposit into the lending pool.
    function deposit(uint256 _poolId, uint256 _amount) external override {
        _accrue(_poolId);
        _deposit(_poolId, msg.sender, _amount);
    }

    /// @notice Withdraws tokens from a lending pool.
    /// @param _poolId ID of the lending pool to withdraw from.
    /// @param _shares Amount of lending pool shares to burn.
    function withdraw(uint256 _poolId, uint256 _shares) external override {
        _accrue(_poolId);
        Market storage pool = LendingPoolStorage.layout().pools[_poolId];
        uint256 amount = ((_shares * totalTokens(_poolId)) / pool.totalShares);
        pool.warchest.burn(msg.sender, _shares);
        unchecked {
            // Overflow is literally impossible as total shares will always be MIN_SHARES at minimum.
            pool.totalShares -= _shares.u112();
        }
        pool.warchest.withdrawReserves(msg.sender, amount);
        emit Withdraw(_poolId, msg.sender, _shares, amount);
    }

    /// @notice Used by workers to access any additional approved assets from a specific user. Used for two sided liquidity provision.
    /// @param _token Token to request from user.
    /// @param _requestedAmount Amount requested to transfer from the user.
    function accessUserAssets(address _token, uint256 _requestedAmount) external override {
        LendingPoolStorage.Layout storage l = LendingPoolStorage.layout();
        LibTransient.TBytes storage scopePtr = LibTransient.tBytes(LendingPoolStorage.EXECUTION_SCOPE_SLOT);
        LendingPoolStorage.ExecScope memory scope = abi.decode(scopePtr.get(), (LendingPoolStorage.ExecScope));
        _require(scope.positionId != type(uint32).max, Errors.NOT_POSITION_IN_EXEC);
        _require(msg.sender == scope.worker, Errors.NOT_WORKER_IN_EXEC);
        _token.safeTransferFrom(l.positions[scope.positionId].owner, scope.worker, _requestedAmount);
    }

    /// @notice Borrows assets from the lending pool and opens a new leveraged yield farming position.
    /// @param _id ID of the position being managed. `0` can be inputted to create a brand new position.
    /// @param _ctx Context of the position. Used to discern parameters related to the position.
    function doHardWork(uint256 _id, WorkContext calldata _ctx) external onlyEOA {
        _accrue(_ctx.poolId);
        LendingPoolStorage.Layout storage l = LendingPoolStorage.layout();
        Market storage pool = l.pools[_ctx.poolId];
        LibTransient.TBytes storage scopePtr = LibTransient.tBytes(LendingPoolStorage.EXECUTION_SCOPE_SLOT);
        pool.underlying.safeTransferFrom(msg.sender, address(pool.warchest), _ctx.amountIn);

        // Validate the current position being managed.
        if (_id == 0) {
            // forgefmt: disable-next-line
            unchecked {_id = l.nextPositionID++;}// Overflow risk unlikely. Can even intervene before we hit 4.2b positions.
            l.positions[_id].worker = _ctx.worker;
            l.positions[_id].owner = msg.sender;
        } else {
            _require(_id < l.nextPositionID, Errors.MALFORMED_POS_ID);
            _require(l.positions[_id].worker == _ctx.worker, Errors.NOT_POS_WORKER);
            _require(l.positions[_id].owner == msg.sender, Errors.NOT_POS_OWNER);
        }
        emit Borrow(_ctx.poolId, _id, _ctx.loan);
        scopePtr.set(abi.encode(LendingPoolStorage.ExecScope({positionId: uint32(_id), worker: _ctx.worker})));

        // Check worker parameters to ensure that debt can be accepted by the worker and clear all debt for recalculation.
        LendingPoolStorage.WorkerDebtParams storage workerParams = l.workerDebtParams[_ctx.worker];
        _require(workerParams.authorizedPoolId == _ctx.poolId, Errors.WORKER_NOT_AUTHORIZED);
        _require(
            _ctx.loan == 0 || (workerParams.borrowable && IWorker(_ctx.worker).healthcheck()), Errors.NOT_ACCEPTING_DEBT
        );
        uint256 debt = (_removeDebt(_ctx.poolId, _id) + _ctx.loan);

        // Send assets to the worker to execute the leveraged yield farming position and calculate tokens received back.
        uint256 back;
        uint256 toInvest = (_ctx.amountIn + _ctx.loan);
        uint256 warchestAssets = pool.underlying.balanceOf(address(pool.warchest));
        bool sufficientLiquidity = warchestAssets >= toInvest;
        uint256 tokensBefore =
            (sufficientLiquidity ? warchestAssets : pool.warchest.underlyingBalanceWithInvestment() - toInvest);
        pool.warchest.withdrawReserves(_ctx.worker, toInvest);
        IWorker(_ctx.worker).work(_id, msg.sender, debt, _ctx.data);
        back = (
            sufficientLiquidity
                ? pool.underlying.balanceOf(address(pool.warchest))
                : pool.warchest.underlyingBalanceWithInvestment() - tokensBefore
        );

        // Update the position accordingly by repaying any current debt.
        uint256 lessDebt = Math.min(debt, Math.min(back, _ctx.maxReturn));
        debt = (debt - lessDebt);
        if (debt > 0) {
            _require(debt >= pool.minDebtSize, Errors.DEBT_TOO_SMALL);
            uint256 health = IWorker(_ctx.worker).health(_id);
            _require((health * workerParams.workFactor) >= (debt * 10000), Errors.UNHEALTHY_POSITION);
            _addDebt(_ctx.poolId, _id, debt.u112());
        }
        // Send any surplus tokens to the user.
        if (back > lessDebt) pool.warchest.withdrawReserves(msg.sender, back - lessDebt);
    }

    /// @notice Increases the supplied collateral for a farming position.
    /// @param _posId ID of the position to add collateral to.
    /// @param _amount Amount of tokens to add to the position.
    /// @param _data Operation data used for contextualizing worker execution.
    function increaseCollateral(uint256 _posId, uint256 _amount, bytes calldata _data) external onlyEOA {
        LendingPoolStorage.Layout storage l = LendingPoolStorage.layout();
        Position storage pos = l.positions[_posId];
        Market storage pool = l.pools[pos.poolId];
        LibTransient.TBytes storage scopePtr = LibTransient.tBytes(LendingPoolStorage.EXECUTION_SCOPE_SLOT);
        _require(_posId != 0 && _posId < l.nextPositionID, Errors.MALFORMED_POS_ID);
        _require(msg.sender == pos.owner, Errors.NOT_POS_OWNER);
        _accrue(pos.poolId);
        scopePtr.set(abi.encode(LendingPoolStorage.ExecScope({positionId: uint32(_posId), worker: pos.worker})));
        uint256 initialHealth = IWorker(pos.worker).health(_posId);
        _require(initialHealth != 0, Errors.INACTIVE_POSITION);

        // Ensure that there is no manipulation going on and add collateral to the position.
        _require(IWorker(pos.worker).healthcheck(), Errors.WORKER_HEALTHCHECK_FAILED);
        uint256 assetsBefore = pool.warchest.underlyingBalanceWithInvestment();
        pool.underlying.safeTransferFrom(msg.sender, pos.worker, _amount);
        IWorker(pos.worker).work(_posId, msg.sender, 0, _data);
        uint256 assetsReceived = pool.warchest.underlyingBalanceWithInvestment() - assetsBefore;
        _require(assetsReceived == 0, Errors.CANNOT_WITHDRAW_IF_INCREASING);
        uint256 healthAfter = IWorker(pos.worker).health(_posId);
        _require(healthAfter > initialHealth, Errors.HEALTH_DID_NOT_INCREASE);

        // Evaluate position health to ensure it's not nearly underwater.
        uint256 currentDebt = debtShareToVal(pos.poolId, pos.debtShare);
        _require(IWorker(pos.worker).healthcheck(), Errors.WORKER_HEALTHCHECK_FAILED);
        _require(
            (currentDebt * 10000) <= healthAfter * (l.workerDebtParams[pos.worker].killFactor - 100),
            Errors.POSITION_NEAR_LIQ_THRESHOLD
        );

        emit IncreaseCollateral(pos.poolId, _posId, _amount, initialHealth, healthAfter);
    }

    /// @notice Liquidates an underwater position if the debt ratio is at the kill factor.
    /// @param _id ID of the position to liquidate.
    function kill(uint256 _id) external onlyEOA {
        LendingPoolStorage.Layout storage l = LendingPoolStorage.layout();
        Position storage pos = l.positions[_id];
        Market storage pool = l.pools[pos.poolId];
        _accrue(pos.poolId);

        // Check if whether or not the position is able to be liquidated.
        _require(pos.debtShare > 0, Errors.NO_DEBT);
        uint256 debt = _removeDebt(pos.poolId, _id);
        uint256 health = IWorker(pos.worker).health(_id);
        _require((health * l.workerDebtParams[pos.worker].killFactor) < (debt * 10000), Errors.CANT_LIQUIDATE);

        // Liquidate the position and calculated the amount liquidated.
        uint256 tokensBefore = pool.warchest.underlyingBalanceWithInvestment();
        IWorker(pos.worker).liquidate(_id);
        uint256 tokensReceived = (pool.warchest.underlyingBalanceWithInvestment() - tokensBefore);
        uint256 liqFee = ((tokensReceived * pool.liquidateBps) / 10000);
        uint256 netRecv = (tokensReceived - liqFee);

        // Transfer liquidation reward to the liquidator and refund the original position owner.
        if (liqFee > 0) pool.warchest.withdrawReserves(msg.sender, liqFee);
        uint256 left = netRecv > debt ? netRecv - debt : 0;
        if (left > 0) pool.warchest.withdrawReserves(pos.owner, left);
        emit Kill(_id, msg.sender, liqFee, left);
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
        l.pools.push(
            Market({
                underlying: _underlying,
                lastAccrueTime: uint32(block.timestamp),
                minDebtSize: _config.minDebtSize,
                reservePoolBps: _config.reservePoolBps,
                liquidateBps: _config.liquidateBps,
                interestRateModel: _config.interestRateModel,
                warchest: IWarchest(_warchest),
                totalShares: 0,
                globalDebtValue: 0,
                globalDebtShare: 0,
                reservePool: 0
            })
        );
        uint256 poolId;
        unchecked {
            poolId = l.pools.length - 1;
        }
        emit MarketCreated(poolId, _underlying, _warchest, _config);

        // Add seed liquidity to the pool if needed.
        if (_seedLiquidity > 0) {
            _accrue(poolId);
            _deposit(poolId, msg.sender, _seedLiquidity);
        }
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

    /// @notice Manages the status of a list of authorized keepers.
    /// @param _action Action to take related to the keepers. `0` to add and `1` to remove.
    /// @param _keepers List of keepers to manage the status of.
    function manageKeepers(uint8 _action, address[] calldata _keepers) external onlyOwner {
        LendingPoolStorage.Layout storage l = LendingPoolStorage.layout();
        bool authorized = _action == 0 ? true : false;
        for (uint256 i; i < _keepers.length;) {
            l.authorizedKeepers[_keepers[i]] = authorized;
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }
    }

    /// @notice Manages the status of a list of authorized liquidators.
    /// @param _action Action to take related to the liquidators. `0` to add and `1`. to remove.
    /// @param _liquidators List of liquidators to manage the status of.
    function manageLiquidators(uint8 _action, address[] calldata _liquidators) external onlyOwner {
        LendingPoolStorage.Layout storage l = LendingPoolStorage.layout();
        bool authorized = _action == 0 ? true : false;
        for (uint256 i; i < _liquidators.length;) {
            l.authorizedKeepers[_liquidators[i]] = authorized;
            // forgefmt: disable-next-line
            unchecked { ++i; }
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
        return (LendingPoolStorage.layout().pools);
    }

    /// @notice Calculates the pending amount of interest that will be accrued for a specific pool.
    /// @param _poolId Pool ID to calculate the pending interest for.
    /// @return The pending amount of interest that will be accrued by the pool.
    function pendingInterest(uint256 _poolId) public view returns (uint112) {
        Market storage lendingPool = LendingPoolStorage.layout().pools[_poolId];
        if (block.timestamp > lendingPool.lastAccrueTime) {
            uint112 timePassed;
            unchecked {
                // @dev Unless we somehow traveled back in time, the odds of this underlying is zero.
                timePassed = uint112(block.timestamp - lendingPool.lastAccrueTime);
            }
            uint112 balance = lendingPool.warchest.underlyingBalanceWithInvestment().u112();
            uint112 ratePerSec =
                _calculateInterestRate(lendingPool.interestRateModel, lendingPool.globalDebtValue, balance).u112();
            return (((ratePerSec * lendingPool.globalDebtValue) * timePassed) / uint112(1e18));
        } else {
            return 0;
        }
    }

    /// @notice Calculates the amount of tokens a specific amount of debt shares are worth.
    /// @param _poolId ID of the pool to calculate share values for.
    /// @param _debtShare The amount of debt shares to calculate the value of.
    /// @return The amount of `underlying` tokens `_debtShares` is worth.
    function debtShareToVal(uint256 _poolId, uint112 _debtShare) public view returns (uint112) {
        Market storage lendingPool = LendingPoolStorage.layout().pools[_poolId];
        if (lendingPool.globalDebtShare == 0) return _debtShare; // When there's no share, 1 share = 1 val.
        return ((_debtShare * lendingPool.globalDebtValue) / lendingPool.globalDebtShare);
    }

    /// @notice Calculates the amount of pool debt shares a specific amount of `underlying` tokens are worth.
    /// @param _poolId ID of the pool to calculate shares from.
    /// @param _debtVal The amount of `underlying` tokens to calculate the shares value of.
    /// @return The amount of shares that `_debtVal` tokens are worth.
    function debtValToShare(uint256 _poolId, uint112 _debtVal) public view returns (uint112) {
        Market storage lendingPool = LendingPoolStorage.layout().pools[_poolId];
        if (lendingPool.globalDebtShare == 0) return _debtVal; // When there's no share, 1 share = 1 val.
        return ((_debtVal * lendingPool.globalDebtShare) / lendingPool.globalDebtValue);
    }

    /// @notice Calculates the current value of a specific position.
    /// @param _id ID of the position to calculate the value of.
    /// @return The total value of the position and the total amount of debt held by the position.
    function positionInfo(uint256 _id) public view returns (uint256, uint112) {
        Position storage pos = LendingPoolStorage.layout().positions[_id];
        return (IWorker(pos.worker).health(_id), debtShareToVal(pos.poolId, pos.debtShare));
    }

    /// @notice Calculates the total amount of tokens held by a specific lending pool.
    /// @param _poolId ID of the pool to calculate the total tokens of.
    /// @return How many tokens are held by the lending pool, including pool debts.
    function totalTokens(uint256 _poolId) public view returns (uint256) {
        Market storage lendingPool = LendingPoolStorage.layout().pools[_poolId];
        return (
            (lendingPool.warchest.underlyingBalanceWithInvestment() + lendingPool.globalDebtValue)
                - lendingPool.reservePool
        );
    }

    function _accrue(uint256 _poolId) internal {
        Market storage pool = LendingPoolStorage.layout().pools[_poolId];
        if (block.timestamp > pool.lastAccrueTime) {
            uint112 interest = pendingInterest(_poolId);
            uint112 toReserve = ((interest * pool.reservePoolBps) / uint112(10000));
            pool.reservePool = (pool.reservePool + toReserve);
            pool.globalDebtValue = (pool.globalDebtValue + interest);
            pool.lastAccrueTime = uint32(block.timestamp); // @dev can upgrade before 2102 (If we're still alive by then).
        }
    }

    function _deposit(uint256 _poolId, address _depositor, uint256 _amount) internal {
        LendingPoolStorage.Layout storage l = LendingPoolStorage.layout();
        Market storage pool = l.pools[_poolId];
        pool.underlying.safeTransferFrom(_depositor, address(pool.warchest), _amount);
        uint256 total = (totalTokens(_poolId) - _amount);
        uint256 share = total == 0 ? _amount - MIN_SHARES : ((_amount * pool.totalShares) / total);
        if (total == 0) {
            unchecked {
                // We reserve a small amount (10 ** 3) of shares to prevent inflation attacks.
                pool.warchest.mint(address(pool.warchest), MIN_SHARES);
                // type(uint112).max is VERY big so the overflow risk really just isn't there.
                pool.totalShares += MIN_SHARES;
            }
        }
        unchecked {
            pool.warchest.mint(msg.sender, share);
            // Same as the unchecked increase.
            pool.totalShares += share.u112();
        }
    }

    function _addDebt(uint256 _poolId, uint256 _posId, uint112 _debtValue) internal {
        Market storage pool = LendingPoolStorage.layout().pools[_poolId];
        Position storage pos = LendingPoolStorage.layout().positions[_posId];
        uint112 debtShare = debtValToShare(_poolId, _debtValue);
        unchecked {
            // @dev Unlikely to overflow, esp when type(uint112).max is HUGE.
            // That said, will do some fuzzing to confirm my findings.
            pos.debtShare += debtShare;
            pool.globalDebtShare += debtShare;
            pool.globalDebtValue += _debtValue;
        }
        emit AddDebt(_poolId, _posId, debtShare);
    }

    function _removeDebt(uint256 _poolId, uint256 _posId) internal returns (uint256) {
        Market storage pool = LendingPoolStorage.layout().pools[_poolId];
        Position storage pos = LendingPoolStorage.layout().positions[_posId];
        uint112 debtShare = pos.debtShare;
        if (debtShare > 0) {
            uint112 debtVal = debtShareToVal(_poolId, debtShare);
            pos.debtShare = 0;
            unchecked {
                // @dev Underflow is very unlikely due to the debt share being derived from the position.
                // This should create the assumption that pool.globalDebtShare >= pos.debtShare.
                // Fuzzing advised to confirm though.
                pool.globalDebtShare -= debtShare;
                pool.globalDebtValue -= debtVal;
            }
            emit RemoveDebt(_poolId, _posId, debtShare);
            return debtVal;
        } else {
            return 0;
        }
    }

    function _calculateInterestRate(InterestRateModel _model, uint256 _debt, uint256 _balance)
        internal
        pure
        returns (uint256)
    {
        // The triple slope interest rate model creates a variable interest rate
        // based on how much a user is utilizing their supplied collateral.
        // At less than 50% utilization, the interest rate is 10% APY.
        // At between 50-95% utilization, the interest rate ranges from 10%-25% APY.
        // At between 95-100% utilization, the interest rate ranges from 25%-100% APY.
        if (_model == InterestRateModel.TripleSlope) {
            uint256 utilization = (_debt * 10000) / (_debt + _balance);
            if (utilization < 5000) {
                return uint256(10e16) / 365 days;
            } else if (utilization < 9500) {
                return (10e16 + (((utilization - 5000) * 15e16) / 10000)) / 365 days;
            } else if (utilization < 10000) {
                return (25e16 + (((utilization - 7500) * 75e16) / 10000)) / 365 days;
            } else {
                return uint256(100e16) / 365 days;
            }
        } else {
            return 0;
        }
    }
}
