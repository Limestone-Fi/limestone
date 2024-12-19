use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::{
    machine::{Behavior, ControlFlow, EventStream},
    messager::{Message, Messager, To},
};
use ethers::types::H160;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use super::*;
use crate::bindings::mock_token::MockToken;

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenAdminQuery {
    TokenLookup(String),
    MintRequest(MintRequest),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<H160>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MintRequest {
    token: String,
    to: H160,
    amount: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAdmin {
    #[serde(skip)]
    pub tokens: Option<HashMap<String, MockToken<ArbiterMiddleware>>>,
    pub token_data: HashMap<String, TokenData>,
    #[serde(skip)]
    pub messager: Option<Messager>,
    #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,
}

#[async_trait::async_trait]
impl Behavior<Message> for TokenAdmin {
    async fn startup(
        &mut self,
        client: Arc<arbiter_core::middleware::ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<Message>>> {
        let mut token_deployments = HashMap::new();
        let mut token_addresses = HashMap::new();
        for (token_id, data) in self.token_data.iter_mut() {
            let deploy_result = MockToken::deploy(client.clone(), ())?.send().await?;
            let token = deploy_result;
            let token_address = token.address();
            data.address = Some(token_address);
            token_deployments.insert(token_id.clone(), token);
            token_addresses.insert(token_id.clone(), token_address);
        }
        self.tokens = Some(token_deployments);
        self.messager = Some(messager.clone());
        self.client = Some(client.clone());

        // Broadcast deployments.
        messager.send(To::All, &token_addresses).await?;

        Ok(Some(messager.clone().stream().unwrap()))
    }

    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        let query: TokenAdminQuery = match serde_json::from_str(&event.data) {
            Ok(query) => query,
            Err(_) => {
                println!("Failed to deserialize event data for TokenAdminQuery");
                return Ok(ControlFlow::Continue);
            }
        };
        match query {
            TokenAdminQuery::TokenLookup(token_name) => {
                if let Some(token_data) = self.token_data.get(&token_name) {
                    let response = if let Some(v) = token_data.address {
                        v
                    } else {
                        return Err(anyhow::anyhow!("Token address could not be found"));
                    };
                    if let Some(messager) = &self.messager {
                        messager
                            .send(To::Agent(event.from.clone()), response)
                            .await?;
                    } else {
                        return Err(anyhow::anyhow!("Messager is not available"));
                    }
                } else {
                    eprintln!("Token not found: {}", token_name);
                }
                Ok(ControlFlow::Continue)
            }
            TokenAdminQuery::MintRequest(request) => Ok(ControlFlow::Continue),
        }
    }
}
