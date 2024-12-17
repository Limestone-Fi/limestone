// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {FuzzBase} from "@perimetersec/fuzzlib/src/FuzzBase.sol";
import {Constants} from "@perimetersec/fuzzlib/src/Constants.sol";
import {IStdCheats} from "@perimetersec/fuzzlib/src/IStdCheats.sol";
import {IHevm, vm} from "@perimetersec/fuzzlib/src/IHevm.sol";
import {IDromeRouter} from "../interfaces/external/IDromeRouter.sol";
import {LendingPoolConfig, InterestRateModel} from "../interfaces/ILendingPool.sol";
import {LendingPoolStorage} from "../LendingPoolStorage.sol";
import {FuzzActor} from "./FuzzActor.sol";
import {DromeMultiModalWorker} from "../workers/DromeMultiModal.sol";
import {MockPool} from "./environment/MockDromePool.sol";
import {MockPoolFactory} from "./environment/MockDromeFactory.sol";
import {MockFactoryRegistry} from "./environment/MockFactoryRegistry.sol";
import {MockGauge} from "./environment/MockDromeGauge.sol";
import {MockRouter} from "./environment/MockDromeRouter.sol";
import {MockToken} from "./environment/MockToken.sol";
import {MockUsdc} from "./environment/MockUsdc.sol";
import {MockAero} from "./environment/MockAero.sol";
import {LendingPoolWrapper} from "./LendingPoolWrapper.sol";
import {MockWarchest} from "./MockWarchest.sol";
import {MockWorker} from "./MockWorker.sol";

/// @title Fuzz Setup
/// @author Chainvisions
/// @notice Mixin for setting up the Limestone fuzzing environment.

