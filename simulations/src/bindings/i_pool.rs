pub use i_pool::*;
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
pub mod i_pool {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("blockTimestampLast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("blockTimestampLast"),
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
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
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
                    ::std::borrow::ToOwned::to_owned("claimFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimFees"),
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
                    ::std::borrow::ToOwned::to_owned("claimable0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimable0"),
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
                    ::std::borrow::ToOwned::to_owned("claimable1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimable1"),
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
                    ::std::borrow::ToOwned::to_owned("currentCumulativePrices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "currentCumulativePrices",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "reserve0Cumulative",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "reserve1Cumulative",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockTimestamp"),
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
                    ::std::borrow::ToOwned::to_owned("factory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("factory"),
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
                    ::std::borrow::ToOwned::to_owned("getAmountOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAmountOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
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
                    ::std::borrow::ToOwned::to_owned("getK"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getK"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReserves"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_reserve0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_reserve1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_blockTimestampLast",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("index0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("index0"),
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
                    ::std::borrow::ToOwned::to_owned("index1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("index1"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token1"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastObservation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastObservation"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromePool.Observation",
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
                    ::std::borrow::ToOwned::to_owned("metadata"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("metadata"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dec0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dec1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("st"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t1"),
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
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
                    ::std::borrow::ToOwned::to_owned("observationLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("observationLength"),
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
                    ::std::borrow::ToOwned::to_owned("periodSize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("periodSize"),
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
                    ::std::borrow::ToOwned::to_owned("poolFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("poolFees"),
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
                    ::std::borrow::ToOwned::to_owned("prices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prices"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("points"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("granularity"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
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
                    ::std::borrow::ToOwned::to_owned("reserve0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reserve0"),
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
                    ::std::borrow::ToOwned::to_owned("reserve0CumulativeLast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "reserve0CumulativeLast",
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
                    ::std::borrow::ToOwned::to_owned("reserve1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reserve1"),
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
                    ::std::borrow::ToOwned::to_owned("reserve1CumulativeLast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "reserve1CumulativeLast",
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
                    ::std::borrow::ToOwned::to_owned("sample"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sample"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("points"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("window"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setName"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("setSymbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSymbol"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("__symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("skim"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("skim"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("stable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stable"),
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
                    ::std::borrow::ToOwned::to_owned("supplyIndex0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supplyIndex0"),
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
                    ::std::borrow::ToOwned::to_owned("supplyIndex1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supplyIndex1"),
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
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                (
                    ::std::borrow::ToOwned::to_owned("sync"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sync"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("token0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token0"),
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
                    ::std::borrow::ToOwned::to_owned("token1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token1"),
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
                    ::std::borrow::ToOwned::to_owned("tokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokens"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("Burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
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
                    ::std::borrow::ToOwned::to_owned("Claim"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Claim"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
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
                    ::std::borrow::ToOwned::to_owned("Fees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Fees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
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
                    ::std::borrow::ToOwned::to_owned("Mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
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
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0In"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1In"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Out"),
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
                    ::std::borrow::ToOwned::to_owned("Sync"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Sync"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve1"),
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
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BelowMinimumK"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("BelowMinimumK"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositsNotEqual"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DepositsNotEqual"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FactoryAlreadySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FactoryAlreadySet"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientInputAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientInputAmount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientLiquidity",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientLiquidityBurned"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientLiquidityBurned",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientLiquidityMinted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientLiquidityMinted",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientOutputAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientOutputAmount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidTo"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IsPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("IsPaused"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("K"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("K"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotEmergencyCouncil"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotEmergencyCouncil",
                            ),
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
    pub static IPOOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IPool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IPool)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPOOL_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blockTimestampLast` (0xc5700a02) function
        pub fn block_timestamp_last(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([197, 112, 10, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x89afcb44) function
        pub fn burn(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([137, 175, 203, 68], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimFees` (0xd294f093) function
        pub fn claim_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([210, 148, 240, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimable0` (0x4d5a9f8a) function
        pub fn claimable_0(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 90, 159, 138], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimable1` (0xa1ac4d13) function
        pub fn claimable_1(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([161, 172, 77, 19], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentCumulativePrices` (0x1df8c717) function
        pub fn current_cumulative_prices(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([29, 248, 199, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountOut` (0xf140a35a) function
        pub fn get_amount_out(
            &self,
            amount_in: ::ethers::core::types::U256,
            token_in: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([241, 64, 163, 90], (amount_in, token_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getK` (0xee39e7a0) function
        pub fn get_k(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([238, 57, 231, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserves` (0x0902f1ac) function
        pub fn get_reserves(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([9, 2, 241, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `index0` (0x32c0defd) function
        pub fn index_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([50, 192, 222, 253], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `index1` (0xbda39cad) function
        pub fn index_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 163, 156, 173], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xe4bbb5a8) function
        pub fn initialize(
            &self,
            token_0: ::ethers::core::types::Address,
            token_1: ::ethers::core::types::Address,
            stable: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 187, 181, 168], (token_0, token_1, stable))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastObservation` (0x8a7b8cf2) function
        pub fn last_observation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, Observation> {
            self.0
                .method_hash([138, 123, 140, 242], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `metadata` (0x392f37e9) function
        pub fn metadata(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([57, 47, 55, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x6a627842) function
        pub fn mint(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([106, 98, 120, 66], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `observationLength` (0xebeb31db) function
        pub fn observation_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([235, 235, 49, 219], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `periodSize` (0xe4463eb2) function
        pub fn period_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([228, 70, 62, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolFees` (0x33580959) function
        pub fn pool_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([51, 88, 9, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prices` (0x5881c475) function
        pub fn prices(
            &self,
            token_in: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            points: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([88, 129, 196, 117], (token_in, amount_in, points))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quote` (0x9e8cc04b) function
        pub fn quote(
            &self,
            token_in: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            granularity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([158, 140, 192, 75], (token_in, amount_in, granularity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reserve0` (0x443cb4bc) function
        pub fn reserve_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([68, 60, 180, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reserve0CumulativeLast` (0xbf944dbc) function
        pub fn reserve_0_cumulative_last(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([191, 148, 77, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reserve1` (0x5a76f25e) function
        pub fn reserve_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([90, 118, 242, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reserve1CumulativeLast` (0xc245febc) function
        pub fn reserve_1_cumulative_last(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([194, 69, 254, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sample` (0x13345fe1) function
        pub fn sample(
            &self,
            token_in: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            points: ::ethers::core::types::U256,
            window: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([19, 52, 95, 225], (token_in, amount_in, points, window))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setName` (0xc47f0027) function
        pub fn set_name(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 127, 0, 39], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSymbol` (0xb84c8246) function
        pub fn set_symbol(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 76, 130, 70], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `skim` (0xbc25cf77) function
        pub fn skim(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 37, 207, 119], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stable` (0x22be3de1) function
        pub fn stable(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([34, 190, 61, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supplyIndex0` (0x9f767c88) function
        pub fn supply_index_0(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([159, 118, 124, 136], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supplyIndex1` (0x205aabf1) function
        pub fn supply_index_1(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([32, 90, 171, 241], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x022c0d9f) function
        pub fn swap(
            &self,
            amount_0_out: ::ethers::core::types::U256,
            amount_1_out: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 44, 13, 159], (amount_0_out, amount_1_out, to, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sync` (0xfff6cae9) function
        pub fn sync(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 246, 202, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token0` (0x0dfe1681) function
        pub fn token_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token1` (0xd21220a7) function
        pub fn token_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([210, 18, 32, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokens` (0x9d63848a) function
        pub fn tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::Address),
        > {
            self.0
                .method_hash([157, 99, 132, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Burn` event
        pub fn burn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BurnFilter> {
            self.0.event()
        }
        ///Gets the contract's `Claim` event
        pub fn claim_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ClaimFilter> {
            self.0.event()
        }
        ///Gets the contract's `Fees` event
        pub fn fees_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeesFilter> {
            self.0.event()
        }
        ///Gets the contract's `Mint` event
        pub fn mint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MintFilter> {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        ///Gets the contract's `Sync` event
        pub fn sync_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SyncFilter> {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IPoolEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IPool<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BelowMinimumK` with signature `BelowMinimumK()` and selector `0x438d3ade`
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
    #[etherror(name = "BelowMinimumK", abi = "BelowMinimumK()")]
    pub struct BelowMinimumK;
    ///Custom Error type `DepositsNotEqual` with signature `DepositsNotEqual()` and selector `0x0a04d7fa`
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
    #[etherror(name = "DepositsNotEqual", abi = "DepositsNotEqual()")]
    pub struct DepositsNotEqual;
    ///Custom Error type `FactoryAlreadySet` with signature `FactoryAlreadySet()` and selector `0x154c51b8`
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
    #[etherror(name = "FactoryAlreadySet", abi = "FactoryAlreadySet()")]
    pub struct FactoryAlreadySet;
    ///Custom Error type `InsufficientInputAmount` with signature `InsufficientInputAmount()` and selector `0x098fb561`
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
    #[etherror(name = "InsufficientInputAmount", abi = "InsufficientInputAmount()")]
    pub struct InsufficientInputAmount;
    ///Custom Error type `InsufficientLiquidity` with signature `InsufficientLiquidity()` and selector `0xbb55fd27`
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
    #[etherror(name = "InsufficientLiquidity", abi = "InsufficientLiquidity()")]
    pub struct InsufficientLiquidity;
    ///Custom Error type `InsufficientLiquidityBurned` with signature `InsufficientLiquidityBurned()` and selector `0x749383ad`
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
        name = "InsufficientLiquidityBurned",
        abi = "InsufficientLiquidityBurned()"
    )]
    pub struct InsufficientLiquidityBurned;
    ///Custom Error type `InsufficientLiquidityMinted` with signature `InsufficientLiquidityMinted()` and selector `0xd226f9d4`
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
        name = "InsufficientLiquidityMinted",
        abi = "InsufficientLiquidityMinted()"
    )]
    pub struct InsufficientLiquidityMinted;
    ///Custom Error type `InsufficientOutputAmount` with signature `InsufficientOutputAmount()` and selector `0x42301c23`
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
    #[etherror(name = "InsufficientOutputAmount", abi = "InsufficientOutputAmount()")]
    pub struct InsufficientOutputAmount;
    ///Custom Error type `InvalidTo` with signature `InvalidTo()` and selector `0x290fa188`
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
    #[etherror(name = "InvalidTo", abi = "InvalidTo()")]
    pub struct InvalidTo;
    ///Custom Error type `IsPaused` with signature `IsPaused()` and selector `0x1309a563`
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
    #[etherror(name = "IsPaused", abi = "IsPaused()")]
    pub struct IsPaused;
    ///Custom Error type `K` with signature `K()` and selector `0xa932492f`
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
    #[etherror(name = "K", abi = "K()")]
    pub struct K;
    ///Custom Error type `NotEmergencyCouncil` with signature `NotEmergencyCouncil()` and selector `0xc5601293`
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
    #[etherror(name = "NotEmergencyCouncil", abi = "NotEmergencyCouncil()")]
    pub struct NotEmergencyCouncil;
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
    pub enum IPoolErrors {
        BelowMinimumK(BelowMinimumK),
        DepositsNotEqual(DepositsNotEqual),
        FactoryAlreadySet(FactoryAlreadySet),
        InsufficientInputAmount(InsufficientInputAmount),
        InsufficientLiquidity(InsufficientLiquidity),
        InsufficientLiquidityBurned(InsufficientLiquidityBurned),
        InsufficientLiquidityMinted(InsufficientLiquidityMinted),
        InsufficientOutputAmount(InsufficientOutputAmount),
        InvalidTo(InvalidTo),
        IsPaused(IsPaused),
        K(K),
        NotEmergencyCouncil(NotEmergencyCouncil),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IPoolErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <BelowMinimumK as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BelowMinimumK(decoded));
            }
            if let Ok(decoded) = <DepositsNotEqual as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositsNotEqual(decoded));
            }
            if let Ok(decoded) = <FactoryAlreadySet as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FactoryAlreadySet(decoded));
            }
            if let Ok(decoded) = <InsufficientInputAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientInputAmount(decoded));
            }
            if let Ok(decoded) = <InsufficientLiquidity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientLiquidity(decoded));
            }
            if let Ok(decoded) = <InsufficientLiquidityBurned as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientLiquidityBurned(decoded));
            }
            if let Ok(decoded) = <InsufficientLiquidityMinted as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientLiquidityMinted(decoded));
            }
            if let Ok(decoded) = <InsufficientOutputAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientOutputAmount(decoded));
            }
            if let Ok(decoded) = <InvalidTo as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidTo(decoded));
            }
            if let Ok(decoded) = <IsPaused as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsPaused(decoded));
            }
            if let Ok(decoded) = <K as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::K(decoded));
            }
            if let Ok(decoded) = <NotEmergencyCouncil as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotEmergencyCouncil(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPoolErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BelowMinimumK(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositsNotEqual(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FactoryAlreadySet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientInputAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientLiquidityBurned(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientLiquidityMinted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientOutputAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::K(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotEmergencyCouncil(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IPoolErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BelowMinimumK as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DepositsNotEqual as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FactoryAlreadySet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientInputAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientLiquidity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientLiquidityBurned as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientLiquidityMinted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientOutputAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTo as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <IsPaused as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <K as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotEmergencyCouncil as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IPoolErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BelowMinimumK(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositsNotEqual(element) => ::core::fmt::Display::fmt(element, f),
                Self::FactoryAlreadySet(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientInputAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientLiquidityBurned(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientLiquidityMinted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientOutputAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::K(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotEmergencyCouncil(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IPoolErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BelowMinimumK> for IPoolErrors {
        fn from(value: BelowMinimumK) -> Self {
            Self::BelowMinimumK(value)
        }
    }
    impl ::core::convert::From<DepositsNotEqual> for IPoolErrors {
        fn from(value: DepositsNotEqual) -> Self {
            Self::DepositsNotEqual(value)
        }
    }
    impl ::core::convert::From<FactoryAlreadySet> for IPoolErrors {
        fn from(value: FactoryAlreadySet) -> Self {
            Self::FactoryAlreadySet(value)
        }
    }
    impl ::core::convert::From<InsufficientInputAmount> for IPoolErrors {
        fn from(value: InsufficientInputAmount) -> Self {
            Self::InsufficientInputAmount(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidity> for IPoolErrors {
        fn from(value: InsufficientLiquidity) -> Self {
            Self::InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidityBurned> for IPoolErrors {
        fn from(value: InsufficientLiquidityBurned) -> Self {
            Self::InsufficientLiquidityBurned(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidityMinted> for IPoolErrors {
        fn from(value: InsufficientLiquidityMinted) -> Self {
            Self::InsufficientLiquidityMinted(value)
        }
    }
    impl ::core::convert::From<InsufficientOutputAmount> for IPoolErrors {
        fn from(value: InsufficientOutputAmount) -> Self {
            Self::InsufficientOutputAmount(value)
        }
    }
    impl ::core::convert::From<InvalidTo> for IPoolErrors {
        fn from(value: InvalidTo) -> Self {
            Self::InvalidTo(value)
        }
    }
    impl ::core::convert::From<IsPaused> for IPoolErrors {
        fn from(value: IsPaused) -> Self {
            Self::IsPaused(value)
        }
    }
    impl ::core::convert::From<K> for IPoolErrors {
        fn from(value: K) -> Self {
            Self::K(value)
        }
    }
    impl ::core::convert::From<NotEmergencyCouncil> for IPoolErrors {
        fn from(value: NotEmergencyCouncil) -> Self {
            Self::NotEmergencyCouncil(value)
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "Burn", abi = "Burn(address,address,uint256,uint256)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
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
    #[ethevent(name = "Claim", abi = "Claim(address,address,uint256,uint256)")]
    pub struct ClaimFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
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
    #[ethevent(name = "Fees", abi = "Fees(address,uint256,uint256)")]
    pub struct FeesFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
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
    #[ethevent(name = "Mint", abi = "Mint(address,uint256,uint256)")]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
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
        name = "Swap",
        abi = "Swap(address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount_0_in: ::ethers::core::types::U256,
        pub amount_1_in: ::ethers::core::types::U256,
        pub amount_0_out: ::ethers::core::types::U256,
        pub amount_1_out: ::ethers::core::types::U256,
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
    #[ethevent(name = "Sync", abi = "Sync(uint256,uint256)")]
    pub struct SyncFilter {
        pub reserve_0: ::ethers::core::types::U256,
        pub reserve_1: ::ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    pub enum IPoolEvents {
        ApprovalFilter(ApprovalFilter),
        BurnFilter(BurnFilter),
        ClaimFilter(ClaimFilter),
        FeesFilter(FeesFilter),
        MintFilter(MintFilter),
        SwapFilter(SwapFilter),
        SyncFilter(SyncFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for IPoolEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(IPoolEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(IPoolEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = ClaimFilter::decode_log(log) {
                return Ok(IPoolEvents::ClaimFilter(decoded));
            }
            if let Ok(decoded) = FeesFilter::decode_log(log) {
                return Ok(IPoolEvents::FeesFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(IPoolEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(IPoolEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = SyncFilter::decode_log(log) {
                return Ok(IPoolEvents::SyncFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(IPoolEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IPoolEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SyncFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for IPoolEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<BurnFilter> for IPoolEvents {
        fn from(value: BurnFilter) -> Self {
            Self::BurnFilter(value)
        }
    }
    impl ::core::convert::From<ClaimFilter> for IPoolEvents {
        fn from(value: ClaimFilter) -> Self {
            Self::ClaimFilter(value)
        }
    }
    impl ::core::convert::From<FeesFilter> for IPoolEvents {
        fn from(value: FeesFilter) -> Self {
            Self::FeesFilter(value)
        }
    }
    impl ::core::convert::From<MintFilter> for IPoolEvents {
        fn from(value: MintFilter) -> Self {
            Self::MintFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for IPoolEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    impl ::core::convert::From<SyncFilter> for IPoolEvents {
        fn from(value: SyncFilter) -> Self {
            Self::SyncFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for IPoolEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `blockTimestampLast` function with signature `blockTimestampLast()` and selector `0xc5700a02`
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
    #[ethcall(name = "blockTimestampLast", abi = "blockTimestampLast()")]
    pub struct BlockTimestampLastCall;
    ///Container type for all input parameters for the `burn` function with signature `burn(address)` and selector `0x89afcb44`
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
    #[ethcall(name = "burn", abi = "burn(address)")]
    pub struct BurnCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `claimFees` function with signature `claimFees()` and selector `0xd294f093`
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
    #[ethcall(name = "claimFees", abi = "claimFees()")]
    pub struct ClaimFeesCall;
    ///Container type for all input parameters for the `claimable0` function with signature `claimable0(address)` and selector `0x4d5a9f8a`
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
    #[ethcall(name = "claimable0", abi = "claimable0(address)")]
    pub struct Claimable0Call(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `claimable1` function with signature `claimable1(address)` and selector `0xa1ac4d13`
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
    #[ethcall(name = "claimable1", abi = "claimable1(address)")]
    pub struct Claimable1Call(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `currentCumulativePrices` function with signature `currentCumulativePrices()` and selector `0x1df8c717`
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
    #[ethcall(name = "currentCumulativePrices", abi = "currentCumulativePrices()")]
    pub struct CurrentCumulativePricesCall;
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `getAmountOut` function with signature `getAmountOut(uint256,address)` and selector `0xf140a35a`
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
    #[ethcall(name = "getAmountOut", abi = "getAmountOut(uint256,address)")]
    pub struct GetAmountOutCall {
        pub amount_in: ::ethers::core::types::U256,
        pub token_in: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getK` function with signature `getK()` and selector `0xee39e7a0`
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
    #[ethcall(name = "getK", abi = "getK()")]
    pub struct GetKCall;
    ///Container type for all input parameters for the `getReserves` function with signature `getReserves()` and selector `0x0902f1ac`
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
    #[ethcall(name = "getReserves", abi = "getReserves()")]
    pub struct GetReservesCall;
    ///Container type for all input parameters for the `index0` function with signature `index0()` and selector `0x32c0defd`
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
    #[ethcall(name = "index0", abi = "index0()")]
    pub struct Index0Call;
    ///Container type for all input parameters for the `index1` function with signature `index1()` and selector `0xbda39cad`
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
    #[ethcall(name = "index1", abi = "index1()")]
    pub struct Index1Call;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,bool)` and selector `0xe4bbb5a8`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,bool)")]
    pub struct InitializeCall {
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
        pub stable: bool,
    }
    ///Container type for all input parameters for the `lastObservation` function with signature `lastObservation()` and selector `0x8a7b8cf2`
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
    #[ethcall(name = "lastObservation", abi = "lastObservation()")]
    pub struct LastObservationCall;
    ///Container type for all input parameters for the `metadata` function with signature `metadata()` and selector `0x392f37e9`
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
    #[ethcall(name = "metadata", abi = "metadata()")]
    pub struct MetadataCall;
    ///Container type for all input parameters for the `mint` function with signature `mint(address)` and selector `0x6a627842`
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
    #[ethcall(name = "mint", abi = "mint(address)")]
    pub struct MintCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `observationLength` function with signature `observationLength()` and selector `0xebeb31db`
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
    #[ethcall(name = "observationLength", abi = "observationLength()")]
    pub struct ObservationLengthCall;
    ///Container type for all input parameters for the `periodSize` function with signature `periodSize()` and selector `0xe4463eb2`
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
    #[ethcall(name = "periodSize", abi = "periodSize()")]
    pub struct PeriodSizeCall;
    ///Container type for all input parameters for the `poolFees` function with signature `poolFees()` and selector `0x33580959`
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
    #[ethcall(name = "poolFees", abi = "poolFees()")]
    pub struct PoolFeesCall;
    ///Container type for all input parameters for the `prices` function with signature `prices(address,uint256,uint256)` and selector `0x5881c475`
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
    #[ethcall(name = "prices", abi = "prices(address,uint256,uint256)")]
    pub struct PricesCall {
        pub token_in: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub points: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quote` function with signature `quote(address,uint256,uint256)` and selector `0x9e8cc04b`
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
    #[ethcall(name = "quote", abi = "quote(address,uint256,uint256)")]
    pub struct QuoteCall {
        pub token_in: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub granularity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `reserve0` function with signature `reserve0()` and selector `0x443cb4bc`
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
    #[ethcall(name = "reserve0", abi = "reserve0()")]
    pub struct Reserve0Call;
    ///Container type for all input parameters for the `reserve0CumulativeLast` function with signature `reserve0CumulativeLast()` and selector `0xbf944dbc`
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
    #[ethcall(name = "reserve0CumulativeLast", abi = "reserve0CumulativeLast()")]
    pub struct Reserve0CumulativeLastCall;
    ///Container type for all input parameters for the `reserve1` function with signature `reserve1()` and selector `0x5a76f25e`
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
    #[ethcall(name = "reserve1", abi = "reserve1()")]
    pub struct Reserve1Call;
    ///Container type for all input parameters for the `reserve1CumulativeLast` function with signature `reserve1CumulativeLast()` and selector `0xc245febc`
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
    #[ethcall(name = "reserve1CumulativeLast", abi = "reserve1CumulativeLast()")]
    pub struct Reserve1CumulativeLastCall;
    ///Container type for all input parameters for the `sample` function with signature `sample(address,uint256,uint256,uint256)` and selector `0x13345fe1`
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
    #[ethcall(name = "sample", abi = "sample(address,uint256,uint256,uint256)")]
    pub struct SampleCall {
        pub token_in: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub points: ::ethers::core::types::U256,
        pub window: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setName` function with signature `setName(string)` and selector `0xc47f0027`
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
    #[ethcall(name = "setName", abi = "setName(string)")]
    pub struct SetNameCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `setSymbol` function with signature `setSymbol(string)` and selector `0xb84c8246`
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
    #[ethcall(name = "setSymbol", abi = "setSymbol(string)")]
    pub struct SetSymbolCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `skim` function with signature `skim(address)` and selector `0xbc25cf77`
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
    #[ethcall(name = "skim", abi = "skim(address)")]
    pub struct SkimCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `stable` function with signature `stable()` and selector `0x22be3de1`
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
    #[ethcall(name = "stable", abi = "stable()")]
    pub struct StableCall;
    ///Container type for all input parameters for the `supplyIndex0` function with signature `supplyIndex0(address)` and selector `0x9f767c88`
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
    #[ethcall(name = "supplyIndex0", abi = "supplyIndex0(address)")]
    pub struct SupplyIndex0Call(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `supplyIndex1` function with signature `supplyIndex1(address)` and selector `0x205aabf1`
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
    #[ethcall(name = "supplyIndex1", abi = "supplyIndex1(address)")]
    pub struct SupplyIndex1Call(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `swap` function with signature `swap(uint256,uint256,address,bytes)` and selector `0x022c0d9f`
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
    #[ethcall(name = "swap", abi = "swap(uint256,uint256,address,bytes)")]
    pub struct SwapCall {
        pub amount_0_out: ::ethers::core::types::U256,
        pub amount_1_out: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `sync` function with signature `sync()` and selector `0xfff6cae9`
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
    #[ethcall(name = "sync", abi = "sync()")]
    pub struct SyncCall;
    ///Container type for all input parameters for the `token0` function with signature `token0()` and selector `0x0dfe1681`
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
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    ///Container type for all input parameters for the `token1` function with signature `token1()` and selector `0xd21220a7`
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
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    ///Container type for all input parameters for the `tokens` function with signature `tokens()` and selector `0x9d63848a`
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
    #[ethcall(name = "tokens", abi = "tokens()")]
    pub struct TokensCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    pub enum IPoolCalls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        BlockTimestampLast(BlockTimestampLastCall),
        Burn(BurnCall),
        ClaimFees(ClaimFeesCall),
        Claimable0(Claimable0Call),
        Claimable1(Claimable1Call),
        CurrentCumulativePrices(CurrentCumulativePricesCall),
        Factory(FactoryCall),
        GetAmountOut(GetAmountOutCall),
        GetK(GetKCall),
        GetReserves(GetReservesCall),
        Index0(Index0Call),
        Index1(Index1Call),
        Initialize(InitializeCall),
        LastObservation(LastObservationCall),
        Metadata(MetadataCall),
        Mint(MintCall),
        ObservationLength(ObservationLengthCall),
        PeriodSize(PeriodSizeCall),
        PoolFees(PoolFeesCall),
        Prices(PricesCall),
        Quote(QuoteCall),
        Reserve0(Reserve0Call),
        Reserve0CumulativeLast(Reserve0CumulativeLastCall),
        Reserve1(Reserve1Call),
        Reserve1CumulativeLast(Reserve1CumulativeLastCall),
        Sample(SampleCall),
        SetName(SetNameCall),
        SetSymbol(SetSymbolCall),
        Skim(SkimCall),
        Stable(StableCall),
        SupplyIndex0(SupplyIndex0Call),
        SupplyIndex1(SupplyIndex1Call),
        Swap(SwapCall),
        Sync(SyncCall),
        Token0(Token0Call),
        Token1(Token1Call),
        Tokens(TokensCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for IPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BlockTimestampLastCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BlockTimestampLast(decoded));
            }
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) = <ClaimFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimFees(decoded));
            }
            if let Ok(decoded) = <Claimable0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Claimable0(decoded));
            }
            if let Ok(decoded) = <Claimable1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Claimable1(decoded));
            }
            if let Ok(decoded) = <CurrentCumulativePricesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CurrentCumulativePrices(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <GetAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAmountOut(decoded));
            }
            if let Ok(decoded) = <GetKCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetK(decoded));
            }
            if let Ok(decoded) = <GetReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReserves(decoded));
            }
            if let Ok(decoded) = <Index0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Index0(decoded));
            }
            if let Ok(decoded) = <Index1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Index1(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <LastObservationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastObservation(decoded));
            }
            if let Ok(decoded) = <MetadataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Metadata(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) = <ObservationLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ObservationLength(decoded));
            }
            if let Ok(decoded) = <PeriodSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PeriodSize(decoded));
            }
            if let Ok(decoded) = <PoolFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolFees(decoded));
            }
            if let Ok(decoded) = <PricesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Prices(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Quote(decoded));
            }
            if let Ok(decoded) = <Reserve0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Reserve0(decoded));
            }
            if let Ok(decoded) = <Reserve0CumulativeLastCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Reserve0CumulativeLast(decoded));
            }
            if let Ok(decoded) = <Reserve1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Reserve1(decoded));
            }
            if let Ok(decoded) = <Reserve1CumulativeLastCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Reserve1CumulativeLast(decoded));
            }
            if let Ok(decoded) = <SampleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sample(decoded));
            }
            if let Ok(decoded) = <SetNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetName(decoded));
            }
            if let Ok(decoded) = <SetSymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetSymbol(decoded));
            }
            if let Ok(decoded) = <SkimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Skim(decoded));
            }
            if let Ok(decoded) = <StableCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Stable(decoded));
            }
            if let Ok(decoded) = <SupplyIndex0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupplyIndex0(decoded));
            }
            if let Ok(decoded) = <SupplyIndex1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupplyIndex1(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <SyncCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sync(decoded));
            }
            if let Ok(decoded) = <Token0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Token0(decoded));
            }
            if let Ok(decoded) = <Token1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Token1(decoded));
            }
            if let Ok(decoded) = <TokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Tokens(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlockTimestampLast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClaimFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Claimable0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Claimable1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentCumulativePrices(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetK(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Index0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Index1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastObservation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Metadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ObservationLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PeriodSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Prices(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Reserve0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Reserve0CumulativeLast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Reserve1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Reserve1CumulativeLast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Sample(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetName(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSymbol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Skim(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Stable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupplyIndex0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupplyIndex1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sync(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Tokens(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IPoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlockTimestampLast(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::Claimable0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Claimable1(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentCumulativePrices(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetK(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::Index0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Index1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastObservation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Metadata(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::ObservationLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeriodSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::Prices(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::Reserve0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Reserve0CumulativeLast(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Reserve1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Reserve1CumulativeLast(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Sample(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetName(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSymbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::Skim(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stable(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyIndex0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyIndex1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sync(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Tokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowanceCall> for IPoolCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for IPoolCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for IPoolCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BlockTimestampLastCall> for IPoolCalls {
        fn from(value: BlockTimestampLastCall) -> Self {
            Self::BlockTimestampLast(value)
        }
    }
    impl ::core::convert::From<BurnCall> for IPoolCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<ClaimFeesCall> for IPoolCalls {
        fn from(value: ClaimFeesCall) -> Self {
            Self::ClaimFees(value)
        }
    }
    impl ::core::convert::From<Claimable0Call> for IPoolCalls {
        fn from(value: Claimable0Call) -> Self {
            Self::Claimable0(value)
        }
    }
    impl ::core::convert::From<Claimable1Call> for IPoolCalls {
        fn from(value: Claimable1Call) -> Self {
            Self::Claimable1(value)
        }
    }
    impl ::core::convert::From<CurrentCumulativePricesCall> for IPoolCalls {
        fn from(value: CurrentCumulativePricesCall) -> Self {
            Self::CurrentCumulativePrices(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for IPoolCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for IPoolCalls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetKCall> for IPoolCalls {
        fn from(value: GetKCall) -> Self {
            Self::GetK(value)
        }
    }
    impl ::core::convert::From<GetReservesCall> for IPoolCalls {
        fn from(value: GetReservesCall) -> Self {
            Self::GetReserves(value)
        }
    }
    impl ::core::convert::From<Index0Call> for IPoolCalls {
        fn from(value: Index0Call) -> Self {
            Self::Index0(value)
        }
    }
    impl ::core::convert::From<Index1Call> for IPoolCalls {
        fn from(value: Index1Call) -> Self {
            Self::Index1(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for IPoolCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LastObservationCall> for IPoolCalls {
        fn from(value: LastObservationCall) -> Self {
            Self::LastObservation(value)
        }
    }
    impl ::core::convert::From<MetadataCall> for IPoolCalls {
        fn from(value: MetadataCall) -> Self {
            Self::Metadata(value)
        }
    }
    impl ::core::convert::From<MintCall> for IPoolCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<ObservationLengthCall> for IPoolCalls {
        fn from(value: ObservationLengthCall) -> Self {
            Self::ObservationLength(value)
        }
    }
    impl ::core::convert::From<PeriodSizeCall> for IPoolCalls {
        fn from(value: PeriodSizeCall) -> Self {
            Self::PeriodSize(value)
        }
    }
    impl ::core::convert::From<PoolFeesCall> for IPoolCalls {
        fn from(value: PoolFeesCall) -> Self {
            Self::PoolFees(value)
        }
    }
    impl ::core::convert::From<PricesCall> for IPoolCalls {
        fn from(value: PricesCall) -> Self {
            Self::Prices(value)
        }
    }
    impl ::core::convert::From<QuoteCall> for IPoolCalls {
        fn from(value: QuoteCall) -> Self {
            Self::Quote(value)
        }
    }
    impl ::core::convert::From<Reserve0Call> for IPoolCalls {
        fn from(value: Reserve0Call) -> Self {
            Self::Reserve0(value)
        }
    }
    impl ::core::convert::From<Reserve0CumulativeLastCall> for IPoolCalls {
        fn from(value: Reserve0CumulativeLastCall) -> Self {
            Self::Reserve0CumulativeLast(value)
        }
    }
    impl ::core::convert::From<Reserve1Call> for IPoolCalls {
        fn from(value: Reserve1Call) -> Self {
            Self::Reserve1(value)
        }
    }
    impl ::core::convert::From<Reserve1CumulativeLastCall> for IPoolCalls {
        fn from(value: Reserve1CumulativeLastCall) -> Self {
            Self::Reserve1CumulativeLast(value)
        }
    }
    impl ::core::convert::From<SampleCall> for IPoolCalls {
        fn from(value: SampleCall) -> Self {
            Self::Sample(value)
        }
    }
    impl ::core::convert::From<SetNameCall> for IPoolCalls {
        fn from(value: SetNameCall) -> Self {
            Self::SetName(value)
        }
    }
    impl ::core::convert::From<SetSymbolCall> for IPoolCalls {
        fn from(value: SetSymbolCall) -> Self {
            Self::SetSymbol(value)
        }
    }
    impl ::core::convert::From<SkimCall> for IPoolCalls {
        fn from(value: SkimCall) -> Self {
            Self::Skim(value)
        }
    }
    impl ::core::convert::From<StableCall> for IPoolCalls {
        fn from(value: StableCall) -> Self {
            Self::Stable(value)
        }
    }
    impl ::core::convert::From<SupplyIndex0Call> for IPoolCalls {
        fn from(value: SupplyIndex0Call) -> Self {
            Self::SupplyIndex0(value)
        }
    }
    impl ::core::convert::From<SupplyIndex1Call> for IPoolCalls {
        fn from(value: SupplyIndex1Call) -> Self {
            Self::SupplyIndex1(value)
        }
    }
    impl ::core::convert::From<SwapCall> for IPoolCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<SyncCall> for IPoolCalls {
        fn from(value: SyncCall) -> Self {
            Self::Sync(value)
        }
    }
    impl ::core::convert::From<Token0Call> for IPoolCalls {
        fn from(value: Token0Call) -> Self {
            Self::Token0(value)
        }
    }
    impl ::core::convert::From<Token1Call> for IPoolCalls {
        fn from(value: Token1Call) -> Self {
            Self::Token1(value)
        }
    }
    impl ::core::convert::From<TokensCall> for IPoolCalls {
        fn from(value: TokensCall) -> Self {
            Self::Tokens(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for IPoolCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for IPoolCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for IPoolCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `blockTimestampLast` function with signature `blockTimestampLast()` and selector `0xc5700a02`
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
    pub struct BlockTimestampLastReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `burn` function with signature `burn(address)` and selector `0x89afcb44`
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
    pub struct BurnReturn {
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `claimFees` function with signature `claimFees()` and selector `0xd294f093`
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
    pub struct ClaimFeesReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `claimable0` function with signature `claimable0(address)` and selector `0x4d5a9f8a`
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
    pub struct Claimable0Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `claimable1` function with signature `claimable1(address)` and selector `0xa1ac4d13`
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
    pub struct Claimable1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `currentCumulativePrices` function with signature `currentCumulativePrices()` and selector `0x1df8c717`
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
    pub struct CurrentCumulativePricesReturn {
        pub reserve_0_cumulative: ::ethers::core::types::U256,
        pub reserve_1_cumulative: ::ethers::core::types::U256,
        pub block_timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getAmountOut` function with signature `getAmountOut(uint256,address)` and selector `0xf140a35a`
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
    pub struct GetAmountOutReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getK` function with signature `getK()` and selector `0xee39e7a0`
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
    pub struct GetKReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getReserves` function with signature `getReserves()` and selector `0x0902f1ac`
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
    pub struct GetReservesReturn {
        pub reserve_0: ::ethers::core::types::U256,
        pub reserve_1: ::ethers::core::types::U256,
        pub block_timestamp_last: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `index0` function with signature `index0()` and selector `0x32c0defd`
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
    pub struct Index0Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `index1` function with signature `index1()` and selector `0xbda39cad`
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
    pub struct Index1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lastObservation` function with signature `lastObservation()` and selector `0x8a7b8cf2`
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
    pub struct LastObservationReturn(pub Observation);
    ///Container type for all return fields from the `metadata` function with signature `metadata()` and selector `0x392f37e9`
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
    pub struct MetadataReturn {
        pub dec_0: ::ethers::core::types::U256,
        pub dec_1: ::ethers::core::types::U256,
        pub r_0: ::ethers::core::types::U256,
        pub r_1: ::ethers::core::types::U256,
        pub st: bool,
        pub t_0: ::ethers::core::types::Address,
        pub t_1: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `mint` function with signature `mint(address)` and selector `0x6a627842`
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
    pub struct MintReturn {
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `observationLength` function with signature `observationLength()` and selector `0xebeb31db`
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
    pub struct ObservationLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `periodSize` function with signature `periodSize()` and selector `0xe4463eb2`
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
    pub struct PeriodSizeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `poolFees` function with signature `poolFees()` and selector `0x33580959`
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
    pub struct PoolFeesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `prices` function with signature `prices(address,uint256,uint256)` and selector `0x5881c475`
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
    pub struct PricesReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `quote` function with signature `quote(address,uint256,uint256)` and selector `0x9e8cc04b`
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
    pub struct QuoteReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `reserve0` function with signature `reserve0()` and selector `0x443cb4bc`
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
    pub struct Reserve0Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `reserve0CumulativeLast` function with signature `reserve0CumulativeLast()` and selector `0xbf944dbc`
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
    pub struct Reserve0CumulativeLastReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `reserve1` function with signature `reserve1()` and selector `0x5a76f25e`
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
    pub struct Reserve1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `reserve1CumulativeLast` function with signature `reserve1CumulativeLast()` and selector `0xc245febc`
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
    pub struct Reserve1CumulativeLastReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `sample` function with signature `sample(address,uint256,uint256,uint256)` and selector `0x13345fe1`
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
    pub struct SampleReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `stable` function with signature `stable()` and selector `0x22be3de1`
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
    pub struct StableReturn(pub bool);
    ///Container type for all return fields from the `supplyIndex0` function with signature `supplyIndex0(address)` and selector `0x9f767c88`
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
    pub struct SupplyIndex0Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `supplyIndex1` function with signature `supplyIndex1(address)` and selector `0x205aabf1`
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
    pub struct SupplyIndex1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `token0` function with signature `token0()` and selector `0x0dfe1681`
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
    pub struct Token0Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `token1` function with signature `token1()` and selector `0xd21220a7`
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
    pub struct Token1Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokens` function with signature `tokens()` and selector `0x9d63848a`
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
    pub struct TokensReturn(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
}
