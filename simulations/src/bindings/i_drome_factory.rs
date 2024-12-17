pub use i_drome_factory::*;
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
pub mod i_drome_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allPoolsLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allPoolsLength"),
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
                    ::std::borrow::ToOwned::to_owned("createPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stable"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_stable"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("getPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stable"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("implementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPaused"),
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
                    ::std::borrow::ToOwned::to_owned("isPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
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
                    ::std::borrow::ToOwned::to_owned("setCustomFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setCustomFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fee"),
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
                    ::std::borrow::ToOwned::to_owned("setFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_stable"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fee"),
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
                    ::std::borrow::ToOwned::to_owned("setFeeManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFeeManager"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_feeManager"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("setPauseState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPauseState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("setPauser"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPauser"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pauser"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("setVoter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setVoter"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_voter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("voter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("voter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PoolCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PoolCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("stable"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("SetCustomFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetCustomFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
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
                    ::std::borrow::ToOwned::to_owned("SetFeeManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetFeeManager"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeManager"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetPauseState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetPauseState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetPauser"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetPauser"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pauser"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetVoter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetVoter"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("voter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FeeInvalid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FeeInvalid"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FeeTooHigh"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidPool"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotFeeManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotFeeManager"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotPauser"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotPauser"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotVoter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotVoter"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolAlreadyExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PoolAlreadyExists"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SameAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SameAddress"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroAddress"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroFee"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IDROMEFACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IDromeFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IDromeFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IDromeFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IDromeFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IDromeFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IDromeFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IDromeFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IDROMEFACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `allPoolsLength` (0xefde4e64) function
        pub fn all_pools_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([239, 222, 78, 100], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPool` (0x36bf95a0) function
        pub fn create_pool(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            stable: bool,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([54, 191, 149, 160], (token_a, token_b, stable))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPool` (0xa1671295) function
        pub fn create_pool_with_token_a_and_token_b_and_fee(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            fee: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([161, 103, 18, 149], (token_a, token_b, fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFee` (0xcc56b2c5) function
        pub fn get_fee(
            &self,
            pool: ::ethers::core::types::Address,
            stable: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([204, 86, 178, 197], (pool, stable))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0x1698ee82) function
        pub fn get_pool(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            fee: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 152, 238, 130], (token_a, token_b, fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0x79bc57d5) function
        pub fn get_pool_with_token_a_and_token_b_and_stable(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            stable: bool,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([121, 188, 87, 213], (token_a, token_b, stable))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `implementation` (0x5c60da1b) function
        pub fn implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPaused` (0xb187bd26) function
        pub fn is_paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 135, 189, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPool` (0x5b16ebb7) function
        pub fn is_pool(
            &self,
            pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([91, 22, 235, 183], pool)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCustomFee` (0xd49466a8) function
        pub fn set_custom_fee(
            &self,
            pool: ::ethers::core::types::Address,
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 148, 102, 168], (pool, fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFee` (0xe1f76b44) function
        pub fn set_fee(
            &self,
            stable: bool,
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 247, 107, 68], (stable, fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeManager` (0x472d35b9) function
        pub fn set_fee_manager(
            &self,
            fee_manager: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 45, 53, 185], fee_manager)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPauseState` (0xcdb88ad1) function
        pub fn set_pause_state(
            &self,
            state: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 184, 138, 209], state)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPauser` (0x2d88af4a) function
        pub fn set_pauser(
            &self,
            pauser: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 136, 175, 74], pauser)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setVoter` (0x4bc2a657) function
        pub fn set_voter(
            &self,
            voter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 194, 166, 87], voter)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voter` (0x46c96aac) function
        pub fn voter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 201, 106, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PoolCreated` event
        pub fn pool_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PoolCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetCustomFee` event
        pub fn set_custom_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetCustomFeeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetFeeManager` event
        pub fn set_fee_manager_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetFeeManagerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetPauseState` event
        pub fn set_pause_state_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetPauseStateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetPauser` event
        pub fn set_pauser_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetPauserFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetVoter` event
        pub fn set_voter_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetVoterFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IDromeFactoryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IDromeFactory<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `FeeInvalid` with signature `FeeInvalid()` and selector `0x52dadcf9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FeeInvalid", abi = "FeeInvalid()")]
    pub struct FeeInvalid;
    ///Custom Error type `FeeTooHigh` with signature `FeeTooHigh()` and selector `0xcd4e6167`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FeeTooHigh", abi = "FeeTooHigh()")]
    pub struct FeeTooHigh;
    ///Custom Error type `InvalidPool` with signature `InvalidPool()` and selector `0x2083cd40`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidPool", abi = "InvalidPool()")]
    pub struct InvalidPool;
    ///Custom Error type `NotFeeManager` with signature `NotFeeManager()` and selector `0xf5d267eb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotFeeManager", abi = "NotFeeManager()")]
    pub struct NotFeeManager;
    ///Custom Error type `NotPauser` with signature `NotPauser()` and selector `0x492f6781`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotPauser", abi = "NotPauser()")]
    pub struct NotPauser;
    ///Custom Error type `NotVoter` with signature `NotVoter()` and selector `0xc18384c1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotVoter", abi = "NotVoter()")]
    pub struct NotVoter;
    ///Custom Error type `PoolAlreadyExists` with signature `PoolAlreadyExists()` and selector `0x03119322`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PoolAlreadyExists", abi = "PoolAlreadyExists()")]
    pub struct PoolAlreadyExists;
    ///Custom Error type `SameAddress` with signature `SameAddress()` and selector `0x367558c3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SameAddress", abi = "SameAddress()")]
    pub struct SameAddress;
    ///Custom Error type `ZeroAddress` with signature `ZeroAddress()` and selector `0xd92e233d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ZeroAddress", abi = "ZeroAddress()")]
    pub struct ZeroAddress;
    ///Custom Error type `ZeroFee` with signature `ZeroFee()` and selector `0xaf13986d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ZeroFee", abi = "ZeroFee()")]
    pub struct ZeroFee;
    ///Container type for all of the contract's custom errors
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
    pub enum IDromeFactoryErrors {
        FeeInvalid(FeeInvalid),
        FeeTooHigh(FeeTooHigh),
        InvalidPool(InvalidPool),
        NotFeeManager(NotFeeManager),
        NotPauser(NotPauser),
        NotVoter(NotVoter),
        PoolAlreadyExists(PoolAlreadyExists),
        SameAddress(SameAddress),
        ZeroAddress(ZeroAddress),
        ZeroFee(ZeroFee),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IDromeFactoryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <FeeInvalid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeInvalid(decoded));
            }
            if let Ok(decoded) = <FeeTooHigh as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeTooHigh(decoded));
            }
            if let Ok(decoded) = <InvalidPool as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidPool(decoded));
            }
            if let Ok(decoded) = <NotFeeManager as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotFeeManager(decoded));
            }
            if let Ok(decoded) = <NotPauser as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotPauser(decoded));
            }
            if let Ok(decoded) = <NotVoter as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotVoter(decoded));
            }
            if let Ok(decoded) = <PoolAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolAlreadyExists(decoded));
            }
            if let Ok(decoded) = <SameAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SameAddress(decoded));
            }
            if let Ok(decoded) = <ZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroAddress(decoded));
            }
            if let Ok(decoded) = <ZeroFee as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroFee(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IDromeFactoryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::FeeInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotFeeManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotPauser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotVoter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SameAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IDromeFactoryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <FeeInvalid as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <FeeTooHigh as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidPool as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotFeeManager as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotPauser as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotVoter as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PoolAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SameAddress as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ZeroAddress as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ZeroFee as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IDromeFactoryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FeeInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotFeeManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotPauser(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotVoter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::SameAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IDromeFactoryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<FeeInvalid> for IDromeFactoryErrors {
        fn from(value: FeeInvalid) -> Self {
            Self::FeeInvalid(value)
        }
    }
    impl ::core::convert::From<FeeTooHigh> for IDromeFactoryErrors {
        fn from(value: FeeTooHigh) -> Self {
            Self::FeeTooHigh(value)
        }
    }
    impl ::core::convert::From<InvalidPool> for IDromeFactoryErrors {
        fn from(value: InvalidPool) -> Self {
            Self::InvalidPool(value)
        }
    }
    impl ::core::convert::From<NotFeeManager> for IDromeFactoryErrors {
        fn from(value: NotFeeManager) -> Self {
            Self::NotFeeManager(value)
        }
    }
    impl ::core::convert::From<NotPauser> for IDromeFactoryErrors {
        fn from(value: NotPauser) -> Self {
            Self::NotPauser(value)
        }
    }
    impl ::core::convert::From<NotVoter> for IDromeFactoryErrors {
        fn from(value: NotVoter) -> Self {
            Self::NotVoter(value)
        }
    }
    impl ::core::convert::From<PoolAlreadyExists> for IDromeFactoryErrors {
        fn from(value: PoolAlreadyExists) -> Self {
            Self::PoolAlreadyExists(value)
        }
    }
    impl ::core::convert::From<SameAddress> for IDromeFactoryErrors {
        fn from(value: SameAddress) -> Self {
            Self::SameAddress(value)
        }
    }
    impl ::core::convert::From<ZeroAddress> for IDromeFactoryErrors {
        fn from(value: ZeroAddress) -> Self {
            Self::ZeroAddress(value)
        }
    }
    impl ::core::convert::From<ZeroFee> for IDromeFactoryErrors {
        fn from(value: ZeroFee) -> Self {
            Self::ZeroFee(value)
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
        name = "PoolCreated",
        abi = "PoolCreated(address,address,bool,address,uint256)"
    )]
    pub struct PoolCreatedFilter {
        #[ethevent(indexed)]
        pub token_0: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub stable: bool,
        pub pool: ::ethers::core::types::Address,
        pub p4: ::ethers::core::types::U256,
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
    #[ethevent(name = "SetCustomFee", abi = "SetCustomFee(address,uint256)")]
    pub struct SetCustomFeeFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        pub fee: ::ethers::core::types::U256,
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
    #[ethevent(name = "SetFeeManager", abi = "SetFeeManager(address)")]
    pub struct SetFeeManagerFilter {
        pub fee_manager: ::ethers::core::types::Address,
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
    #[ethevent(name = "SetPauseState", abi = "SetPauseState(bool)")]
    pub struct SetPauseStateFilter {
        pub state: bool,
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
    #[ethevent(name = "SetPauser", abi = "SetPauser(address)")]
    pub struct SetPauserFilter {
        pub pauser: ::ethers::core::types::Address,
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
    #[ethevent(name = "SetVoter", abi = "SetVoter(address)")]
    pub struct SetVoterFilter {
        pub voter: ::ethers::core::types::Address,
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
    pub enum IDromeFactoryEvents {
        PoolCreatedFilter(PoolCreatedFilter),
        SetCustomFeeFilter(SetCustomFeeFilter),
        SetFeeManagerFilter(SetFeeManagerFilter),
        SetPauseStateFilter(SetPauseStateFilter),
        SetPauserFilter(SetPauserFilter),
        SetVoterFilter(SetVoterFilter),
    }
    impl ::ethers::contract::EthLogDecode for IDromeFactoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PoolCreatedFilter::decode_log(log) {
                return Ok(IDromeFactoryEvents::PoolCreatedFilter(decoded));
            }
            if let Ok(decoded) = SetCustomFeeFilter::decode_log(log) {
                return Ok(IDromeFactoryEvents::SetCustomFeeFilter(decoded));
            }
            if let Ok(decoded) = SetFeeManagerFilter::decode_log(log) {
                return Ok(IDromeFactoryEvents::SetFeeManagerFilter(decoded));
            }
            if let Ok(decoded) = SetPauseStateFilter::decode_log(log) {
                return Ok(IDromeFactoryEvents::SetPauseStateFilter(decoded));
            }
            if let Ok(decoded) = SetPauserFilter::decode_log(log) {
                return Ok(IDromeFactoryEvents::SetPauserFilter(decoded));
            }
            if let Ok(decoded) = SetVoterFilter::decode_log(log) {
                return Ok(IDromeFactoryEvents::SetVoterFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IDromeFactoryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PoolCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCustomFeeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFeeManagerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPauseStateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPauserFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetVoterFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PoolCreatedFilter> for IDromeFactoryEvents {
        fn from(value: PoolCreatedFilter) -> Self {
            Self::PoolCreatedFilter(value)
        }
    }
    impl ::core::convert::From<SetCustomFeeFilter> for IDromeFactoryEvents {
        fn from(value: SetCustomFeeFilter) -> Self {
            Self::SetCustomFeeFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeManagerFilter> for IDromeFactoryEvents {
        fn from(value: SetFeeManagerFilter) -> Self {
            Self::SetFeeManagerFilter(value)
        }
    }
    impl ::core::convert::From<SetPauseStateFilter> for IDromeFactoryEvents {
        fn from(value: SetPauseStateFilter) -> Self {
            Self::SetPauseStateFilter(value)
        }
    }
    impl ::core::convert::From<SetPauserFilter> for IDromeFactoryEvents {
        fn from(value: SetPauserFilter) -> Self {
            Self::SetPauserFilter(value)
        }
    }
    impl ::core::convert::From<SetVoterFilter> for IDromeFactoryEvents {
        fn from(value: SetVoterFilter) -> Self {
            Self::SetVoterFilter(value)
        }
    }
    ///Container type for all input parameters for the `allPoolsLength` function with signature `allPoolsLength()` and selector `0xefde4e64`
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
    #[ethcall(name = "allPoolsLength", abi = "allPoolsLength()")]
    pub struct AllPoolsLengthCall;
    ///Container type for all input parameters for the `createPool` function with signature `createPool(address,address,bool)` and selector `0x36bf95a0`
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
    #[ethcall(name = "createPool", abi = "createPool(address,address,bool)")]
    pub struct CreatePoolCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub stable: bool,
    }
    ///Container type for all input parameters for the `createPool` function with signature `createPool(address,address,uint24)` and selector `0xa1671295`
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
    #[ethcall(name = "createPool", abi = "createPool(address,address,uint24)")]
    pub struct CreatePoolWithTokenAAndTokenBAndFeeCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub fee: u32,
    }
    ///Container type for all input parameters for the `getFee` function with signature `getFee(address,bool)` and selector `0xcc56b2c5`
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
    #[ethcall(name = "getFee", abi = "getFee(address,bool)")]
    pub struct GetFeeCall {
        pub pool: ::ethers::core::types::Address,
        pub stable: bool,
    }
    ///Container type for all input parameters for the `getPool` function with signature `getPool(address,address,uint24)` and selector `0x1698ee82`
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
    #[ethcall(name = "getPool", abi = "getPool(address,address,uint24)")]
    pub struct GetPoolCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub fee: u32,
    }
    ///Container type for all input parameters for the `getPool` function with signature `getPool(address,address,bool)` and selector `0x79bc57d5`
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
    #[ethcall(name = "getPool", abi = "getPool(address,address,bool)")]
    pub struct GetPoolWithTokenAAndTokenBAndStableCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub stable: bool,
    }
    ///Container type for all input parameters for the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    #[ethcall(name = "implementation", abi = "implementation()")]
    pub struct ImplementationCall;
    ///Container type for all input parameters for the `isPaused` function with signature `isPaused()` and selector `0xb187bd26`
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
    #[ethcall(name = "isPaused", abi = "isPaused()")]
    pub struct IsPausedCall;
    ///Container type for all input parameters for the `isPool` function with signature `isPool(address)` and selector `0x5b16ebb7`
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
    #[ethcall(name = "isPool", abi = "isPool(address)")]
    pub struct IsPoolCall {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setCustomFee` function with signature `setCustomFee(address,uint256)` and selector `0xd49466a8`
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
    #[ethcall(name = "setCustomFee", abi = "setCustomFee(address,uint256)")]
    pub struct SetCustomFeeCall {
        pub pool: ::ethers::core::types::Address,
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setFee` function with signature `setFee(bool,uint256)` and selector `0xe1f76b44`
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
    #[ethcall(name = "setFee", abi = "setFee(bool,uint256)")]
    pub struct SetFeeCall {
        pub stable: bool,
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setFeeManager` function with signature `setFeeManager(address)` and selector `0x472d35b9`
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
    #[ethcall(name = "setFeeManager", abi = "setFeeManager(address)")]
    pub struct SetFeeManagerCall {
        pub fee_manager: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPauseState` function with signature `setPauseState(bool)` and selector `0xcdb88ad1`
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
    #[ethcall(name = "setPauseState", abi = "setPauseState(bool)")]
    pub struct SetPauseStateCall {
        pub state: bool,
    }
    ///Container type for all input parameters for the `setPauser` function with signature `setPauser(address)` and selector `0x2d88af4a`
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
    #[ethcall(name = "setPauser", abi = "setPauser(address)")]
    pub struct SetPauserCall {
        pub pauser: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setVoter` function with signature `setVoter(address)` and selector `0x4bc2a657`
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
    #[ethcall(name = "setVoter", abi = "setVoter(address)")]
    pub struct SetVoterCall {
        pub voter: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `voter` function with signature `voter()` and selector `0x46c96aac`
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
    #[ethcall(name = "voter", abi = "voter()")]
    pub struct VoterCall;
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
    pub enum IDromeFactoryCalls {
        AllPoolsLength(AllPoolsLengthCall),
        CreatePool(CreatePoolCall),
        CreatePoolWithTokenAAndTokenBAndFee(CreatePoolWithTokenAAndTokenBAndFeeCall),
        GetFee(GetFeeCall),
        GetPool(GetPoolCall),
        GetPoolWithTokenAAndTokenBAndStable(GetPoolWithTokenAAndTokenBAndStableCall),
        Implementation(ImplementationCall),
        IsPaused(IsPausedCall),
        IsPool(IsPoolCall),
        SetCustomFee(SetCustomFeeCall),
        SetFee(SetFeeCall),
        SetFeeManager(SetFeeManagerCall),
        SetPauseState(SetPauseStateCall),
        SetPauser(SetPauserCall),
        SetVoter(SetVoterCall),
        Voter(VoterCall),
    }
    impl ::ethers::core::abi::AbiDecode for IDromeFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllPoolsLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllPoolsLength(decoded));
            }
            if let Ok(decoded) = <CreatePoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreatePool(decoded));
            }
            if let Ok(decoded) = <CreatePoolWithTokenAAndTokenBAndFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreatePoolWithTokenAAndTokenBAndFee(decoded));
            }
            if let Ok(decoded) = <GetFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFee(decoded));
            }
            if let Ok(decoded) = <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPool(decoded));
            }
            if let Ok(decoded) = <GetPoolWithTokenAAndTokenBAndStableCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolWithTokenAAndTokenBAndStable(decoded));
            }
            if let Ok(decoded) = <ImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Implementation(decoded));
            }
            if let Ok(decoded) = <IsPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsPaused(decoded));
            }
            if let Ok(decoded) = <IsPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsPool(decoded));
            }
            if let Ok(decoded) = <SetCustomFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetCustomFee(decoded));
            }
            if let Ok(decoded) = <SetFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFee(decoded));
            }
            if let Ok(decoded) = <SetFeeManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFeeManager(decoded));
            }
            if let Ok(decoded) = <SetPauseStateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPauseState(decoded));
            }
            if let Ok(decoded) = <SetPauserCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPauser(decoded));
            }
            if let Ok(decoded) = <SetVoterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetVoter(decoded));
            }
            if let Ok(decoded) = <VoterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Voter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IDromeFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AllPoolsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePoolWithTokenAAndTokenBAndFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolWithTokenAAndTokenBAndStable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Implementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetCustomFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeeManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauseState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetVoter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IDromeFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllPoolsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePoolWithTokenAAndTokenBAndFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolWithTokenAAndTokenBAndStable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Implementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCustomFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauseState(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauser(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetVoter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Voter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllPoolsLengthCall> for IDromeFactoryCalls {
        fn from(value: AllPoolsLengthCall) -> Self {
            Self::AllPoolsLength(value)
        }
    }
    impl ::core::convert::From<CreatePoolCall> for IDromeFactoryCalls {
        fn from(value: CreatePoolCall) -> Self {
            Self::CreatePool(value)
        }
    }
    impl ::core::convert::From<CreatePoolWithTokenAAndTokenBAndFeeCall>
    for IDromeFactoryCalls {
        fn from(value: CreatePoolWithTokenAAndTokenBAndFeeCall) -> Self {
            Self::CreatePoolWithTokenAAndTokenBAndFee(value)
        }
    }
    impl ::core::convert::From<GetFeeCall> for IDromeFactoryCalls {
        fn from(value: GetFeeCall) -> Self {
            Self::GetFee(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for IDromeFactoryCalls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetPoolWithTokenAAndTokenBAndStableCall>
    for IDromeFactoryCalls {
        fn from(value: GetPoolWithTokenAAndTokenBAndStableCall) -> Self {
            Self::GetPoolWithTokenAAndTokenBAndStable(value)
        }
    }
    impl ::core::convert::From<ImplementationCall> for IDromeFactoryCalls {
        fn from(value: ImplementationCall) -> Self {
            Self::Implementation(value)
        }
    }
    impl ::core::convert::From<IsPausedCall> for IDromeFactoryCalls {
        fn from(value: IsPausedCall) -> Self {
            Self::IsPaused(value)
        }
    }
    impl ::core::convert::From<IsPoolCall> for IDromeFactoryCalls {
        fn from(value: IsPoolCall) -> Self {
            Self::IsPool(value)
        }
    }
    impl ::core::convert::From<SetCustomFeeCall> for IDromeFactoryCalls {
        fn from(value: SetCustomFeeCall) -> Self {
            Self::SetCustomFee(value)
        }
    }
    impl ::core::convert::From<SetFeeCall> for IDromeFactoryCalls {
        fn from(value: SetFeeCall) -> Self {
            Self::SetFee(value)
        }
    }
    impl ::core::convert::From<SetFeeManagerCall> for IDromeFactoryCalls {
        fn from(value: SetFeeManagerCall) -> Self {
            Self::SetFeeManager(value)
        }
    }
    impl ::core::convert::From<SetPauseStateCall> for IDromeFactoryCalls {
        fn from(value: SetPauseStateCall) -> Self {
            Self::SetPauseState(value)
        }
    }
    impl ::core::convert::From<SetPauserCall> for IDromeFactoryCalls {
        fn from(value: SetPauserCall) -> Self {
            Self::SetPauser(value)
        }
    }
    impl ::core::convert::From<SetVoterCall> for IDromeFactoryCalls {
        fn from(value: SetVoterCall) -> Self {
            Self::SetVoter(value)
        }
    }
    impl ::core::convert::From<VoterCall> for IDromeFactoryCalls {
        fn from(value: VoterCall) -> Self {
            Self::Voter(value)
        }
    }
    ///Container type for all return fields from the `allPoolsLength` function with signature `allPoolsLength()` and selector `0xefde4e64`
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
    pub struct AllPoolsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `createPool` function with signature `createPool(address,address,bool)` and selector `0x36bf95a0`
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
    pub struct CreatePoolReturn {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createPool` function with signature `createPool(address,address,uint24)` and selector `0xa1671295`
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
    pub struct CreatePoolWithTokenAAndTokenBAndFeeReturn {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getFee` function with signature `getFee(address,bool)` and selector `0xcc56b2c5`
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
    pub struct GetFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPool` function with signature `getPool(address,address,uint24)` and selector `0x1698ee82`
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
    pub struct GetPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPool` function with signature `getPool(address,address,bool)` and selector `0x79bc57d5`
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
    pub struct GetPoolWithTokenAAndTokenBAndStableReturn(
        pub ::ethers::core::types::Address,
    );
    ///Container type for all return fields from the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    pub struct ImplementationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isPaused` function with signature `isPaused()` and selector `0xb187bd26`
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
    pub struct IsPausedReturn(pub bool);
    ///Container type for all return fields from the `isPool` function with signature `isPool(address)` and selector `0x5b16ebb7`
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
    pub struct IsPoolReturn(pub bool);
    ///Container type for all return fields from the `voter` function with signature `voter()` and selector `0x46c96aac`
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
    pub struct VoterReturn(pub ::ethers::core::types::Address);
}
