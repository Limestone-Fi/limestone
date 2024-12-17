// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {IDromeFactory} from "../../interfaces/external/IDromeFactory.sol";
import {IDromePool} from "../../interfaces/external/IDromePool.sol";
import {Clones} from "@openzeppelin/proxy/Clones.sol";

contract MockPoolFactory {
    address public immutable implementation;

    bool public isPaused;
    address public pauser;

    uint256 public stableFee;
    uint256 public volatileFee;
    uint256 public constant MAX_FEE = 300; // 3%
    // Override to indicate there is custom 0% fee - as a 0 value in the customFee mapping indicates
    // that no custom fee rate has been set
    uint256 public constant ZERO_FEE_INDICATOR = 420;
    address public feeManager;

    /// @dev used to change the name/symbol of the pool by calling emergencyCouncil
    address public voter;

    mapping(address => mapping(address => mapping(bool => address))) private _getPool;
    address[] public allPools;
    mapping(address => bool) private _isPool; // simplified check if its a pool, given that `stable` flag might not be available in peripherals
    mapping(address => uint256) public customFee; // override for custom fees

    address internal _temp0;
    address internal _temp1;
    bool internal _temp;

    constructor(address _implementation) {
        implementation = _implementation;
        voter = msg.sender;
        pauser = msg.sender;
        feeManager = msg.sender;
        isPaused = false;
        stableFee = 5; // 0.05%
        volatileFee = 30; // 0.3%
    }

    function allPoolsLength() external view returns (uint256) {
        return allPools.length;
    }

    function getPool(address tokenA, address tokenB, uint24 fee) external view returns (address) {
        return fee > 1 ? address(0) : fee == 1 ? _getPool[tokenA][tokenB][true] : _getPool[tokenA][tokenB][false];
    }

    function getPool(address tokenA, address tokenB, bool stable) external view returns (address) {
        return _getPool[tokenA][tokenB][stable];
    }

    function isPool(address pool) external view returns (bool) {
        return _isPool[pool];
    }

    function setPauser(address _pauser) external {
        if (msg.sender != pauser) revert IDromeFactory.NotPauser();
        if (_pauser == address(0)) revert IDromeFactory.ZeroAddress();
        pauser = _pauser;
        emit IDromeFactory.SetPauser(_pauser);
    }

    function setPauseState(bool _state) external {
        if (msg.sender != pauser) revert IDromeFactory.NotPauser();
        isPaused = _state;
        emit IDromeFactory.SetPauseState(_state);
    }

    function setFeeManager(address _feeManager) external {
        if (msg.sender != feeManager) revert IDromeFactory.NotFeeManager();
        if (_feeManager == address(0)) revert IDromeFactory.ZeroAddress();
        feeManager = _feeManager;
        emit IDromeFactory.SetFeeManager(_feeManager);
    }

    function setFee(bool _stable, uint256 _fee) external {
        if (msg.sender != feeManager) revert IDromeFactory.NotFeeManager();
        if (_fee > MAX_FEE) revert IDromeFactory.FeeTooHigh();
        if (_fee == 0) revert IDromeFactory.ZeroFee();
        if (_stable) {
            stableFee = _fee;
        } else {
            volatileFee = _fee;
        }
    }

    function setCustomFee(address pool, uint256 fee) external {
        if (msg.sender != feeManager) revert IDromeFactory.NotFeeManager();
        if (fee > MAX_FEE && fee != ZERO_FEE_INDICATOR) revert IDromeFactory.FeeTooHigh();
        if (!_isPool[pool]) revert IDromeFactory.InvalidPool();

        customFee[pool] = fee;
        emit IDromeFactory.SetCustomFee(pool, fee);
    }

    function getFee(address pool, bool _stable) public view returns (uint256) {
        uint256 fee = customFee[pool];
        return fee == ZERO_FEE_INDICATOR ? 0 : fee != 0 ? fee : _stable ? stableFee : volatileFee;
    }

    function createPool(address tokenA, address tokenB, uint24 fee) external returns (address pool) {
        if (fee > 1) revert IDromeFactory.FeeInvalid();
        bool stable = fee == 1;
        return createPool(tokenA, tokenB, stable);
    }

    function createPool(address tokenA, address tokenB, bool stable) public returns (address pool) {
        if (tokenA == tokenB) revert IDromeFactory.SameAddress();
        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        if (token0 == address(0)) revert IDromeFactory.ZeroAddress();
        if (_getPool[token0][token1][stable] != address(0)) revert IDromeFactory.PoolAlreadyExists();
        bytes32 salt = keccak256(abi.encodePacked(token0, token1, stable)); // salt includes stable as well, 3 parameters
        pool = Clones.cloneDeterministic(implementation, salt);
        IDromePool(pool).initialize(token0, token1, stable);
        _getPool[token0][token1][stable] = pool;
        _getPool[token1][token0][stable] = pool; // populate mapping in the reverse direction
        allPools.push(pool);
        _isPool[pool] = true;
        emit IDromeFactory.PoolCreated(token0, token1, stable, pool, allPools.length);
    }
}
