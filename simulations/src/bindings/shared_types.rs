///`Observation(uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Observation {
    pub timestamp: ::ethers::core::types::U256,
    pub reserve_0_cumulative: ::ethers::core::types::U256,
    pub reserve_1_cumulative: ::ethers::core::types::U256,
}
///`Route(address,address,bool,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Route {
    pub from: ::ethers::core::types::Address,
    pub to: ::ethers::core::types::Address,
    pub stable: bool,
    pub factory: ::ethers::core::types::Address,
}
///`Zap(address,address,bool,address,uint256,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Zap {
    pub token_a: ::ethers::core::types::Address,
    pub token_b: ::ethers::core::types::Address,
    pub stable: bool,
    pub factory: ::ethers::core::types::Address,
    pub amount_out_min_a: ::ethers::core::types::U256,
    pub amount_out_min_b: ::ethers::core::types::U256,
    pub amount_a_min: ::ethers::core::types::U256,
    pub amount_b_min: ::ethers::core::types::U256,
}
///`FacetCut(address,uint8,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FacetCut {
    pub target: ::ethers::core::types::Address,
    pub action: u8,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
///`Facet(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Facet {
    pub target: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
///`WorkContext(uint256,address,uint256,uint256,uint256,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct WorkContext {
    pub pool_id: ::ethers::core::types::U256,
    pub worker: ::ethers::core::types::Address,
    pub amount_in: ::ethers::core::types::U256,
    pub loan: ::ethers::core::types::U256,
    pub max_return: ::ethers::core::types::U256,
    pub data: ::ethers::core::types::Bytes,
}
///`LendingPoolConfig(uint16,uint16,uint8,uint96)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct LendingPoolConfig {
    pub reserve_pool_bps: u16,
    pub liquidate_bps: u16,
    pub interest_rate_model: u8,
    pub min_debt_size: u128,
}
///`WorkerDebtParams(uint16,uint16,uint16,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct WorkerDebtParams {
    pub authorized_pool_id: u16,
    pub work_factor: u16,
    pub kill_factor: u16,
    pub borrowable: bool,
}
///`Market(uint112,uint32,uint112,uint112,uint16,uint16,uint112,uint8,address,uint88,uint96,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Market {
    pub total_shares: u128,
    pub last_accrue_time: u32,
    pub global_debt_value: u128,
    pub global_debt_share: u128,
    pub reserve_pool_bps: u16,
    pub liquidate_bps: u16,
    pub reserve_pool: u128,
    pub interest_rate_model: u8,
    pub warchest: ::ethers::core::types::Address,
    pub delegated_debt_available: u128,
    pub min_debt_size: u128,
    pub underlying: ::ethers::core::types::Address,
}
///`MultiModalPosition(address,uint112,uint32,uint32,uint112,uint112)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct MultiModalPosition {
    pub owner: ::ethers::core::types::Address,
    pub position_shares: u128,
    pub debt_0_pool_id: u32,
    pub debt_1_pool_id: u32,
    pub debt_share_0: u128,
    pub debt_share_1: u128,
}
///`Position(address,address,uint32,uint112)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Position {
    pub worker: ::ethers::core::types::Address,
    pub owner: ::ethers::core::types::Address,
    pub pool_id: u32,
    pub debt_share: u128,
}
///`V2LikePositionDivestmentContext(uint256,address,uint16,uint256,uint256,uint256,uint256,uint8,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct V2LikePositionDivestmentContext {
    pub position_id: ::ethers::core::types::U256,
    pub worker: ::ethers::core::types::Address,
    pub position_bps: u16,
    pub min_token_0_out: ::ethers::core::types::U256,
    pub min_token_1_out: ::ethers::core::types::U256,
    pub token_0_repay: ::ethers::core::types::U256,
    pub token_1_repay: ::ethers::core::types::U256,
    pub side: u8,
    pub minimal_withdrawal: bool,
}
///`V2LikePositionInvestmentContext(uint256,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct V2LikePositionInvestmentContext {
    pub position_id: ::ethers::core::types::U256,
    pub worker: ::ethers::core::types::Address,
    pub token_0_in: ::ethers::core::types::U256,
    pub token_1_in: ::ethers::core::types::U256,
    pub token_0_pool_id: ::ethers::core::types::U256,
    pub token_1_pool_id: ::ethers::core::types::U256,
    pub token_0_borrow: ::ethers::core::types::U256,
    pub token_1_borrow: ::ethers::core::types::U256,
    pub min_liquidity_minted: ::ethers::core::types::U256,
    pub skip_healthcheck: bool,
}
///`V2LikePositionLiquidationContext(uint256,address,uint16,uint256,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct V2LikePositionLiquidationContext {
    pub position_id: ::ethers::core::types::U256,
    pub worker: ::ethers::core::types::Address,
    pub position_bps: u16,
    pub token_0_repay_in: ::ethers::core::types::U256,
    pub token_1_repay_in: ::ethers::core::types::U256,
    pub min_token_0_out: ::ethers::core::types::U256,
    pub min_token_1_out: ::ethers::core::types::U256,
}
