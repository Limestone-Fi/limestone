pub use i_multi_modal_worker::*;
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
pub mod i_multi_modal_worker {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("calculatePositionValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculatePositionValue",
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionEquity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionDebt"),
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
                    ::std::borrow::ToOwned::to_owned("divest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("divest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPosition"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(112usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(112usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(112usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct MultiModalPosition",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("invest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invest"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_debtShare0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        112usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint112"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_debtShare1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        112usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint112"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                    name: ::std::borrow::ToOwned::to_owned("_liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("repayDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repayDebt"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_repayToken0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_repayToken1"),
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
                                        112usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint112"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        112usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint112"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IMULTIMODALWORKER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IMultiModalWorker<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IMultiModalWorker<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IMultiModalWorker<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IMultiModalWorker<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IMultiModalWorker<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IMultiModalWorker))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IMultiModalWorker<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IMULTIMODALWORKER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `calculatePositionValue` (0x02a50329) function
        pub fn calculate_position_value(
            &self,
            position_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([2, 165, 3, 41], position_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `divest` (0xa1271c4b) function
        pub fn divest(
            &self,
            p0: V2LikePositionDivestmentContext,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([161, 39, 28, 75], (p0,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPosition` (0xeb02c301) function
        pub fn get_position(
            &self,
            position_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, MultiModalPosition> {
            self.0
                .method_hash([235, 2, 195, 1], position_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invest` (0x6fe864c5) function
        pub fn invest(
            &self,
            ctx: V2LikePositionInvestmentContext,
            borrower: ::ethers::core::types::Address,
            debt_share_0: u128,
            debt_share_1: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [111, 232, 100, 197],
                    (ctx, borrower, debt_share_0, debt_share_1),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0xe90d4c77) function
        pub fn liquidate(
            &self,
            liquidator: ::ethers::core::types::Address,
            ctx: V2LikePositionLiquidationContext,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 13, 76, 119], (liquidator, ctx))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reinvest` (0xfdb5a03e) function
        pub fn reinvest(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 181, 160, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayDebt` (0x80594009) function
        pub fn repay_debt(
            &self,
            position_id: ::ethers::core::types::U256,
            repay_token_0: ::ethers::core::types::U256,
            repay_token_1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash(
                    [128, 89, 64, 9],
                    (position_id, repay_token_0, repay_token_1),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IMultiModalWorker<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `calculatePositionValue` function with signature `calculatePositionValue(uint256)` and selector `0x02a50329`
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
    #[ethcall(name = "calculatePositionValue", abi = "calculatePositionValue(uint256)")]
    pub struct CalculatePositionValueCall {
        pub position_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `divest` function with signature `divest((uint256,address,uint16,uint256,uint256,uint256,uint256,uint8,bool))` and selector `0xa1271c4b`
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
        name = "divest",
        abi = "divest((uint256,address,uint16,uint256,uint256,uint256,uint256,uint8,bool))"
    )]
    pub struct DivestCall(pub V2LikePositionDivestmentContext);
    ///Container type for all input parameters for the `getPosition` function with signature `getPosition(uint256)` and selector `0xeb02c301`
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
    #[ethcall(name = "getPosition", abi = "getPosition(uint256)")]
    pub struct GetPositionCall {
        pub position_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `invest` function with signature `invest((uint256,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,bool),address,uint112,uint112)` and selector `0x6fe864c5`
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
        name = "invest",
        abi = "invest((uint256,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,bool),address,uint112,uint112)"
    )]
    pub struct InvestCall {
        pub ctx: V2LikePositionInvestmentContext,
        pub borrower: ::ethers::core::types::Address,
        pub debt_share_0: u128,
        pub debt_share_1: u128,
    }
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate(address,(uint256,address,uint16,uint256,uint256,uint256,uint256))` and selector `0xe90d4c77`
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
        name = "liquidate",
        abi = "liquidate(address,(uint256,address,uint16,uint256,uint256,uint256,uint256))"
    )]
    pub struct LiquidateCall {
        pub liquidator: ::ethers::core::types::Address,
        pub ctx: V2LikePositionLiquidationContext,
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
    ///Container type for all input parameters for the `repayDebt` function with signature `repayDebt(uint256,uint256,uint256)` and selector `0x80594009`
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
    #[ethcall(name = "repayDebt", abi = "repayDebt(uint256,uint256,uint256)")]
    pub struct RepayDebtCall {
        pub position_id: ::ethers::core::types::U256,
        pub repay_token_0: ::ethers::core::types::U256,
        pub repay_token_1: ::ethers::core::types::U256,
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
    pub enum IMultiModalWorkerCalls {
        CalculatePositionValue(CalculatePositionValueCall),
        Divest(DivestCall),
        GetPosition(GetPositionCall),
        Invest(InvestCall),
        Liquidate(LiquidateCall),
        Reinvest(ReinvestCall),
        RepayDebt(RepayDebtCall),
    }
    impl ::ethers::core::abi::AbiDecode for IMultiModalWorkerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CalculatePositionValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculatePositionValue(decoded));
            }
            if let Ok(decoded) = <DivestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Divest(decoded));
            }
            if let Ok(decoded) = <GetPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPosition(decoded));
            }
            if let Ok(decoded) = <InvestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invest(decoded));
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
            if let Ok(decoded) = <RepayDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RepayDebt(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IMultiModalWorkerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CalculatePositionValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Divest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Reinvest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IMultiModalWorkerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CalculatePositionValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Divest(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invest(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Reinvest(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayDebt(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CalculatePositionValueCall> for IMultiModalWorkerCalls {
        fn from(value: CalculatePositionValueCall) -> Self {
            Self::CalculatePositionValue(value)
        }
    }
    impl ::core::convert::From<DivestCall> for IMultiModalWorkerCalls {
        fn from(value: DivestCall) -> Self {
            Self::Divest(value)
        }
    }
    impl ::core::convert::From<GetPositionCall> for IMultiModalWorkerCalls {
        fn from(value: GetPositionCall) -> Self {
            Self::GetPosition(value)
        }
    }
    impl ::core::convert::From<InvestCall> for IMultiModalWorkerCalls {
        fn from(value: InvestCall) -> Self {
            Self::Invest(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for IMultiModalWorkerCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<ReinvestCall> for IMultiModalWorkerCalls {
        fn from(value: ReinvestCall) -> Self {
            Self::Reinvest(value)
        }
    }
    impl ::core::convert::From<RepayDebtCall> for IMultiModalWorkerCalls {
        fn from(value: RepayDebtCall) -> Self {
            Self::RepayDebt(value)
        }
    }
    ///Container type for all return fields from the `calculatePositionValue` function with signature `calculatePositionValue(uint256)` and selector `0x02a50329`
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
    pub struct CalculatePositionValueReturn {
        pub position_equity: ::ethers::core::types::U256,
        pub position_debt: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `divest` function with signature `divest((uint256,address,uint16,uint256,uint256,uint256,uint256,uint8,bool))` and selector `0xa1271c4b`
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
    pub struct DivestReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getPosition` function with signature `getPosition(uint256)` and selector `0xeb02c301`
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
    pub struct GetPositionReturn(pub MultiModalPosition);
    ///Container type for all return fields from the `invest` function with signature `invest((uint256,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,bool),address,uint112,uint112)` and selector `0x6fe864c5`
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
    pub struct InvestReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `repayDebt` function with signature `repayDebt(uint256,uint256,uint256)` and selector `0x80594009`
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
    pub struct RepayDebtReturn(pub u128, pub u128);
}
