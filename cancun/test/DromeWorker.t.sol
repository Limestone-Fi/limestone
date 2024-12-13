// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";

/// @dev Diamond stuff.
import {SolidStateDiamond} from "@solidstate/proxy/diamond/SolidStateDiamond.sol";
import {IERC2535DiamondCut} from "@solidstate/interfaces/IERC2535DiamondCut.sol";
import {IERC2535DiamondCutInternal} from "@solidstate/interfaces/IERC2535DiamondCutInternal.sol";

/// @dev Test suite.
import {Test} from "forge-std/Test.sol";
import {StdCheats} from "forge-std/StdCheats.sol";

/// @dev Local contracts.
import {LendingPool, LendingPoolConfig, InterestRateModel, LendingPoolStorage} from "../src/LendingPool.sol";
import {
    PositionCoordinator,
    V2LikePositionInvestmentContext,
    V2LikePositionDivestmentContext
} from "../src/PositionCoordinator.sol";
import {DromeMultiModalWorker, IDromeRouter, IDromeGauge} from "../src/workers/DromeMultiModal.sol";

/// @dev Mocks, addresses, etc.
import {AddressBook} from "./utils/AddressBook.sol";
import {MockWarchest} from "./mocks/MockWarchest.sol";

contract DromeWorkerTest is Test, AddressBook {
    address public constant LIMESTONE_DIAMOND = 0x01000006b888030018000000D1e1AA171700fb8D;

    address public constant LIQUIDITY_POOL = 0x6cDcb1C4A4D1C3C6d054b27AC5B77e89eAFb971d;

    address public constant WARCHEST_IMPL = 0x000000000000a860e7AD03105Ae43AA153Bb46A4;

    address public constant TESTER = 0xBF9542E69A59e2bb8EB9c1B50D59a9CCde591CFb;

    address public worker;

    function setUp() public {
        // Setup position coordinator facet.
        PositionCoordinator coordinatorFacet = new PositionCoordinator();
        LendingPool lendingFacet = new LendingPool();
        IERC2535DiamondCutInternal.FacetCut[] memory facetCuts = new IERC2535DiamondCutInternal.FacetCut[](2);
        bytes4[] memory selectors = new bytes4[](6);
        uint256 selectorIndex;
        selectors[selectorIndex++] = PositionCoordinator.investInV2LikePosition.selector;
        selectors[selectorIndex++] = PositionCoordinator.divestFromV2LikePosition.selector;
        selectors[selectorIndex++] = PositionCoordinator.repayV2LikeLiquidityPositionDebt.selector;
        selectors[selectorIndex++] = PositionCoordinator.liquidateV2LikePosition.selector;
        selectors[selectorIndex++] = PositionCoordinator.accessAssets.selector;
        selectors[selectorIndex++] = PositionCoordinator.reinvestmentFeeNumerator.selector;
        facetCuts[0] = IERC2535DiamondCutInternal.FacetCut({
            target: address(coordinatorFacet),
            action: IERC2535DiamondCutInternal.FacetCutAction.ADD,
            selectors: selectors
        });

        /*
        // Remove old selectors to save time.
        selectorIndex = 0;
        selectors = new bytes4[](14);
        selectors[selectorIndex++] = 0xde75ddea;
        selectors[selectorIndex++] = 0x13fffc1d;
        selectors[selectorIndex++] = 0xbb2226e4;
        selectors[selectorIndex++] = 0xe893ce07;
        selectors[selectorIndex++] = 0xe2bbb158;
        selectors[selectorIndex++] = 0x4c1f28db;
        selectors[selectorIndex++] = 0xd2ed4cec;
        selectors[selectorIndex++] = 0x74b791a8;
        selectors[selectorIndex++] = 0xd29a0025;
        selectors[selectorIndex++] = 0x24c2bf68;
        selectors[selectorIndex++] = 0x362f6e24;
        selectors[selectorIndex++] = 0x1113ef52;
        selectors[selectorIndex++] = 0x5e068877;
        selectors[selectorIndex++] = 0x441a3e70;
        facetCuts[1] = IERC2535DiamondCutInternal.FacetCut({
            target: address(0),
            action: IERC2535DiamondCutInternal.FacetCutAction.REMOVE,
            selectors: selectors
        });*/

        // Setup replacement cut for the new lending pool facet.
        selectorIndex = 0;
        selectors = new bytes4[](22);
        selectors[selectorIndex++] = LendingPool.deposit.selector;
        selectors[selectorIndex++] = LendingPool.withdraw.selector;
        selectors[selectorIndex++] = LendingPool.doHardWork.selector;
        selectors[selectorIndex++] = LendingPool.increaseCollateral.selector;
        selectors[selectorIndex++] = LendingPool.kill.selector;
        selectors[selectorIndex++] = LendingPool.totalTokens.selector;
        selectors[selectorIndex++] = LendingPool.pools.selector;
        selectors[selectorIndex++] = LendingPool.positions.selector;
        selectors[selectorIndex++] = LendingPool.positionInfo.selector;
        selectors[selectorIndex++] = LendingPool.addWorkers.selector;
        selectors[selectorIndex++] = LendingPool.pendingInterest.selector;
        selectors[selectorIndex++] = LendingPool.salvage.selector;
        selectors[selectorIndex++] = LendingPool.addLendingPool.selector;
        selectors[selectorIndex++] = LendingPool.authorizedKeepers.selector;
        selectors[selectorIndex++] = LendingPool.authorizedLiquidators.selector;
        selectors[selectorIndex++] = LendingPool.accessUserAssets.selector;
        selectors[selectorIndex++] = LendingPool.setPermissionedLiquidation.selector;
        selectors[selectorIndex++] = LendingPool.collectReserves.selector;
        selectors[selectorIndex++] = LendingPool.distributeReserves.selector;
        selectors[selectorIndex++] = LendingPool.debtShareToVal.selector;
        selectors[selectorIndex++] = LendingPool.debtValToShare.selector;
        selectors[selectorIndex++] = LendingPool.setDelegatedDebt.selector;
        facetCuts[1] = IERC2535DiamondCutInternal.FacetCut({
            target: address(lendingFacet),
            action: IERC2535DiamondCutInternal.FacetCutAction.ADD,
            selectors: selectors
        });

        /*
        // Remove some unused functions as well.
        selectorIndex = 0;
        selectors = new bytes4[](2);
        selectors[selectorIndex++] = LendingPool.manageKeepers.selector;
        selectors[selectorIndex++] = LendingPool.manageLiquidators.selector;
        facetCuts[3] = IERC2535DiamondCutInternal.FacetCut({
            target: address(0),
            action: IERC2535DiamondCutInternal.FacetCutAction.REMOVE,
            selectors: selectors
        });*/

        vm.prank(ONETICKWARRIOR);
        SolidStateDiamond(payable(LIMESTONE_DIAMOND)).diamondCut(
            facetCuts, address(lendingFacet), abi.encodeWithSelector(LendingPool.scrubStorage.selector)
        );

        // Deploy MultiModalWorker.
        DromeMultiModalWorker workerDeployment = new DromeMultiModalWorker();
        address[] memory rewards = new address[](1);
        rewards[0] = AERO;
        workerDeployment.initialize(LIQUIDITY_POOL, AERO_USDC_GAUGE, AERO_ROUTER, rewards);
        worker = address(workerDeployment);

        // Seed the lending pool.
        StdCheats.deal(WETH, ONETICKWARRIOR, 10 ether);
        StdCheats.deal(BASE_USDC, ONETICKWARRIOR, 100_000 * 1e6);
        StdCheats.deal(AERO, ONETICKWARRIOR, 1000 ether);
        vm.startPrank(ONETICKWARRIOR);
        MockWarchest chest = new MockWarchest(WETH);
        MockWarchest usdcChest = new MockWarchest(BASE_USDC);
        MockWarchest aeroChest = new MockWarchest(AERO);
        IERC20(WETH).approve(LIMESTONE_DIAMOND, 10 ether);
        IERC20(BASE_USDC).approve(LIMESTONE_DIAMOND, 100_000 * 1e6);
        IERC20(AERO).approve(LIMESTONE_DIAMOND, 1000 ether);
        LendingPool(LIMESTONE_DIAMOND).addLendingPool(
            LendingPoolConfig({
                reservePoolBps: 250,
                liquidateBps: 800,
                interestRateModel: InterestRateModel.TripleSlope,
                minDebtSize: 0.1 ether
            }),
            WETH,
            address(chest),
            10 ether
        );
        LendingPool(LIMESTONE_DIAMOND).addLendingPool(
            LendingPoolConfig({
                reservePoolBps: 250,
                liquidateBps: 800,
                interestRateModel: InterestRateModel.TripleSlope,
                minDebtSize: 100 * 1e6
            }),
            BASE_USDC,
            address(usdcChest),
            100_000 * 1e6
        );
        LendingPool(LIMESTONE_DIAMOND).addLendingPool(
            LendingPoolConfig({
                reservePoolBps: 250,
                liquidateBps: 800,
                interestRateModel: InterestRateModel.TripleSlope,
                minDebtSize: 50 ether
            }),
            AERO,
            address(aeroChest),
            1000 ether
        );
        address[] memory workers = new address[](1);
        workers[0] = worker;
        LendingPoolStorage.WorkerDebtParams[] memory debtParams = new LendingPoolStorage.WorkerDebtParams[](1);
        debtParams[0] = LendingPoolStorage.WorkerDebtParams({
            authorizedPoolId: 0,
            borrowable: true,
            workFactor: 300,
            killFactor: 8333
        });
        LendingPool(LIMESTONE_DIAMOND).addWorkers(workers, debtParams);

        // Update worker routers.
        IDromeRouter.Route[] memory tokenRoute = new IDromeRouter.Route[](1);
        tokenRoute[0] = IDromeRouter.Route({
            from: AERO,
            to: BASE_USDC,
            stable: false,
            factory: IDromeRouter(AERO_ROUTER).defaultFactory()
        });
        DromeMultiModalWorker(worker).setRoute(AERO, BASE_USDC, tokenRoute);
        vm.stopPrank();

        // Set approvals to save time.
        vm.startPrank(TESTER);
        IERC20(BASE_USDC).approve(LIMESTONE_DIAMOND, type(uint256).max);
        IERC20(AERO).approve(LIMESTONE_DIAMOND, type(uint256).max);
        IERC20(WETH).approve(LIMESTONE_DIAMOND, type(uint256).max);
        vm.stopPrank();
    }

    function testOpenPositionToken0() public {
        StdCheats.deal(BASE_USDC, TESTER, 100 * 1e6);
        vm.startPrank(TESTER, TESTER);
        V2LikePositionInvestmentContext memory ctx = V2LikePositionInvestmentContext({
            positionId: 0,
            worker: worker,
            token0In: 100 * 1e6,
            token1In: 0,
            token0PoolId: 1,
            token1PoolId: 2,
            token0Borrow: 0,
            token1Borrow: 0,
            minLiquidityMinted: 0,
            skipHealthcheck: false
        });
        PositionCoordinator(LIMESTONE_DIAMOND).investInV2LikePosition(ctx);
        (uint256 posValue, uint256 posDebtVal) = DromeMultiModalWorker(worker).calculatePositionValue(1);
        assertApproxEqRel(posValue, 100 * 1e6, 0.05e18);
        assertEq(posDebtVal, 0);
    }

    function testOpenPositionToken1() public {
        StdCheats.deal(AERO, TESTER, 53.7 ether);
        vm.startPrank(TESTER, TESTER);
        V2LikePositionInvestmentContext memory ctx = V2LikePositionInvestmentContext({
            positionId: 0,
            worker: worker,
            token0In: 0,
            token1In: 53.7 ether,
            token0PoolId: 1,
            token1PoolId: 2,
            token0Borrow: 0,
            token1Borrow: 0,
            minLiquidityMinted: 0,
            skipHealthcheck: false
        });
        ctx.worker = worker;
        PositionCoordinator(LIMESTONE_DIAMOND).investInV2LikePosition(ctx);
        (uint256 posValue, uint256 posDebtVal) = DromeMultiModalWorker(worker).calculatePositionValue(1);
        assertApproxEqRel(posValue, 100 * 1e6, 0.075e18);
        assertEq(posDebtVal, 0);
    }

    function testOpenPositionBothSidesBalanced() public {
        StdCheats.deal(BASE_USDC, TESTER, 100 * 1e6);
        StdCheats.deal(AERO, TESTER, 53.7 ether);
        vm.startPrank(TESTER, TESTER);
        V2LikePositionInvestmentContext memory ctx = V2LikePositionInvestmentContext({
            positionId: 0,
            worker: worker,
            token0In: 100 * 1e6,
            token1In: 53.7 ether,
            token0PoolId: 1,
            token1PoolId: 2,
            token0Borrow: 0,
            token1Borrow: 0,
            minLiquidityMinted: 0,
            skipHealthcheck: false
        });
        PositionCoordinator(LIMESTONE_DIAMOND).investInV2LikePosition(ctx);
        (uint256 posValue, uint256 posDebtVal) = DromeMultiModalWorker(worker).calculatePositionValue(1);
        assertApproxEqRel(posValue, 200 * 1e6, 0.05e18);
        assertEq(posDebtVal, 0);
    }

    function testOpenAndBorrowOpposite() public {
        StdCheats.deal(BASE_USDC, TESTER, 1e8);
        vm.startPrank(TESTER, TESTER);
        V2LikePositionInvestmentContext memory ctx = V2LikePositionInvestmentContext({
            positionId: 0,
            worker: worker,
            token0In: 1e8,
            token1In: 0,
            token0PoolId: 1,
            token1PoolId: 2,
            token0Borrow: 0,
            token1Borrow: 100 ether,
            minLiquidityMinted: 0,
            skipHealthcheck: false
        });
        PositionCoordinator(LIMESTONE_DIAMOND).investInV2LikePosition(ctx);
        (uint256 posValue, uint256 posDebtVal) = DromeMultiModalWorker(worker).calculatePositionValue(1);
        assertApproxEqRel(posValue, 288 * 1e6, 0.05e18);
        assertApproxEqRel(posDebtVal, 188 * 1e6, 0.1e18);
    }

    function testOpenAndBorrowBothSides() public {
        StdCheats.deal(BASE_USDC, TESTER, 100 * 1e6);
        vm.startPrank(TESTER, TESTER);
        V2LikePositionInvestmentContext memory ctx = V2LikePositionInvestmentContext({
            positionId: 0,
            worker: worker,
            token0In: 1e8,
            token1In: 0,
            token0PoolId: 1,
            token1PoolId: 2,
            token0Borrow: 1e8,
            token1Borrow: 50 ether,
            minLiquidityMinted: 0,
            skipHealthcheck: false
        });
        PositionCoordinator(LIMESTONE_DIAMOND).investInV2LikePosition(ctx);
        (uint256 posValue, uint256 posDebtVal) = DromeMultiModalWorker(worker).calculatePositionValue(1);
        assertApproxEqRel(posValue, 3e8, 0.07e18);
        assertApproxEqRel(posDebtVal, 2e8, 0.05e18);
    }

    function testFailManipulationAttack() public {
        StdCheats.deal(BASE_USDC, TESTER, 1e8);
        vm.startPrank(TESTER, TESTER);
        V2LikePositionInvestmentContext memory ctx = V2LikePositionInvestmentContext({
            positionId: 0,
            worker: worker,
            token0In: 1e8,
            token1In: 0,
            token0PoolId: 1,
            token1PoolId: 2,
            token0Borrow: 1.5e8,
            token1Borrow: 0,
            minLiquidityMinted: 0,
            skipHealthcheck: false
        });
        PositionCoordinator(LIMESTONE_DIAMOND).investInV2LikePosition(ctx);
        (uint256 startingValue, uint256 posDebtVal) = DromeMultiModalWorker(worker).calculatePositionValue(1);
        posDebtVal;
        uint256 flashloaned = 350_000e6;
        StdCheats.deal(BASE_USDC, TESTER, flashloaned);
        IDromeRouter.Route[] memory route = new IDromeRouter.Route[](1);
        route[0] = IDromeRouter.Route({
            from: BASE_USDC,
            to: AERO,
            stable: false,
            factory: IDromeRouter(AERO_ROUTER).defaultFactory()
        });
        IERC20(BASE_USDC).approve(AERO_ROUTER, flashloaned);
        uint256[] memory amounts =
            IDromeRouter(AERO_ROUTER).swapExactTokensForTokens(flashloaned, 0, route, address(TESTER), block.timestamp);
        (uint256 newValue, uint256 newDebtVal) = DromeMultiModalWorker(worker).calculatePositionValue(1);
        newDebtVal;
        assertGt(newValue, startingValue);
        V2LikePositionDivestmentContext memory divCtx = V2LikePositionDivestmentContext({
            positionId: 1,
            worker: worker,
            positionBps: 10000,
            minToken0Out: 0,
            minToken1Out: 0,
            token0Repay: 1.5e8,
            token1Repay: 0,
            side: 0,
            minimalWithdrawal: false
        });
        PositionCoordinator(LIMESTONE_DIAMOND).divestFromV2LikePosition(divCtx);
        route = new IDromeRouter.Route[](1);
        route[0] = IDromeRouter.Route({
            from: AERO,
            to: BASE_USDC,
            stable: false,
            factory: IDromeRouter(AERO_ROUTER).defaultFactory()
        });
        uint256 currentAero = amounts[amounts.length - 1];
        IERC20(AERO).approve(AERO_ROUTER, currentAero);
        IDromeRouter(AERO_ROUTER).swapExactTokensForTokens(currentAero, 0, route, address(TESTER), block.timestamp);
        uint256 balance = IERC20(BASE_USDC).balanceOf(TESTER);
        assertGt(balance, flashloaned + 1e8);
    }

    function testFarmingNormal() public {
        StdCheats.deal(BASE_USDC, TESTER, 10_000 * 1e6);
        vm.startPrank(TESTER, TESTER);
        uint256 startingBalance = IERC20(BASE_USDC).balanceOf(TESTER);
        V2LikePositionInvestmentContext memory ctx = V2LikePositionInvestmentContext({
            positionId: 0,
            worker: worker,
            token0In: 10_000 * 1e6,
            token1In: 0,
            token0PoolId: 1,
            token1PoolId: 2,
            token0Borrow: 19_000 * 1e6,
            token1Borrow: 0,
            minLiquidityMinted: 0,
            skipHealthcheck: false
        });
        PositionCoordinator(LIMESTONE_DIAMOND).investInV2LikePosition(ctx);
        (uint256 initialValue, uint256 initialDebt) = DromeMultiModalWorker(worker).calculatePositionValue(1);
        vm.stopPrank();
        for (uint256 i; i < 10;) {
            vm.startPrank(AERO_VOTER, AERO_VOTER);
            uint256 notifyAmount = 2_500_000e18;
            StdCheats.deal(AERO, AERO_VOTER, notifyAmount);
            if (IERC20(AERO).allowance(AERO_VOTER, AERO_USDC_GAUGE) < notifyAmount) {
                IERC20(AERO).approve(AERO_USDC_GAUGE, notifyAmount);
            }
            IDromeGauge(AERO_USDC_GAUGE).notifyRewardAmount(notifyAmount);
            vm.warp(block.timestamp + 24 hours);
            vm.stopPrank();
            DromeMultiModalWorker(worker).reinvest();
            vm.prank(TESTER, TESTER);
            LendingPool(LIMESTONE_DIAMOND).deposit(1, 0); // Accrue some interest.
                // forgefmt: disable-next-line
          unchecked { ++i; }
        }
        vm.startPrank(TESTER, TESTER);
        (uint256 newValue, uint256 newDebt) = DromeMultiModalWorker(worker).calculatePositionValue(1);
        assertGt(newValue, initialValue);
        assertGt(newDebt, initialDebt);
        V2LikePositionDivestmentContext memory divCtx = V2LikePositionDivestmentContext({
            positionId: 1,
            worker: worker,
            positionBps: 10000,
            minToken0Out: 0,
            minToken1Out: 0,
            token0Repay: 21_000 * 1e6,
            token1Repay: 0,
            side: 0,
            minimalWithdrawal: false
        });
        PositionCoordinator(LIMESTONE_DIAMOND).divestFromV2LikePosition(divCtx);
        uint256 newBalance = IERC20(BASE_USDC).balanceOf(TESTER);
        assertGt(newBalance, startingBalance);
    }

    function _createInvestmentCtx(
        address _worker,
        uint256 _token0In,
        uint256 _token1In,
        uint256 _token0Borrow,
        uint256 _token1Borrow,
        uint256 _token0Pid,
        uint256 _token1Pid
    ) internal view returns (V2LikePositionInvestmentContext memory ctx) {
        ctx.worker = _worker;
        ctx.token0In = _token0In;
        ctx.token1In = _token1In;
        ctx.token0PoolId = _token0Pid;
        ctx.token1PoolId = _token1Pid;
        ctx.token0Borrow = _token0Borrow;
        ctx.token1Borrow = _token1Borrow;
    }
}
