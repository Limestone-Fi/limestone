pub use mock_pool::*;
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
pub mod mock_pool {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
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
                                    name: ::std::borrow::ToOwned::to_owned("claimed0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("claimed1"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("eip712Domain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eip712Domain"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fields"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        1usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes1"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("verifyingContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("extensions"),
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
                    ::std::borrow::ToOwned::to_owned("mutateSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mutateSupply"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_newSupply"),
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("observations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("observations"),
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
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("EIP712DomainChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EIP712DomainChanged",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignature",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignatureLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
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
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignatureS",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InsufficientAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InsufficientAllowance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allowance"),
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
                    ::std::borrow::ToOwned::to_owned("ERC20InsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidApprover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InvalidApprover",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InvalidReceiver",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ERC20InvalidSender"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidSpender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InvalidSpender",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC2612ExpiredSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC2612ExpiredSignature",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                    ::std::borrow::ToOwned::to_owned("ERC2612InvalidSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC2612InvalidSigner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("InvalidAccountNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidAccountNonce",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentNonce"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidShortString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidShortString"),
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
                    ::std::borrow::ToOwned::to_owned("ReentrancyGuardReentrantCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReentrancyGuardReentrantCall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeERC20FailedOperation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StringTooLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("StringTooLong"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("str"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKPOOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01``@R4\x80\x15a\0\x10W__\xFD[P`@\x80Q` \x80\x82\x01\x83R_\x80\x83R\x83Q\x80\x85\x01\x85R`\x01\x81R`1`\xF8\x1B\x81\x84\x01R\x84Q\x80\x84\x01\x86R\x82\x81R\x85Q\x93\x84\x01\x90\x95R\x90\x82R\x91\x92\x83\x92\x91`\x03a\0Z\x83\x82a\x026V[P`\x04a\0g\x82\x82a\x026V[Pa\0w\x91P\x83\x90P`\x05a\x01&V[a\x01 Ra\0\x86\x81`\x06a\x01&V[a\x01@R\x81Q` \x80\x84\x01\x91\x90\x91 `\xE0R\x81Q\x90\x82\x01 a\x01\0RF`\xA0Ra\x01\x12`\xE0Qa\x01\0Q`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R_\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x80RPP0`\xC0RP`\x01`\x08Ua\x03HV[_` \x83Q\x10\x15a\x01AWa\x01:\x83a\x01XV[\x90Pa\x01RV[\x81a\x01L\x84\x82a\x026V[P`\xFF\x90P[\x92\x91PPV[__\x82\x90P`\x1F\x81Q\x11\x15a\x01\x8BW\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\x01\x82\x91\x90a\x02\xF0V[`@Q\x80\x91\x03\x90\xFD[\x80Qa\x01\x96\x82a\x03%V[\x17\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\xC6W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\xE4WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x021W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x02\x0FWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02.W_\x81U`\x01\x01a\x02\x1BV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02OWa\x02Oa\x01\x9EV[a\x02c\x81a\x02]\x84Ta\x01\xB2V[\x84a\x01\xEAV[` `\x1F\x82\x11`\x01\x81\x14a\x02\x95W_\x83\x15a\x02~WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x02.V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02\xC4W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02\xA4V[P\x84\x82\x10\x15a\x02\xE1W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x01\xE4W_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa=\x12a\x03\x99_9_a(\xEF\x01R_a(\xC2\x01R_a(3\x01R_a(\x0B\x01R_a'f\x01R_a'\x90\x01R_a'\xBA\x01Ra=\x12_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\xE5W_5`\xE0\x1C\x80c\x89\xAF\xCBD\x11a\x01\x95W\x80c\xC4\x1C\xE1\xB9\x11a\0\xE4W\x80c\xD5\x05\xAC\xCF\x11a\0\x9EW\x80c\xE4\xBB\xB5\xA8\x11a\0yW\x80c\xE4\xBB\xB5\xA8\x14a\x076W\x80c\xEB\xEB1\xDB\x14a\x07IW\x80c\xF1@\xA3Z\x14a\x07QW\x80c\xFF\xF6\xCA\xE9\x14a\x07dW__\xFD[\x80c\xD5\x05\xAC\xCF\x14a\x06\xE2W\x80c\xDDb\xED>\x14a\x06\xF5W\x80c\xE4F>\xB2\x14a\x07-W__\xFD[\x80c\xC4\x1C\xE1\xB9\x14a\x06\x85W\x80c\xC4Z\x01U\x14a\x06\x98W\x80c\xC4\x7F\0'\x14a\x06\xABW\x80c\xC5p\n\x02\x14a\x06\xBEW\x80c\xD2\x12 \xA7\x14a\x06\xC7W\x80c\xD2\x94\xF0\x93\x14a\x06\xDAW__\xFD[\x80c\xA1\xACM\x13\x11a\x01OW\x80c\xBC%\xCFw\x11a\x01*W\x80c\xBC%\xCFw\x14a\x06WW\x80c\xBD\xA3\x9C\xAD\x14a\x06jW\x80c\xBF\x94M\xBC\x14a\x06sW\x80c\xC2E\xFE\xBC\x14a\x06|W__\xFD[\x80c\xA1\xACM\x13\x14a\x06\x12W\x80c\xA9\x05\x9C\xBB\x14a\x061W\x80c\xB8L\x82F\x14a\x06DW__\xFD[\x80c\x89\xAF\xCBD\x14a\x05[W\x80c\x8A{\x8C\xF2\x14a\x05\x83W\x80c\x95\xD8\x9BA\x14a\x05\xADW\x80c\x9Dc\x84\x8A\x14a\x05\xB5W\x80c\x9E\x8C\xC0K\x14a\x05\xE0W\x80c\x9Fv|\x88\x14a\x05\xF3W__\xFD[\x80c1<\xE5g\x11a\x02QW\x80cMZ\x9F\x8A\x11a\x02\x0BW\x80cjbxB\x11a\x01\xE6W\x80cjbxB\x14a\x04\xF2W\x80cp\xA0\x821\x14a\x05\x05W\x80c~\xCE\xBE\0\x14a\x05-W\x80c\x84\xB0\x19n\x14a\x05@W__\xFD[\x80cMZ\x9F\x8A\x14a\x04\xB7W\x80cX\x81\xC4u\x14a\x04\xD6W\x80cZv\xF2^\x14a\x04\xE9W__\xFD[\x80c1<\xE5g\x14a\x04 W\x80c2\xC0\xDE\xFD\x14a\x04/W\x80c3X\tY\x14a\x048W\x80c6D\xE5\x15\x14a\x04KW\x80c9/7\xE9\x14a\x04SW\x80cD<\xB4\xBC\x14a\x04\xAEW__\xFD[\x80c\x18\x16\r\xDD\x11a\x02\xA2W\x80c\x18\x16\r\xDD\x14a\x03\xB4W\x80c\x1D\xF8\xC7\x17\x14a\x03\xC6W\x80c Z\xAB\xF1\x14a\x03\xCEW\x80c\"\xBE=\xE1\x14a\x03\xEDW\x80c#\xB8r\xDD\x14a\x03\xFAW\x80c%,\t\xD7\x14a\x04\rW__\xFD[\x80c\x02,\r\x9F\x14a\x02\xE9W\x80c\x06\xFD\xDE\x03\x14a\x02\xFEW\x80c\t\x02\xF1\xAC\x14a\x03\x1CW\x80c\t^\xA7\xB3\x14a\x03AW\x80c\r\xFE\x16\x81\x14a\x03dW\x80c\x134_\xE1\x14a\x03\x94W[__\xFD[a\x02\xFCa\x02\xF76`\x04a2\xCEV[a\x07lV[\0[a\x03\x06a\r\x8BV[`@Qa\x03\x13\x91\x90a3_V[`@Q\x80\x91\x03\x90\xF3[`\x12T`\x13T`\x14T[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03\x13V[a\x03Ta\x03O6`\x04a3qV[a\x0E\x1BV[`@Q\x90\x15\x15\x81R` \x01a\x03\x13V[`\x0BTa\x03|\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x13V[a\x03\xA7a\x03\xA26`\x04a3\x99V[a\x0E4V[`@Qa\x03\x13\x91\x90a4\tV[`\x02T[`@Q\x90\x81R` \x01a\x03\x13V[a\x03&a\x10\x1BV[a\x03\xB8a\x03\xDC6`\x04a4\x1BV[`\x1A` R_\x90\x81R`@\x90 T\x81V[`\x0BTa\x03T\x90`\xFF\x16\x81V[a\x03Ta\x04\x086`\x04a44V[a\x10\x88V[a\x03&a\x04\x1B6`\x04a4nV[a\x10\xADV[`@Q`\x12\x81R` \x01a\x03\x13V[a\x03\xB8`\x17T\x81V[`\rTa\x03|\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xB8a\x10\xDEV[`\x10T`\x11T`\x12T`\x13T`\x0BT`\x0CT`@\x80Q\x96\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\xFF\x81\x16\x15\x15`\x80\x84\x01R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16`\xA0\x84\x01R\x16`\xC0\x82\x01R`\xE0\x01a\x03\x13V[a\x03\xB8`\x12T\x81V[a\x03\xB8a\x04\xC56`\x04a4\x1BV[`\x1B` R_\x90\x81R`@\x90 T\x81V[a\x03\xA7a\x04\xE46`\x04a4\x85V[a\x10\xECV[a\x03\xB8`\x13T\x81V[a\x03\xB8a\x05\x006`\x04a4\x1BV[a\x10\xFBV[a\x03\xB8a\x05\x136`\x04a4\x1BV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x03\xB8a\x05;6`\x04a4\x1BV[a\x14+V[a\x05Ha\x14HV[`@Qa\x03\x13\x97\x96\x95\x94\x93\x92\x91\x90a4\xB5V[a\x05na\x05i6`\x04a4\x1BV[a\x14\x8AV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03\x13V[a\x05\x8Ba\x17XV[`@\x80Q\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R\x91\x81\x01Q\x90\x82\x01R``\x01a\x03\x13V[a\x03\x06a\x17\xD2V[`\x0BT`\x0CT`@\x80Qa\x01\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R\x90\x91\x16` \x83\x01R\x01a\x03\x13V[a\x03\xB8a\x05\xEE6`\x04a4\x85V[a\x17\xE1V[a\x03\xB8a\x06\x016`\x04a4\x1BV[`\x19` R_\x90\x81R`@\x90 T\x81V[a\x03\xB8a\x06 6`\x04a4\x1BV[`\x1C` R_\x90\x81R`@\x90 T\x81V[a\x03Ta\x06?6`\x04a3qV[a\x18FV[a\x02\xFCa\x06R6`\x04a5$V[a\x18SV[a\x02\xFCa\x06e6`\x04a4\x1BV[a\x18eV[a\x03\xB8`\x18T\x81V[a\x03\xB8`\x15T\x81V[a\x03\xB8`\x16T\x81V[a\x02\xFCa\x06\x936`\x04a4nV[a\x19\xA2V[`\x0ETa\x03|\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xFCa\x06\xB96`\x04a5$V[a\x19\xADV[a\x03\xB8`\x14T\x81V[`\x0CTa\x03|\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05na\x19\xBAV[a\x02\xFCa\x06\xF06`\x04a5cV[a\x1A\xBBV[a\x03\xB8a\x07\x036`\x04a5\xD0V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x03\xB8a\x07\x08\x81V[a\x02\xFCa\x07D6`\x04a6\x0EV[a\x1B\xF6V[`\x0FTa\x03\xB8V[a\x03\xB8a\x07_6`\x04a6RV[a\x1E\x9BV[a\x02\xFCa\x1FWV[a\x07ta sV[`\x0E_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xB1\x87\xBD&`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xC4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE8\x91\x90a6sV[\x15a\x08\x06W`@Qc\x13\t\xA5c`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x15\x80\x15a\x08\x12WP\x83\x15[\x15a\x080W`@QcB0\x1C#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x12T`\x13T\x81\x87\x10\x15\x80a\x08EWP\x80\x86\x10\x15[\x15a\x08cW`@Qc\xBBU\xFD'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT`\x0CT_\x91\x82\x91`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x92\x04\x82\x16\x91\x90\x81\x16\x90\x89\x16\x82\x14\x80a\x08\xA2WP\x80`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x08\xC0W`@Qc\x05!\xF41`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x8A\x15a\x08\xDAWa\x08\xDA`\x01`\x01`\xA0\x1B\x03\x83\x16\x8A\x8Da \x9DV[\x89\x15a\x08\xF4Wa\x08\xF4`\x01`\x01`\xA0\x1B\x03\x82\x16\x8A\x8Ca \x9DV[\x86\x15a\t\\W`@Qc\x9A{\xFFy`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\x9A{\xFFy\x90a\t.\x903\x90\x8F\x90\x8F\x90\x8E\x90\x8E\x90`\x04\x01a6\x8EV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\tEW__\xFD[PZ\xF1\x15\x80\x15a\tWW=__>=_\xFD[PPPP[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xC2\x91\x90a6\xD9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x94P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n+\x91\x90a6\xD9V[\x92PPP_\x89\x85a\n<\x91\x90a7\x04V[\x83\x11a\nHW_a\n\\V[a\nR\x8A\x86a7\x04V[a\n\\\x90\x84a7\x04V[\x90P_a\ni\x8A\x86a7\x04V[\x83\x11a\nuW_a\n\x89V[a\n\x7F\x8A\x86a7\x04V[a\n\x89\x90\x84a7\x04V[\x90P\x81\x15\x80\x15a\n\x97WP\x80\x15[\x15a\n\xB5W`@Qc\t\x8F\xB5a`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT`\x0CT`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x92\x04\x82\x16\x91\x16\x83\x15a\x0BmW`\x0ET`\x0BT`@Qc\xCCV\xB2\xC5`\xE0\x1B\x81R0`\x04\x82\x01R`\xFF\x90\x91\x16\x15\x15`$\x82\x01Ra\x0Bm\x91a'\x10\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCCV\xB2\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BT\x91\x90a6\xD9V[a\x0B^\x90\x87a7\x17V[a\x0Bh\x91\x90a7.V[a \xEFV[\x82\x15a\x0C\rW`\x0ET`\x0BT`@Qc\xCCV\xB2\xC5`\xE0\x1B\x81R0`\x04\x82\x01R`\xFF\x90\x91\x16\x15\x15`$\x82\x01Ra\x0C\r\x91a'\x10\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCCV\xB2\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xD0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF4\x91\x90a6\xD9V[a\x0B\xFE\x90\x86a7\x17V[a\x0C\x08\x91\x90a7.V[a!\x9FV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0COW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cs\x91\x90a6\xD9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x96P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xB8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xDC\x91\x90a6\xD9V[\x94Pa\x0C\xE8\x88\x88a\"CV[a\x0C\xF2\x87\x87a\"CV[\x10\x15a\r\x11W`@Qc\xA92I/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa\r\x1F\x84\x84\x88\x88a#7V[`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x90\x81\x01\x8C\x90R``\x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x903\x90\x7F\xB3\xE2w6\x06\xAB\xFD6\xB5\xBD\x919K:T\xD19\x836\xC6P\x05\xBA\xF7\xBFz\x05\xEF\xEF\xFA\xF7[\x90`\x80\x01`@Q\x80\x91\x03\x90\xA3PPPPPPa\r\x84`\x01`\x08UV[PPPPPV[```\t\x80Ta\r\x9A\x90a7MV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xC6\x90a7MV[\x80\x15a\x0E\x11W\x80`\x1F\x10a\r\xE8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\x11V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xF4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_3a\x0E(\x81\x85\x85a$\xC5V[`\x01\x91PP[\x92\x91PPV[``_\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EPWa\x0EPa7\x85V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0EyW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x0FT\x90\x91P_\x90a\x0E\x8E\x90`\x01\x90a7\x04V[\x90P_a\x0E\x9B\x85\x87a7\x17V[a\x0E\xA5\x90\x83a7\x04V[\x90P_\x80[\x83\x83\x10\x15a\x10\x0BWa\x0E\xBC\x87\x84a7\x99V[\x91P_`\x0F\x84\x81T\x81\x10a\x0E\xD2Wa\x0E\xD2a7\xACV[\x90_R` _ \x90`\x03\x02\x01_\x01T`\x0F\x84\x81T\x81\x10a\x0E\xF4Wa\x0E\xF4a7\xACV[\x90_R` _ \x90`\x03\x02\x01_\x01Ta\x0F\r\x91\x90a7\x04V[\x90P_\x81`\x0F\x86\x81T\x81\x10a\x0F$Wa\x0F$a7\xACV[\x90_R` _ \x90`\x03\x02\x01`\x01\x01T`\x0F\x86\x81T\x81\x10a\x0FGWa\x0FGa7\xACV[\x90_R` _ \x90`\x03\x02\x01`\x01\x01Ta\x0Fa\x91\x90a7\x04V[a\x0Fk\x91\x90a7.V[\x90P_\x82`\x0F\x87\x81T\x81\x10a\x0F\x82Wa\x0F\x82a7\xACV[\x90_R` _ \x90`\x03\x02\x01`\x02\x01T`\x0F\x87\x81T\x81\x10a\x0F\xA5Wa\x0F\xA5a7\xACV[\x90_R` _ \x90`\x03\x02\x01`\x02\x01Ta\x0F\xBF\x91\x90a7\x04V[a\x0F\xC9\x91\x90a7.V[\x90Pa\x0F\xD7\x8C\x8E\x84\x84a$\xD2V[\x88\x85\x81Q\x81\x10a\x0F\xE9Wa\x0F\xE9a7\xACV[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a\x10\x04\x87\x84a7\x99V[\x92Pa\x0E\xAAV[P\x92\x93PPPP[\x94\x93PPPPV[`\x15T`\x16TB_\x80\x80a\x108`\x12T`\x13T`\x14T\x91\x92\x90\x91\x90V[\x92P\x92P\x92P\x83\x81\x14a\x10\x80W_a\x10P\x82\x86a7\x04V[\x90Pa\x10\\\x81\x85a7\x17V[a\x10f\x90\x88a7\x99V[\x96Pa\x10r\x81\x84a7\x17V[a\x10|\x90\x87a7\x99V[\x95PP[PPP\x90\x91\x92V[_3a\x10\x95\x85\x82\x85a&\x82V[a\x10\xA0\x85\x85\x85a&\xFDV[`\x01\x91PP[\x93\x92PPPV[`\x0F\x81\x81T\x81\x10a\x10\xBCW_\x80\xFD[_\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x92P\x83V[_a\x10\xE7a'ZV[\x90P\x90V[``a\x10\x13\x84\x84\x84`\x01a\x0E4V[_a\x11\x04a sV[`\x12T`\x13T`\x0BT`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x91a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11UW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11y\x91\x90a6\xD9V[`\x0CT`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92P_\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xC4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xE8\x91\x90a6\xD9V[\x90P_a\x11\xF5\x85\x84a7\x04V[\x90P_a\x12\x02\x85\x84a7\x04V[\x90P_a\x12\x0E`\x02T\x90V[\x90P\x80_\x03a\x13fWa\x03\xE8a\x12\xB3a\x12'\x84\x86a7\x17V[p\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Ch\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cd\xFF\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Cb\xFF\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1Cb\x01\0\0\x01`\xB5`\x01\x92\x83\x1C\x1B\x02`\x12\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[a\x12\xBD\x91\x90a7\x04V[\x97Pa\x12\xCC`\x01a\x03\xE8a(\x83V[`\x0BT`\xFF\x16\x15a\x13aW`\x11Ta\x12\xEC\x83g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a\x12\xF6\x91\x90a7.V[`\x10Ta\x13\x0B\x85g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a\x13\x15\x91\x90a7.V[\x14a\x133W`@Qc\x05\x02k\xFD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[d\x02T\x0B\xE4\0a\x13C\x84\x84a\"CV[\x11a\x13aW`@Qc!\xC6\x9Do`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13\xA1V[a\x13\x9E\x87a\x13t\x83\x86a7\x17V[a\x13~\x91\x90a7.V[\x87a\x13\x89\x84\x86a7\x17V[a\x13\x93\x91\x90a7.V[\x80\x82\x18\x90\x82\x11\x02\x18\x90V[\x97P[a\x03\xE8\x88\x10\x15a\x13\xC4W`@Qc4\x89\xBEu`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13\xCE\x89\x89a(\x83V[a\x13\xDA\x85\x85\x89\x89a#7V[`@\x80Q\x84\x81R` \x81\x01\x84\x90R3\x91\x7FL \x9B_\xC8\xADPu\x8F\x13\xE2\xE1\x08\x8B\xA5jV\r\xFFi\n\x1Co\xEF&9OL\x03\x82\x1CO\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPa\x14&`\x01`\x08UV[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x07` R`@\x81 Ta\x0E.V[_``\x80___``a\x14Ya(\xBBV[a\x14aa(\xE8V[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[__a\x14\x94a sV[`\x12T`\x13T`\x0BT`\x0CT`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x91\x16\x90_\x90\x83\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x14\x91\x90a6\xD9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15[W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x7F\x91\x90a6\xD9V[0_\x90\x81R` \x81\x90R`@\x90 T`\x02T\x91\x92P\x90\x80a\x15\xA0\x85\x84a7\x17V[a\x15\xAA\x91\x90a7.V[\x99P\x80a\x15\xB7\x84\x84a7\x17V[a\x15\xC1\x91\x90a7.V[\x98P\x89\x15\x80a\x15\xCEWP\x88\x15[\x15a\x15\xECW`@Qct\x93\x83\xAD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15\xF60\x83a)\x15V[a\x16\n`\x01`\x01`\xA0\x1B\x03\x87\x16\x8C\x8Ca \x9DV[a\x16\x1E`\x01`\x01`\xA0\x1B\x03\x86\x16\x8C\x8Ba \x9DV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16`W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x84\x91\x90a6\xD9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x94P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xED\x91\x90a6\xD9V[\x92Pa\x16\xFB\x84\x84\x8A\x8Aa#7V[`@\x80Q\x8B\x81R` \x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x8D\x16\x913\x91\x7F]bJ\xA9\xC1H\x15:\xB3Dl\x1B\x15Of\x0E\xE7p\x1ET\x9F\xE9\xB6-\xABqq\xB1\xC8\x0Eo\xA2\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPPa\x17S`\x01`\x08UV[\x91P\x91V[a\x17y`@Q\x80``\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`\x0F\x80Ta\x17\x89\x90`\x01\x90a7\x04V[\x81T\x81\x10a\x17\x99Wa\x17\x99a7\xACV[\x90_R` _ \x90`\x03\x02\x01`@Q\x80``\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPP\x90P\x90V[```\n\x80Ta\r\x9A\x90a7MV[__a\x17\xF0\x85\x85\x85`\x01a\x0E4V[\x80Q\x90\x91P_\x90\x81[\x81\x81\x10\x15a\x180W\x83\x81\x81Q\x81\x10a\x18\x13Wa\x18\x13a7\xACV[` \x02` \x01\x01Q\x83a\x18&\x91\x90a7\x99V[\x92P`\x01\x01a\x17\xF9V[Pa\x18;\x85\x83a7.V[\x97\x96PPPPPPPV[_3a\x0E(\x81\x85\x85a&\xFDV[`\na\x18`\x82\x84\x83a8\x04V[PPPV[a\x18ma sV[`\x0BT`\x0CT`\x12T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x01\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x92\x16\x91a\x19\t\x91\x85\x91\x85\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xCAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xEE\x91\x90a6\xD9V[a\x18\xF8\x91\x90a7\x04V[`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x90a \x9DV[`\x13T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x19\x93\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19x\x91\x90a6\xD9V[a\x19\x82\x91\x90a7\x04V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x90a \x9DV[PPa\x19\x9F`\x01`\x08UV[PV[a\x19\x9F`\x01\x82a(\x83V[`\ta\x18`\x82\x84\x83a8\x04V[__a\x19\xC53a)IV[PP3_\x90\x81R`\x1B` \x90\x81R`@\x80\x83 T`\x1C\x90\x92R\x90\x91 T\x81\x15\x15\x80a\x19\xEFWP_\x81\x11[\x15a\x1A\xB7W3_\x81\x81R`\x1B` \x90\x81R`@\x80\x83 \x83\x90U`\x1C\x90\x91R\x80\x82 \x91\x90\x91U`\rT\x90Qc)\x9Ez\xE7`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01\x84\x90R`D\x82\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x16\x90cS<\xF5\xCE\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1AcW__\xFD[PZ\xF1\x15\x80\x15a\x1AuW=__>=_\xFD[PP`@\x80Q\x85\x81R` \x81\x01\x85\x90R3\x93P\x83\x92P\x7F\x86\\\xA0\x8DY\xF5\xCBEn\x85\xCD/~\xF66d\xEAOs2t\x14\xE9\xD8\x15,AX\xB0\xE9FE\x91\x01`@Q\x80\x91\x03\x90\xA3[\x90\x91V[\x83B\x11\x15a\x1A\xE4W`@Qc1<\x89\x81`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[_\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a\x1B/\x8C`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x07` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x90V[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_a\x1B\x89\x82a*\xA1V[\x90P_a\x1B\x98\x82\x87\x87\x87a*\xCDV[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xDFW`@Qc%\xC0\x07#`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x8B\x16`$\x82\x01R`D\x01a\x1A\xDBV[a\x1B\xEA\x8A\x8A\x8Aa$\xC5V[PPPPPPPPPPV[`\x0ET`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1C W`@Qc\x02\xA9\x8A7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x163\x17\x90\x91U`\x0B\x80T`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x90\x95\x16\x17\x90U\x91\x85\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x84\x15\x15\x16`\x01`\x01`\xA8\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x91\x90\x91\x17\x90U`@Q\x83\x90\x83\x90a\x1C\x8A\x90a2fV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x1C\xBAW=__>=_\xFD[P`\r\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q\x80\x82\x01\x82R`\x04\x80\x82RcUSDC`\xE0\x1B` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x90\x94R\x90\x83RcAERO`\xE0\x1B\x90\x83\x01R\x90\x82\x15a\x1D\x7FW\x81\x81`@Q` \x01a\x1D+\x92\x91\x90a8\xD5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\t\x90\x81a\x1DH\x91\x90a9\x0CV[P\x81\x81`@Q` \x01a\x1D\\\x92\x91\x90a9\xC7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\n\x90\x81a\x1Dy\x91\x90a9\x0CV[Pa\x1D\xE2V[\x81\x81`@Q` \x01a\x1D\x92\x92\x91\x90a9\xE2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\t\x90\x81a\x1D\xAF\x91\x90a9\x0CV[P\x81\x81`@Q` \x01a\x1D\xC3\x92\x91\x90a:\x07V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\n\x90\x81a\x1D\xE0\x91\x90a9\x0CV[P[PPb\x0FB@`\x10UPPg\r\xE0\xB6\xB3\xA7d\0\0`\x11UP`@\x80Q``\x81\x01\x82RB\x81R_` \x82\x01\x81\x81R\x92\x82\x01\x81\x81R`\x0F\x80T`\x01\x81\x01\x82U\x92R\x91Q\x7F\x8D\x11\x08\xE1\x0B\xCB|'\xDD\xDF\xC0.\xD9\xD6\x93\xA0t\x03\x9D\x02l\xF4\xEAB@\xB4\x0F}X\x1A\xC8\x02`\x03\x90\x92\x02\x91\x82\x01U\x91Q\x7F\x8D\x11\x08\xE1\x0B\xCB|'\xDD\xDF\xC0.\xD9\xD6\x93\xA0t\x03\x9D\x02l\xF4\xEAB@\xB4\x0F}X\x1A\xC8\x03\x83\x01UQ\x7F\x8D\x11\x08\xE1\x0B\xCB|'\xDD\xDF\xC0.\xD9\xD6\x93\xA0t\x03\x9D\x02l\xF4\xEAB@\xB4\x0F}X\x1A\xC8\x04\x90\x91\x01UV[`\x12T`\x13T`\x0ET`\x0BT`@Qc\xCCV\xB2\xC5`\xE0\x1B\x81R0`\x04\x82\x01R`\xFF\x90\x91\x16\x15\x15`$\x82\x01R_\x93\x92\x91a'\x10\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCCV\xB2\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\"\x91\x90a6\xD9V[a\x1F,\x90\x87a7\x17V[a\x1F6\x91\x90a7.V[a\x1F@\x90\x86a7\x04V[\x94Pa\x1FN\x85\x85\x84\x84a$\xD2V[\x95\x94PPPPPV[a\x1F_a sV[`\x02T_\x03a\x1F\x81W`@Qc\xBBU\xFD'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra g\x91a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xF2\x91\x90a6\xD9V[`\x0CT`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a 8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \\\x91\x90a6\xD9V[`\x12T`\x13Ta#7V[a q`\x01`\x08UV[V[`\x02`\x08T\x03a \x96W`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x08UV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x18`\x90\x84\x90a*\xF9V[\x80_\x03a \xF9WPV[`\rT`\x0BTa!\x1B\x91`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x92\x04\x82\x16\x91\x16\x83a \x9DV[_a!%`\x02T\x90V[a!7\x83g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a!A\x91\x90a7.V[\x90P\x80\x15a!`W\x80`\x17_\x82\x82Ta!Z\x91\x90a7\x99V[\x90\x91UPP[`@\x80Q\x83\x81R_` \x82\x01R3\x91\x7F\x11,%i\x02\xBFUKn\xD8\x82\xD2\x93f\x87\xAA\xEBB%\xE8\xCD[Q0<\x90\xCAl\xF4:\x86\x02\x91\x01[`@Q\x80\x91\x03\x90\xA2PPV[\x80_\x03a!\xA9WPV[`\rT`\x0CTa!\xC6\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x83a \x9DV[_a!\xD0`\x02T\x90V[a!\xE2\x83g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a!\xEC\x91\x90a7.V[\x90P\x80\x15a\"\x0BW\x80`\x18_\x82\x82Ta\"\x05\x91\x90a7\x99V[\x90\x91UPP[`@\x80Q_\x81R` \x81\x01\x84\x90R3\x91\x7F\x11,%i\x02\xBFUKn\xD8\x82\xD2\x93f\x87\xAA\xEBB%\xE8\xCD[Q0<\x90\xCAl\xF4:\x86\x02\x91\x01a!\x93V[`\x0BT_\x90`\xFF\x16\x15a#&W`\x10T_\x90a\"g\x85g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a\"q\x91\x90a7.V[\x90P_`\x11T\x84g\r\xE0\xB6\xB3\xA7d\0\0a\"\x8B\x91\x90a7\x17V[a\"\x95\x91\x90a7.V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a\"\xAB\x83\x85a7\x17V[a\"\xB5\x91\x90a7.V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a\"\xCB\x84\x80a7\x17V[a\"\xD5\x91\x90a7.V[g\r\xE0\xB6\xB3\xA7d\0\0a\"\xE8\x86\x80a7\x17V[a\"\xF2\x91\x90a7.V[a\"\xFC\x91\x90a7\x99V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a#\x11\x82\x84a7\x17V[a#\x1B\x91\x90a7.V[\x94PPPPPa\x0E.V[a#0\x82\x84a7\x17V[\x90Pa\x0E.V[`\x14TB\x90_\x90a#H\x90\x83a7\x04V[\x90P_\x81\x11\x80\x15a#XWP\x83\x15\x15[\x80\x15a#cWP\x82\x15\x15[\x15a#\xA8Wa#r\x81\x85a7\x17V[`\x15_\x82\x82Ta#\x82\x91\x90a7\x99V[\x90\x91UPa#\x92\x90P\x81\x84a7\x17V[`\x16_\x82\x82Ta#\xA2\x91\x90a7\x99V[\x90\x91UPP[_a#\xB1a\x17XV[\x80Q\x90\x91Pa#\xC0\x90\x84a7\x04V[\x91Pa\x07\x08\x82\x11\x15a$tW`@\x80Q``\x81\x01\x82R\x84\x81R`\x15T` \x82\x01\x90\x81R`\x16T\x92\x82\x01\x92\x83R`\x0F\x80T`\x01\x81\x01\x82U_\x91\x90\x91R\x91Q\x7F\x8D\x11\x08\xE1\x0B\xCB|'\xDD\xDF\xC0.\xD9\xD6\x93\xA0t\x03\x9D\x02l\xF4\xEAB@\xB4\x0F}X\x1A\xC8\x02`\x03\x90\x93\x02\x92\x83\x01UQ\x7F\x8D\x11\x08\xE1\x0B\xCB|'\xDD\xDF\xC0.\xD9\xD6\x93\xA0t\x03\x9D\x02l\xF4\xEAB@\xB4\x0F}X\x1A\xC8\x03\x82\x01U\x90Q\x7F\x8D\x11\x08\xE1\x0B\xCB|'\xDD\xDF\xC0.\xD9\xD6\x93\xA0t\x03\x9D\x02l\xF4\xEAB@\xB4\x0F}X\x1A\xC8\x04\x90\x91\x01U[`\x12\x87\x90U`\x13\x86\x90U`\x14\x83\x90U`@\x80Q\x88\x81R` \x81\x01\x88\x90R\x7F\xCF*\xA5\x08v\xCD\xFB\xB5A o\x89\xAF\x0E\xE7\x8DD\xA2\xAB\xF8\xD3(\xE3\x7F\xA4\x91\x7F\x98!I\x84\x8A\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPPPPV[a\x18`\x83\x83\x83`\x01a+eV[`\x0BT_\x90`\xFF\x16\x15a&.W_a$\xEA\x84\x84a\"CV[`\x10T\x90\x91Pa%\x02\x85g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a%\x0C\x91\x90a7.V[`\x11T\x90\x94Pa%$\x84g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a%.\x91\x90a7.V[`\x0BT\x90\x93P_\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x88\x81\x16a\x01\0\x90\x92\x04\x16\x14a%VW\x84\x86a%YV[\x85\x85[`\x0BT\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x88\x81\x16a\x01\0\x90\x92\x04\x16\x14a%\x9CW`\x11Ta%\x8D\x89g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a%\x97\x91\x90a7.V[a%\xBBV[`\x10Ta%\xB1\x89g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a%\xBB\x91\x90a7.V[\x97P_a%\xD2a%\xCB\x84\x8Ba7\x99V[\x85\x84a,7V[a%\xDC\x90\x83a7\x04V[`\x0BT\x90\x91Pg\r\xE0\xB6\xB3\xA7d\0\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16a\x01\0\x90\x92\x04\x16\x14a&\x0BW`\x10Ta&\x0FV[`\x11T[a&\x19\x90\x83a7\x17V[a&#\x91\x90a7.V[\x94PPPPPa\x10\x13V[`\x0BT_\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x87\x81\x16a\x01\0\x90\x92\x04\x16\x14a&SW\x83\x85a&VV[\x84\x84[\x90\x92P\x90Pa&e\x87\x83a7\x99V[a&o\x82\x89a7\x17V[a&y\x91\x90a7.V[\x92PPPa\x10\x13V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T_\x19\x81\x14a&\xF7W\x81\x81\x10\x15a&\xE9W`@Qc}\xC7\xA0\xD9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x1A\xDBV[a&\xF7\x84\x84\x84\x84\x03_a+eV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a'&W`@QcKc~\x8F`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x1A\xDBV[`\x01`\x01`\xA0\x1B\x03\x82\x16a'OW`@Qc\xECD/\x05`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x1A\xDBV[a\x18`\x83\x83\x83a-\x9DV[_0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a'\xB2WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a'\xDCWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x10\xE7`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R_\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16a(\xACW`@Qc\xECD/\x05`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x1A\xDBV[a(\xB7_\x83\x83a-\x9DV[PPV[``a\x10\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x05a-\xBAV[``a\x10\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x06a-\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16a)>W`@QcKc~\x8F`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x1A\xDBV[a(\xB7\x82_\x83a-\x9DV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R` \x81\x90R`@\x90 T\x80\x15a*pW`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x19` \x90\x81R`@\x80\x83 \x80T`\x1A\x80\x85R\x92\x85 \x80T`\x17T`\x18T\x94\x81\x90U\x94\x90\x95R\x82\x90U\x93a)\xA9\x85\x84a7\x04V[\x90P_a)\xB6\x85\x84a7\x04V[\x90P\x81\x15a*\x0FW_g\r\xE0\xB6\xB3\xA7d\0\0a)\xD2\x84\x8Aa7\x17V[a)\xDC\x91\x90a7.V[`\x01`\x01`\xA0\x1B\x03\x8A\x16_\x90\x81R`\x1B` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a*\x08\x90\x84\x90a7\x99V[\x90\x91UPPP[\x80\x15a*fW_g\r\xE0\xB6\xB3\xA7d\0\0a*)\x83\x8Aa7\x17V[a*3\x91\x90a7.V[`\x01`\x01`\xA0\x1B\x03\x8A\x16_\x90\x81R`\x1C` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a*_\x90\x84\x90a7\x99V[\x90\x91UPPP[PPPPPPPPV[`\x17T`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x19` \x90\x81R`@\x80\x83 \x93\x90\x93U`\x18T`\x1A\x90\x91R\x91\x90 UPPV[_a\x0E.a*\xADa'ZV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[____a*\xDD\x88\x88\x88\x88a.\\V[\x92P\x92P\x92Pa*\xED\x82\x82a/$V[P\x90\x96\x95PPPPPPV[__` _\x84Q` \x86\x01_\x88Z\xF1\x80a+\x18W`@Q=_\x82>=\x81\xFD[PP_Q=\x91P\x81\x15a+/W\x80`\x01\x14\x15a+<V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a&\xF7W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x1A\xDBV[`\x01`\x01`\xA0\x1B\x03\x84\x16a+\x8EW`@Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x1A\xDBV[`\x01`\x01`\xA0\x1B\x03\x83\x16a+\xB7W`@QcJ\x14\x06\xB1`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x1A\xDBV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x82\x90U\x80\x15a&\xF7W\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa,)\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPV[_\x80[`\xFF\x81\x10\x15a-oW_a,N\x86\x85a/\xDCV[\x90P\x84\x81\x10\x15a,\xEAW_a,c\x87\x86a0aV[a,m\x83\x88a7\x04V[a,\x7F\x90g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a,\x89\x91\x90a7.V[\x90P\x80_\x03a,\xD8W\x85\x82\x03a,\xA4W\x84\x93PPPPa\x10\xA6V[\x85a,\xB9\x88a,\xB4\x88`\x01a7\x99V[a\"CV[\x11\x15a,\xD4Wa,\xCA\x85`\x01a7\x99V[\x93PPPPa\x10\xA6V[P`\x01[a,\xE2\x81\x86a7\x99V[\x94PPa-fV[_a,\xF5\x87\x86a0aV[a,\xFF\x87\x84a7\x04V[a-\x11\x90g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a-\x1B\x91\x90a7.V[\x90P\x80_\x03a-XW\x85\x82\x14\x80a-DWP\x85a-B\x88a-=`\x01\x89a7\x04V[a/\xDCV[\x10[\x15a-TW\x84\x93PPPPa\x10\xA6V[P`\x01[a-b\x81\x86a7\x04V[\x94PP[P`\x01\x01a,:V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra!y`\xF0\x1B`D\x82\x01R`d\x01a\x1A\xDBV[a-\xA6\x83a)IV[a-\xAF\x82a)IV[a\x18`\x83\x83\x83a0\xDCV[```\xFF\x83\x14a-\xCDWa#0\x83a2\x02V[\x81\x80Ta-\xD9\x90a7MV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta.\x05\x90a7MV[\x80\x15a.PW\x80`\x1F\x10a.'Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a.PV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a.3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x0E.V[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a.\x95WP_\x91P`\x03\x90P\x82a/\x1AV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a.\xE6W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a/\x11WP_\x92P`\x01\x91P\x82\x90Pa/\x1AV[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[_\x82`\x03\x81\x11\x15a/7Wa/7a:\"V[\x03a/@WPPV[`\x01\x82`\x03\x81\x11\x15a/TWa/Ta:\"V[\x03a/rW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a/\x86Wa/\x86a:\"V[\x03a/\xA7W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x1A\xDBV[`\x03\x82`\x03\x81\x11\x15a/\xBBWa/\xBBa:\"V[\x03a(\xB7W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x1A\xDBV[_\x80g\r\xE0\xB6\xB3\xA7d\0\0a/\xF1\x84\x86a7\x17V[a/\xFB\x91\x90a7.V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a0\x11\x85\x80a7\x17V[a0\x1B\x91\x90a7.V[g\r\xE0\xB6\xB3\xA7d\0\0a0.\x87\x80a7\x17V[a08\x91\x90a7.V[a0B\x91\x90a7\x99V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a0W\x82\x84a7\x17V[a\x1FN\x91\x90a7.V[_g\r\xE0\xB6\xB3\xA7d\0\0\x83\x81a0w\x82\x80a7\x17V[a0\x81\x91\x90a7.V[a0\x8B\x91\x90a7\x17V[a0\x95\x91\x90a7.V[g\r\xE0\xB6\xB3\xA7d\0\0\x80a0\xA9\x85\x80a7\x17V[a0\xB3\x91\x90a7.V[a0\xBE\x86`\x03a7\x17V[a0\xC8\x91\x90a7\x17V[a0\xD2\x91\x90a7.V[a\x10\xA6\x91\x90a7\x99V[`\x01`\x01`\xA0\x1B\x03\x83\x16a1\x06W\x80`\x02_\x82\x82Ta0\xFB\x91\x90a7\x99V[\x90\x91UPa1v\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a1XW`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x1A\xDBV[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R` \x81\x90R`@\x90 \x90\x82\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a1\x92W`\x02\x80T\x82\x90\x03\x90Ua1\xB0V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x81\x90R`@\x90 \x80T\x82\x01\x90U[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x83`@Qa1\xF5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[``_a2\x0E\x83a2?V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x0E.W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02\xCF\x80a:7\x839\x01\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14&W__\xFD[__\x83`\x1F\x84\x01\x12a2\x99W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xB0W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a2\xC7W__\xFD[\x92P\x92\x90PV[_____`\x80\x86\x88\x03\x12\x15a2\xE2W__\xFD[\x855\x94P` \x86\x015\x93Pa2\xF9`@\x87\x01a2sV[\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\x14W__\xFD[a3 \x88\x82\x89\x01a2\x89V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x10\xA6` \x83\x01\x84a31V[__`@\x83\x85\x03\x12\x15a3\x82W__\xFD[a3\x8B\x83a2sV[\x94` \x93\x90\x93\x015\x93PPPV[____`\x80\x85\x87\x03\x12\x15a3\xACW__\xFD[a3\xB5\x85a2sV[\x96` \x86\x015\x96P`@\x86\x015\x95``\x015\x94P\x92PPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a3\xFFW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a3\xE1V[P\x93\x94\x93PPPPV[` \x81R_a\x10\xA6` \x83\x01\x84a3\xCFV[_` \x82\x84\x03\x12\x15a4+W__\xFD[a\x10\xA6\x82a2sV[___``\x84\x86\x03\x12\x15a4FW__\xFD[a4O\x84a2sV[\x92Pa4]` \x85\x01a2sV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a4~W__\xFD[P5\x91\x90PV[___``\x84\x86\x03\x12\x15a4\x97W__\xFD[a4\xA0\x84a2sV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_a4\xD3`\xE0\x83\x01\x89a31V[\x82\x81\x03`@\x84\x01Ra4\xE5\x81\x89a31V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x90Pa5\x16\x81\x85a3\xCFV[\x9A\x99PPPPPPPPPPV[__` \x83\x85\x03\x12\x15a55W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5KW__\xFD[a5W\x85\x82\x86\x01a2\x89V[\x90\x96\x90\x95P\x93PPPPV[_______`\xE0\x88\x8A\x03\x12\x15a5yW__\xFD[a5\x82\x88a2sV[\x96Pa5\x90` \x89\x01a2sV[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a5\xB3W__\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a5\xE1W__\xFD[a5\xEA\x83a2sV[\x91Pa5\xF8` \x84\x01a2sV[\x90P\x92P\x92\x90PV[\x80\x15\x15\x81\x14a\x19\x9FW__\xFD[___``\x84\x86\x03\x12\x15a6 W__\xFD[a6)\x84a2sV[\x92Pa67` \x85\x01a2sV[\x91P`@\x84\x015a6G\x81a6\x01V[\x80\x91PP\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a6cW__\xFD[\x825\x91Pa5\xF8` \x84\x01a2sV[_` \x82\x84\x03\x12\x15a6\x83W__\xFD[\x81Qa\x10\xA6\x81a6\x01V[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R\x83`@\x82\x01R`\x80``\x82\x01R\x81`\x80\x82\x01R\x81\x83`\xA0\x83\x017_\x81\x83\x01`\xA0\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a6\xE9W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E.Wa\x0E.a6\xF0V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E.Wa\x0E.a6\xF0V[_\x82a7HWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a7aW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a7\x7FWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0E.Wa\x0E.a6\xF0V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x1F\x82\x11\x15a\x18`W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a7\xE5WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\r\x84W_\x81U`\x01\x01a7\xF1V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a8\x1CWa8\x1Ca7\x85V[a80\x83a8*\x83Ta7MV[\x83a7\xC0V[_`\x1F\x84\x11`\x01\x81\x14a8aW_\x85\x15a8JWP\x83\x82\x015[_\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\r\x84V[_\x83\x81R` \x81 `\x1F\x19\x87\x16\x91[\x82\x81\x10\x15a8\x90W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a8pV[P\x86\x82\x10\x15a8\xACW_\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[n\x02\x9B\xA3\x0B\x13c*\xB1\x91\x02\nji\x01i`\x8D\x1B\x81R_a8\xF8`\x0F\x83\x01\x85a8\xBEV[`/`\xF8\x1B\x81Ra\x1FN`\x01\x82\x01\x85a8\xBEV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9&Wa9&a7\x85V[a9:\x81a94\x84Ta7MV[\x84a7\xC0V[` `\x1F\x82\x11`\x01\x81\x14a9lW_\x83\x15a9UWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\r\x84V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a9\x9BW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a9{V[P\x84\x82\x10\x15a9\xB8W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[fsAMMV2-`\xC8\x1B\x81R_a8\xF8`\x07\x83\x01\x85a8\xBEV[p\x02\xB3{c\x0B\xA3Kc*\xB1\x91\x02\nji\x01i`}\x1B\x81R_a8\xF8`\x11\x83\x01\x85a8\xBEV[fvAMMV2-`\xC8\x1B\x81R_a8\xF8`\x07\x83\x01\x85a8\xBEV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD\xFE`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x02\xCF8\x03\x80a\x02\xCF\x839\x81\x01`@\x81\x90Ra\0.\x91a\0dV[3`\x80R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xA0R\x16`\xC0Ra\0\x95V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0_W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\0uW__\xFD[a\0~\x83a\0IV[\x91Pa\0\x8C` \x84\x01a\0IV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Qa\x02\x13a\0\xBC_9_`\xD8\x01R_`\x9E\x01R_`M\x01Ra\x02\x13_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80cS<\xF5\xCE\x14a\0-W[__\xFD[a\0@a\0;6`\x04a\x01\xC9V[a\0BV[\0[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\x8BW`@Qcoa\xF6A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15a\0\xC5Wa\0\xC5`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a\x01\x04V[\x80\x15a\0\xFFWa\0\xFF`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x83a\x01\x04V[PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x80\x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x82Qa\0\xFF\x93\x87\x93\x90\x92_\x92\x83\x92\x91\x83\x91\x90\x82\x88Z\xF1\x80a\x01rW`@Q=_\x82>=\x81\xFD[PP_Q=\x91P\x81\x15a\x01\x89W\x80`\x01\x14\x15a\x01\x96V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x01\xC3W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[___``\x84\x86\x03\x12\x15a\x01\xDBW__\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xF1W__\xFD[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV\xFE\xA1dsolcC\0\x08\x1C\0\n\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static MOCKPOOL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\xE5W_5`\xE0\x1C\x80c\x89\xAF\xCBD\x11a\x01\x95W\x80c\xC4\x1C\xE1\xB9\x11a\0\xE4W\x80c\xD5\x05\xAC\xCF\x11a\0\x9EW\x80c\xE4\xBB\xB5\xA8\x11a\0yW\x80c\xE4\xBB\xB5\xA8\x14a\x076W\x80c\xEB\xEB1\xDB\x14a\x07IW\x80c\xF1@\xA3Z\x14a\x07QW\x80c\xFF\xF6\xCA\xE9\x14a\x07dW__\xFD[\x80c\xD5\x05\xAC\xCF\x14a\x06\xE2W\x80c\xDDb\xED>\x14a\x06\xF5W\x80c\xE4F>\xB2\x14a\x07-W__\xFD[\x80c\xC4\x1C\xE1\xB9\x14a\x06\x85W\x80c\xC4Z\x01U\x14a\x06\x98W\x80c\xC4\x7F\0'\x14a\x06\xABW\x80c\xC5p\n\x02\x14a\x06\xBEW\x80c\xD2\x12 \xA7\x14a\x06\xC7W\x80c\xD2\x94\xF0\x93\x14a\x06\xDAW__\xFD[\x80c\xA1\xACM\x13\x11a\x01OW\x80c\xBC%\xCFw\x11a\x01*W\x80c\xBC%\xCFw\x14a\x06WW\x80c\xBD\xA3\x9C\xAD\x14a\x06jW\x80c\xBF\x94M\xBC\x14a\x06sW\x80c\xC2E\xFE\xBC\x14a\x06|W__\xFD[\x80c\xA1\xACM\x13\x14a\x06\x12W\x80c\xA9\x05\x9C\xBB\x14a\x061W\x80c\xB8L\x82F\x14a\x06DW__\xFD[\x80c\x89\xAF\xCBD\x14a\x05[W\x80c\x8A{\x8C\xF2\x14a\x05\x83W\x80c\x95\xD8\x9BA\x14a\x05\xADW\x80c\x9Dc\x84\x8A\x14a\x05\xB5W\x80c\x9E\x8C\xC0K\x14a\x05\xE0W\x80c\x9Fv|\x88\x14a\x05\xF3W__\xFD[\x80c1<\xE5g\x11a\x02QW\x80cMZ\x9F\x8A\x11a\x02\x0BW\x80cjbxB\x11a\x01\xE6W\x80cjbxB\x14a\x04\xF2W\x80cp\xA0\x821\x14a\x05\x05W\x80c~\xCE\xBE\0\x14a\x05-W\x80c\x84\xB0\x19n\x14a\x05@W__\xFD[\x80cMZ\x9F\x8A\x14a\x04\xB7W\x80cX\x81\xC4u\x14a\x04\xD6W\x80cZv\xF2^\x14a\x04\xE9W__\xFD[\x80c1<\xE5g\x14a\x04 W\x80c2\xC0\xDE\xFD\x14a\x04/W\x80c3X\tY\x14a\x048W\x80c6D\xE5\x15\x14a\x04KW\x80c9/7\xE9\x14a\x04SW\x80cD<\xB4\xBC\x14a\x04\xAEW__\xFD[\x80c\x18\x16\r\xDD\x11a\x02\xA2W\x80c\x18\x16\r\xDD\x14a\x03\xB4W\x80c\x1D\xF8\xC7\x17\x14a\x03\xC6W\x80c Z\xAB\xF1\x14a\x03\xCEW\x80c\"\xBE=\xE1\x14a\x03\xEDW\x80c#\xB8r\xDD\x14a\x03\xFAW\x80c%,\t\xD7\x14a\x04\rW__\xFD[\x80c\x02,\r\x9F\x14a\x02\xE9W\x80c\x06\xFD\xDE\x03\x14a\x02\xFEW\x80c\t\x02\xF1\xAC\x14a\x03\x1CW\x80c\t^\xA7\xB3\x14a\x03AW\x80c\r\xFE\x16\x81\x14a\x03dW\x80c\x134_\xE1\x14a\x03\x94W[__\xFD[a\x02\xFCa\x02\xF76`\x04a2\xCEV[a\x07lV[\0[a\x03\x06a\r\x8BV[`@Qa\x03\x13\x91\x90a3_V[`@Q\x80\x91\x03\x90\xF3[`\x12T`\x13T`\x14T[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x03\x13V[a\x03Ta\x03O6`\x04a3qV[a\x0E\x1BV[`@Q\x90\x15\x15\x81R` \x01a\x03\x13V[`\x0BTa\x03|\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x13V[a\x03\xA7a\x03\xA26`\x04a3\x99V[a\x0E4V[`@Qa\x03\x13\x91\x90a4\tV[`\x02T[`@Q\x90\x81R` \x01a\x03\x13V[a\x03&a\x10\x1BV[a\x03\xB8a\x03\xDC6`\x04a4\x1BV[`\x1A` R_\x90\x81R`@\x90 T\x81V[`\x0BTa\x03T\x90`\xFF\x16\x81V[a\x03Ta\x04\x086`\x04a44V[a\x10\x88V[a\x03&a\x04\x1B6`\x04a4nV[a\x10\xADV[`@Q`\x12\x81R` \x01a\x03\x13V[a\x03\xB8`\x17T\x81V[`\rTa\x03|\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xB8a\x10\xDEV[`\x10T`\x11T`\x12T`\x13T`\x0BT`\x0CT`@\x80Q\x96\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\xFF\x81\x16\x15\x15`\x80\x84\x01R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16`\xA0\x84\x01R\x16`\xC0\x82\x01R`\xE0\x01a\x03\x13V[a\x03\xB8`\x12T\x81V[a\x03\xB8a\x04\xC56`\x04a4\x1BV[`\x1B` R_\x90\x81R`@\x90 T\x81V[a\x03\xA7a\x04\xE46`\x04a4\x85V[a\x10\xECV[a\x03\xB8`\x13T\x81V[a\x03\xB8a\x05\x006`\x04a4\x1BV[a\x10\xFBV[a\x03\xB8a\x05\x136`\x04a4\x1BV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x03\xB8a\x05;6`\x04a4\x1BV[a\x14+V[a\x05Ha\x14HV[`@Qa\x03\x13\x97\x96\x95\x94\x93\x92\x91\x90a4\xB5V[a\x05na\x05i6`\x04a4\x1BV[a\x14\x8AV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03\x13V[a\x05\x8Ba\x17XV[`@\x80Q\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R\x91\x81\x01Q\x90\x82\x01R``\x01a\x03\x13V[a\x03\x06a\x17\xD2V[`\x0BT`\x0CT`@\x80Qa\x01\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R\x90\x91\x16` \x83\x01R\x01a\x03\x13V[a\x03\xB8a\x05\xEE6`\x04a4\x85V[a\x17\xE1V[a\x03\xB8a\x06\x016`\x04a4\x1BV[`\x19` R_\x90\x81R`@\x90 T\x81V[a\x03\xB8a\x06 6`\x04a4\x1BV[`\x1C` R_\x90\x81R`@\x90 T\x81V[a\x03Ta\x06?6`\x04a3qV[a\x18FV[a\x02\xFCa\x06R6`\x04a5$V[a\x18SV[a\x02\xFCa\x06e6`\x04a4\x1BV[a\x18eV[a\x03\xB8`\x18T\x81V[a\x03\xB8`\x15T\x81V[a\x03\xB8`\x16T\x81V[a\x02\xFCa\x06\x936`\x04a4nV[a\x19\xA2V[`\x0ETa\x03|\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xFCa\x06\xB96`\x04a5$V[a\x19\xADV[a\x03\xB8`\x14T\x81V[`\x0CTa\x03|\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05na\x19\xBAV[a\x02\xFCa\x06\xF06`\x04a5cV[a\x1A\xBBV[a\x03\xB8a\x07\x036`\x04a5\xD0V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x03\xB8a\x07\x08\x81V[a\x02\xFCa\x07D6`\x04a6\x0EV[a\x1B\xF6V[`\x0FTa\x03\xB8V[a\x03\xB8a\x07_6`\x04a6RV[a\x1E\x9BV[a\x02\xFCa\x1FWV[a\x07ta sV[`\x0E_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xB1\x87\xBD&`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xC4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE8\x91\x90a6sV[\x15a\x08\x06W`@Qc\x13\t\xA5c`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x15\x80\x15a\x08\x12WP\x83\x15[\x15a\x080W`@QcB0\x1C#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x12T`\x13T\x81\x87\x10\x15\x80a\x08EWP\x80\x86\x10\x15[\x15a\x08cW`@Qc\xBBU\xFD'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT`\x0CT_\x91\x82\x91`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x92\x04\x82\x16\x91\x90\x81\x16\x90\x89\x16\x82\x14\x80a\x08\xA2WP\x80`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x08\xC0W`@Qc\x05!\xF41`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x8A\x15a\x08\xDAWa\x08\xDA`\x01`\x01`\xA0\x1B\x03\x83\x16\x8A\x8Da \x9DV[\x89\x15a\x08\xF4Wa\x08\xF4`\x01`\x01`\xA0\x1B\x03\x82\x16\x8A\x8Ca \x9DV[\x86\x15a\t\\W`@Qc\x9A{\xFFy`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\x9A{\xFFy\x90a\t.\x903\x90\x8F\x90\x8F\x90\x8E\x90\x8E\x90`\x04\x01a6\x8EV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\tEW__\xFD[PZ\xF1\x15\x80\x15a\tWW=__>=_\xFD[PPPP[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xC2\x91\x90a6\xD9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x94P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n+\x91\x90a6\xD9V[\x92PPP_\x89\x85a\n<\x91\x90a7\x04V[\x83\x11a\nHW_a\n\\V[a\nR\x8A\x86a7\x04V[a\n\\\x90\x84a7\x04V[\x90P_a\ni\x8A\x86a7\x04V[\x83\x11a\nuW_a\n\x89V[a\n\x7F\x8A\x86a7\x04V[a\n\x89\x90\x84a7\x04V[\x90P\x81\x15\x80\x15a\n\x97WP\x80\x15[\x15a\n\xB5W`@Qc\t\x8F\xB5a`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT`\x0CT`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x92\x04\x82\x16\x91\x16\x83\x15a\x0BmW`\x0ET`\x0BT`@Qc\xCCV\xB2\xC5`\xE0\x1B\x81R0`\x04\x82\x01R`\xFF\x90\x91\x16\x15\x15`$\x82\x01Ra\x0Bm\x91a'\x10\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCCV\xB2\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BT\x91\x90a6\xD9V[a\x0B^\x90\x87a7\x17V[a\x0Bh\x91\x90a7.V[a \xEFV[\x82\x15a\x0C\rW`\x0ET`\x0BT`@Qc\xCCV\xB2\xC5`\xE0\x1B\x81R0`\x04\x82\x01R`\xFF\x90\x91\x16\x15\x15`$\x82\x01Ra\x0C\r\x91a'\x10\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCCV\xB2\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xD0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF4\x91\x90a6\xD9V[a\x0B\xFE\x90\x86a7\x17V[a\x0C\x08\x91\x90a7.V[a!\x9FV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0COW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cs\x91\x90a6\xD9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x96P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xB8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xDC\x91\x90a6\xD9V[\x94Pa\x0C\xE8\x88\x88a\"CV[a\x0C\xF2\x87\x87a\"CV[\x10\x15a\r\x11W`@Qc\xA92I/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa\r\x1F\x84\x84\x88\x88a#7V[`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x90\x81\x01\x8C\x90R``\x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x903\x90\x7F\xB3\xE2w6\x06\xAB\xFD6\xB5\xBD\x919K:T\xD19\x836\xC6P\x05\xBA\xF7\xBFz\x05\xEF\xEF\xFA\xF7[\x90`\x80\x01`@Q\x80\x91\x03\x90\xA3PPPPPPa\r\x84`\x01`\x08UV[PPPPPV[```\t\x80Ta\r\x9A\x90a7MV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xC6\x90a7MV[\x80\x15a\x0E\x11W\x80`\x1F\x10a\r\xE8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\x11V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xF4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_3a\x0E(\x81\x85\x85a$\xC5V[`\x01\x91PP[\x92\x91PPV[``_\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EPWa\x0EPa7\x85V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0EyW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x0FT\x90\x91P_\x90a\x0E\x8E\x90`\x01\x90a7\x04V[\x90P_a\x0E\x9B\x85\x87a7\x17V[a\x0E\xA5\x90\x83a7\x04V[\x90P_\x80[\x83\x83\x10\x15a\x10\x0BWa\x0E\xBC\x87\x84a7\x99V[\x91P_`\x0F\x84\x81T\x81\x10a\x0E\xD2Wa\x0E\xD2a7\xACV[\x90_R` _ \x90`\x03\x02\x01_\x01T`\x0F\x84\x81T\x81\x10a\x0E\xF4Wa\x0E\xF4a7\xACV[\x90_R` _ \x90`\x03\x02\x01_\x01Ta\x0F\r\x91\x90a7\x04V[\x90P_\x81`\x0F\x86\x81T\x81\x10a\x0F$Wa\x0F$a7\xACV[\x90_R` _ \x90`\x03\x02\x01`\x01\x01T`\x0F\x86\x81T\x81\x10a\x0FGWa\x0FGa7\xACV[\x90_R` _ \x90`\x03\x02\x01`\x01\x01Ta\x0Fa\x91\x90a7\x04V[a\x0Fk\x91\x90a7.V[\x90P_\x82`\x0F\x87\x81T\x81\x10a\x0F\x82Wa\x0F\x82a7\xACV[\x90_R` _ \x90`\x03\x02\x01`\x02\x01T`\x0F\x87\x81T\x81\x10a\x0F\xA5Wa\x0F\xA5a7\xACV[\x90_R` _ \x90`\x03\x02\x01`\x02\x01Ta\x0F\xBF\x91\x90a7\x04V[a\x0F\xC9\x91\x90a7.V[\x90Pa\x0F\xD7\x8C\x8E\x84\x84a$\xD2V[\x88\x85\x81Q\x81\x10a\x0F\xE9Wa\x0F\xE9a7\xACV[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a\x10\x04\x87\x84a7\x99V[\x92Pa\x0E\xAAV[P\x92\x93PPPP[\x94\x93PPPPV[`\x15T`\x16TB_\x80\x80a\x108`\x12T`\x13T`\x14T\x91\x92\x90\x91\x90V[\x92P\x92P\x92P\x83\x81\x14a\x10\x80W_a\x10P\x82\x86a7\x04V[\x90Pa\x10\\\x81\x85a7\x17V[a\x10f\x90\x88a7\x99V[\x96Pa\x10r\x81\x84a7\x17V[a\x10|\x90\x87a7\x99V[\x95PP[PPP\x90\x91\x92V[_3a\x10\x95\x85\x82\x85a&\x82V[a\x10\xA0\x85\x85\x85a&\xFDV[`\x01\x91PP[\x93\x92PPPV[`\x0F\x81\x81T\x81\x10a\x10\xBCW_\x80\xFD[_\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x92P\x83V[_a\x10\xE7a'ZV[\x90P\x90V[``a\x10\x13\x84\x84\x84`\x01a\x0E4V[_a\x11\x04a sV[`\x12T`\x13T`\x0BT`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x91a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11UW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11y\x91\x90a6\xD9V[`\x0CT`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92P_\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xC4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xE8\x91\x90a6\xD9V[\x90P_a\x11\xF5\x85\x84a7\x04V[\x90P_a\x12\x02\x85\x84a7\x04V[\x90P_a\x12\x0E`\x02T\x90V[\x90P\x80_\x03a\x13fWa\x03\xE8a\x12\xB3a\x12'\x84\x86a7\x17V[p\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Ch\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cd\xFF\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Cb\xFF\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1Cb\x01\0\0\x01`\xB5`\x01\x92\x83\x1C\x1B\x02`\x12\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[a\x12\xBD\x91\x90a7\x04V[\x97Pa\x12\xCC`\x01a\x03\xE8a(\x83V[`\x0BT`\xFF\x16\x15a\x13aW`\x11Ta\x12\xEC\x83g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a\x12\xF6\x91\x90a7.V[`\x10Ta\x13\x0B\x85g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a\x13\x15\x91\x90a7.V[\x14a\x133W`@Qc\x05\x02k\xFD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[d\x02T\x0B\xE4\0a\x13C\x84\x84a\"CV[\x11a\x13aW`@Qc!\xC6\x9Do`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13\xA1V[a\x13\x9E\x87a\x13t\x83\x86a7\x17V[a\x13~\x91\x90a7.V[\x87a\x13\x89\x84\x86a7\x17V[a\x13\x93\x91\x90a7.V[\x80\x82\x18\x90\x82\x11\x02\x18\x90V[\x97P[a\x03\xE8\x88\x10\x15a\x13\xC4W`@Qc4\x89\xBEu`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13\xCE\x89\x89a(\x83V[a\x13\xDA\x85\x85\x89\x89a#7V[`@\x80Q\x84\x81R` \x81\x01\x84\x90R3\x91\x7FL \x9B_\xC8\xADPu\x8F\x13\xE2\xE1\x08\x8B\xA5jV\r\xFFi\n\x1Co\xEF&9OL\x03\x82\x1CO\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPa\x14&`\x01`\x08UV[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x07` R`@\x81 Ta\x0E.V[_``\x80___``a\x14Ya(\xBBV[a\x14aa(\xE8V[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[__a\x14\x94a sV[`\x12T`\x13T`\x0BT`\x0CT`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x91\x16\x90_\x90\x83\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x14\x91\x90a6\xD9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15[W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x7F\x91\x90a6\xD9V[0_\x90\x81R` \x81\x90R`@\x90 T`\x02T\x91\x92P\x90\x80a\x15\xA0\x85\x84a7\x17V[a\x15\xAA\x91\x90a7.V[\x99P\x80a\x15\xB7\x84\x84a7\x17V[a\x15\xC1\x91\x90a7.V[\x98P\x89\x15\x80a\x15\xCEWP\x88\x15[\x15a\x15\xECW`@Qct\x93\x83\xAD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15\xF60\x83a)\x15V[a\x16\n`\x01`\x01`\xA0\x1B\x03\x87\x16\x8C\x8Ca \x9DV[a\x16\x1E`\x01`\x01`\xA0\x1B\x03\x86\x16\x8C\x8Ba \x9DV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16`W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x84\x91\x90a6\xD9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x94P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xED\x91\x90a6\xD9V[\x92Pa\x16\xFB\x84\x84\x8A\x8Aa#7V[`@\x80Q\x8B\x81R` \x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x8D\x16\x913\x91\x7F]bJ\xA9\xC1H\x15:\xB3Dl\x1B\x15Of\x0E\xE7p\x1ET\x9F\xE9\xB6-\xABqq\xB1\xC8\x0Eo\xA2\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPPa\x17S`\x01`\x08UV[\x91P\x91V[a\x17y`@Q\x80``\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`\x0F\x80Ta\x17\x89\x90`\x01\x90a7\x04V[\x81T\x81\x10a\x17\x99Wa\x17\x99a7\xACV[\x90_R` _ \x90`\x03\x02\x01`@Q\x80``\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPP\x90P\x90V[```\n\x80Ta\r\x9A\x90a7MV[__a\x17\xF0\x85\x85\x85`\x01a\x0E4V[\x80Q\x90\x91P_\x90\x81[\x81\x81\x10\x15a\x180W\x83\x81\x81Q\x81\x10a\x18\x13Wa\x18\x13a7\xACV[` \x02` \x01\x01Q\x83a\x18&\x91\x90a7\x99V[\x92P`\x01\x01a\x17\xF9V[Pa\x18;\x85\x83a7.V[\x97\x96PPPPPPPV[_3a\x0E(\x81\x85\x85a&\xFDV[`\na\x18`\x82\x84\x83a8\x04V[PPPV[a\x18ma sV[`\x0BT`\x0CT`\x12T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x01\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x92\x16\x91a\x19\t\x91\x85\x91\x85\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xCAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xEE\x91\x90a6\xD9V[a\x18\xF8\x91\x90a7\x04V[`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x90a \x9DV[`\x13T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x19\x93\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19x\x91\x90a6\xD9V[a\x19\x82\x91\x90a7\x04V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x90a \x9DV[PPa\x19\x9F`\x01`\x08UV[PV[a\x19\x9F`\x01\x82a(\x83V[`\ta\x18`\x82\x84\x83a8\x04V[__a\x19\xC53a)IV[PP3_\x90\x81R`\x1B` \x90\x81R`@\x80\x83 T`\x1C\x90\x92R\x90\x91 T\x81\x15\x15\x80a\x19\xEFWP_\x81\x11[\x15a\x1A\xB7W3_\x81\x81R`\x1B` \x90\x81R`@\x80\x83 \x83\x90U`\x1C\x90\x91R\x80\x82 \x91\x90\x91U`\rT\x90Qc)\x9Ez\xE7`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01\x84\x90R`D\x82\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x16\x90cS<\xF5\xCE\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1AcW__\xFD[PZ\xF1\x15\x80\x15a\x1AuW=__>=_\xFD[PP`@\x80Q\x85\x81R` \x81\x01\x85\x90R3\x93P\x83\x92P\x7F\x86\\\xA0\x8DY\xF5\xCBEn\x85\xCD/~\xF66d\xEAOs2t\x14\xE9\xD8\x15,AX\xB0\xE9FE\x91\x01`@Q\x80\x91\x03\x90\xA3[\x90\x91V[\x83B\x11\x15a\x1A\xE4W`@Qc1<\x89\x81`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[_\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a\x1B/\x8C`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x07` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x90V[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_a\x1B\x89\x82a*\xA1V[\x90P_a\x1B\x98\x82\x87\x87\x87a*\xCDV[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xDFW`@Qc%\xC0\x07#`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x8B\x16`$\x82\x01R`D\x01a\x1A\xDBV[a\x1B\xEA\x8A\x8A\x8Aa$\xC5V[PPPPPPPPPPV[`\x0ET`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1C W`@Qc\x02\xA9\x8A7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x163\x17\x90\x91U`\x0B\x80T`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x90\x95\x16\x17\x90U\x91\x85\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x84\x15\x15\x16`\x01`\x01`\xA8\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x91\x90\x91\x17\x90U`@Q\x83\x90\x83\x90a\x1C\x8A\x90a2fV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x1C\xBAW=__>=_\xFD[P`\r\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q\x80\x82\x01\x82R`\x04\x80\x82RcUSDC`\xE0\x1B` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x90\x94R\x90\x83RcAERO`\xE0\x1B\x90\x83\x01R\x90\x82\x15a\x1D\x7FW\x81\x81`@Q` \x01a\x1D+\x92\x91\x90a8\xD5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\t\x90\x81a\x1DH\x91\x90a9\x0CV[P\x81\x81`@Q` \x01a\x1D\\\x92\x91\x90a9\xC7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\n\x90\x81a\x1Dy\x91\x90a9\x0CV[Pa\x1D\xE2V[\x81\x81`@Q` \x01a\x1D\x92\x92\x91\x90a9\xE2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\t\x90\x81a\x1D\xAF\x91\x90a9\x0CV[P\x81\x81`@Q` \x01a\x1D\xC3\x92\x91\x90a:\x07V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\n\x90\x81a\x1D\xE0\x91\x90a9\x0CV[P[PPb\x0FB@`\x10UPPg\r\xE0\xB6\xB3\xA7d\0\0`\x11UP`@\x80Q``\x81\x01\x82RB\x81R_` \x82\x01\x81\x81R\x92\x82\x01\x81\x81R`\x0F\x80T`\x01\x81\x01\x82U\x92R\x91Q\x7F\x8D\x11\x08\xE1\x0B\xCB|'\xDD\xDF\xC0.\xD9\xD6\x93\xA0t\x03\x9D\x02l\xF4\xEAB@\xB4\x0F}X\x1A\xC8\x02`\x03\x90\x92\x02\x91\x82\x01U\x91Q\x7F\x8D\x11\x08\xE1\x0B\xCB|'\xDD\xDF\xC0.\xD9\xD6\x93\xA0t\x03\x9D\x02l\xF4\xEAB@\xB4\x0F}X\x1A\xC8\x03\x83\x01UQ\x7F\x8D\x11\x08\xE1\x0B\xCB|'\xDD\xDF\xC0.\xD9\xD6\x93\xA0t\x03\x9D\x02l\xF4\xEAB@\xB4\x0F}X\x1A\xC8\x04\x90\x91\x01UV[`\x12T`\x13T`\x0ET`\x0BT`@Qc\xCCV\xB2\xC5`\xE0\x1B\x81R0`\x04\x82\x01R`\xFF\x90\x91\x16\x15\x15`$\x82\x01R_\x93\x92\x91a'\x10\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xCCV\xB2\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\"\x91\x90a6\xD9V[a\x1F,\x90\x87a7\x17V[a\x1F6\x91\x90a7.V[a\x1F@\x90\x86a7\x04V[\x94Pa\x1FN\x85\x85\x84\x84a$\xD2V[\x95\x94PPPPPV[a\x1F_a sV[`\x02T_\x03a\x1F\x81W`@Qc\xBBU\xFD'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra g\x91a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xF2\x91\x90a6\xD9V[`\x0CT`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a 8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \\\x91\x90a6\xD9V[`\x12T`\x13Ta#7V[a q`\x01`\x08UV[V[`\x02`\x08T\x03a \x96W`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x08UV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x18`\x90\x84\x90a*\xF9V[\x80_\x03a \xF9WPV[`\rT`\x0BTa!\x1B\x91`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x92\x04\x82\x16\x91\x16\x83a \x9DV[_a!%`\x02T\x90V[a!7\x83g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a!A\x91\x90a7.V[\x90P\x80\x15a!`W\x80`\x17_\x82\x82Ta!Z\x91\x90a7\x99V[\x90\x91UPP[`@\x80Q\x83\x81R_` \x82\x01R3\x91\x7F\x11,%i\x02\xBFUKn\xD8\x82\xD2\x93f\x87\xAA\xEBB%\xE8\xCD[Q0<\x90\xCAl\xF4:\x86\x02\x91\x01[`@Q\x80\x91\x03\x90\xA2PPV[\x80_\x03a!\xA9WPV[`\rT`\x0CTa!\xC6\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x83a \x9DV[_a!\xD0`\x02T\x90V[a!\xE2\x83g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a!\xEC\x91\x90a7.V[\x90P\x80\x15a\"\x0BW\x80`\x18_\x82\x82Ta\"\x05\x91\x90a7\x99V[\x90\x91UPP[`@\x80Q_\x81R` \x81\x01\x84\x90R3\x91\x7F\x11,%i\x02\xBFUKn\xD8\x82\xD2\x93f\x87\xAA\xEBB%\xE8\xCD[Q0<\x90\xCAl\xF4:\x86\x02\x91\x01a!\x93V[`\x0BT_\x90`\xFF\x16\x15a#&W`\x10T_\x90a\"g\x85g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a\"q\x91\x90a7.V[\x90P_`\x11T\x84g\r\xE0\xB6\xB3\xA7d\0\0a\"\x8B\x91\x90a7\x17V[a\"\x95\x91\x90a7.V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a\"\xAB\x83\x85a7\x17V[a\"\xB5\x91\x90a7.V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a\"\xCB\x84\x80a7\x17V[a\"\xD5\x91\x90a7.V[g\r\xE0\xB6\xB3\xA7d\0\0a\"\xE8\x86\x80a7\x17V[a\"\xF2\x91\x90a7.V[a\"\xFC\x91\x90a7\x99V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a#\x11\x82\x84a7\x17V[a#\x1B\x91\x90a7.V[\x94PPPPPa\x0E.V[a#0\x82\x84a7\x17V[\x90Pa\x0E.V[`\x14TB\x90_\x90a#H\x90\x83a7\x04V[\x90P_\x81\x11\x80\x15a#XWP\x83\x15\x15[\x80\x15a#cWP\x82\x15\x15[\x15a#\xA8Wa#r\x81\x85a7\x17V[`\x15_\x82\x82Ta#\x82\x91\x90a7\x99V[\x90\x91UPa#\x92\x90P\x81\x84a7\x17V[`\x16_\x82\x82Ta#\xA2\x91\x90a7\x99V[\x90\x91UPP[_a#\xB1a\x17XV[\x80Q\x90\x91Pa#\xC0\x90\x84a7\x04V[\x91Pa\x07\x08\x82\x11\x15a$tW`@\x80Q``\x81\x01\x82R\x84\x81R`\x15T` \x82\x01\x90\x81R`\x16T\x92\x82\x01\x92\x83R`\x0F\x80T`\x01\x81\x01\x82U_\x91\x90\x91R\x91Q\x7F\x8D\x11\x08\xE1\x0B\xCB|'\xDD\xDF\xC0.\xD9\xD6\x93\xA0t\x03\x9D\x02l\xF4\xEAB@\xB4\x0F}X\x1A\xC8\x02`\x03\x90\x93\x02\x92\x83\x01UQ\x7F\x8D\x11\x08\xE1\x0B\xCB|'\xDD\xDF\xC0.\xD9\xD6\x93\xA0t\x03\x9D\x02l\xF4\xEAB@\xB4\x0F}X\x1A\xC8\x03\x82\x01U\x90Q\x7F\x8D\x11\x08\xE1\x0B\xCB|'\xDD\xDF\xC0.\xD9\xD6\x93\xA0t\x03\x9D\x02l\xF4\xEAB@\xB4\x0F}X\x1A\xC8\x04\x90\x91\x01U[`\x12\x87\x90U`\x13\x86\x90U`\x14\x83\x90U`@\x80Q\x88\x81R` \x81\x01\x88\x90R\x7F\xCF*\xA5\x08v\xCD\xFB\xB5A o\x89\xAF\x0E\xE7\x8DD\xA2\xAB\xF8\xD3(\xE3\x7F\xA4\x91\x7F\x98!I\x84\x8A\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPPPPV[a\x18`\x83\x83\x83`\x01a+eV[`\x0BT_\x90`\xFF\x16\x15a&.W_a$\xEA\x84\x84a\"CV[`\x10T\x90\x91Pa%\x02\x85g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a%\x0C\x91\x90a7.V[`\x11T\x90\x94Pa%$\x84g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a%.\x91\x90a7.V[`\x0BT\x90\x93P_\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x88\x81\x16a\x01\0\x90\x92\x04\x16\x14a%VW\x84\x86a%YV[\x85\x85[`\x0BT\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x88\x81\x16a\x01\0\x90\x92\x04\x16\x14a%\x9CW`\x11Ta%\x8D\x89g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a%\x97\x91\x90a7.V[a%\xBBV[`\x10Ta%\xB1\x89g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a%\xBB\x91\x90a7.V[\x97P_a%\xD2a%\xCB\x84\x8Ba7\x99V[\x85\x84a,7V[a%\xDC\x90\x83a7\x04V[`\x0BT\x90\x91Pg\r\xE0\xB6\xB3\xA7d\0\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16a\x01\0\x90\x92\x04\x16\x14a&\x0BW`\x10Ta&\x0FV[`\x11T[a&\x19\x90\x83a7\x17V[a&#\x91\x90a7.V[\x94PPPPPa\x10\x13V[`\x0BT_\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x87\x81\x16a\x01\0\x90\x92\x04\x16\x14a&SW\x83\x85a&VV[\x84\x84[\x90\x92P\x90Pa&e\x87\x83a7\x99V[a&o\x82\x89a7\x17V[a&y\x91\x90a7.V[\x92PPPa\x10\x13V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T_\x19\x81\x14a&\xF7W\x81\x81\x10\x15a&\xE9W`@Qc}\xC7\xA0\xD9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x1A\xDBV[a&\xF7\x84\x84\x84\x84\x03_a+eV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a'&W`@QcKc~\x8F`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x1A\xDBV[`\x01`\x01`\xA0\x1B\x03\x82\x16a'OW`@Qc\xECD/\x05`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x1A\xDBV[a\x18`\x83\x83\x83a-\x9DV[_0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a'\xB2WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a'\xDCWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x10\xE7`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R_\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16a(\xACW`@Qc\xECD/\x05`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x1A\xDBV[a(\xB7_\x83\x83a-\x9DV[PPV[``a\x10\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x05a-\xBAV[``a\x10\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x06a-\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16a)>W`@QcKc~\x8F`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x1A\xDBV[a(\xB7\x82_\x83a-\x9DV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R` \x81\x90R`@\x90 T\x80\x15a*pW`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x19` \x90\x81R`@\x80\x83 \x80T`\x1A\x80\x85R\x92\x85 \x80T`\x17T`\x18T\x94\x81\x90U\x94\x90\x95R\x82\x90U\x93a)\xA9\x85\x84a7\x04V[\x90P_a)\xB6\x85\x84a7\x04V[\x90P\x81\x15a*\x0FW_g\r\xE0\xB6\xB3\xA7d\0\0a)\xD2\x84\x8Aa7\x17V[a)\xDC\x91\x90a7.V[`\x01`\x01`\xA0\x1B\x03\x8A\x16_\x90\x81R`\x1B` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a*\x08\x90\x84\x90a7\x99V[\x90\x91UPPP[\x80\x15a*fW_g\r\xE0\xB6\xB3\xA7d\0\0a*)\x83\x8Aa7\x17V[a*3\x91\x90a7.V[`\x01`\x01`\xA0\x1B\x03\x8A\x16_\x90\x81R`\x1C` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a*_\x90\x84\x90a7\x99V[\x90\x91UPPP[PPPPPPPPV[`\x17T`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x19` \x90\x81R`@\x80\x83 \x93\x90\x93U`\x18T`\x1A\x90\x91R\x91\x90 UPPV[_a\x0E.a*\xADa'ZV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[____a*\xDD\x88\x88\x88\x88a.\\V[\x92P\x92P\x92Pa*\xED\x82\x82a/$V[P\x90\x96\x95PPPPPPV[__` _\x84Q` \x86\x01_\x88Z\xF1\x80a+\x18W`@Q=_\x82>=\x81\xFD[PP_Q=\x91P\x81\x15a+/W\x80`\x01\x14\x15a+<V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a&\xF7W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x1A\xDBV[`\x01`\x01`\xA0\x1B\x03\x84\x16a+\x8EW`@Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x1A\xDBV[`\x01`\x01`\xA0\x1B\x03\x83\x16a+\xB7W`@QcJ\x14\x06\xB1`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x1A\xDBV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x82\x90U\x80\x15a&\xF7W\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa,)\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPV[_\x80[`\xFF\x81\x10\x15a-oW_a,N\x86\x85a/\xDCV[\x90P\x84\x81\x10\x15a,\xEAW_a,c\x87\x86a0aV[a,m\x83\x88a7\x04V[a,\x7F\x90g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a,\x89\x91\x90a7.V[\x90P\x80_\x03a,\xD8W\x85\x82\x03a,\xA4W\x84\x93PPPPa\x10\xA6V[\x85a,\xB9\x88a,\xB4\x88`\x01a7\x99V[a\"CV[\x11\x15a,\xD4Wa,\xCA\x85`\x01a7\x99V[\x93PPPPa\x10\xA6V[P`\x01[a,\xE2\x81\x86a7\x99V[\x94PPa-fV[_a,\xF5\x87\x86a0aV[a,\xFF\x87\x84a7\x04V[a-\x11\x90g\r\xE0\xB6\xB3\xA7d\0\0a7\x17V[a-\x1B\x91\x90a7.V[\x90P\x80_\x03a-XW\x85\x82\x14\x80a-DWP\x85a-B\x88a-=`\x01\x89a7\x04V[a/\xDCV[\x10[\x15a-TW\x84\x93PPPPa\x10\xA6V[P`\x01[a-b\x81\x86a7\x04V[\x94PP[P`\x01\x01a,:V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra!y`\xF0\x1B`D\x82\x01R`d\x01a\x1A\xDBV[a-\xA6\x83a)IV[a-\xAF\x82a)IV[a\x18`\x83\x83\x83a0\xDCV[```\xFF\x83\x14a-\xCDWa#0\x83a2\x02V[\x81\x80Ta-\xD9\x90a7MV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta.\x05\x90a7MV[\x80\x15a.PW\x80`\x1F\x10a.'Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a.PV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a.3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x0E.V[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a.\x95WP_\x91P`\x03\x90P\x82a/\x1AV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a.\xE6W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a/\x11WP_\x92P`\x01\x91P\x82\x90Pa/\x1AV[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[_\x82`\x03\x81\x11\x15a/7Wa/7a:\"V[\x03a/@WPPV[`\x01\x82`\x03\x81\x11\x15a/TWa/Ta:\"V[\x03a/rW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a/\x86Wa/\x86a:\"V[\x03a/\xA7W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x1A\xDBV[`\x03\x82`\x03\x81\x11\x15a/\xBBWa/\xBBa:\"V[\x03a(\xB7W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x1A\xDBV[_\x80g\r\xE0\xB6\xB3\xA7d\0\0a/\xF1\x84\x86a7\x17V[a/\xFB\x91\x90a7.V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a0\x11\x85\x80a7\x17V[a0\x1B\x91\x90a7.V[g\r\xE0\xB6\xB3\xA7d\0\0a0.\x87\x80a7\x17V[a08\x91\x90a7.V[a0B\x91\x90a7\x99V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a0W\x82\x84a7\x17V[a\x1FN\x91\x90a7.V[_g\r\xE0\xB6\xB3\xA7d\0\0\x83\x81a0w\x82\x80a7\x17V[a0\x81\x91\x90a7.V[a0\x8B\x91\x90a7\x17V[a0\x95\x91\x90a7.V[g\r\xE0\xB6\xB3\xA7d\0\0\x80a0\xA9\x85\x80a7\x17V[a0\xB3\x91\x90a7.V[a0\xBE\x86`\x03a7\x17V[a0\xC8\x91\x90a7\x17V[a0\xD2\x91\x90a7.V[a\x10\xA6\x91\x90a7\x99V[`\x01`\x01`\xA0\x1B\x03\x83\x16a1\x06W\x80`\x02_\x82\x82Ta0\xFB\x91\x90a7\x99V[\x90\x91UPa1v\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a1XW`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x1A\xDBV[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R` \x81\x90R`@\x90 \x90\x82\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a1\x92W`\x02\x80T\x82\x90\x03\x90Ua1\xB0V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x81\x90R`@\x90 \x80T\x82\x01\x90U[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x83`@Qa1\xF5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[``_a2\x0E\x83a2?V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x0E.W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02\xCF\x80a:7\x839\x01\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14&W__\xFD[__\x83`\x1F\x84\x01\x12a2\x99W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xB0W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a2\xC7W__\xFD[\x92P\x92\x90PV[_____`\x80\x86\x88\x03\x12\x15a2\xE2W__\xFD[\x855\x94P` \x86\x015\x93Pa2\xF9`@\x87\x01a2sV[\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\x14W__\xFD[a3 \x88\x82\x89\x01a2\x89V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x10\xA6` \x83\x01\x84a31V[__`@\x83\x85\x03\x12\x15a3\x82W__\xFD[a3\x8B\x83a2sV[\x94` \x93\x90\x93\x015\x93PPPV[____`\x80\x85\x87\x03\x12\x15a3\xACW__\xFD[a3\xB5\x85a2sV[\x96` \x86\x015\x96P`@\x86\x015\x95``\x015\x94P\x92PPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a3\xFFW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a3\xE1V[P\x93\x94\x93PPPPV[` \x81R_a\x10\xA6` \x83\x01\x84a3\xCFV[_` \x82\x84\x03\x12\x15a4+W__\xFD[a\x10\xA6\x82a2sV[___``\x84\x86\x03\x12\x15a4FW__\xFD[a4O\x84a2sV[\x92Pa4]` \x85\x01a2sV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a4~W__\xFD[P5\x91\x90PV[___``\x84\x86\x03\x12\x15a4\x97W__\xFD[a4\xA0\x84a2sV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_a4\xD3`\xE0\x83\x01\x89a31V[\x82\x81\x03`@\x84\x01Ra4\xE5\x81\x89a31V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x90Pa5\x16\x81\x85a3\xCFV[\x9A\x99PPPPPPPPPPV[__` \x83\x85\x03\x12\x15a55W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5KW__\xFD[a5W\x85\x82\x86\x01a2\x89V[\x90\x96\x90\x95P\x93PPPPV[_______`\xE0\x88\x8A\x03\x12\x15a5yW__\xFD[a5\x82\x88a2sV[\x96Pa5\x90` \x89\x01a2sV[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a5\xB3W__\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a5\xE1W__\xFD[a5\xEA\x83a2sV[\x91Pa5\xF8` \x84\x01a2sV[\x90P\x92P\x92\x90PV[\x80\x15\x15\x81\x14a\x19\x9FW__\xFD[___``\x84\x86\x03\x12\x15a6 W__\xFD[a6)\x84a2sV[\x92Pa67` \x85\x01a2sV[\x91P`@\x84\x015a6G\x81a6\x01V[\x80\x91PP\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a6cW__\xFD[\x825\x91Pa5\xF8` \x84\x01a2sV[_` \x82\x84\x03\x12\x15a6\x83W__\xFD[\x81Qa\x10\xA6\x81a6\x01V[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R\x83`@\x82\x01R`\x80``\x82\x01R\x81`\x80\x82\x01R\x81\x83`\xA0\x83\x017_\x81\x83\x01`\xA0\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a6\xE9W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E.Wa\x0E.a6\xF0V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E.Wa\x0E.a6\xF0V[_\x82a7HWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a7aW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a7\x7FWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0E.Wa\x0E.a6\xF0V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x1F\x82\x11\x15a\x18`W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a7\xE5WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\r\x84W_\x81U`\x01\x01a7\xF1V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a8\x1CWa8\x1Ca7\x85V[a80\x83a8*\x83Ta7MV[\x83a7\xC0V[_`\x1F\x84\x11`\x01\x81\x14a8aW_\x85\x15a8JWP\x83\x82\x015[_\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\r\x84V[_\x83\x81R` \x81 `\x1F\x19\x87\x16\x91[\x82\x81\x10\x15a8\x90W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a8pV[P\x86\x82\x10\x15a8\xACW_\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[n\x02\x9B\xA3\x0B\x13c*\xB1\x91\x02\nji\x01i`\x8D\x1B\x81R_a8\xF8`\x0F\x83\x01\x85a8\xBEV[`/`\xF8\x1B\x81Ra\x1FN`\x01\x82\x01\x85a8\xBEV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9&Wa9&a7\x85V[a9:\x81a94\x84Ta7MV[\x84a7\xC0V[` `\x1F\x82\x11`\x01\x81\x14a9lW_\x83\x15a9UWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\r\x84V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a9\x9BW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a9{V[P\x84\x82\x10\x15a9\xB8W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[fsAMMV2-`\xC8\x1B\x81R_a8\xF8`\x07\x83\x01\x85a8\xBEV[p\x02\xB3{c\x0B\xA3Kc*\xB1\x91\x02\nji\x01i`}\x1B\x81R_a8\xF8`\x11\x83\x01\x85a8\xBEV[fvAMMV2-`\xC8\x1B\x81R_a8\xF8`\x07\x83\x01\x85a8\xBEV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD\xFE`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x02\xCF8\x03\x80a\x02\xCF\x839\x81\x01`@\x81\x90Ra\0.\x91a\0dV[3`\x80R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xA0R\x16`\xC0Ra\0\x95V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0_W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\0uW__\xFD[a\0~\x83a\0IV[\x91Pa\0\x8C` \x84\x01a\0IV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Qa\x02\x13a\0\xBC_9_`\xD8\x01R_`\x9E\x01R_`M\x01Ra\x02\x13_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80cS<\xF5\xCE\x14a\0-W[__\xFD[a\0@a\0;6`\x04a\x01\xC9V[a\0BV[\0[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\x8BW`@Qcoa\xF6A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15a\0\xC5Wa\0\xC5`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a\x01\x04V[\x80\x15a\0\xFFWa\0\xFF`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x83a\x01\x04V[PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x80\x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x82Qa\0\xFF\x93\x87\x93\x90\x92_\x92\x83\x92\x91\x83\x91\x90\x82\x88Z\xF1\x80a\x01rW`@Q=_\x82>=\x81\xFD[PP_Q=\x91P\x81\x15a\x01\x89W\x80`\x01\x14\x15a\x01\x96V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x01\xC3W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[___``\x84\x86\x03\x12\x15a\x01\xDBW__\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xF1W__\xFD[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV\xFE\xA1dsolcC\0\x08\x1C\0\n\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static MOCKPOOL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockPool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockPool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockPool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockPool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockPool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockPool)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockPool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKPOOL_ABI.clone(),
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
                MOCKPOOL_ABI.clone(),
                MOCKPOOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eip712Domain` (0x84b0196e) function
        pub fn eip_712_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                [u8; 1],
                ::std::string::String,
                ::std::string::String,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                [u8; 32],
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([132, 176, 25, 110], ())
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
        ///Calls the contract's `mutateSupply` (0xc41ce1b9) function
        pub fn mutate_supply(
            &self,
            new_supply: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 28, 225, 185], new_supply)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
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
        ///Calls the contract's `observations` (0x252c09d7) function
        pub fn observations(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([37, 44, 9, 215], p0)
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
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
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
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
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
        ///Gets the contract's `EIP712DomainChanged` event
        pub fn eip712_domain_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Eip712DomainChangedFilter,
        > {
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MockPoolEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockPool<M> {
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
    ///Custom Error type `ECDSAInvalidSignature` with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`
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
    #[etherror(name = "ECDSAInvalidSignature", abi = "ECDSAInvalidSignature()")]
    pub struct ECDSAInvalidSignature;
    ///Custom Error type `ECDSAInvalidSignatureLength` with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`
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
        name = "ECDSAInvalidSignatureLength",
        abi = "ECDSAInvalidSignatureLength(uint256)"
    )]
    pub struct ECDSAInvalidSignatureLength {
        pub length: ::ethers::core::types::U256,
    }
    ///Custom Error type `ECDSAInvalidSignatureS` with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`
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
    #[etherror(name = "ECDSAInvalidSignatureS", abi = "ECDSAInvalidSignatureS(bytes32)")]
    pub struct ECDSAInvalidSignatureS {
        pub s: [u8; 32],
    }
    ///Custom Error type `ERC20InsufficientAllowance` with signature `ERC20InsufficientAllowance(address,uint256,uint256)` and selector `0xfb8f41b2`
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
        name = "ERC20InsufficientAllowance",
        abi = "ERC20InsufficientAllowance(address,uint256,uint256)"
    )]
    pub struct ERC20InsufficientAllowance {
        pub spender: ::ethers::core::types::Address,
        pub allowance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InsufficientBalance` with signature `ERC20InsufficientBalance(address,uint256,uint256)` and selector `0xe450d38c`
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
        name = "ERC20InsufficientBalance",
        abi = "ERC20InsufficientBalance(address,uint256,uint256)"
    )]
    pub struct ERC20InsufficientBalance {
        pub sender: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InvalidApprover` with signature `ERC20InvalidApprover(address)` and selector `0xe602df05`
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
    #[etherror(name = "ERC20InvalidApprover", abi = "ERC20InvalidApprover(address)")]
    pub struct ERC20InvalidApprover {
        pub approver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidReceiver` with signature `ERC20InvalidReceiver(address)` and selector `0xec442f05`
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
    #[etherror(name = "ERC20InvalidReceiver", abi = "ERC20InvalidReceiver(address)")]
    pub struct ERC20InvalidReceiver {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidSender` with signature `ERC20InvalidSender(address)` and selector `0x96c6fd1e`
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
    #[etherror(name = "ERC20InvalidSender", abi = "ERC20InvalidSender(address)")]
    pub struct ERC20InvalidSender {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidSpender` with signature `ERC20InvalidSpender(address)` and selector `0x94280d62`
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
    #[etherror(name = "ERC20InvalidSpender", abi = "ERC20InvalidSpender(address)")]
    pub struct ERC20InvalidSpender {
        pub spender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC2612ExpiredSignature` with signature `ERC2612ExpiredSignature(uint256)` and selector `0x62791302`
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
        name = "ERC2612ExpiredSignature",
        abi = "ERC2612ExpiredSignature(uint256)"
    )]
    pub struct ERC2612ExpiredSignature {
        pub deadline: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC2612InvalidSigner` with signature `ERC2612InvalidSigner(address,address)` and selector `0x4b800e46`
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
        name = "ERC2612InvalidSigner",
        abi = "ERC2612InvalidSigner(address,address)"
    )]
    pub struct ERC2612InvalidSigner {
        pub signer: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
    }
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
    ///Custom Error type `InvalidAccountNonce` with signature `InvalidAccountNonce(address,uint256)` and selector `0x752d88c0`
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
        name = "InvalidAccountNonce",
        abi = "InvalidAccountNonce(address,uint256)"
    )]
    pub struct InvalidAccountNonce {
        pub account: ::ethers::core::types::Address,
        pub current_nonce: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidShortString` with signature `InvalidShortString()` and selector `0xb3512b0c`
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
    #[etherror(name = "InvalidShortString", abi = "InvalidShortString()")]
    pub struct InvalidShortString;
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
    ///Custom Error type `ReentrancyGuardReentrantCall` with signature `ReentrancyGuardReentrantCall()` and selector `0x3ee5aeb5`
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
        name = "ReentrancyGuardReentrantCall",
        abi = "ReentrancyGuardReentrantCall()"
    )]
    pub struct ReentrancyGuardReentrantCall;
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
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
        name = "SafeERC20FailedOperation",
        abi = "SafeERC20FailedOperation(address)"
    )]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `StringTooLong` with signature `StringTooLong(string)` and selector `0x305a27a9`
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
    #[etherror(name = "StringTooLong", abi = "StringTooLong(string)")]
    pub struct StringTooLong {
        pub str: ::std::string::String,
    }
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
    pub enum MockPoolErrors {
        BelowMinimumK(BelowMinimumK),
        DepositsNotEqual(DepositsNotEqual),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        ERC20InsufficientAllowance(ERC20InsufficientAllowance),
        ERC20InsufficientBalance(ERC20InsufficientBalance),
        ERC20InvalidApprover(ERC20InvalidApprover),
        ERC20InvalidReceiver(ERC20InvalidReceiver),
        ERC20InvalidSender(ERC20InvalidSender),
        ERC20InvalidSpender(ERC20InvalidSpender),
        ERC2612ExpiredSignature(ERC2612ExpiredSignature),
        ERC2612InvalidSigner(ERC2612InvalidSigner),
        FactoryAlreadySet(FactoryAlreadySet),
        InsufficientInputAmount(InsufficientInputAmount),
        InsufficientLiquidity(InsufficientLiquidity),
        InsufficientLiquidityBurned(InsufficientLiquidityBurned),
        InsufficientLiquidityMinted(InsufficientLiquidityMinted),
        InsufficientOutputAmount(InsufficientOutputAmount),
        InvalidAccountNonce(InvalidAccountNonce),
        InvalidShortString(InvalidShortString),
        InvalidTo(InvalidTo),
        IsPaused(IsPaused),
        K(K),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        StringTooLong(StringTooLong),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockPoolErrors {
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
            if let Ok(decoded) = <ECDSAInvalidSignature as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ECDSAInvalidSignature(decoded));
            }
            if let Ok(decoded) = <ECDSAInvalidSignatureLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ECDSAInvalidSignatureLength(decoded));
            }
            if let Ok(decoded) = <ECDSAInvalidSignatureS as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ECDSAInvalidSignatureS(decoded));
            }
            if let Ok(decoded) = <ERC20InsufficientAllowance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InsufficientAllowance(decoded));
            }
            if let Ok(decoded) = <ERC20InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InsufficientBalance(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidApprover as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidApprover(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidReceiver as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidReceiver(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidSender(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidSpender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidSpender(decoded));
            }
            if let Ok(decoded) = <ERC2612ExpiredSignature as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC2612ExpiredSignature(decoded));
            }
            if let Ok(decoded) = <ERC2612InvalidSigner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC2612InvalidSigner(decoded));
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
            if let Ok(decoded) = <InvalidAccountNonce as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidAccountNonce(decoded));
            }
            if let Ok(decoded) = <InvalidShortString as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidShortString(decoded));
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
            if let Ok(decoded) = <ReentrancyGuardReentrantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReentrancyGuardReentrantCall(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            if let Ok(decoded) = <StringTooLong as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StringTooLong(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockPoolErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BelowMinimumK(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositsNotEqual(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InsufficientAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidSpender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC2612ExpiredSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC2612InvalidSigner(element) => {
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
                Self::InvalidAccountNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidShortString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::K(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StringTooLong(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockPoolErrors {
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
                    == <ECDSAInvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InsufficientAllowance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidApprover as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidReceiver as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidSpender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC2612ExpiredSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC2612InvalidSigner as ::ethers::contract::EthError>::selector() => {
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
                    == <InvalidAccountNonce as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidShortString as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTo as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <IsPaused as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <K as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ReentrancyGuardReentrantCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StringTooLong as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockPoolErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BelowMinimumK(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositsNotEqual(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InsufficientAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidApprover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidReceiver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidSpender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC2612ExpiredSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC2612InvalidSigner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::InvalidAccountNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidShortString(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::K(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StringTooLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockPoolErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BelowMinimumK> for MockPoolErrors {
        fn from(value: BelowMinimumK) -> Self {
            Self::BelowMinimumK(value)
        }
    }
    impl ::core::convert::From<DepositsNotEqual> for MockPoolErrors {
        fn from(value: DepositsNotEqual) -> Self {
            Self::DepositsNotEqual(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature> for MockPoolErrors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength> for MockPoolErrors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS> for MockPoolErrors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientAllowance> for MockPoolErrors {
        fn from(value: ERC20InsufficientAllowance) -> Self {
            Self::ERC20InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientBalance> for MockPoolErrors {
        fn from(value: ERC20InsufficientBalance) -> Self {
            Self::ERC20InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidApprover> for MockPoolErrors {
        fn from(value: ERC20InvalidApprover) -> Self {
            Self::ERC20InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidReceiver> for MockPoolErrors {
        fn from(value: ERC20InvalidReceiver) -> Self {
            Self::ERC20InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSender> for MockPoolErrors {
        fn from(value: ERC20InvalidSender) -> Self {
            Self::ERC20InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSpender> for MockPoolErrors {
        fn from(value: ERC20InvalidSpender) -> Self {
            Self::ERC20InvalidSpender(value)
        }
    }
    impl ::core::convert::From<ERC2612ExpiredSignature> for MockPoolErrors {
        fn from(value: ERC2612ExpiredSignature) -> Self {
            Self::ERC2612ExpiredSignature(value)
        }
    }
    impl ::core::convert::From<ERC2612InvalidSigner> for MockPoolErrors {
        fn from(value: ERC2612InvalidSigner) -> Self {
            Self::ERC2612InvalidSigner(value)
        }
    }
    impl ::core::convert::From<FactoryAlreadySet> for MockPoolErrors {
        fn from(value: FactoryAlreadySet) -> Self {
            Self::FactoryAlreadySet(value)
        }
    }
    impl ::core::convert::From<InsufficientInputAmount> for MockPoolErrors {
        fn from(value: InsufficientInputAmount) -> Self {
            Self::InsufficientInputAmount(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidity> for MockPoolErrors {
        fn from(value: InsufficientLiquidity) -> Self {
            Self::InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidityBurned> for MockPoolErrors {
        fn from(value: InsufficientLiquidityBurned) -> Self {
            Self::InsufficientLiquidityBurned(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidityMinted> for MockPoolErrors {
        fn from(value: InsufficientLiquidityMinted) -> Self {
            Self::InsufficientLiquidityMinted(value)
        }
    }
    impl ::core::convert::From<InsufficientOutputAmount> for MockPoolErrors {
        fn from(value: InsufficientOutputAmount) -> Self {
            Self::InsufficientOutputAmount(value)
        }
    }
    impl ::core::convert::From<InvalidAccountNonce> for MockPoolErrors {
        fn from(value: InvalidAccountNonce) -> Self {
            Self::InvalidAccountNonce(value)
        }
    }
    impl ::core::convert::From<InvalidShortString> for MockPoolErrors {
        fn from(value: InvalidShortString) -> Self {
            Self::InvalidShortString(value)
        }
    }
    impl ::core::convert::From<InvalidTo> for MockPoolErrors {
        fn from(value: InvalidTo) -> Self {
            Self::InvalidTo(value)
        }
    }
    impl ::core::convert::From<IsPaused> for MockPoolErrors {
        fn from(value: IsPaused) -> Self {
            Self::IsPaused(value)
        }
    }
    impl ::core::convert::From<K> for MockPoolErrors {
        fn from(value: K) -> Self {
            Self::K(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for MockPoolErrors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for MockPoolErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<StringTooLong> for MockPoolErrors {
        fn from(value: StringTooLong) -> Self {
            Self::StringTooLong(value)
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
    #[ethevent(name = "EIP712DomainChanged", abi = "EIP712DomainChanged()")]
    pub struct Eip712DomainChangedFilter;
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
    pub enum MockPoolEvents {
        ApprovalFilter(ApprovalFilter),
        BurnFilter(BurnFilter),
        ClaimFilter(ClaimFilter),
        Eip712DomainChangedFilter(Eip712DomainChangedFilter),
        FeesFilter(FeesFilter),
        MintFilter(MintFilter),
        SwapFilter(SwapFilter),
        SyncFilter(SyncFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockPoolEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockPoolEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(MockPoolEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = ClaimFilter::decode_log(log) {
                return Ok(MockPoolEvents::ClaimFilter(decoded));
            }
            if let Ok(decoded) = Eip712DomainChangedFilter::decode_log(log) {
                return Ok(MockPoolEvents::Eip712DomainChangedFilter(decoded));
            }
            if let Ok(decoded) = FeesFilter::decode_log(log) {
                return Ok(MockPoolEvents::FeesFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(MockPoolEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(MockPoolEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = SyncFilter::decode_log(log) {
                return Ok(MockPoolEvents::SyncFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockPoolEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockPoolEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712DomainChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SyncFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for MockPoolEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<BurnFilter> for MockPoolEvents {
        fn from(value: BurnFilter) -> Self {
            Self::BurnFilter(value)
        }
    }
    impl ::core::convert::From<ClaimFilter> for MockPoolEvents {
        fn from(value: ClaimFilter) -> Self {
            Self::ClaimFilter(value)
        }
    }
    impl ::core::convert::From<Eip712DomainChangedFilter> for MockPoolEvents {
        fn from(value: Eip712DomainChangedFilter) -> Self {
            Self::Eip712DomainChangedFilter(value)
        }
    }
    impl ::core::convert::From<FeesFilter> for MockPoolEvents {
        fn from(value: FeesFilter) -> Self {
            Self::FeesFilter(value)
        }
    }
    impl ::core::convert::From<MintFilter> for MockPoolEvents {
        fn from(value: MintFilter) -> Self {
            Self::MintFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for MockPoolEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    impl ::core::convert::From<SyncFilter> for MockPoolEvents {
        fn from(value: SyncFilter) -> Self {
            Self::SyncFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockPoolEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
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
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
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
    #[ethcall(name = "eip712Domain", abi = "eip712Domain()")]
    pub struct Eip712DomainCall;
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
    ///Container type for all input parameters for the `mutateSupply` function with signature `mutateSupply(uint256)` and selector `0xc41ce1b9`
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
    #[ethcall(name = "mutateSupply", abi = "mutateSupply(uint256)")]
    pub struct MutateSupplyCall {
        pub new_supply: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `observations` function with signature `observations(uint256)` and selector `0x252c09d7`
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
    #[ethcall(name = "observations", abi = "observations(uint256)")]
    pub struct ObservationsCall(pub ::ethers::core::types::U256);
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
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
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
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
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
    pub enum MockPoolCalls {
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        BlockTimestampLast(BlockTimestampLastCall),
        Burn(BurnCall),
        ClaimFees(ClaimFeesCall),
        Claimable0(Claimable0Call),
        Claimable1(Claimable1Call),
        CurrentCumulativePrices(CurrentCumulativePricesCall),
        Decimals(DecimalsCall),
        Eip712Domain(Eip712DomainCall),
        Factory(FactoryCall),
        GetAmountOut(GetAmountOutCall),
        GetReserves(GetReservesCall),
        Index0(Index0Call),
        Index1(Index1Call),
        Initialize(InitializeCall),
        LastObservation(LastObservationCall),
        Metadata(MetadataCall),
        Mint(MintCall),
        MutateSupply(MutateSupplyCall),
        Name(NameCall),
        Nonces(NoncesCall),
        ObservationLength(ObservationLengthCall),
        Observations(ObservationsCall),
        PeriodSize(PeriodSizeCall),
        Permit(PermitCall),
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
        Symbol(SymbolCall),
        Sync(SyncCall),
        Token0(Token0Call),
        Token1(Token1Call),
        Tokens(TokensCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
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
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <Eip712DomainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Eip712Domain(decoded));
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
            if let Ok(decoded) = <MutateSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MutateSupply(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <ObservationLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ObservationLength(decoded));
            }
            if let Ok(decoded) = <ObservationsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Observations(decoded));
            }
            if let Ok(decoded) = <PeriodSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PeriodSize(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit(decoded));
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
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
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
    impl ::ethers::core::abi::AbiEncode for MockPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Eip712Domain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::MutateSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ObservationLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Observations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PeriodSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
    impl ::core::fmt::Display for MockPoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712Domain(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::Index0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Index1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastObservation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Metadata(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::MutateSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::ObservationLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Observations(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeriodSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<DomainSeparatorCall> for MockPoolCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for MockPoolCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for MockPoolCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockPoolCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BlockTimestampLastCall> for MockPoolCalls {
        fn from(value: BlockTimestampLastCall) -> Self {
            Self::BlockTimestampLast(value)
        }
    }
    impl ::core::convert::From<BurnCall> for MockPoolCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<ClaimFeesCall> for MockPoolCalls {
        fn from(value: ClaimFeesCall) -> Self {
            Self::ClaimFees(value)
        }
    }
    impl ::core::convert::From<Claimable0Call> for MockPoolCalls {
        fn from(value: Claimable0Call) -> Self {
            Self::Claimable0(value)
        }
    }
    impl ::core::convert::From<Claimable1Call> for MockPoolCalls {
        fn from(value: Claimable1Call) -> Self {
            Self::Claimable1(value)
        }
    }
    impl ::core::convert::From<CurrentCumulativePricesCall> for MockPoolCalls {
        fn from(value: CurrentCumulativePricesCall) -> Self {
            Self::CurrentCumulativePrices(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for MockPoolCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<Eip712DomainCall> for MockPoolCalls {
        fn from(value: Eip712DomainCall) -> Self {
            Self::Eip712Domain(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for MockPoolCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for MockPoolCalls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetReservesCall> for MockPoolCalls {
        fn from(value: GetReservesCall) -> Self {
            Self::GetReserves(value)
        }
    }
    impl ::core::convert::From<Index0Call> for MockPoolCalls {
        fn from(value: Index0Call) -> Self {
            Self::Index0(value)
        }
    }
    impl ::core::convert::From<Index1Call> for MockPoolCalls {
        fn from(value: Index1Call) -> Self {
            Self::Index1(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for MockPoolCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LastObservationCall> for MockPoolCalls {
        fn from(value: LastObservationCall) -> Self {
            Self::LastObservation(value)
        }
    }
    impl ::core::convert::From<MetadataCall> for MockPoolCalls {
        fn from(value: MetadataCall) -> Self {
            Self::Metadata(value)
        }
    }
    impl ::core::convert::From<MintCall> for MockPoolCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<MutateSupplyCall> for MockPoolCalls {
        fn from(value: MutateSupplyCall) -> Self {
            Self::MutateSupply(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockPoolCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for MockPoolCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<ObservationLengthCall> for MockPoolCalls {
        fn from(value: ObservationLengthCall) -> Self {
            Self::ObservationLength(value)
        }
    }
    impl ::core::convert::From<ObservationsCall> for MockPoolCalls {
        fn from(value: ObservationsCall) -> Self {
            Self::Observations(value)
        }
    }
    impl ::core::convert::From<PeriodSizeCall> for MockPoolCalls {
        fn from(value: PeriodSizeCall) -> Self {
            Self::PeriodSize(value)
        }
    }
    impl ::core::convert::From<PermitCall> for MockPoolCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<PoolFeesCall> for MockPoolCalls {
        fn from(value: PoolFeesCall) -> Self {
            Self::PoolFees(value)
        }
    }
    impl ::core::convert::From<PricesCall> for MockPoolCalls {
        fn from(value: PricesCall) -> Self {
            Self::Prices(value)
        }
    }
    impl ::core::convert::From<QuoteCall> for MockPoolCalls {
        fn from(value: QuoteCall) -> Self {
            Self::Quote(value)
        }
    }
    impl ::core::convert::From<Reserve0Call> for MockPoolCalls {
        fn from(value: Reserve0Call) -> Self {
            Self::Reserve0(value)
        }
    }
    impl ::core::convert::From<Reserve0CumulativeLastCall> for MockPoolCalls {
        fn from(value: Reserve0CumulativeLastCall) -> Self {
            Self::Reserve0CumulativeLast(value)
        }
    }
    impl ::core::convert::From<Reserve1Call> for MockPoolCalls {
        fn from(value: Reserve1Call) -> Self {
            Self::Reserve1(value)
        }
    }
    impl ::core::convert::From<Reserve1CumulativeLastCall> for MockPoolCalls {
        fn from(value: Reserve1CumulativeLastCall) -> Self {
            Self::Reserve1CumulativeLast(value)
        }
    }
    impl ::core::convert::From<SampleCall> for MockPoolCalls {
        fn from(value: SampleCall) -> Self {
            Self::Sample(value)
        }
    }
    impl ::core::convert::From<SetNameCall> for MockPoolCalls {
        fn from(value: SetNameCall) -> Self {
            Self::SetName(value)
        }
    }
    impl ::core::convert::From<SetSymbolCall> for MockPoolCalls {
        fn from(value: SetSymbolCall) -> Self {
            Self::SetSymbol(value)
        }
    }
    impl ::core::convert::From<SkimCall> for MockPoolCalls {
        fn from(value: SkimCall) -> Self {
            Self::Skim(value)
        }
    }
    impl ::core::convert::From<StableCall> for MockPoolCalls {
        fn from(value: StableCall) -> Self {
            Self::Stable(value)
        }
    }
    impl ::core::convert::From<SupplyIndex0Call> for MockPoolCalls {
        fn from(value: SupplyIndex0Call) -> Self {
            Self::SupplyIndex0(value)
        }
    }
    impl ::core::convert::From<SupplyIndex1Call> for MockPoolCalls {
        fn from(value: SupplyIndex1Call) -> Self {
            Self::SupplyIndex1(value)
        }
    }
    impl ::core::convert::From<SwapCall> for MockPoolCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockPoolCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<SyncCall> for MockPoolCalls {
        fn from(value: SyncCall) -> Self {
            Self::Sync(value)
        }
    }
    impl ::core::convert::From<Token0Call> for MockPoolCalls {
        fn from(value: Token0Call) -> Self {
            Self::Token0(value)
        }
    }
    impl ::core::convert::From<Token1Call> for MockPoolCalls {
        fn from(value: Token1Call) -> Self {
            Self::Token1(value)
        }
    }
    impl ::core::convert::From<TokensCall> for MockPoolCalls {
        fn from(value: TokensCall) -> Self {
            Self::Tokens(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for MockPoolCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for MockPoolCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockPoolCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
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
    pub struct ClaimFeesReturn {
        pub claimed_0: ::ethers::core::types::U256,
        pub claimed_1: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
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
    pub struct Eip712DomainReturn {
        pub fields: [u8; 1],
        pub name: ::std::string::String,
        pub version: ::std::string::String,
        pub chain_id: ::ethers::core::types::U256,
        pub verifying_contract: ::ethers::core::types::Address,
        pub salt: [u8; 32],
        pub extensions: ::std::vec::Vec<::ethers::core::types::U256>,
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
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `observations` function with signature `observations(uint256)` and selector `0x252c09d7`
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
    pub struct ObservationsReturn {
        pub timestamp: ::ethers::core::types::U256,
        pub reserve_0_cumulative: ::ethers::core::types::U256,
        pub reserve_1_cumulative: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
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
