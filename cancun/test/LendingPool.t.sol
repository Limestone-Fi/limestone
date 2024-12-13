// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test, console} from "forge-std/Test.sol";
import {StdInvariant} from "forge-std/StdInvariant.sol";
import {LibZip} from "solady/src/utils/LibZip.sol";
import {LendingPool, LendingPoolConfig, InterestRateModel, LendingPoolStorage} from "../src/LendingPool.sol";
import {MockToken} from "./mocks/MockToken.sol";
import {MockWarchest} from "./mocks/MockWarchest.sol";
import {MockWorker} from "./mocks/MockWorker.sol";

contract LendingPoolTest is Test {
    MockToken public collateral;

    MockWarchest public warchest;

    MockWorker public worker;

    LendingPool public lp;

    function setUp() public {
        collateral = new MockToken();
        warchest = new MockWarchest(address(collateral));
        worker = new MockWorker(address(collateral));
        lp = new LendingPool();
        lp.initialize();
        lp.addLendingPool(
            LendingPoolConfig({
                reservePoolBps: 250,
                liquidateBps: 1000,
                interestRateModel: InterestRateModel.TripleSlope,
                minDebtSize: 0.1 ether
            }),
            address(collateral),
            address(warchest),
            0
        );
        address[] memory workers = new address[](1);
        LendingPoolStorage.WorkerDebtParams[] memory workerParams = new LendingPoolStorage.WorkerDebtParams[](1);
        workers[0] = address(worker);
        workerParams[0] = LendingPoolStorage.WorkerDebtParams({
            authorizedPoolId: 0,
            borrowable: true,
            workFactor: 7500,
            killFactor: 8000
        });
        //StdInvariant.targetContract(address(lp));
    }

    function testDeposit() public {
        vm.startPrank(address(1), address(1));
        collateral.mint(address(1), 1 ether);
        collateral.approve(address(lp), 1 ether);
        lp.deposit(0, 1 ether);
    }

    function testDepositAndWithdraw() public {
        vm.startPrank(address(1), address(1));
        collateral.mint(address(1), 1 ether);
        collateral.approve(address(lp), 1 ether);
        lp.deposit(0, 1 ether);
        lp.withdraw(0, 1 ether - 10 ** 3);
    }

    function testWorkNormal() public {
        vm.startPrank(address(1), address(1));
        collateral.mint(address(1), 1 ether);
        collateral.approve(address(lp), 1 ether);
        lp.deposit(0, 1 ether);
        vm.stopPrank();
        collateral.mint(address(2), 0.1 ether);
        vm.startPrank(address(2), address(2));
        collateral.approve(address(lp), 0.1 ether);
        LendingPool.WorkContext memory ctx = LendingPool.WorkContext({
            poolId: 0,
            worker: address(worker),
            amountIn: 0.1 ether,
            loan: 0,
            maxReturn: 0,
            data: abi.encode(0)
        });
        lp.doHardWork(0, ctx);
    }

    function testFuzzDepositAndWithdraw(uint256 _amount) public {
        vm.assume(_amount > 10 ** 3);
        vm.assume(_amount < type(uint112).max);
        vm.startPrank(address(1), address(1));
        collateral.mint(address(1), _amount);
        uint256 startingBalance = collateral.balanceOf(address(1));
        collateral.approve(address(lp), _amount);
        lp.deposit(0, _amount);
        assertEq(warchest.balanceOf(address(1)), _amount - 10 ** 3);
        lp.withdraw(0, _amount - 10 ** 3);
        assertEq(collateral.balanceOf(address(1)), startingBalance - 10 ** 3);
    }

    /*
    function invariant_totalTokensAndSupply() public {
        assert(lp.totalTokens(0) >= warchest.totalSupply());
    }*/
}
