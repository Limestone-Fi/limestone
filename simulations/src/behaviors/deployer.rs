use anyhow::Result;
use arbiter_engine::{
    machine::{Behavior, EventStream},
    messager::{Messager, To},
};
use ethers::{
    types::{H160, U256},
    utils::parse_units,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bindings::drome_multi_modal_worker::DromeMultiModalWorker;
use crate::bindings::lending_pool_wrapper::{LendingPoolConfig, LendingPoolWrapper};
use crate::bindings::mock_factory_registry::MockFactoryRegistry;
use crate::bindings::mock_gauge::MockGauge;
use crate::bindings::mock_pool::MockPool;
use crate::bindings::mock_pool_factory::MockPoolFactory;
use crate::bindings::mock_router::MockRouter;
use crate::bindings::mock_warchest::MockWarchest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Deployments {
    pub amm_factory: H160,
    pub amm_router: H160,
    pub amm_pair: H160,
    pub amm_gauge: H160,
    pub lending_pool: H160,
    pub worker: H160,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deployer;

#[async_trait::async_trait]
impl Behavior<()> for Deployer {
    async fn startup(
        &mut self,
        client: std::sync::Arc<arbiter_core::middleware::ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        let tokens: HashMap<String, H160> =
            match serde_json::from_str(messager.get_next().await?.data.as_str()) {
                Ok(tokens) => Some(tokens),
                Err(_) => {
                    eprintln!("Failed to fetch token deployments!");
                    None
                }
            }
            .expect("Token deployments fetch failed");

        // Deploy mock AMM environment.
        let pool_implementation = MockPool::deploy(client.clone(), ())?.send().await?;
        let factory_deployment =
            MockPoolFactory::deploy(client.clone(), pool_implementation.address())?
                .send()
                .await?;
        let factory_registry_deployment = MockFactoryRegistry::deploy(
            client.clone(),
            (
                factory_deployment.clone().address(),
                H160::default(),
                H160::default(),
                H160::default(),
            ),
        )?
        .send()
        .await?;
        let token0 = tokens.get("USDC").unwrap();
        let token1 = tokens.get("AERO").unwrap();
        factory_deployment
            .create_pool(token0.clone(), token1.clone(), false)
            .send()
            .await?;
        let pair_address = factory_deployment
            .get_pool_with_token_a_and_token_b_and_stable(token0.clone(), token1.clone(), false)
            .call()
            .await?;
        let gauge_deployment = MockGauge::deploy(
            client.clone(),
            (pair_address.clone(), H160::default(), token1.clone(), true),
        )?
        .send()
        .await?;
        let router_deployment = MockRouter::deploy(
            client.clone(),
            (
                factory_registry_deployment.address(),
                factory_deployment.clone().address(),
                H160::default(),
            ),
        )?
        .send()
        .await?;

        // Deploy Limestone related contracts.
        let token0_warchest_deployment = MockWarchest::deploy(client.clone(), token0.clone())?
            .send()
            .await?;
        let token1_warchest_deployment = MockWarchest::deploy(client.clone(), token1.clone())?
            .send()
            .await?;
        let lending_pool_deployment = LendingPoolWrapper::deploy(client.clone(), ())?
            .send()
            .await?;
        lending_pool_deployment.initialize().send().await?;
        lending_pool_deployment.add_lending_pool(
            LendingPoolConfig {
                reserve_pool_bps: 250,
                liquidate_bps: 1000,
                interest_rate_model: 0,
                min_debt_size: U256::from(parse_units("100", 6).unwrap()).as_u128(),
            },
            token0.clone(),
            token0_warchest_deployment.address(),
            0.into(),
        );
        lending_pool_deployment.add_lending_pool(
            LendingPoolConfig {
                reserve_pool_bps: 250,
                liquidate_bps: 1000,
                interest_rate_model: 0,
                min_debt_size: U256::from(parse_units("100", 18).unwrap()).as_u128(),
            },
            token1.clone(),
            token1_warchest_deployment.address(),
            0.into(),
        );
        let worker_deployment = DromeMultiModalWorker::deploy(client.clone(), ())?
            .send()
            .await?;
        worker_deployment
            .initialize(
                pair_address.clone(),
                gauge_deployment.clone().address(),
                router_deployment.clone().address(),
                vec![token0.clone()],
            )
            .send()
            .await?;

        // Send addresses to all agents.
        messager
            .send(
                To::All,
                Deployments {
                    amm_factory: factory_deployment.address(),
                    amm_router: router_deployment.address(),
                    amm_pair: pair_address,
                    amm_gauge: gauge_deployment.address(),
                    lending_pool: lending_pool_deployment.address(),
                    worker: worker_deployment.address(),
                },
            )
            .await?;
        Ok(None)
    }
}
