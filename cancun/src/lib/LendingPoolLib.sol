// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {LibTransient} from "solady/src/utils/LibTransient.sol";
import {LendingPoolStorage, Market, InterestRateModel} from "../LendingPoolStorage.sol";
import {_require, Errors} from "./Errors.sol";
import {Cast} from "./Cast.sol";

/// @title LendingPoolLib
/// @author Chainvisions
/// @notice A helper library for Limestone's lending pool.
/// @dev Mostly provides syntactic sugar, but also offers potential bytecode advantages.

library LendingPoolLib {
    using LibTransient for LibTransient.TBytes;
    using Cast for uint256;

    function _accrue(uint256 _poolId) internal {
        Market storage pool = LendingPoolStorage.layout().pools[_poolId];
        if (block.timestamp > pool.lastAccrueTime) {
            uint112 interest = _pendingInterest(_poolId);
            uint112 toReserve = ((interest * pool.reservePoolBps) / uint112(10000));
            unchecked {
                // @dev The fun part about this specifically is that if it does *somehow* overflow,
                // it won't even affect user funds, it'll just redistribute these reserves to the lenders.
                // So basically free money for them and a loss of revenue on our end. But also it's very unlikely
                // that we could accrue so much interest that it'll be worth more than type(uint112).max (and not redeem before that happens).
                pool.reservePool += toReserve;
                // @dev Remember the movie Fight Club? That shouldn't happen here unless `interest` is somehow gigantic.
                pool.globalDebtValue = (pool.globalDebtValue + interest);
            }
            pool.lastAccrueTime = uint32(block.timestamp); // @dev can upgrade before 2102 (If we're still alive by then).
        }
    }

    function _setExecutionScope(uint32 _posId, address _worker) internal {
        LibTransient.TBytes storage scopePtr = LibTransient.tBytes(LendingPoolStorage.EXECUTION_SCOPE_SLOT);
        scopePtr.set(abi.encode(LendingPoolStorage.ExecScope({positionId: _posId, worker: _worker})));
    }

    function _increaseDelegatedDebt(address _debtHolder, uint256 _poolId, uint256 _assetsBorrowed)
        internal
        returns (uint112 debtShares)
    {
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        Market storage pool = $.pools[_poolId];
        // Update current debt.
        debtShares = _debtValToShare(_poolId, _assetsBorrowed.u112());
        unchecked {
            // @dev Overflow unlikely. Can test though. We however, safe cast which should weed out
            // potential cases where this has the possibility to overflow. If not, then we're kinda fucked?
            pool.globalDebtValue += _assetsBorrowed.u112();
            pool.globalDebtShare += debtShares;
            $.delegatedDebt[_debtHolder][_poolId] += debtShares;
        }
    }

    function _decreaseDelegatedDebt(address _debtHolder, uint256 _poolId, uint256 _assetsRepaid) internal {
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        Market storage pool = $.pools[_poolId];
        // Update current debt.
        uint112 debtShares = _debtValToShare(_poolId, _assetsRepaid.u112());
        uint112 holderShares = $.delegatedDebt[_debtHolder][_poolId];
        if (debtShares > holderShares) {
            // If the debt holder trying to repay more than what's owed, adjust their total amount
            // so that it instead repays only the debt they owe and not a surplus that alters the entire pool's debt.
            debtShares = holderShares;
            _assetsRepaid = _debtShareToVal(_poolId, debtShares);
        }
        unchecked {
            // @dev Underflow unlikely. Can test though. We however, safe cast and also validate the holder's shares,
            // which should mitigate any potential cases that may arise. That said, we are once again fucked if that fails.
            pool.globalDebtValue -= _assetsRepaid.u112();
            pool.globalDebtShare -= debtShares;
            $.delegatedDebt[_debtHolder][_poolId] -= debtShares;
        }
    }

    function _verifyBorrowerPermissions(address _borrower, uint256 _poolId, uint256 _amount, bool _repaying) internal {
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        Market storage pool = $.pools[_poolId];
        _require($.authorizedContractBorrowers[_borrower], Errors.NOT_PRIVILEGED_BORROWER);
        if (_repaying) {
            pool.delegatedDebtAvailable += uint88(_amount);
        } else {
            pool.delegatedDebtAvailable -= uint88(_amount);
        }
    }

    /// @notice Calculates the amount of tokens a specific amount of debt shares are worth.
    /// @param _poolId ID of the pool to calculate share values for.
    /// @param _debtShare The amount of debt shares to calculate the value of.
    /// @return The amount of `underlying` tokens `_debtShares` is worth.
    function _debtShareToVal(uint256 _poolId, uint112 _debtShare) internal view returns (uint112) {
        Market storage lendingPool = LendingPoolStorage.layout().pools[_poolId];
        if (lendingPool.globalDebtShare == 0) return _debtShare; // When there's no share, 1 share = 1 val.
        return ((_debtShare * lendingPool.globalDebtValue) / lendingPool.globalDebtShare);
    }

    /// @notice Calculates the amount of pool debt shares a specific amount of `underlying` tokens are worth.
    /// @param _poolId ID of the pool to calculate shares from.
    /// @param _debtVal The amount of `underlying` tokens to calculate the shares value of.
    /// @return The amount of shares that `_debtVal` tokens are worth.
    function _debtValToShare(uint256 _poolId, uint112 _debtVal) internal view returns (uint112) {
        Market storage lendingPool = LendingPoolStorage.layout().pools[_poolId];
        if (lendingPool.globalDebtShare == 0) return _debtVal; // When there's no share, 1 share = 1 val.
        return ((_debtVal * lendingPool.globalDebtShare) / lendingPool.globalDebtValue);
    }

    /// @notice Calculates the pending amount of interest that will be accrued for a specific pool.
    /// @param _poolId Pool ID to calculate the pending interest for.
    /// @return The pending amount of interest that will be accrued by the pool.
    function _pendingInterest(uint256 _poolId) internal view returns (uint112) {
        Market storage lendingPool = LendingPoolStorage.layout().pools[_poolId];
        if (block.timestamp > lendingPool.lastAccrueTime) {
            uint112 timePassed;
            unchecked {
                // @dev Unless we somehow traveled back in time, the odds of this underflowing is zero.
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

    function _readExecutionScope() internal view returns (LendingPoolStorage.ExecScope memory scope) {
        LibTransient.TBytes storage scopePtr = LibTransient.tBytes(LendingPoolStorage.EXECUTION_SCOPE_SLOT);
        scope = abi.decode(scopePtr.get(), (LendingPoolStorage.ExecScope));
    }

    function _calculateMaxWorkFactorForUser(address _user, address _worker) internal view returns (uint256) {
        _user;
        LendingPoolStorage.Layout storage $ = LendingPoolStorage.layout();
        return $.workerDebtParams[_worker].workFactor;
    }
}
