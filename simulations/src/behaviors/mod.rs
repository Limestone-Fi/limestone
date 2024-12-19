use arbiter_engine::machine::{CreateStateMachine, Engine, StateMachine};
use arbiter_macros::Behaviors;
use serde::{Deserialize, Serialize};

pub mod deployer;
pub mod tokens;

use deployer::Deployer;
use tokens::TokenAdmin;

#[derive(Debug, Serialize, Deserialize, Behaviors)]
pub enum Behaviors {
    Deployer(Deployer),
    Tokens(TokenAdmin),
}
