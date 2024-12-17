pub use mock_pool_factory::*;
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
pub mod mock_pool_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_implementation"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MAX_FEE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_FEE"),
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
                    ::std::borrow::ToOwned::to_owned("ZERO_FEE_INDICATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ZERO_FEE_INDICATOR"),
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
                    ::std::borrow::ToOwned::to_owned("allPools"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allPools"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("customFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("customFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("feeManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeManager"),
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
                    ::std::borrow::ToOwned::to_owned("getFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
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
                    ::std::borrow::ToOwned::to_owned("pauser"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauser"),
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
                    ::std::borrow::ToOwned::to_owned("setCustomFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setCustomFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
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
                    ::std::borrow::ToOwned::to_owned("stableFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stableFee"),
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
                    ::std::borrow::ToOwned::to_owned("volatileFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("volatileFee"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FailedDeployment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedDeployment"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("InsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("needed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
    pub static MOCKPOOLFACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\ra8\x03\x80a\ra\x839\x81\x01`@\x81\x90Ra\0.\x91a\0\x8AV[`\x01`\x01`\xA0\x1B\x03\x16`\x80R`\x04\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x81\x17\x90\x92U_\x80T`\x03\x80T\x90\x93\x16\x84\x17\x90\x92U`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x16a\x01\0\x90\x92\x02`\xFF\x19\x16\x91\x90\x91\x17\x90U`\x05`\x01U`\x1E`\x02Ua\0\xB7V[_` \x82\x84\x03\x12\x15a\0\x9AW__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xB0W__\xFD[\x93\x92PPPV[`\x80Qa\x0C\x8Ba\0\xD6_9_\x81\x81a\x02e\x01Ra\x05\xD8\x01Ra\x0C\x8B_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01HW_5`\xE0\x1C\x80c\\`\xDA\x1B\x11a\0\xBFW\x80c\xCCV\xB2\xC5\x11a\0yW\x80c\xCCV\xB2\xC5\x14a\x03\tW\x80c\xCD\xB8\x8A\xD1\x14a\x03\x1CW\x80c\xD0\xFB\x02\x03\x14a\x03/W\x80c\xD4\x94f\xA8\x14a\x03BW\x80c\xE1\xF7kD\x14a\x03UW\x80c\xEF\xDENd\x14a\x03hW__\xFD[\x80c\\`\xDA\x1B\x14a\x02`W\x80cy\xBCW\xD5\x14a\x02\x87W\x80c\x9F\xD0Pm\x14a\x02\xCAW\x80c\xA1g\x12\x95\x14a\x02\xE1W\x80c\xB1\x87\xBD&\x14a\x02\xF4W\x80c\xBC\x06>\x1A\x14a\x03\0W__\xFD[\x80cA\xD1\xDE\x97\x11a\x01\x10W\x80cA\xD1\xDE\x97\x14a\x01\xC4W\x80cF\xC9j\xAC\x14a\x01\xD7W\x80cG-5\xB9\x14a\x01\xEAW\x80cMA\x9A\xBC\x14a\x01\xFDW\x80cP\x84\xED\x03\x14a\x02\x1CW\x80c[\x16\xEB\xB7\x14a\x02%W__\xFD[\x80c\x16\x98\xEE\x82\x14a\x01LW\x80c-\x88\xAFJ\x14a\x01|W\x80c6\xBF\x95\xA0\x14a\x01\x91W\x80c8\xC5]F\x14a\x01\xA4W\x80c@\xBB\xD7u\x14a\x01\xBBW[__\xFD[a\x01_a\x01Z6`\x04a\x0B(V[a\x03pV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Fa\x01\x8A6`\x04a\x0BsV[a\x04\0V[\0[a\x01_a\x01\x9F6`\x04a\x0B\x9BV[a\x04\xB2V[a\x01\xADa\x01\xA4\x81V[`@Q\x90\x81R` \x01a\x01sV[a\x01\xAD`\x01T\x81V[a\x01_a\x01\xD26`\x04a\x0B\xDBV[a\x07gV[`\x04Ta\x01_\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x8Fa\x01\xF86`\x04a\x0BsV[a\x07\x8FV[a\x01\xADa\x02\x0B6`\x04a\x0BsV[`\x08` R_\x90\x81R`@\x90 T\x81V[a\x01\xAD`\x02T\x81V[a\x02Pa\x0236`\x04a\x0BsV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x07` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01sV[a\x01_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01_a\x02\x956`\x04a\x0B\x9BV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x86\x16\x83R\x93\x81R\x83\x82 \x92\x15\x15\x82R\x91\x90\x91R T\x16\x90V[_Ta\x01_\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01_a\x02\xEF6`\x04a\x0B(V[a\x08/V[_Ta\x02P\x90`\xFF\x16\x81V[a\x01\xADa\x01,\x81V[a\x01\xADa\x03\x176`\x04a\x0B\xF2V[a\x08tV[a\x01\x8Fa\x03*6`\x04a\x0C#V[a\x08\xC1V[`\x03Ta\x01_\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x8Fa\x03P6`\x04a\x0C<V[a\t0V[a\x01\x8Fa\x03c6`\x04a\x0CdV[a\n\x1BV[`\x06Ta\x01\xADV[_`\x01\x82b\xFF\xFF\xFF\x16\x11a\x03\xF4W\x81b\xFF\xFF\xFF\x16`\x01\x14a\x03\xBFW`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x87\x85\x16\x84R\x82R\x80\x83 \x83\x80R\x90\x91R\x90 T\x16a\x03\xF6V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x87\x85\x16\x84R\x82R\x80\x83 `\x01\x84R\x90\x91R\x90 T\x16a\x03\xF6V[_[\x90P[\x93\x92PPPV[_Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04/W`@QcI/g\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04VW`@Qc\xD9.#=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@Q\x90\x81R\x7F\xE0.\xFB\x9E\x8F\x0F\xC2\x15Fs\n\xB3-YOb\xD5\x86\xE1\xBB\xB1[\xB5\x04^\xDD\x0B\x18x\xA7{5\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[_\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03a\x04\xE5W`@Qc6uX\xC3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10a\x05\x07W\x84\x86a\x05\nV[\x85\x85[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\x056W`@Qc\xD9.#=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x85\x85\x16\x84R\x82R\x80\x83 \x88\x15\x15\x84R\x90\x91R\x90 T\x16\x15a\x05\x85W`@Qc\x01\x88\xC9\x91`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x81\x1B\x82\x16` \x84\x01R\x83\x90\x1B\x16`4\x82\x01R\x84\x15\x15`\xF8\x1B`H\x82\x01R_\x90`I\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x05\xFD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\n\x9BV[`@Qc\x1C\x97v\xB5`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R\x87\x15\x15`D\x83\x01R\x91\x95P\x90\x85\x16\x90c\xE4\xBB\xB5\xA8\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06QW__\xFD[PZ\xF1\x15\x80\x15a\x06cW=__>=_\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x05` \x81\x81R`@\x80\x84 \x88\x87\x16\x80\x86R\x90\x83R\x81\x85 \x8C\x15\x15\x80\x87R\x90\x84R\x82\x86 \x80T\x98\x8D\x16`\x01`\x01`\xA0\x1B\x03\x19\x99\x8A\x16\x81\x17\x90\x91U\x82\x87R\x94\x84R\x82\x86 \x87\x87R\x84R\x82\x86 \x81\x87R\x84R\x82\x86 \x80T\x89\x16\x86\x17\x90U`\x06\x80T`\x01\x81\x81\x01\x83U\x7F\xF6R\"#\x13\xE2\x84YR\x8D\x92\x0Be\x11\\\x16\xC0O>\xFC\x82\xAA\xED\xC9{\xE5\x9F?7|\r?\x90\x91\x01\x80T\x90\x9A\x16\x87\x17\x90\x99U\x85\x87R`\x07\x85R\x95\x83\x90 \x80T`\xFF\x19\x16\x90\x98\x17\x90\x97U\x93T\x81Q\x93\x84R\x91\x83\x01\x91\x90\x91R\x91\x92\x91\x7F!(\xD8\x8D\x14\xC8\x0C\xB0\x81\xC1%*Z\xCF\xF7\xA2dg\x1B\xF1\x99\xCE\"kSx\x8F\xB2`e\0^\x91\x01`@Q\x80\x91\x03\x90\xA4PPP\x93\x92PPPV[`\x06\x81\x81T\x81\x10a\x07vW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xBAW`@Qc\xF5\xD2g\xEB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\xE1W`@Qc\xD9.#=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F]\x05\x17\xE3\xA4\xEA\xBE\xA8\x92\xD9u\x018\xCD!\xD4\xA6\xCF;\x93[C\xD0Y\x8D\xF7\x05_F8\x19\xB2\x90` \x01a\x04\xA7V[_`\x01\x82b\xFF\xFF\xFF\x16\x11\x15a\x08WW`@QcR\xDA\xDC\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01b\xFF\xFF\xFF\x83\x16\x14a\x08k\x85\x85\x83a\x04\xB2V[\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x08` R`@\x81 Ta\x01\xA4\x81\x14a\x08\xB7W\x80_\x03a\x08\xB1W\x82a\x08\xA9W`\x02Ta\x08\xB9V[`\x01Ta\x08\xB9V[\x80a\x08\xB9V[_[\x94\x93PPPPV[_Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xF0W`@QcI/g\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16\x82\x15\x15\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\rvS\x8E\xFC@\x83\x18\xA0Q\x13|' \xA9\xE8)\x02\xAC\xDB\xD4k\x80-H\x8Bt\xCA:\t\xA1\x16\x90` \x01a\x04\xA7V[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t[W`@Qc\xF5\xD2g\xEB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01,\x81\x11\x80\x15a\tnWPa\x01\xA4\x81\x14\x15[\x15a\t\x8CW`@Qc\xCDNag`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x07` R`@\x90 T`\xFF\x16a\t\xC3W`@Qb\x82\x0F5`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\x08` R`@\x90\x81\x90 \x83\x90UQ\x7F\xAEF\x8C\xE5\x86\xF9\xA8v`\xFD\xFF\xC1D\x8C\xEE\x94 B\xC1j\xE2\xF0 F\xB14\xB5\"O1\x93k\x90a\n\x0F\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\nFW`@Qc\xF5\xD2g\xEB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01,\x81\x11\x15a\niW`@Qc\xCDNag`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80_\x03a\n\x89W`@Qc\xAF\x13\x98m`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15a\n\x95W`\x01UPV[`\x02UPV[_a\x03\xF9\x83\x83\x83\x80v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x84``\x1B`\xE8\x1C\x17_RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x84`x\x1B\x17` R\x82`7`\t\x84\xF5\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xF9W`@Qc\xB0n\xBF=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B#W__\xFD[\x91\x90PV[___``\x84\x86\x03\x12\x15a\x0B:W__\xFD[a\x0BC\x84a\x0B\rV[\x92Pa\x0BQ` \x85\x01a\x0B\rV[\x91P`@\x84\x015b\xFF\xFF\xFF\x81\x16\x81\x14a\x0BhW__\xFD[\x80\x91PP\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a\x0B\x83W__\xFD[a\x03\xF9\x82a\x0B\rV[\x805\x80\x15\x15\x81\x14a\x0B#W__\xFD[___``\x84\x86\x03\x12\x15a\x0B\xADW__\xFD[a\x0B\xB6\x84a\x0B\rV[\x92Pa\x0B\xC4` \x85\x01a\x0B\rV[\x91Pa\x0B\xD2`@\x85\x01a\x0B\x8CV[\x90P\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a\x0B\xEBW__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x0C\x03W__\xFD[a\x0C\x0C\x83a\x0B\rV[\x91Pa\x0C\x1A` \x84\x01a\x0B\x8CV[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x0C3W__\xFD[a\x03\xF9\x82a\x0B\x8CV[__`@\x83\x85\x03\x12\x15a\x0CMW__\xFD[a\x0CV\x83a\x0B\rV[\x94` \x93\x90\x93\x015\x93PPPV[__`@\x83\x85\x03\x12\x15a\x0CuW__\xFD[a\x0CV\x83a\x0B\x8CV\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static MOCKPOOLFACTORY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01HW_5`\xE0\x1C\x80c\\`\xDA\x1B\x11a\0\xBFW\x80c\xCCV\xB2\xC5\x11a\0yW\x80c\xCCV\xB2\xC5\x14a\x03\tW\x80c\xCD\xB8\x8A\xD1\x14a\x03\x1CW\x80c\xD0\xFB\x02\x03\x14a\x03/W\x80c\xD4\x94f\xA8\x14a\x03BW\x80c\xE1\xF7kD\x14a\x03UW\x80c\xEF\xDENd\x14a\x03hW__\xFD[\x80c\\`\xDA\x1B\x14a\x02`W\x80cy\xBCW\xD5\x14a\x02\x87W\x80c\x9F\xD0Pm\x14a\x02\xCAW\x80c\xA1g\x12\x95\x14a\x02\xE1W\x80c\xB1\x87\xBD&\x14a\x02\xF4W\x80c\xBC\x06>\x1A\x14a\x03\0W__\xFD[\x80cA\xD1\xDE\x97\x11a\x01\x10W\x80cA\xD1\xDE\x97\x14a\x01\xC4W\x80cF\xC9j\xAC\x14a\x01\xD7W\x80cG-5\xB9\x14a\x01\xEAW\x80cMA\x9A\xBC\x14a\x01\xFDW\x80cP\x84\xED\x03\x14a\x02\x1CW\x80c[\x16\xEB\xB7\x14a\x02%W__\xFD[\x80c\x16\x98\xEE\x82\x14a\x01LW\x80c-\x88\xAFJ\x14a\x01|W\x80c6\xBF\x95\xA0\x14a\x01\x91W\x80c8\xC5]F\x14a\x01\xA4W\x80c@\xBB\xD7u\x14a\x01\xBBW[__\xFD[a\x01_a\x01Z6`\x04a\x0B(V[a\x03pV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Fa\x01\x8A6`\x04a\x0BsV[a\x04\0V[\0[a\x01_a\x01\x9F6`\x04a\x0B\x9BV[a\x04\xB2V[a\x01\xADa\x01\xA4\x81V[`@Q\x90\x81R` \x01a\x01sV[a\x01\xAD`\x01T\x81V[a\x01_a\x01\xD26`\x04a\x0B\xDBV[a\x07gV[`\x04Ta\x01_\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x8Fa\x01\xF86`\x04a\x0BsV[a\x07\x8FV[a\x01\xADa\x02\x0B6`\x04a\x0BsV[`\x08` R_\x90\x81R`@\x90 T\x81V[a\x01\xAD`\x02T\x81V[a\x02Pa\x0236`\x04a\x0BsV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x07` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01sV[a\x01_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01_a\x02\x956`\x04a\x0B\x9BV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x86\x16\x83R\x93\x81R\x83\x82 \x92\x15\x15\x82R\x91\x90\x91R T\x16\x90V[_Ta\x01_\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01_a\x02\xEF6`\x04a\x0B(V[a\x08/V[_Ta\x02P\x90`\xFF\x16\x81V[a\x01\xADa\x01,\x81V[a\x01\xADa\x03\x176`\x04a\x0B\xF2V[a\x08tV[a\x01\x8Fa\x03*6`\x04a\x0C#V[a\x08\xC1V[`\x03Ta\x01_\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x8Fa\x03P6`\x04a\x0C<V[a\t0V[a\x01\x8Fa\x03c6`\x04a\x0CdV[a\n\x1BV[`\x06Ta\x01\xADV[_`\x01\x82b\xFF\xFF\xFF\x16\x11a\x03\xF4W\x81b\xFF\xFF\xFF\x16`\x01\x14a\x03\xBFW`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x87\x85\x16\x84R\x82R\x80\x83 \x83\x80R\x90\x91R\x90 T\x16a\x03\xF6V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x87\x85\x16\x84R\x82R\x80\x83 `\x01\x84R\x90\x91R\x90 T\x16a\x03\xF6V[_[\x90P[\x93\x92PPPV[_Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04/W`@QcI/g\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04VW`@Qc\xD9.#=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@Q\x90\x81R\x7F\xE0.\xFB\x9E\x8F\x0F\xC2\x15Fs\n\xB3-YOb\xD5\x86\xE1\xBB\xB1[\xB5\x04^\xDD\x0B\x18x\xA7{5\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[_\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03a\x04\xE5W`@Qc6uX\xC3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10a\x05\x07W\x84\x86a\x05\nV[\x85\x85[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\x056W`@Qc\xD9.#=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x85\x85\x16\x84R\x82R\x80\x83 \x88\x15\x15\x84R\x90\x91R\x90 T\x16\x15a\x05\x85W`@Qc\x01\x88\xC9\x91`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x81\x1B\x82\x16` \x84\x01R\x83\x90\x1B\x16`4\x82\x01R\x84\x15\x15`\xF8\x1B`H\x82\x01R_\x90`I\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x05\xFD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\n\x9BV[`@Qc\x1C\x97v\xB5`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R\x87\x15\x15`D\x83\x01R\x91\x95P\x90\x85\x16\x90c\xE4\xBB\xB5\xA8\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06QW__\xFD[PZ\xF1\x15\x80\x15a\x06cW=__>=_\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x05` \x81\x81R`@\x80\x84 \x88\x87\x16\x80\x86R\x90\x83R\x81\x85 \x8C\x15\x15\x80\x87R\x90\x84R\x82\x86 \x80T\x98\x8D\x16`\x01`\x01`\xA0\x1B\x03\x19\x99\x8A\x16\x81\x17\x90\x91U\x82\x87R\x94\x84R\x82\x86 \x87\x87R\x84R\x82\x86 \x81\x87R\x84R\x82\x86 \x80T\x89\x16\x86\x17\x90U`\x06\x80T`\x01\x81\x81\x01\x83U\x7F\xF6R\"#\x13\xE2\x84YR\x8D\x92\x0Be\x11\\\x16\xC0O>\xFC\x82\xAA\xED\xC9{\xE5\x9F?7|\r?\x90\x91\x01\x80T\x90\x9A\x16\x87\x17\x90\x99U\x85\x87R`\x07\x85R\x95\x83\x90 \x80T`\xFF\x19\x16\x90\x98\x17\x90\x97U\x93T\x81Q\x93\x84R\x91\x83\x01\x91\x90\x91R\x91\x92\x91\x7F!(\xD8\x8D\x14\xC8\x0C\xB0\x81\xC1%*Z\xCF\xF7\xA2dg\x1B\xF1\x99\xCE\"kSx\x8F\xB2`e\0^\x91\x01`@Q\x80\x91\x03\x90\xA4PPP\x93\x92PPPV[`\x06\x81\x81T\x81\x10a\x07vW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xBAW`@Qc\xF5\xD2g\xEB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\xE1W`@Qc\xD9.#=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F]\x05\x17\xE3\xA4\xEA\xBE\xA8\x92\xD9u\x018\xCD!\xD4\xA6\xCF;\x93[C\xD0Y\x8D\xF7\x05_F8\x19\xB2\x90` \x01a\x04\xA7V[_`\x01\x82b\xFF\xFF\xFF\x16\x11\x15a\x08WW`@QcR\xDA\xDC\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01b\xFF\xFF\xFF\x83\x16\x14a\x08k\x85\x85\x83a\x04\xB2V[\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x08` R`@\x81 Ta\x01\xA4\x81\x14a\x08\xB7W\x80_\x03a\x08\xB1W\x82a\x08\xA9W`\x02Ta\x08\xB9V[`\x01Ta\x08\xB9V[\x80a\x08\xB9V[_[\x94\x93PPPPV[_Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xF0W`@QcI/g\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16\x82\x15\x15\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\rvS\x8E\xFC@\x83\x18\xA0Q\x13|' \xA9\xE8)\x02\xAC\xDB\xD4k\x80-H\x8Bt\xCA:\t\xA1\x16\x90` \x01a\x04\xA7V[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t[W`@Qc\xF5\xD2g\xEB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01,\x81\x11\x80\x15a\tnWPa\x01\xA4\x81\x14\x15[\x15a\t\x8CW`@Qc\xCDNag`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x07` R`@\x90 T`\xFF\x16a\t\xC3W`@Qb\x82\x0F5`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\x08` R`@\x90\x81\x90 \x83\x90UQ\x7F\xAEF\x8C\xE5\x86\xF9\xA8v`\xFD\xFF\xC1D\x8C\xEE\x94 B\xC1j\xE2\xF0 F\xB14\xB5\"O1\x93k\x90a\n\x0F\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\nFW`@Qc\xF5\xD2g\xEB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01,\x81\x11\x15a\niW`@Qc\xCDNag`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80_\x03a\n\x89W`@Qc\xAF\x13\x98m`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15a\n\x95W`\x01UPV[`\x02UPV[_a\x03\xF9\x83\x83\x83\x80v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x84``\x1B`\xE8\x1C\x17_RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x84`x\x1B\x17` R\x82`7`\t\x84\xF5\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xF9W`@Qc\xB0n\xBF=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B#W__\xFD[\x91\x90PV[___``\x84\x86\x03\x12\x15a\x0B:W__\xFD[a\x0BC\x84a\x0B\rV[\x92Pa\x0BQ` \x85\x01a\x0B\rV[\x91P`@\x84\x015b\xFF\xFF\xFF\x81\x16\x81\x14a\x0BhW__\xFD[\x80\x91PP\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a\x0B\x83W__\xFD[a\x03\xF9\x82a\x0B\rV[\x805\x80\x15\x15\x81\x14a\x0B#W__\xFD[___``\x84\x86\x03\x12\x15a\x0B\xADW__\xFD[a\x0B\xB6\x84a\x0B\rV[\x92Pa\x0B\xC4` \x85\x01a\x0B\rV[\x91Pa\x0B\xD2`@\x85\x01a\x0B\x8CV[\x90P\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a\x0B\xEBW__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x0C\x03W__\xFD[a\x0C\x0C\x83a\x0B\rV[\x91Pa\x0C\x1A` \x84\x01a\x0B\x8CV[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x0C3W__\xFD[a\x03\xF9\x82a\x0B\x8CV[__`@\x83\x85\x03\x12\x15a\x0CMW__\xFD[a\x0CV\x83a\x0B\rV[\x94` \x93\x90\x93\x015\x93PPPV[__`@\x83\x85\x03\x12\x15a\x0CuW__\xFD[a\x0CV\x83a\x0B\x8CV\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static MOCKPOOLFACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockPoolFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockPoolFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockPoolFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockPoolFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockPoolFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockPoolFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockPoolFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKPOOLFACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MOCKPOOLFACTORY_ABI.clone(),
                MOCKPOOLFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MAX_FEE` (0xbc063e1a) function
        pub fn max_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([188, 6, 62, 26], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ZERO_FEE_INDICATOR` (0x38c55d46) function
        pub fn zero_fee_indicator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([56, 197, 93, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allPools` (0x41d1de97) function
        pub fn all_pools(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([65, 209, 222, 151], p0)
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `customFee` (0x4d419abc) function
        pub fn custom_fee(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 65, 154, 188], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeManager` (0xd0fb0203) function
        pub fn fee_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([208, 251, 2, 3], ())
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
        ///Calls the contract's `pauser` (0x9fd0506d) function
        pub fn pauser(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([159, 208, 80, 109], ())
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
        ///Calls the contract's `stableFee` (0x40bbd775) function
        pub fn stable_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([64, 187, 215, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `volatileFee` (0x5084ed03) function
        pub fn volatile_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([80, 132, 237, 3], ())
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MockPoolFactoryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockPoolFactory<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `FailedDeployment` with signature `FailedDeployment()` and selector `0xb06ebf3d`
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
    #[etherror(name = "FailedDeployment", abi = "FailedDeployment()")]
    pub struct FailedDeployment;
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
    ///Custom Error type `InsufficientBalance` with signature `InsufficientBalance(uint256,uint256)` and selector `0xcf479181`
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
    #[etherror(
        name = "InsufficientBalance",
        abi = "InsufficientBalance(uint256,uint256)"
    )]
    pub struct InsufficientBalance {
        pub balance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
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
    pub enum MockPoolFactoryErrors {
        FailedDeployment(FailedDeployment),
        FeeInvalid(FeeInvalid),
        FeeTooHigh(FeeTooHigh),
        InsufficientBalance(InsufficientBalance),
        InvalidPool(InvalidPool),
        NotFeeManager(NotFeeManager),
        NotPauser(NotPauser),
        PoolAlreadyExists(PoolAlreadyExists),
        SameAddress(SameAddress),
        ZeroAddress(ZeroAddress),
        ZeroFee(ZeroFee),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockPoolFactoryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <FailedDeployment as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedDeployment(decoded));
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
            if let Ok(decoded) = <InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientBalance(decoded));
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
    impl ::ethers::core::abi::AbiEncode for MockPoolFactoryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::FailedDeployment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalance(element) => {
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
    impl ::ethers::contract::ContractRevert for MockPoolFactoryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <FailedDeployment as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FeeInvalid as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <FeeTooHigh as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPool as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotFeeManager as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotPauser as ::ethers::contract::EthError>::selector() => true,
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
    impl ::core::fmt::Display for MockPoolFactoryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FailedDeployment(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotFeeManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotPauser(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::SameAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockPoolFactoryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<FailedDeployment> for MockPoolFactoryErrors {
        fn from(value: FailedDeployment) -> Self {
            Self::FailedDeployment(value)
        }
    }
    impl ::core::convert::From<FeeInvalid> for MockPoolFactoryErrors {
        fn from(value: FeeInvalid) -> Self {
            Self::FeeInvalid(value)
        }
    }
    impl ::core::convert::From<FeeTooHigh> for MockPoolFactoryErrors {
        fn from(value: FeeTooHigh) -> Self {
            Self::FeeTooHigh(value)
        }
    }
    impl ::core::convert::From<InsufficientBalance> for MockPoolFactoryErrors {
        fn from(value: InsufficientBalance) -> Self {
            Self::InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<InvalidPool> for MockPoolFactoryErrors {
        fn from(value: InvalidPool) -> Self {
            Self::InvalidPool(value)
        }
    }
    impl ::core::convert::From<NotFeeManager> for MockPoolFactoryErrors {
        fn from(value: NotFeeManager) -> Self {
            Self::NotFeeManager(value)
        }
    }
    impl ::core::convert::From<NotPauser> for MockPoolFactoryErrors {
        fn from(value: NotPauser) -> Self {
            Self::NotPauser(value)
        }
    }
    impl ::core::convert::From<PoolAlreadyExists> for MockPoolFactoryErrors {
        fn from(value: PoolAlreadyExists) -> Self {
            Self::PoolAlreadyExists(value)
        }
    }
    impl ::core::convert::From<SameAddress> for MockPoolFactoryErrors {
        fn from(value: SameAddress) -> Self {
            Self::SameAddress(value)
        }
    }
    impl ::core::convert::From<ZeroAddress> for MockPoolFactoryErrors {
        fn from(value: ZeroAddress) -> Self {
            Self::ZeroAddress(value)
        }
    }
    impl ::core::convert::From<ZeroFee> for MockPoolFactoryErrors {
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
    pub enum MockPoolFactoryEvents {
        PoolCreatedFilter(PoolCreatedFilter),
        SetCustomFeeFilter(SetCustomFeeFilter),
        SetFeeManagerFilter(SetFeeManagerFilter),
        SetPauseStateFilter(SetPauseStateFilter),
        SetPauserFilter(SetPauserFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockPoolFactoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PoolCreatedFilter::decode_log(log) {
                return Ok(MockPoolFactoryEvents::PoolCreatedFilter(decoded));
            }
            if let Ok(decoded) = SetCustomFeeFilter::decode_log(log) {
                return Ok(MockPoolFactoryEvents::SetCustomFeeFilter(decoded));
            }
            if let Ok(decoded) = SetFeeManagerFilter::decode_log(log) {
                return Ok(MockPoolFactoryEvents::SetFeeManagerFilter(decoded));
            }
            if let Ok(decoded) = SetPauseStateFilter::decode_log(log) {
                return Ok(MockPoolFactoryEvents::SetPauseStateFilter(decoded));
            }
            if let Ok(decoded) = SetPauserFilter::decode_log(log) {
                return Ok(MockPoolFactoryEvents::SetPauserFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockPoolFactoryEvents {
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
            }
        }
    }
    impl ::core::convert::From<PoolCreatedFilter> for MockPoolFactoryEvents {
        fn from(value: PoolCreatedFilter) -> Self {
            Self::PoolCreatedFilter(value)
        }
    }
    impl ::core::convert::From<SetCustomFeeFilter> for MockPoolFactoryEvents {
        fn from(value: SetCustomFeeFilter) -> Self {
            Self::SetCustomFeeFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeManagerFilter> for MockPoolFactoryEvents {
        fn from(value: SetFeeManagerFilter) -> Self {
            Self::SetFeeManagerFilter(value)
        }
    }
    impl ::core::convert::From<SetPauseStateFilter> for MockPoolFactoryEvents {
        fn from(value: SetPauseStateFilter) -> Self {
            Self::SetPauseStateFilter(value)
        }
    }
    impl ::core::convert::From<SetPauserFilter> for MockPoolFactoryEvents {
        fn from(value: SetPauserFilter) -> Self {
            Self::SetPauserFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_FEE` function with signature `MAX_FEE()` and selector `0xbc063e1a`
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
    #[ethcall(name = "MAX_FEE", abi = "MAX_FEE()")]
    pub struct MaxFeeCall;
    ///Container type for all input parameters for the `ZERO_FEE_INDICATOR` function with signature `ZERO_FEE_INDICATOR()` and selector `0x38c55d46`
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
    #[ethcall(name = "ZERO_FEE_INDICATOR", abi = "ZERO_FEE_INDICATOR()")]
    pub struct ZeroFeeIndicatorCall;
    ///Container type for all input parameters for the `allPools` function with signature `allPools(uint256)` and selector `0x41d1de97`
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
    #[ethcall(name = "allPools", abi = "allPools(uint256)")]
    pub struct AllPoolsCall(pub ::ethers::core::types::U256);
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
    ///Container type for all input parameters for the `customFee` function with signature `customFee(address)` and selector `0x4d419abc`
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
    #[ethcall(name = "customFee", abi = "customFee(address)")]
    pub struct CustomFeeCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `feeManager` function with signature `feeManager()` and selector `0xd0fb0203`
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
    #[ethcall(name = "feeManager", abi = "feeManager()")]
    pub struct FeeManagerCall;
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
    ///Container type for all input parameters for the `pauser` function with signature `pauser()` and selector `0x9fd0506d`
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
    #[ethcall(name = "pauser", abi = "pauser()")]
    pub struct PauserCall;
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
    ///Container type for all input parameters for the `stableFee` function with signature `stableFee()` and selector `0x40bbd775`
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
    #[ethcall(name = "stableFee", abi = "stableFee()")]
    pub struct StableFeeCall;
    ///Container type for all input parameters for the `volatileFee` function with signature `volatileFee()` and selector `0x5084ed03`
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
    #[ethcall(name = "volatileFee", abi = "volatileFee()")]
    pub struct VolatileFeeCall;
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
    pub enum MockPoolFactoryCalls {
        MaxFee(MaxFeeCall),
        ZeroFeeIndicator(ZeroFeeIndicatorCall),
        AllPools(AllPoolsCall),
        AllPoolsLength(AllPoolsLengthCall),
        CreatePool(CreatePoolCall),
        CreatePoolWithTokenAAndTokenBAndFee(CreatePoolWithTokenAAndTokenBAndFeeCall),
        CustomFee(CustomFeeCall),
        FeeManager(FeeManagerCall),
        GetFee(GetFeeCall),
        GetPool(GetPoolCall),
        GetPoolWithTokenAAndTokenBAndStable(GetPoolWithTokenAAndTokenBAndStableCall),
        Implementation(ImplementationCall),
        IsPaused(IsPausedCall),
        IsPool(IsPoolCall),
        Pauser(PauserCall),
        SetCustomFee(SetCustomFeeCall),
        SetFee(SetFeeCall),
        SetFeeManager(SetFeeManagerCall),
        SetPauseState(SetPauseStateCall),
        SetPauser(SetPauserCall),
        StableFee(StableFeeCall),
        VolatileFee(VolatileFeeCall),
        Voter(VoterCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockPoolFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MaxFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxFee(decoded));
            }
            if let Ok(decoded) = <ZeroFeeIndicatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroFeeIndicator(decoded));
            }
            if let Ok(decoded) = <AllPoolsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllPools(decoded));
            }
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
            if let Ok(decoded) = <CustomFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CustomFee(decoded));
            }
            if let Ok(decoded) = <FeeManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeManager(decoded));
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
            if let Ok(decoded) = <PauserCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pauser(decoded));
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
            if let Ok(decoded) = <StableFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StableFee(decoded));
            }
            if let Ok(decoded) = <VolatileFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VolatileFee(decoded));
            }
            if let Ok(decoded) = <VoterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Voter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockPoolFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroFeeIndicator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllPools(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllPoolsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePoolWithTokenAAndTokenBAndFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CustomFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeManager(element) => {
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
                Self::Pauser(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::StableFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VolatileFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockPoolFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroFeeIndicator(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllPools(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllPoolsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePoolWithTokenAAndTokenBAndFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CustomFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolWithTokenAAndTokenBAndStable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Implementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pauser(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCustomFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauseState(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauser(element) => ::core::fmt::Display::fmt(element, f),
                Self::StableFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::VolatileFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Voter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxFeeCall> for MockPoolFactoryCalls {
        fn from(value: MaxFeeCall) -> Self {
            Self::MaxFee(value)
        }
    }
    impl ::core::convert::From<ZeroFeeIndicatorCall> for MockPoolFactoryCalls {
        fn from(value: ZeroFeeIndicatorCall) -> Self {
            Self::ZeroFeeIndicator(value)
        }
    }
    impl ::core::convert::From<AllPoolsCall> for MockPoolFactoryCalls {
        fn from(value: AllPoolsCall) -> Self {
            Self::AllPools(value)
        }
    }
    impl ::core::convert::From<AllPoolsLengthCall> for MockPoolFactoryCalls {
        fn from(value: AllPoolsLengthCall) -> Self {
            Self::AllPoolsLength(value)
        }
    }
    impl ::core::convert::From<CreatePoolCall> for MockPoolFactoryCalls {
        fn from(value: CreatePoolCall) -> Self {
            Self::CreatePool(value)
        }
    }
    impl ::core::convert::From<CreatePoolWithTokenAAndTokenBAndFeeCall>
    for MockPoolFactoryCalls {
        fn from(value: CreatePoolWithTokenAAndTokenBAndFeeCall) -> Self {
            Self::CreatePoolWithTokenAAndTokenBAndFee(value)
        }
    }
    impl ::core::convert::From<CustomFeeCall> for MockPoolFactoryCalls {
        fn from(value: CustomFeeCall) -> Self {
            Self::CustomFee(value)
        }
    }
    impl ::core::convert::From<FeeManagerCall> for MockPoolFactoryCalls {
        fn from(value: FeeManagerCall) -> Self {
            Self::FeeManager(value)
        }
    }
    impl ::core::convert::From<GetFeeCall> for MockPoolFactoryCalls {
        fn from(value: GetFeeCall) -> Self {
            Self::GetFee(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for MockPoolFactoryCalls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetPoolWithTokenAAndTokenBAndStableCall>
    for MockPoolFactoryCalls {
        fn from(value: GetPoolWithTokenAAndTokenBAndStableCall) -> Self {
            Self::GetPoolWithTokenAAndTokenBAndStable(value)
        }
    }
    impl ::core::convert::From<ImplementationCall> for MockPoolFactoryCalls {
        fn from(value: ImplementationCall) -> Self {
            Self::Implementation(value)
        }
    }
    impl ::core::convert::From<IsPausedCall> for MockPoolFactoryCalls {
        fn from(value: IsPausedCall) -> Self {
            Self::IsPaused(value)
        }
    }
    impl ::core::convert::From<IsPoolCall> for MockPoolFactoryCalls {
        fn from(value: IsPoolCall) -> Self {
            Self::IsPool(value)
        }
    }
    impl ::core::convert::From<PauserCall> for MockPoolFactoryCalls {
        fn from(value: PauserCall) -> Self {
            Self::Pauser(value)
        }
    }
    impl ::core::convert::From<SetCustomFeeCall> for MockPoolFactoryCalls {
        fn from(value: SetCustomFeeCall) -> Self {
            Self::SetCustomFee(value)
        }
    }
    impl ::core::convert::From<SetFeeCall> for MockPoolFactoryCalls {
        fn from(value: SetFeeCall) -> Self {
            Self::SetFee(value)
        }
    }
    impl ::core::convert::From<SetFeeManagerCall> for MockPoolFactoryCalls {
        fn from(value: SetFeeManagerCall) -> Self {
            Self::SetFeeManager(value)
        }
    }
    impl ::core::convert::From<SetPauseStateCall> for MockPoolFactoryCalls {
        fn from(value: SetPauseStateCall) -> Self {
            Self::SetPauseState(value)
        }
    }
    impl ::core::convert::From<SetPauserCall> for MockPoolFactoryCalls {
        fn from(value: SetPauserCall) -> Self {
            Self::SetPauser(value)
        }
    }
    impl ::core::convert::From<StableFeeCall> for MockPoolFactoryCalls {
        fn from(value: StableFeeCall) -> Self {
            Self::StableFee(value)
        }
    }
    impl ::core::convert::From<VolatileFeeCall> for MockPoolFactoryCalls {
        fn from(value: VolatileFeeCall) -> Self {
            Self::VolatileFee(value)
        }
    }
    impl ::core::convert::From<VoterCall> for MockPoolFactoryCalls {
        fn from(value: VoterCall) -> Self {
            Self::Voter(value)
        }
    }
    ///Container type for all return fields from the `MAX_FEE` function with signature `MAX_FEE()` and selector `0xbc063e1a`
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
    pub struct MaxFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ZERO_FEE_INDICATOR` function with signature `ZERO_FEE_INDICATOR()` and selector `0x38c55d46`
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
    pub struct ZeroFeeIndicatorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `allPools` function with signature `allPools(uint256)` and selector `0x41d1de97`
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
    pub struct AllPoolsReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `customFee` function with signature `customFee(address)` and selector `0x4d419abc`
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
    pub struct CustomFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `feeManager` function with signature `feeManager()` and selector `0xd0fb0203`
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
    pub struct FeeManagerReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `pauser` function with signature `pauser()` and selector `0x9fd0506d`
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
    pub struct PauserReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `stableFee` function with signature `stableFee()` and selector `0x40bbd775`
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
    pub struct StableFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `volatileFee` function with signature `volatileFee()` and selector `0x5084ed03`
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
    pub struct VolatileFeeReturn(pub ::ethers::core::types::U256);
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
