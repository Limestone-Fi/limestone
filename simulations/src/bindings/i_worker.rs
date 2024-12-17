pub use i_worker::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_worker {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("health"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("health"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("healthcheck"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("healthcheck"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reinvest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reinvest"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("work"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("work"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_debt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddShare"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddShare"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("share"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Reinvest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Reinvest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reward"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("bounty"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RemoveShare"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemoveShare"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("share"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RewardInjection"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RewardInjection"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rewardToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rewardAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RewardPaid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RewardPaid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rewardToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IWORKER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IWorker<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IWorker<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IWorker<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IWorker<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IWorker<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IWorker)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IWorker<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IWORKER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `health` (0x783b6d60) function
        pub fn health(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([120, 59, 109, 96], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `healthcheck` (0x4f42fc02) function
        pub fn healthcheck(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([79, 66, 252, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0x415f1240) function
        pub fn liquidate(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 95, 18, 64], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reinvest` (0xfdb5a03e) function
        pub fn reinvest(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 181, 160, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `work` (0x48db8cc4) function
        pub fn work(
            &self,
            id: ::ethers::core::types::U256,
            user: ::ethers::core::types::Address,
            debt: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 219, 140, 196], (id, user, debt, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddShare` event
        pub fn add_share_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddShareFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Liquidate` event
        pub fn liquidate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Reinvest` event
        pub fn reinvest_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReinvestFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemoveShare` event
        pub fn remove_share_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemoveShareFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RewardInjection` event
        pub fn reward_injection_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RewardInjectionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RewardPaid` event
        pub fn reward_paid_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RewardPaidFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IWorkerEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IWorker<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AddShare", abi = "AddShare(uint256,uint256)")]
    pub struct AddShareFilter {
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
        pub share: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Liquidate", abi = "Liquidate(uint256,uint256)")]
    pub struct LiquidateFilter {
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Reinvest", abi = "Reinvest(address,uint256,uint256)")]
    pub struct ReinvestFilter {
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        pub reward: ::ethers::core::types::U256,
        pub bounty: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RemoveShare", abi = "RemoveShare(uint256,uint256)")]
    pub struct RemoveShareFilter {
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
        pub share: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RewardInjection", abi = "RewardInjection(address,uint256)")]
    pub struct RewardInjectionFilter {
        #[ethevent(indexed)]
        pub reward_token: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RewardPaid", abi = "RewardPaid(address,address,uint256)")]
    pub struct RewardPaidFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub reward_token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum IWorkerEvents {
        AddShareFilter(AddShareFilter),
        LiquidateFilter(LiquidateFilter),
        ReinvestFilter(ReinvestFilter),
        RemoveShareFilter(RemoveShareFilter),
        RewardInjectionFilter(RewardInjectionFilter),
        RewardPaidFilter(RewardPaidFilter),
    }
    impl ::ethers::contract::EthLogDecode for IWorkerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddShareFilter::decode_log(log) {
                return Ok(IWorkerEvents::AddShareFilter(decoded));
            }
            if let Ok(decoded) = LiquidateFilter::decode_log(log) {
                return Ok(IWorkerEvents::LiquidateFilter(decoded));
            }
            if let Ok(decoded) = ReinvestFilter::decode_log(log) {
                return Ok(IWorkerEvents::ReinvestFilter(decoded));
            }
            if let Ok(decoded) = RemoveShareFilter::decode_log(log) {
                return Ok(IWorkerEvents::RemoveShareFilter(decoded));
            }
            if let Ok(decoded) = RewardInjectionFilter::decode_log(log) {
                return Ok(IWorkerEvents::RewardInjectionFilter(decoded));
            }
            if let Ok(decoded) = RewardPaidFilter::decode_log(log) {
                return Ok(IWorkerEvents::RewardPaidFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IWorkerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddShareFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReinvestFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveShareFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardInjectionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardPaidFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddShareFilter> for IWorkerEvents {
        fn from(value: AddShareFilter) -> Self {
            Self::AddShareFilter(value)
        }
    }
    impl ::core::convert::From<LiquidateFilter> for IWorkerEvents {
        fn from(value: LiquidateFilter) -> Self {
            Self::LiquidateFilter(value)
        }
    }
    impl ::core::convert::From<ReinvestFilter> for IWorkerEvents {
        fn from(value: ReinvestFilter) -> Self {
            Self::ReinvestFilter(value)
        }
    }
    impl ::core::convert::From<RemoveShareFilter> for IWorkerEvents {
        fn from(value: RemoveShareFilter) -> Self {
            Self::RemoveShareFilter(value)
        }
    }
    impl ::core::convert::From<RewardInjectionFilter> for IWorkerEvents {
        fn from(value: RewardInjectionFilter) -> Self {
            Self::RewardInjectionFilter(value)
        }
    }
    impl ::core::convert::From<RewardPaidFilter> for IWorkerEvents {
        fn from(value: RewardPaidFilter) -> Self {
            Self::RewardPaidFilter(value)
        }
    }
    ///Container type for all input parameters for the `health` function with signature `health(uint256)` and selector `0x783b6d60`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "health", abi = "health(uint256)")]
    pub struct HealthCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `healthcheck` function with signature `healthcheck()` and selector `0x4f42fc02`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "healthcheck", abi = "healthcheck()")]
    pub struct HealthcheckCall;
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate(uint256)` and selector `0x415f1240`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "liquidate", abi = "liquidate(uint256)")]
    pub struct LiquidateCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `reinvest` function with signature `reinvest()` and selector `0xfdb5a03e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "reinvest", abi = "reinvest()")]
    pub struct ReinvestCall;
    ///Container type for all input parameters for the `work` function with signature `work(uint256,address,uint256,bytes)` and selector `0x48db8cc4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "work", abi = "work(uint256,address,uint256,bytes)")]
    pub struct WorkCall {
        pub id: ::ethers::core::types::U256,
        pub user: ::ethers::core::types::Address,
        pub debt: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum IWorkerCalls {
        Health(HealthCall),
        Healthcheck(HealthcheckCall),
        Liquidate(LiquidateCall),
        Reinvest(ReinvestCall),
        Work(WorkCall),
    }
    impl ::ethers::core::abi::AbiDecode for IWorkerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <HealthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Health(decoded));
            }
            if let Ok(decoded) = <HealthcheckCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Healthcheck(decoded));
            }
            if let Ok(decoded) = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Liquidate(decoded));
            }
            if let Ok(decoded) = <ReinvestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Reinvest(decoded));
            }
            if let Ok(decoded) = <WorkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Work(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IWorkerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Health(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Healthcheck(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Reinvest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Work(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IWorkerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Health(element) => ::core::fmt::Display::fmt(element, f),
                Self::Healthcheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Reinvest(element) => ::core::fmt::Display::fmt(element, f),
                Self::Work(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<HealthCall> for IWorkerCalls {
        fn from(value: HealthCall) -> Self {
            Self::Health(value)
        }
    }
    impl ::core::convert::From<HealthcheckCall> for IWorkerCalls {
        fn from(value: HealthcheckCall) -> Self {
            Self::Healthcheck(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for IWorkerCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<ReinvestCall> for IWorkerCalls {
        fn from(value: ReinvestCall) -> Self {
            Self::Reinvest(value)
        }
    }
    impl ::core::convert::From<WorkCall> for IWorkerCalls {
        fn from(value: WorkCall) -> Self {
            Self::Work(value)
        }
    }
    ///Container type for all return fields from the `health` function with signature `health(uint256)` and selector `0x783b6d60`
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
    pub struct HealthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `healthcheck` function with signature `healthcheck()` and selector `0x4f42fc02`
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
    pub struct HealthcheckReturn(pub bool);
}
