pub use i_position_coordinator::*;
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
pub mod i_position_coordinator {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("accessAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accessAssets"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                    ::std::borrow::ToOwned::to_owned("divestFromV2LikePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "divestFromV2LikePosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ctx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct V2LikePositionDivestmentContext",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("investInV2LikePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "investInV2LikePosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ctx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct V2LikePositionInvestmentContext",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("liquidateV2LikePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "liquidateV2LikePosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ctx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct V2LikePositionLiquidationContext",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("reinvestmentFeeNumerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "reinvestmentFeeNumerator",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("repayV2LikeLiquidityPositionDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "repayV2LikeLiquidityPositionDebt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_positionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_worker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token0Repay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token1Repay"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PositionDivested"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PositionDivested"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("positionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("worker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityBurnt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token0Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token1Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token0Repaid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token1Repaid"),
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
                    ::std::borrow::ToOwned::to_owned("PositionInvested"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PositionInvested"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("positionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("worker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token0In"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token1In"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token0Borrowed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token1Borrowed"),
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
    pub static IPOSITIONCOORDINATOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IPositionCoordinator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPositionCoordinator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPositionCoordinator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPositionCoordinator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPositionCoordinator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IPositionCoordinator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPositionCoordinator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPOSITIONCOORDINATOR_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `accessAssets` (0x75df049e) function
        pub fn access_assets(
            &self,
            user: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 223, 4, 158], (user, tokens, amounts))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `divestFromV2LikePosition` (0xf3e215a6) function
        pub fn divest_from_v2_like_position(
            &self,
            ctx: V2LikePositionDivestmentContext,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 226, 21, 166], (ctx,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `investInV2LikePosition` (0x00698b37) function
        pub fn invest_in_v2_like_position(
            &self,
            ctx: V2LikePositionInvestmentContext,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 105, 139, 55], (ctx,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidateV2LikePosition` (0xe5f266e6) function
        pub fn liquidate_v2_like_position(
            &self,
            ctx: V2LikePositionLiquidationContext,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 242, 102, 230], (ctx,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reinvestmentFeeNumerator` (0x61346071) function
        pub fn reinvestment_fee_numerator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([97, 52, 96, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayV2LikeLiquidityPositionDebt` (0x538dbb9a) function
        pub fn repay_v2_like_liquidity_position_debt(
            &self,
            position_id: ::ethers::core::types::U256,
            worker: ::ethers::core::types::Address,
            token_0_repay: ::ethers::core::types::U256,
            token_1_repay: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [83, 141, 187, 154],
                    (position_id, worker, token_0_repay, token_1_repay),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PositionDivested` event
        pub fn position_divested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionDivestedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PositionInvested` event
        pub fn position_invested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionInvestedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IPositionCoordinatorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IPositionCoordinator<M> {
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
    #[ethevent(
        name = "PositionDivested",
        abi = "PositionDivested(uint256,address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct PositionDivestedFilter {
        #[ethevent(indexed)]
        pub position_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub worker: ::ethers::core::types::Address,
        pub liquidity_burnt: ::ethers::core::types::U256,
        pub token_0_out: ::ethers::core::types::U256,
        pub token_1_out: ::ethers::core::types::U256,
        pub token_0_repaid: ::ethers::core::types::U256,
        pub token_1_repaid: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "PositionInvested",
        abi = "PositionInvested(uint256,address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct PositionInvestedFilter {
        #[ethevent(indexed)]
        pub position_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub worker: ::ethers::core::types::Address,
        pub token_0_in: ::ethers::core::types::U256,
        pub token_1_in: ::ethers::core::types::U256,
        pub token_0_borrowed: ::ethers::core::types::U256,
        pub token_1_borrowed: ::ethers::core::types::U256,
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
    pub enum IPositionCoordinatorEvents {
        PositionDivestedFilter(PositionDivestedFilter),
        PositionInvestedFilter(PositionInvestedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IPositionCoordinatorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PositionDivestedFilter::decode_log(log) {
                return Ok(IPositionCoordinatorEvents::PositionDivestedFilter(decoded));
            }
            if let Ok(decoded) = PositionInvestedFilter::decode_log(log) {
                return Ok(IPositionCoordinatorEvents::PositionInvestedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IPositionCoordinatorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PositionDivestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionInvestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<PositionDivestedFilter> for IPositionCoordinatorEvents {
        fn from(value: PositionDivestedFilter) -> Self {
            Self::PositionDivestedFilter(value)
        }
    }
    impl ::core::convert::From<PositionInvestedFilter> for IPositionCoordinatorEvents {
        fn from(value: PositionInvestedFilter) -> Self {
            Self::PositionInvestedFilter(value)
        }
    }
    ///Container type for all input parameters for the `accessAssets` function with signature `accessAssets(address,address[],uint256[])` and selector `0x75df049e`
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
    #[ethcall(name = "accessAssets", abi = "accessAssets(address,address[],uint256[])")]
    pub struct AccessAssetsCall {
        pub user: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `divestFromV2LikePosition` function with signature `divestFromV2LikePosition((uint256,address,uint16,uint256,uint256,uint256,uint256,uint8,bool))` and selector `0xf3e215a6`
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
    #[ethcall(
        name = "divestFromV2LikePosition",
        abi = "divestFromV2LikePosition((uint256,address,uint16,uint256,uint256,uint256,uint256,uint8,bool))"
    )]
    pub struct DivestFromV2LikePositionCall {
        pub ctx: V2LikePositionDivestmentContext,
    }
    ///Container type for all input parameters for the `investInV2LikePosition` function with signature `investInV2LikePosition((uint256,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,bool))` and selector `0x00698b37`
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
    #[ethcall(
        name = "investInV2LikePosition",
        abi = "investInV2LikePosition((uint256,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,bool))"
    )]
    pub struct InvestInV2LikePositionCall {
        pub ctx: V2LikePositionInvestmentContext,
    }
    ///Container type for all input parameters for the `liquidateV2LikePosition` function with signature `liquidateV2LikePosition((uint256,address,uint16,uint256,uint256,uint256,uint256))` and selector `0xe5f266e6`
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
    #[ethcall(
        name = "liquidateV2LikePosition",
        abi = "liquidateV2LikePosition((uint256,address,uint16,uint256,uint256,uint256,uint256))"
    )]
    pub struct LiquidateV2LikePositionCall {
        pub ctx: V2LikePositionLiquidationContext,
    }
    ///Container type for all input parameters for the `reinvestmentFeeNumerator` function with signature `reinvestmentFeeNumerator()` and selector `0x61346071`
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
    #[ethcall(name = "reinvestmentFeeNumerator", abi = "reinvestmentFeeNumerator()")]
    pub struct ReinvestmentFeeNumeratorCall;
    ///Container type for all input parameters for the `repayV2LikeLiquidityPositionDebt` function with signature `repayV2LikeLiquidityPositionDebt(uint256,address,uint256,uint256)` and selector `0x538dbb9a`
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
    #[ethcall(
        name = "repayV2LikeLiquidityPositionDebt",
        abi = "repayV2LikeLiquidityPositionDebt(uint256,address,uint256,uint256)"
    )]
    pub struct RepayV2LikeLiquidityPositionDebtCall {
        pub position_id: ::ethers::core::types::U256,
        pub worker: ::ethers::core::types::Address,
        pub token_0_repay: ::ethers::core::types::U256,
        pub token_1_repay: ::ethers::core::types::U256,
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
    pub enum IPositionCoordinatorCalls {
        AccessAssets(AccessAssetsCall),
        DivestFromV2LikePosition(DivestFromV2LikePositionCall),
        InvestInV2LikePosition(InvestInV2LikePositionCall),
        LiquidateV2LikePosition(LiquidateV2LikePositionCall),
        ReinvestmentFeeNumerator(ReinvestmentFeeNumeratorCall),
        RepayV2LikeLiquidityPositionDebt(RepayV2LikeLiquidityPositionDebtCall),
    }
    impl ::ethers::core::abi::AbiDecode for IPositionCoordinatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccessAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessAssets(decoded));
            }
            if let Ok(decoded) = <DivestFromV2LikePositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivestFromV2LikePosition(decoded));
            }
            if let Ok(decoded) = <InvestInV2LikePositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvestInV2LikePosition(decoded));
            }
            if let Ok(decoded) = <LiquidateV2LikePositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidateV2LikePosition(decoded));
            }
            if let Ok(decoded) = <ReinvestmentFeeNumeratorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReinvestmentFeeNumerator(decoded));
            }
            if let Ok(decoded) = <RepayV2LikeLiquidityPositionDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RepayV2LikeLiquidityPositionDebt(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPositionCoordinatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccessAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DivestFromV2LikePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvestInV2LikePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidateV2LikePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReinvestmentFeeNumerator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayV2LikeLiquidityPositionDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IPositionCoordinatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivestFromV2LikePosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvestInV2LikePosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidateV2LikePosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReinvestmentFeeNumerator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RepayV2LikeLiquidityPositionDebt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AccessAssetsCall> for IPositionCoordinatorCalls {
        fn from(value: AccessAssetsCall) -> Self {
            Self::AccessAssets(value)
        }
    }
    impl ::core::convert::From<DivestFromV2LikePositionCall>
    for IPositionCoordinatorCalls {
        fn from(value: DivestFromV2LikePositionCall) -> Self {
            Self::DivestFromV2LikePosition(value)
        }
    }
    impl ::core::convert::From<InvestInV2LikePositionCall>
    for IPositionCoordinatorCalls {
        fn from(value: InvestInV2LikePositionCall) -> Self {
            Self::InvestInV2LikePosition(value)
        }
    }
    impl ::core::convert::From<LiquidateV2LikePositionCall>
    for IPositionCoordinatorCalls {
        fn from(value: LiquidateV2LikePositionCall) -> Self {
            Self::LiquidateV2LikePosition(value)
        }
    }
    impl ::core::convert::From<ReinvestmentFeeNumeratorCall>
    for IPositionCoordinatorCalls {
        fn from(value: ReinvestmentFeeNumeratorCall) -> Self {
            Self::ReinvestmentFeeNumerator(value)
        }
    }
    impl ::core::convert::From<RepayV2LikeLiquidityPositionDebtCall>
    for IPositionCoordinatorCalls {
        fn from(value: RepayV2LikeLiquidityPositionDebtCall) -> Self {
            Self::RepayV2LikeLiquidityPositionDebt(value)
        }
    }
    ///Container type for all return fields from the `reinvestmentFeeNumerator` function with signature `reinvestmentFeeNumerator()` and selector `0x61346071`
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
    pub struct ReinvestmentFeeNumeratorReturn(pub ::ethers::core::types::U256);
}
