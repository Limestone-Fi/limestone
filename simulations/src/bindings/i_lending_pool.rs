pub use i_lending_pool::*;
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
pub mod i_lending_pool {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("accessUserAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accessUserAssets"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_requestedAmount"),
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
                    ::std::borrow::ToOwned::to_owned("authorizedKeepers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("authorizedKeepers"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_keeper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("authorizedLiquidators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "authorizedLiquidators",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("debtShareToVal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("debtShareToVal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_debtShare"),
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
                                        112usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint112"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("debtValToShare"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("debtValToShare"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_debtVal"),
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
                                        112usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint112"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                    ::std::borrow::ToOwned::to_owned("permissionedLiquidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "permissionedLiquidation",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("pools"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pools"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(112usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(112usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(112usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(112usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(88usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Market[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("positions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("positions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_posId"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(112usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Position"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                    ::std::borrow::ToOwned::to_owned("AddDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddDebt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("posId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtShare"),
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
                    ::std::borrow::ToOwned::to_owned("Borrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Borrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("posId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("loan"),
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
                    ::std::borrow::ToOwned::to_owned("DelegatedBorrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DelegatedBorrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtHolder"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("borrowed"),
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
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                    ::std::borrow::ToOwned::to_owned("IncreaseCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IncreaseCollateral"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("posId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collateralAdded"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("healthBefore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("healthAfter"),
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
                    ::std::borrow::ToOwned::to_owned("Kill"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Kill"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("prize"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("left"),
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
                    ::std::borrow::ToOwned::to_owned("MarketCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MarketCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("underlying"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("warchest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("parameters"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RemoveDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemoveDebt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("posId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtShare"),
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
                    ::std::borrow::ToOwned::to_owned("Withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("totalShares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountUnderlying"),
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
    pub static ILENDINGPOOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct ILendingPool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ILendingPool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ILendingPool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ILendingPool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ILendingPool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ILendingPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ILendingPool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ILENDINGPOOL_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `accessUserAssets` (0xde75ddea) function
        pub fn access_user_assets(
            &self,
            token: ::ethers::core::types::Address,
            requested_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 117, 221, 234], (token, requested_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `authorizedKeepers` (0xc56f264d) function
        pub fn authorized_keepers(
            &self,
            keeper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([197, 111, 38, 77], keeper)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `authorizedLiquidators` (0xdfa48f87) function
        pub fn authorized_liquidators(
            &self,
            liquidator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([223, 164, 143, 135], liquidator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `debtShareToVal` (0xc923d913) function
        pub fn debt_share_to_val(
            &self,
            pool_id: ::ethers::core::types::U256,
            debt_share: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([201, 35, 217, 19], (pool_id, debt_share))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `debtValToShare` (0xa0ffb7b8) function
        pub fn debt_val_to_share(
            &self,
            pool_id: ::ethers::core::types::U256,
            debt_val: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([160, 255, 183, 184], (pool_id, debt_val))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xe2bbb158) function
        pub fn deposit(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 187, 177, 88], (pool_id, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permissionedLiquidation` (0x4245d780) function
        pub fn permissioned_liquidation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([66, 69, 215, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pools` (0xc5c51dca) function
        pub fn pools(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Market>> {
            self.0
                .method_hash([197, 197, 29, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `positions` (0x99fbab88) function
        pub fn positions(
            &self,
            pos_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Position> {
            self.0
                .method_hash([153, 251, 171, 136], pos_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x441a3e70) function
        pub fn withdraw(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 26, 62, 112], (pool_id, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddDebt` event
        pub fn add_debt_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddDebtFilter> {
            self.0.event()
        }
        ///Gets the contract's `Borrow` event
        pub fn borrow_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BorrowFilter> {
            self.0.event()
        }
        ///Gets the contract's `DelegatedBorrow` event
        pub fn delegated_borrow_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DelegatedBorrowFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `IncreaseCollateral` event
        pub fn increase_collateral_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IncreaseCollateralFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Kill` event
        pub fn kill_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, KillFilter> {
            self.0.event()
        }
        ///Gets the contract's `MarketCreated` event
        pub fn market_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MarketCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemoveDebt` event
        pub fn remove_debt_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemoveDebtFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdraw` event
        pub fn withdraw_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ILendingPoolEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ILendingPool<M> {
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
    #[ethevent(name = "AddDebt", abi = "AddDebt(uint256,uint256,uint256)")]
    pub struct AddDebtFilter {
        #[ethevent(indexed)]
        pub pool_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub pos_id: ::ethers::core::types::U256,
        pub debt_share: ::ethers::core::types::U256,
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
    #[ethevent(name = "Borrow", abi = "Borrow(uint256,uint256,uint256)")]
    pub struct BorrowFilter {
        #[ethevent(indexed)]
        pub pool_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub pos_id: ::ethers::core::types::U256,
        pub loan: ::ethers::core::types::U256,
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
        name = "DelegatedBorrow",
        abi = "DelegatedBorrow(uint256,address,uint256)"
    )]
    pub struct DelegatedBorrowFilter {
        #[ethevent(indexed)]
        pub pool_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub debt_holder: ::ethers::core::types::Address,
        pub borrowed: ::ethers::core::types::U256,
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
    #[ethevent(name = "Deposit", abi = "Deposit(uint256,address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub pool_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "IncreaseCollateral",
        abi = "IncreaseCollateral(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct IncreaseCollateralFilter {
        #[ethevent(indexed)]
        pub pool_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub pos_id: ::ethers::core::types::U256,
        pub collateral_added: ::ethers::core::types::U256,
        pub health_before: ::ethers::core::types::U256,
        pub health_after: ::ethers::core::types::U256,
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
    #[ethevent(name = "Kill", abi = "Kill(uint256,address,uint256,uint256)")]
    pub struct KillFilter {
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub liquidator: ::ethers::core::types::Address,
        pub prize: ::ethers::core::types::U256,
        pub left: ::ethers::core::types::U256,
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
        name = "MarketCreated",
        abi = "MarketCreated(uint256,address,address,(uint16,uint16,uint8,uint96))"
    )]
    pub struct MarketCreatedFilter {
        #[ethevent(indexed)]
        pub pool_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub underlying: ::ethers::core::types::Address,
        pub warchest: ::ethers::core::types::Address,
        pub parameters: LendingPoolConfig,
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
    #[ethevent(name = "RemoveDebt", abi = "RemoveDebt(uint256,uint256,uint256)")]
    pub struct RemoveDebtFilter {
        #[ethevent(indexed)]
        pub pool_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub pos_id: ::ethers::core::types::U256,
        pub debt_share: ::ethers::core::types::U256,
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
    #[ethevent(name = "Withdraw", abi = "Withdraw(uint256,address,uint256,uint256)")]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub pool_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        pub total_shares: ::ethers::core::types::U256,
        pub amount_underlying: ::ethers::core::types::U256,
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
    pub enum ILendingPoolEvents {
        AddDebtFilter(AddDebtFilter),
        BorrowFilter(BorrowFilter),
        DelegatedBorrowFilter(DelegatedBorrowFilter),
        DepositFilter(DepositFilter),
        IncreaseCollateralFilter(IncreaseCollateralFilter),
        KillFilter(KillFilter),
        MarketCreatedFilter(MarketCreatedFilter),
        RemoveDebtFilter(RemoveDebtFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for ILendingPoolEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddDebtFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::AddDebtFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = DelegatedBorrowFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::DelegatedBorrowFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = IncreaseCollateralFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::IncreaseCollateralFilter(decoded));
            }
            if let Ok(decoded) = KillFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::KillFilter(decoded));
            }
            if let Ok(decoded) = MarketCreatedFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::MarketCreatedFilter(decoded));
            }
            if let Ok(decoded) = RemoveDebtFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::RemoveDebtFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ILendingPoolEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddDebtFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BorrowFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegatedBorrowFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseCollateralFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::KillFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarketCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveDebtFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddDebtFilter> for ILendingPoolEvents {
        fn from(value: AddDebtFilter) -> Self {
            Self::AddDebtFilter(value)
        }
    }
    impl ::core::convert::From<BorrowFilter> for ILendingPoolEvents {
        fn from(value: BorrowFilter) -> Self {
            Self::BorrowFilter(value)
        }
    }
    impl ::core::convert::From<DelegatedBorrowFilter> for ILendingPoolEvents {
        fn from(value: DelegatedBorrowFilter) -> Self {
            Self::DelegatedBorrowFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for ILendingPoolEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<IncreaseCollateralFilter> for ILendingPoolEvents {
        fn from(value: IncreaseCollateralFilter) -> Self {
            Self::IncreaseCollateralFilter(value)
        }
    }
    impl ::core::convert::From<KillFilter> for ILendingPoolEvents {
        fn from(value: KillFilter) -> Self {
            Self::KillFilter(value)
        }
    }
    impl ::core::convert::From<MarketCreatedFilter> for ILendingPoolEvents {
        fn from(value: MarketCreatedFilter) -> Self {
            Self::MarketCreatedFilter(value)
        }
    }
    impl ::core::convert::From<RemoveDebtFilter> for ILendingPoolEvents {
        fn from(value: RemoveDebtFilter) -> Self {
            Self::RemoveDebtFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for ILendingPoolEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
    }
    ///Container type for all input parameters for the `accessUserAssets` function with signature `accessUserAssets(address,uint256)` and selector `0xde75ddea`
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
    #[ethcall(name = "accessUserAssets", abi = "accessUserAssets(address,uint256)")]
    pub struct AccessUserAssetsCall {
        pub token: ::ethers::core::types::Address,
        pub requested_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `authorizedKeepers` function with signature `authorizedKeepers(address)` and selector `0xc56f264d`
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
    #[ethcall(name = "authorizedKeepers", abi = "authorizedKeepers(address)")]
    pub struct AuthorizedKeepersCall {
        pub keeper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `authorizedLiquidators` function with signature `authorizedLiquidators(address)` and selector `0xdfa48f87`
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
    #[ethcall(name = "authorizedLiquidators", abi = "authorizedLiquidators(address)")]
    pub struct AuthorizedLiquidatorsCall {
        pub liquidator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `debtShareToVal` function with signature `debtShareToVal(uint256,uint112)` and selector `0xc923d913`
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
    #[ethcall(name = "debtShareToVal", abi = "debtShareToVal(uint256,uint112)")]
    pub struct DebtShareToValCall {
        pub pool_id: ::ethers::core::types::U256,
        pub debt_share: u128,
    }
    ///Container type for all input parameters for the `debtValToShare` function with signature `debtValToShare(uint256,uint112)` and selector `0xa0ffb7b8`
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
    #[ethcall(name = "debtValToShare", abi = "debtValToShare(uint256,uint112)")]
    pub struct DebtValToShareCall {
        pub pool_id: ::ethers::core::types::U256,
        pub debt_val: u128,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(uint256,uint256)` and selector `0xe2bbb158`
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
    #[ethcall(name = "deposit", abi = "deposit(uint256,uint256)")]
    pub struct DepositCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `permissionedLiquidation` function with signature `permissionedLiquidation()` and selector `0x4245d780`
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
    #[ethcall(name = "permissionedLiquidation", abi = "permissionedLiquidation()")]
    pub struct PermissionedLiquidationCall;
    ///Container type for all input parameters for the `pools` function with signature `pools()` and selector `0xc5c51dca`
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
    #[ethcall(name = "pools", abi = "pools()")]
    pub struct PoolsCall;
    ///Container type for all input parameters for the `positions` function with signature `positions(uint256)` and selector `0x99fbab88`
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
    #[ethcall(name = "positions", abi = "positions(uint256)")]
    pub struct PositionsCall {
        pub pos_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256,uint256)` and selector `0x441a3e70`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,uint256)")]
    pub struct WithdrawCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
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
    pub enum ILendingPoolCalls {
        AccessUserAssets(AccessUserAssetsCall),
        AuthorizedKeepers(AuthorizedKeepersCall),
        AuthorizedLiquidators(AuthorizedLiquidatorsCall),
        DebtShareToVal(DebtShareToValCall),
        DebtValToShare(DebtValToShareCall),
        Deposit(DepositCall),
        PermissionedLiquidation(PermissionedLiquidationCall),
        Pools(PoolsCall),
        Positions(PositionsCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for ILendingPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccessUserAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessUserAssets(decoded));
            }
            if let Ok(decoded) = <AuthorizedKeepersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AuthorizedKeepers(decoded));
            }
            if let Ok(decoded) = <AuthorizedLiquidatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AuthorizedLiquidators(decoded));
            }
            if let Ok(decoded) = <DebtShareToValCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DebtShareToVal(decoded));
            }
            if let Ok(decoded) = <DebtValToShareCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DebtValToShare(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <PermissionedLiquidationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PermissionedLiquidation(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded) = <PositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Positions(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ILendingPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccessUserAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AuthorizedKeepers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AuthorizedLiquidators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DebtShareToVal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DebtValToShare(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PermissionedLiquidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Positions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ILendingPoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessUserAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuthorizedKeepers(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuthorizedLiquidators(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DebtShareToVal(element) => ::core::fmt::Display::fmt(element, f),
                Self::DebtValToShare(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermissionedLiquidation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::Positions(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccessUserAssetsCall> for ILendingPoolCalls {
        fn from(value: AccessUserAssetsCall) -> Self {
            Self::AccessUserAssets(value)
        }
    }
    impl ::core::convert::From<AuthorizedKeepersCall> for ILendingPoolCalls {
        fn from(value: AuthorizedKeepersCall) -> Self {
            Self::AuthorizedKeepers(value)
        }
    }
    impl ::core::convert::From<AuthorizedLiquidatorsCall> for ILendingPoolCalls {
        fn from(value: AuthorizedLiquidatorsCall) -> Self {
            Self::AuthorizedLiquidators(value)
        }
    }
    impl ::core::convert::From<DebtShareToValCall> for ILendingPoolCalls {
        fn from(value: DebtShareToValCall) -> Self {
            Self::DebtShareToVal(value)
        }
    }
    impl ::core::convert::From<DebtValToShareCall> for ILendingPoolCalls {
        fn from(value: DebtValToShareCall) -> Self {
            Self::DebtValToShare(value)
        }
    }
    impl ::core::convert::From<DepositCall> for ILendingPoolCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<PermissionedLiquidationCall> for ILendingPoolCalls {
        fn from(value: PermissionedLiquidationCall) -> Self {
            Self::PermissionedLiquidation(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for ILendingPoolCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<PositionsCall> for ILendingPoolCalls {
        fn from(value: PositionsCall) -> Self {
            Self::Positions(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for ILendingPoolCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `authorizedKeepers` function with signature `authorizedKeepers(address)` and selector `0xc56f264d`
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
    pub struct AuthorizedKeepersReturn(pub bool);
    ///Container type for all return fields from the `authorizedLiquidators` function with signature `authorizedLiquidators(address)` and selector `0xdfa48f87`
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
    pub struct AuthorizedLiquidatorsReturn(pub bool);
    ///Container type for all return fields from the `debtShareToVal` function with signature `debtShareToVal(uint256,uint112)` and selector `0xc923d913`
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
    pub struct DebtShareToValReturn(pub u128);
    ///Container type for all return fields from the `debtValToShare` function with signature `debtValToShare(uint256,uint112)` and selector `0xa0ffb7b8`
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
    pub struct DebtValToShareReturn(pub u128);
    ///Container type for all return fields from the `permissionedLiquidation` function with signature `permissionedLiquidation()` and selector `0x4245d780`
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
    pub struct PermissionedLiquidationReturn(pub bool);
    ///Container type for all return fields from the `pools` function with signature `pools()` and selector `0xc5c51dca`
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
    pub struct PoolsReturn(pub ::std::vec::Vec<Market>);
    ///Container type for all return fields from the `positions` function with signature `positions(uint256)` and selector `0x99fbab88`
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
    pub struct PositionsReturn(pub Position);
}