contract FuzzSetup is FuzzActor, FuzzBase {
    error FuzzRequireError();

    MockToken weth;
    MockToken usdc;
    MockToken aero;

    MockWarchest warchestWeth;
    MockWarchest warchestUsdc;
    MockWarchest warchestAero;

    LendingPoolWrapper internal lendingPool;
    address pair;
    address dromeRouter;
    address gauge;
    address worker;

    address legacyUsdcWorker;
    address legacyAeroWorker;

    IStdCheats internal STDCHEATS = IStdCheats(Constants.ADDRESS_CHEATS);

    constructor() FuzzBase() {
        // Deploy lending pool contract.
        lendingPool = LendingPoolWrapper(address(LIMESTONE_DIAMOND));
        lendingPool.initialize();

        // Deploy our simulated on-chain environment.
        weth = new MockToken();
        usdc = MockToken(address(ONCHAIN_USDC_ADDR));
        aero = MockToken(address(ONCHAIN_AERO_ADDR));
        warchestWeth = new MockWarchest(address(weth));
        warchestUsdc = new MockWarchest(address(usdc));
        warchestAero = new MockWarchest(address(aero));

        MockPool poolImpl = new MockPool();
        MockPoolFactory dromeFactory = new MockPoolFactory(address(poolImpl));
        MockFactoryRegistry registry =
            new MockFactoryRegistry(address(dromeFactory), address(0), address(0), address(0));
        dromeRouter = address(new MockRouter(address(registry), address(dromeFactory), address(0)));
        legacyUsdcWorker = address(new MockWorker(address(usdc)));
        legacyAeroWorker = address(new MockWorker(address(aero)));

        // Create our fake AERO-USDC pair and mutate its state to mimic the real world pool.
        pair = dromeFactory.createPool(address(usdc), address(aero), false);

        usdc.mint(pair, AERO_PAIR_USDC);
        aero.mint(pair, AERO_PAIR_AERO);
        MockPool(pair).mutateSupply(AERO_PAIR_SUPPLY);
        MockPool(pair).sync();
        gauge = address(new MockGauge(pair, address(0), address(aero), false));
        MockGauge(gauge).mutateTotal(92 ether);
        for (uint256 i; i < 5; i++) {
            // We have to pass through multiple time points for a working TWAP.
            STDCHEATS.warp(block.timestamp + 30 minutes);
            MockPool(pair).sync();
        }
        aero.mint(address(this), AERO_NOTIFY_AMOUNT);
        aero.approve(gauge, AERO_NOTIFY_AMOUNT);
        MockGauge(gauge).notifyRewardAmount(AERO_NOTIFY_AMOUNT);
        lastGaugeInjection = block.timestamp;

        // Add our lending pools.
        lendingPool.addLendingPool(
            LendingPoolConfig({
                reservePoolBps: 250,
                liquidateBps: 800,
                interestRateModel: InterestRateModel.TripleSlope,
                minDebtSize: 0.1 ether
            }),
            address(weth),
            address(warchestWeth),
            0
        );
        lendingPool.addLendingPool(
            LendingPoolConfig({
                reservePoolBps: 250,
                liquidateBps: 800,
                interestRateModel: InterestRateModel.TripleSlope,
                minDebtSize: 50 * 1e6
            }),
            address(usdc),
            address(warchestUsdc),
            0
        );
        lendingPool.addLendingPool(
            LendingPoolConfig({
                reservePoolBps: 250,
                liquidateBps: 800,
                interestRateModel: InterestRateModel.TripleSlope,
                minDebtSize: 50 ether
            }),
            address(aero),
            address(warchestAero),
            0
        );

        // Seed the first three lending pools (WETH, USDC, AERO)
        weth.mint(ADDRESS_OUTSIDER_SEEDER, SEED_WETH);
        usdc.mint(ADDRESS_OUTSIDER_SEEDER, SEED_USDC);
        aero.mint(ADDRESS_OUTSIDER_SEEDER, SEED_AERO);
        STDCHEATS.prank(ADDRESS_OUTSIDER_SEEDER);
        weth.approve(address(lendingPool), SEED_WETH);

        STDCHEATS.prank(ADDRESS_OUTSIDER_SEEDER);
        usdc.approve(address(lendingPool), SEED_USDC);

        STDCHEATS.prank(ADDRESS_OUTSIDER_SEEDER);
        aero.approve(address(lendingPool), SEED_AERO);

        STDCHEATS.prank(ADDRESS_OUTSIDER_SEEDER);
        lendingPool.deposit(0, SEED_WETH);

        STDCHEATS.prank(ADDRESS_OUTSIDER_SEEDER);
        lendingPool.deposit(1, SEED_USDC);

        STDCHEATS.prank(ADDRESS_OUTSIDER_SEEDER);
        lendingPool.deposit(2, SEED_AERO);

        totalUsdcShares += SEED_USDC;
        totalUsdcLiquidity += SEED_USDC;
        totalAeroShares += SEED_AERO;
        totalAeroLiquidity += SEED_AERO;
        usdcLastUpdate = block.timestamp;
        aeroLastUpdate = block.timestamp;

        // Create the Drome worker.
        worker = address(new DromeMultiModalWorker());
        address[] memory rewards = new address[](1);
        rewards[0] = address(aero);
        DromeMultiModalWorker(worker).initialize(pair, gauge, dromeRouter, rewards);
        IDromeRouter.Route[] memory route = new IDromeRouter.Route[](1);
        route[0] =
            IDromeRouter.Route({from: address(aero), to: address(usdc), stable: false, factory: address(dromeFactory)});
        DromeMultiModalWorker(worker).setRoute(address(aero), address(usdc), route);

        // Add workers with parameters.
        address[] memory workers = new address[](3);
        LendingPoolStorage.WorkerDebtParams[] memory params = new LendingPoolStorage.WorkerDebtParams[](3);
        workers[0] = legacyUsdcWorker;
        params[0] = LendingPoolStorage.WorkerDebtParams({
            borrowable: true,
            workFactor: 6667,
            killFactor: 8333,
            authorizedPoolId: 1
        });
        workers[1] = legacyAeroWorker;
        params[1] = LendingPoolStorage.WorkerDebtParams({
            borrowable: true,
            workFactor: 6667,
            killFactor: 8333,
            authorizedPoolId: 2
        });
        workers[2] = worker;
        params[2] = LendingPoolStorage.WorkerDebtParams({
            borrowable: true,
            workFactor: 300,
            killFactor: 8333,
            authorizedPoolId: 0
        });
        lendingPool.addWorkers(workers, params);

        // Setup actors.
        for (uint256 i; i < ACTORS.length; i++) {
            _setupActor(ACTORS[i], STARTING_BALANCE);
        }
    }

    function _setupActor(address _actor, uint256 _amount) internal {
        usdc.mint(_actor, _amount);
        aero.mint(_actor, _amount);
        STDCHEATS.prank(_actor);
        usdc.approve(address(lendingPool), type(uint256).max);
        STDCHEATS.prank(_actor);
        aero.approve(address(lendingPool), type(uint256).max);
    }
}
