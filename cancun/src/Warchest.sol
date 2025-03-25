// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {IERC20Metadata} from "@solidstate/token/ERC20/metadata/IERC20Metadata.sol";
import {SolidStateERC20} from "@solidstate/token/ERC20/SolidStateERC20.sol";
import {ERC20BaseStorage} from "@solidstate/token/ERC20/base/ERC20BaseStorage.sol";
import {Initializable} from "@solidstate/security/initializable/Initializable.sol";
import {Math} from "@solidstate/utils/Math.sol";
import {Ownable} from "solady/src/auth/Ownable.sol";
import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";
import {ILendingPool} from "./interfaces/ILendingPool.sol";
import {IStrategy} from "./interfaces/IStrategy.sol";
import {IWarchest} from "./interfaces/IWarchest.sol";
import {Cast} from "./lib/Cast.sol";
import {WarchestStorage} from "./WarchestStorage.sol";

/// @title Warchest
/// @author Chainvisions
/// @notice Vault contract responsible for holding lending pool assets.

contract Warchest is IWarchest, Ownable, SolidStateERC20, Initializable {
    using WarchestStorage for WarchestStorage.Layout;
    using SafeTransferLib for address;
    using Cast for uint256;

    /// @notice Thrown when the caller is not the lending pool contract.
    error CallerNotLendingPool();

    /// @notice Thrown when the circuit breaker is currently active.
    error CircuitBreakerActive();

    /// @notice Thrown when the circuit breaker is not active.
    error CircuitBreakerNotActive();

    /// @notice Thrown when attempting to withdraw when there is no warchest shares.
    error VaultHasNoShares();

    /// @notice Thrown when attempting to withdraw `0` shares.
    error SharesMustNotBeZero();

    /// @notice Thrown when attempting to call `doHardWork()` when there are no strategies.
    error NoStrategies();

    /// @notice Lending pool contract.
    ILendingPool public immutable LENDING_POOL;

    /// @notice Modifier for restricting calls to lending pool.
    modifier onlyLendingPool() {
        require(msg.sender == address(LENDING_POOL), CallerNotLendingPool());
        _;
    }

    /// @notice Warchest constructor.
    /// @param _lendingPool Lending Pool contract.
    constructor(ILendingPool _lendingPool) {
        LENDING_POOL = _lendingPool;
    }

    /// @notice Initializes the Warchest contract.
    /// @param _underlying Underlying token of the Warchest.
    function initialize(address _underlying) external initializer {
        // Initialize receipt token metadata.
        _setName(string.concat("Limestone Interest Bearing ", IERC20Metadata(address(_underlying)).symbol()));
        _setSymbol(string.concat("lib", IERC20Metadata(address(_underlying)).symbol()));
        _setDecimals(IERC20Metadata(_underlying).decimals());

        // Initialize storage.
        _setOwner(msg.sender);
        WarchestStorage.layout().underlying = _underlying;

        // Approve lending pool to utilize assets from Warchest.
        _underlying.safeApprove(address(LENDING_POOL), type(uint256).max);
    }

    /// @notice Mints Warchest tokens.
    /// @param _to Address to mint tokens to.
    /// @param _amount Amount of tokens to mint.
    function mint(address _to, uint256 _amount) external override onlyLendingPool {
        _mint(_to, _amount);
    }

    /// @notice Burns Warchest tokens.
    /// @param _from Address to burn tokens from.
    /// @param _amount Amount of tokens to burn.
    function burn(address _from, uint256 _amount) external override onlyLendingPool {
        _burn(_from, _amount);
    }

    /// @notice Withdraws tokens from the Warchest's strategies.
    /// @param _destination Address to send tokens to.
    /// @param _amountToWithdraw Amount of tokens to withdraw.
    function withdrawReserves(address _destination, uint256 _amountToWithdraw) external override onlyLendingPool {
        WarchestStorage.Layout storage l = WarchestStorage.layout();
        require(WarchestStorage.layout().circuitBreakerActive == false, CircuitBreakerActive());
        uint256 balance = l.underlying.balanceOf(address(this));
        if (_amountToWithdraw > balance) {
            _withdrawAmountFromStrategies(_amountToWithdraw - balance);
        }
        l.underlying.safeTransfer(_destination, _amountToWithdraw);
    }

    /// @notice Circuit breaker in the case of an emergency.
    /// With this circuit breaker, the Warchest can be completely
    /// cut off from the main lending pool contract. Allowing lenders to
    /// safely withdraw their deposits so long as it is flipped fast enough.
    function emergencyCircuitBreaker() external onlyOwner {
        // Flip circuit breaker and cut off approvals.
        WarchestStorage.layout().circuitBreakerActive = true;
        WarchestStorage.layout().underlying.safeApprove(address(LENDING_POOL), 0);
        emit CircuitBreakerActivated(block.timestamp);
    }

    /// @notice Adds strategies to the Warchest.
    /// @param strategies Strategies to add.
    function addStrategies(WarchestStorage.Strategy[] memory strategies) external onlyOwner {
        WarchestStorage.Strategy[] memory currentStrategies = WarchestStorage.layout().strategies;
        if (currentStrategies.length > 0) {
            for (uint256 i; i < currentStrategies.length;) {
                IStrategy(currentStrategies[i].strategyAddress).withdrawAllToVault();
                // forgefmt: disable-next-line
                unchecked { ++i; }
            }
        }
        delete WarchestStorage.layout().strategies;
        WarchestStorage.layout().strategies = strategies;

        // Find the highest weight strategy.
        uint256 totalNumerator;
        address highestWeightStrat;
        uint256 highestWeight;
        for (uint256 i; i < strategies.length;) {
            WarchestStorage.Strategy memory strat = strategies[i];
            if (strat.investmentNumerator > highestWeight) {
                highestWeightStrat = strat.strategyAddress;
            }
            totalNumerator += strat.investmentNumerator;
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }

        WarchestStorage.layout().highestWeightStrategy = highestWeightStrat;
        WarchestStorage.layout().totalInvestmentNumerator = totalNumerator.u32();
    }

    /// @notice Completely kills every strategy on the Warchest.
    function killAllStrategies() external onlyOwner {
        WarchestStorage.Strategy[] memory currentStrategies = WarchestStorage.layout().strategies;
        for (uint256 i; i < currentStrategies.length;) {
            IStrategy(currentStrategies[i].strategyAddress).withdrawAllToVault();
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }
        delete WarchestStorage.layout().strategies;
        delete WarchestStorage.layout().highestWeightStrategy;
        delete WarchestStorage.layout().totalInvestmentNumerator;
    }

    /// @notice Performs an emergency exit from the Warchest contract.
    /// @param _amountInShares Amount of shares to burn from the contract.
    function emergencyExit(uint256 _amountInShares) external {
        require(WarchestStorage.layout().circuitBreakerActive == true, CircuitBreakerNotActive());
        require(_totalSupply() > 0, VaultHasNoShares());
        require(_amountInShares > 0, SharesMustNotBeZero());

        address _underlying = WarchestStorage.layout().underlying;
        uint256 supplySnapshot = _totalSupply();
        _burn(msg.sender, _amountInShares);

        uint256 underlyingAmountToWithdraw = ((underlyingBalanceWithInvestment() * _amountInShares) / supplySnapshot);
        uint256 underlyingBalanceInWarchest = _underlying.balanceOf(address(this));
        if (underlyingAmountToWithdraw > underlyingBalanceInWarchest) {
            // Withdraw everything from the strategy to accurately check the share value.
            if (_amountInShares == supplySnapshot) {
                _withdrawFromAllStrategies();
            } else {
                uint256 missing = (underlyingAmountToWithdraw - underlyingBalanceInWarchest);
                _withdrawAmountFromStrategies(missing);
            }
            // Recalculate to improve accuracy.
            underlyingAmountToWithdraw = Math.min(
                ((underlyingBalanceWithInvestment() * _amountInShares) / supplySnapshot),
                _underlying.balanceOf(address(this))
            );
        }
    }

    /// @notice Invests funds into the strategy and harvests yield.
    function doHardWork() external {
        uint256 amountToInvest = availableToInvestOut();
        address _underlying = WarchestStorage.layout().underlying;
        WarchestStorage.Strategy[] memory strategies = WarchestStorage.layout().strategies;
        require(strategies.length > 0, NoStrategies());
        for (uint256 i; i < strategies.length;) {
            WarchestStorage.Strategy memory strategy = strategies[i];
            uint256 strategyInvestment = ((amountToInvest * strategy.investmentNumerator) / 10000);
            if (strategyInvestment > 0) {
                _underlying.safeTransfer(address(strategy.strategyAddress), strategyInvestment);
                emit Invested(address(strategy.strategyAddress), strategyInvestment);
            }
            IStrategy(strategy.strategyAddress).doHardWork();
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }
    }

    /// @notice Amount of underlying tokens available to invest
    /// @return Amount of investable underlying for the strategies.
    function availableToInvestOut() public view returns (uint256) {
        (uint256 total, uint256 balance, uint256 inStrategies) = _underlyingBalanceWithInvestment();
        uint256 toInvestTotal = (total * WarchestStorage.layout().totalInvestmentNumerator) / 10000;
        if (inStrategies >= toInvestTotal) {
            return 0;
        } else {
            uint256 remainingToInvest = (toInvestTotal - inStrategies);
            return remainingToInvest <= balance ? remainingToInvest : balance;
        }
    }

    /// @notice Calculates the total amount of assets held by the Warchest.
    /// @return Warchest's funds included ones currently invested in strategies.
    function underlyingBalanceWithInvestment() public view override returns (uint256) {
        (uint256 total,,) = _underlyingBalanceWithInvestment();
        return total;
    }

    function _withdrawAmountFromStrategies(uint256 _amountNeeded) internal {
        // Make a withdraw from the highest weight strategy for the highest guarantee of liquidity.
        IStrategy _highestWeightStrategy = IStrategy(WarchestStorage.layout().highestWeightStrategy);
        uint256 withdrawable = Math.min(_amountNeeded, _highestWeightStrategy.investedUnderlyingBalance());
        _highestWeightStrategy.withdrawToVault(withdrawable);

        // Check if withdrawn amount was enough.
        if (withdrawable != _amountNeeded) {
            // In the case of it not being enough, we need to check other strategies.
            WarchestStorage.Strategy[] memory strategies = WarchestStorage.layout().strategies;
            uint256 remainderNeeded = _amountNeeded - withdrawable;
            for (uint256 i; i < strategies.length;) {
                IStrategy _strategy = IStrategy(strategies[i].strategyAddress);
                withdrawable = Math.min(remainderNeeded, _strategy.investedUnderlyingBalance());
                _strategy.withdrawToVault(withdrawable);

                if (withdrawable == remainderNeeded) {
                    break;
                }

                // forgefmt: disable-next-line
                unchecked { ++i; }
            }
        }
    }

    function _withdrawFromAllStrategies() internal {
        WarchestStorage.Strategy[] memory strategies = WarchestStorage.layout().strategies;
        for (uint256 i; i < strategies.length;) {
            IStrategy(strategies[i].strategyAddress).withdrawAllToVault();
            // forgefmt: disable-next-line
            unchecked { ++i; }
        }
    }

    function _underlyingBalanceWithInvestment()
        internal
        view
        returns (uint256 total, uint256 balance, uint256 inStrategies)
    {
        total = WarchestStorage.layout().underlying.balanceOf(address(this));
        balance = total;
        WarchestStorage.Strategy[] memory strategies = WarchestStorage.layout().strategies;
        if (strategies.length > 0) {
            for (uint256 i; i < strategies.length;) {
                inStrategies += IStrategy(strategies[i].strategyAddress).investedUnderlyingBalance();
                // forgefmt: disable-next-line
                unchecked { ++i; }
            }
        }
        total += inStrategies;
    }
}
