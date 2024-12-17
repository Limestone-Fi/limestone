pub use mock_router::*;
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
pub mod mock_router {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_factoryRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_factory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_voter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ETHER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ETHER"),
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
                    ::std::borrow::ToOwned::to_owned("UNSAFE_swapExactTokensForTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UNSAFE_swapExactTokensForTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addLiquidity"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountADesired"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBDesired"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountAMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBMin"),
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
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("addLiquidityETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addLiquidityETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "amountTokenDesired",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountTokenMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETHMin"),
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
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETH"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultFactory"),
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
                    ::std::borrow::ToOwned::to_owned("factoryRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("factoryRegistry"),
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
                    ::std::borrow::ToOwned::to_owned("generateZapInParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "generateZapInParams",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_factory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountInA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountInB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routesA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routesB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMinA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMinB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountAMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBMin"),
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
                    ::std::borrow::ToOwned::to_owned("generateZapOutParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "generateZapOutParams",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_factory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routesA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routesB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMinA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMinB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountAMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBMin"),
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
                    ::std::borrow::ToOwned::to_owned("getAmountsOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAmountsOut"),
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
                                    name: ::std::borrow::ToOwned::to_owned("routes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
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
                    ::std::borrow::ToOwned::to_owned("getReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReserves"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_factory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveB"),
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
                    ::std::borrow::ToOwned::to_owned("poolFor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("poolFor"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_factory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quoteAddLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quoteAddLiquidity"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_factory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountADesired"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBDesired"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quoteRemoveLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "quoteRemoveLiquidity",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_factory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountB"),
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
                    ::std::borrow::ToOwned::to_owned("quoteStableLiquidityRatio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "quoteStableLiquidityRatio",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("_factory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ratio"),
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
                    ::std::borrow::ToOwned::to_owned("removeLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeLiquidity"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountAMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBMin"),
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
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountB"),
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
                    ::std::borrow::ToOwned::to_owned("removeLiquidityETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeLiquidityETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountTokenMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETHMin"),
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
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETH"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "removeLiquidityETHSupportingFeeOnTransferTokens",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeLiquidityETHSupportingFeeOnTransferTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountTokenMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountETHMin"),
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
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountETH"),
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
                    ::std::borrow::ToOwned::to_owned("sortTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sortTokens"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapExactETHForTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactETHForTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "swapExactETHForTokensSupportingFeeOnTransferTokens",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactETHForTokensSupportingFeeOnTransferTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapExactTokensForETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactTokensForETH",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "swapExactTokensForETHSupportingFeeOnTransferTokens",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactTokensForETHSupportingFeeOnTransferTokens",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                    ::std::borrow::ToOwned::to_owned("swapExactTokensForTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactTokensForTokens",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "swapExactTokensForTokensSupportingFeeOnTransferTokens",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "swapExactTokensForTokensSupportingFeeOnTransferTokens",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
                (
                    ::std::borrow::ToOwned::to_owned("weth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("weth"),
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
                    ::std::borrow::ToOwned::to_owned("zapIn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("zapIn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountInA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountInB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("zapInPool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IDromeRouter.Zap"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routesA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routesB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("stake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("zapOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("zapOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("zapOutPool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IDromeRouter.Zap"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routesA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("routesB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDromeRouter.Route[]",
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ETHTransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ETHTransferFailed"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Expired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Expired"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InsufficientAmount"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientAmountA"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientAmountA",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientAmountADesired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientAmountADesired",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientAmountAOptimal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientAmountAOptimal",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientAmountB"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientAmountB",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientAmountBDesired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientAmountBDesired",
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
                    ::std::borrow::ToOwned::to_owned("InvalidAmountInForETHDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidAmountInForETHDeposit",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPath"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidPath"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidRouteA"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidRouteA"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidRouteB"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidRouteB"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTokenInForETHDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidTokenInForETHDeposit",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyWETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OnlyWETH"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolDoesNotExist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PoolDoesNotExist"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolFactoryDoesNotExist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PoolFactoryDoesNotExist",
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
                    ::std::borrow::ToOwned::to_owned("SameAddresses"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SameAddresses"),
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
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKROUTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@QaN\xAC8\x03\x80aN\xAC\x839\x81\x01`@\x81\x90Ra\0/\x91a\0gV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x80R\x90\x82\x16`\xA0R\x16`\xC0Ra\0\xA7V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0bW__\xFD[\x91\x90PV[___``\x84\x86\x03\x12\x15a\0yW__\xFD[a\0\x82\x84a\0LV[\x92Pa\0\x90` \x85\x01a\0LV[\x91Pa\0\x9E`@\x85\x01a\0LV[\x90P\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Q`\xE0QaM1a\x01{_9_\x81\x81a\x01\xC2\x01R\x81\x81a\x03\x0C\x01R\x81\x81a\x08\xD7\x01R\x81\x81a\n\x9D\x01R\x81\x81a\x18\r\x01R\x81\x81a\x19\xBF\x01R\x81\x81a\x1C\x99\x01R\x81\x81a\x1C\xCE\x01R\x81\x81a\x1D$\x01R\x81\x81a\x1E>\x01R\x81\x81a%\x01\x01R\x81\x81a(\x12\x01R\x81\x81a)\x89\x01R\x81\x81a6\xE4\x01Ra7s\x01R_a\x03\x92\x01R_\x81\x81a\x05\x9A\x01R\x81\x81a\x07\x8E\x01R\x81\x81a\x0F\xB7\x01R\x81\x81a\x11\xC6\x01R\x81\x81a\x13\xB8\x01R\x81\x81a\x1C\xF0\x01R\x81\x81a2p\x01R\x81\x81a3\x0F\x01Ra3\x95\x01R_\x81\x81a\x02\xAE\x01Ra\x14\x11\x01RaM1_\xF3\xFE`\x80`@R`\x046\x10a\x01\xB2W_5`\xE0\x1C\x80c\x88\xCD\x82\x1E\x11a\0\xE7W\x80c\xCA\xC8\x8E\xA9\x11a\0\x87W\x80c\xD7\xB0\xE0\xA5\x11a\0bW\x80c\xD7\xB0\xE0\xA5\x14a\x05\xBCW\x80c\xF5\xBAS\xC7\x14a\x05\xDBW\x80c\xFBI\xBA\xFD\x14a\x06\x08W\x80c\xFEA\x1F\x14\x14a\x06\x1BW__\xFD[\x80c\xCA\xC8\x8E\xA9\x14a\x05KW\x80c\xCEp\x0C)\x14a\x05jW\x80c\xD4\xB6\x84m\x14a\x05\x89W__\xFD[\x80c\xA8\x1B\x91Y\x11a\0\xC2W\x80c\xA8\x1B\x91Y\x14a\x04\xDBW\x80c\xB7\xE0\xD4\xC0\x14a\x04\xFAW\x80c\xC6\xB7\xF1\xB6\x14a\x05\rW\x80c\xC9-\xE3\xEC\x14a\x05,W__\xFD[\x80c\x88\xCD\x82\x1E\x14a\x04\x8AW\x80c\x8C\x007\xDC\x14a\x04\xA9W\x80c\x9068\xA4\x14a\x04\xC8W__\xFD[\x80cB\xCB\x1F\xBC\x11a\x01RW\x80cU\t\xA1\xAC\x11a\x01-W\x80cU\t\xA1\xAC\x14a\x03\xF3W\x80cZG\xDD\xC3\x14a\x04\x12W\x80cu9\xD4\x13\x14a\x04LW\x80c\x87@)\xD9\x14a\x04kW__\xFD[\x80cB\xCB\x1F\xBC\x14a\x03ZW\x80cF\xC9j\xAC\x14a\x03\x81W\x80cTL\xAAV\x14a\x03\xB4W__\xFD[\x80c;\xF0\xC9\xFB\x11a\x01\x8DW\x80c;\xF0\xC9\xFB\x14a\x02\x9DW\x80c=\xA5\xAC\xBA\x14a\x02\xE8W\x80c?\xC8\xCE\xF3\x14a\x02\xFBW\x80cA\x11\xD5\x97\x14a\x03.W__\xFD[\x80c\x07\xDBP\xFA\x14a\x02\x06W\x80c\r\xED\xE6\xC4\x14a\x02JW\x80c\x12\xBC:\xCA\x14a\x02~W__\xFD[6a\x02\x02W3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\0W`@Qc\x01\xF1\x80\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\0[__\xFD[4\x80\x15a\x02\x11W__\xFD[Pa\x02%a\x02 6`\x04a@cV[a\x06:V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02UW__\xFD[Pa\x02ia\x02d6`\x04aA/V[a\x07yV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02AV[4\x80\x15a\x02\x89W__\xFD[Pa\x02\0a\x02\x986`\x04aA\xAFV[a\x08\xC3V[4\x80\x15a\x02\xA8W__\xFD[Pa\x02\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02AV[a\x02\0a\x02\xF66`\x04aB\x1CV[a\n\x91V[4\x80\x15a\x03\x06W__\xFD[Pa\x02\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x039W__\xFD[Pa\x03Ma\x03H6`\x04aB\xE3V[a\x0C\xE0V[`@Qa\x02A\x91\x90aC\xAEV[4\x80\x15a\x03eW__\xFD[Pa\x02\xD0s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x81V[4\x80\x15a\x03\x8CW__\xFD[Pa\x02\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\xBFW__\xFD[Pa\x03\xD3a\x03\xCE6`\x04aC\xF0V[a\x0E2V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x02AV[4\x80\x15a\x03\xFEW__\xFD[Pa\x03Ma\x04\r6`\x04aD\xADV[a\x0E\xBCV[4\x80\x15a\x04\x1DW__\xFD[Pa\x041a\x04,6`\x04aESV[a\x11\x9CV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02AV[4\x80\x15a\x04WW__\xFD[Pa\x02%a\x04f6`\x04aE\xDDV[a\x12\x80V[4\x80\x15a\x04vW__\xFD[Pa\x02\xD0a\x04\x856`\x04aF\x9EV[a\x13\xB5V[4\x80\x15a\x04\x95W__\xFD[Pa\x02\0a\x04\xA46`\x04aA\xAFV[a\x15\xCFV[4\x80\x15a\x04\xB4W__\xFD[Pa\x02ia\x04\xC36`\x04aF\x9EV[a\x17JV[a\x03Ma\x04\xD66`\x04aB\x1CV[a\x17\xFFV[4\x80\x15a\x04\xE6W__\xFD[Pa\x02\0a\x04\xF56`\x04aG\x0EV[a\x19oV[a\x041a\x05\x086`\x04aG\xABV[a\x1C\x86V[4\x80\x15a\x05\x18W__\xFD[Pa\x03Ma\x05'6`\x04aA\xAFV[a\x1E(V[4\x80\x15a\x057W__\xFD[Pa\x02ia\x05F6`\x04aH\x19V[a =V[4\x80\x15a\x05VW__\xFD[Pa\x03Ma\x05e6`\x04aA\xAFV[a!}V[4\x80\x15a\x05uW__\xFD[Pa\x041a\x05\x846`\x04aHyV[a\"\x89V[4\x80\x15a\x05\x94W__\xFD[Pa\x02\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\xC7W__\xFD[Pa\x02ia\x05\xD66`\x04aG\xABV[a$\xEFV[4\x80\x15a\x05\xE6W__\xFD[Pa\x05\xFAa\x05\xF56`\x04aH\xE3V[a%RV[`@Q\x90\x81R` \x01a\x02AV[a\x05\xFAa\x06\x166`\x04aI+V[a'\xBBV[4\x80\x15a\x06&W__\xFD[Pa\x05\xFAa\x0656`\x04aG\xABV[a)xV[\x85\x85_\x80``\x87\x15a\x06\xCDWa\x06\xA1\x8B\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa\x06\x88`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x06kV[PPPPPa\x0E\xBCV[\x90P\x80`\x01\x82Qa\x06\xB2\x91\x90aJ-V[\x81Q\x81\x10a\x06\xC2Wa\x06\xC2aJFV[` \x02` \x01\x01Q\x94P[\x85\x15a\x07PWa\x07$\x8A\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa\x07\x15`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x06\xF8V[\x90P\x80`\x01\x82Qa\x075\x91\x90aJ-V[\x81Q\x81\x10a\x07EWa\x07EaJFV[` \x02` \x01\x01Q\x93P[a\x07^\x8F\x8F\x8F\x8F\x89\x89a\"\x89V[P\x80\x93P\x81\x94PPPP\x9AP\x9AP\x9AP\x9A\x96PPPPPPPV[__\x82a\x07\x85\x81a*@V[_a\x07\xB2\x8C\x8C\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13\xB5V[\x90Pa\x07\xC9`\x01`\x01`\xA0\x1B\x03\x82\x163\x83\x8Ca*dV[`@Qc\"k\xF2\xD1`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R_\x91\x82\x91\x84\x16\x90c\x89\xAF\xCBD\x90`$\x01`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\x11W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x085\x91\x90aJZV[\x91P\x91P_a\x08D\x8F\x8Fa\x0E2V[P\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x8F`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08gW\x81\x83a\x08jV[\x82\x82[\x90\x97P\x95P\x8A\x87\x10\x15a\x08\x90W`@Qc#\xD9\xBB\x05`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x89\x86\x10\x15a\x08\xB1W`@Qc\r2A\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPP\x98P\x98\x96PPPPPPPV[\x80a\x08\xCD\x81a*@V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85\x85a\t\x04`\x01\x82aJ-V[\x81\x81\x10a\t\x13Wa\t\x13aJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\t+\x91\x90aJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\tRW`@Qc \xDB\x82g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n-\x85\x85_\x81\x81\x10a\tgWa\tgaJFV[a\t}\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[3a\n'\x88\x88_\x81\x81\x10a\t\x93Wa\t\x93aJFV[a\t\xA9\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[\x89\x89_\x81\x81\x10a\t\xBBWa\t\xBBaJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\t\xD3\x91\x90aJ|V[\x8A\x8A_\x81\x81\x10a\t\xE5Wa\t\xE5aJFV[\x90P`\x80\x02\x01`@\x01` \x81\x01\x90a\t\xFD\x91\x90aJ\x97V[\x8B\x8B_\x81\x81\x10a\n\x0FWa\n\x0FaJFV[\x90P`\x80\x02\x01``\x01` \x81\x01\x90a\x04\x85\x91\x90aJ|V[\x8Aa*\xD1V[a\n\x88\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\n}Wa\nn`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\nQV[PPPPP0a+\xC1V[PPPPPPPV[\x80a\n\x9B\x81a*@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x85\x85_\x81\x81\x10a\n\xD7Wa\n\xD7aJFV[a\n\xED\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\x14W`@Qc \xDB\x82g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[4_a\x0B!`\x01\x87aJ-V[\x90P_\x87\x87\x83\x81\x81\x10a\x0B6Wa\x0B6aJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x0BN\x91\x90aJ|V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xB8\x91\x90aJ\xB2V[\x90Pa\x0C\x15\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\nWa\x0B\xFB`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x0B\xDEV[PPPPP\x87a+\xC1V[\x88\x81\x89\x89\x85\x81\x81\x10a\x0C)Wa\x0C)aJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x0CA\x91\x90aJ|V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xAC\x91\x90aJ\xB2V[a\x0C\xB6\x91\x90aJ-V[\x10\x15a\x0C\xD5W`@QcB0\x1C#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPV[``\x81a\x0C\xEC\x81a*@V[a\r\xC8\x86\x86_\x81\x81\x10a\r\x01Wa\r\x01aJFV[a\r\x17\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[3a\r\xA9\x89\x89_\x81\x81\x10a\r-Wa\r-aJFV[a\rC\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[\x8A\x8A_\x81\x81\x10a\rUWa\rUaJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\rm\x91\x90aJ|V[\x8B\x8B_\x81\x81\x10a\r\x7FWa\r\x7FaJFV[\x90P`\x80\x02\x01`@\x01` \x81\x01\x90a\r\x97\x91\x90aJ\x97V[\x8C\x8C_\x81\x81\x10a\n\x0FWa\n\x0FaJFV[\x8A_\x81Q\x81\x10a\r\xBBWa\r\xBBaJFV[` \x02` \x01\x01Qa*\xD1V[a\x0E$\x87\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0E\x19Wa\x0E\n`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\r\xEDV[PPPPP\x86a/\xCAV[\x86\x91P[P\x95\x94PPPPPV[__\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0EfW`@Qc2\x95\xF3\xFD`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0E\x86W\x82\x84a\x0E\x89V[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0E\xB5W`@Qc\xD9.#=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x92P\x92\x90PV[```\x01\x82Q\x10\x15a\x0E\xE1W`@Qc \xDB\x82g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Qa\x0E\xEE\x90`\x01aJ\xC9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\x05Wa\x0F\x05aB}V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F.W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82\x81_\x81Q\x81\x10a\x0FDWa\x0FDaJFV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81Q_[\x81\x81\x10\x15a\x11\x94W__`\x01`\x01`\xA0\x1B\x03\x16\x85\x83\x81Q\x81\x10a\x0FxWa\x0FxaJFV[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\xB5W\x84\x82\x81Q\x81\x10a\x0F\xA4Wa\x0F\xA4aJFV[` \x02` \x01\x01Q``\x01Qa\x0F\xD7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[\x90P_a\x10<\x86\x84\x81Q\x81\x10a\x0F\xEFWa\x0F\xEFaJFV[` \x02` \x01\x01Q_\x01Q\x87\x85\x81Q\x81\x10a\x10\x0CWa\x10\x0CaJFV[` \x02` \x01\x01Q` \x01Q\x88\x86\x81Q\x81\x10a\x10*Wa\x10*aJFV[` \x02` \x01\x01Q`@\x01Q\x85a\x13\xB5V[`@Qc[\x16\xEB\xB7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x91\x92P\x90\x83\x16\x90c[\x16\xEB\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xA8\x91\x90aJ\xDCV[\x15a\x11\x8AW\x80`\x01`\x01`\xA0\x1B\x03\x16c\xF1@\xA3Z\x86\x85\x81Q\x81\x10a\x10\xCEWa\x10\xCEaJFV[` \x02` \x01\x01Q\x88\x86\x81Q\x81\x10a\x10\xE8Wa\x10\xE8aJFV[` \x02` \x01\x01Q_\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\"\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11a\x91\x90aJ\xB2V[\x85a\x11m\x85`\x01aJ\xC9V[\x81Q\x81\x10a\x11}Wa\x11}aJFV[` \x02` \x01\x01\x81\x81RPP[PP`\x01\x01a\x0FSV[PP\x92\x91PPV[___\x83a\x11\xA9\x81a*@V[a\x11\xB8\x8D\x8D\x8D\x8D\x8D\x8D\x8Da2\x13V[\x90\x94P\x92P_a\x11\xEA\x8E\x8E\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13\xB5V[\x90Pa\x11\xF8\x8E3\x83\x88a*\xD1V[a\x12\x04\x8D3\x83\x87a*\xD1V[`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x82\x16\x90cjbxB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12IW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12m\x91\x90aJ\xB2V[\x92PPP\x99P\x99P\x99\x96PPPPPPPV[____a\x12\x91\x8D\x8D\x8D\x8D\x8Da =V[\x90\x94P\x92P\x83\x91P\x82\x90P``\x87\x15a\x13!Wa\x12\xF5\x83\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa\x12\xE6`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x12\xC9V[\x90P\x80`\x01\x82Qa\x13\x06\x91\x90aJ-V[\x81Q\x81\x10a\x13\x16Wa\x13\x16aJFV[` \x02` \x01\x01Q\x94P[\x85\x15a\x13\xA4Wa\x13x\x82\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa\x13i`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x13LV[\x90P\x80`\x01\x82Qa\x13\x89\x91\x90aJ-V[\x81Q\x81\x10a\x13\x99Wa\x13\x99aJFV[` \x02` \x01\x01Q\x93P[P\x99P\x99P\x99P\x99\x95PPPPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x13\xEDW\x83a\x13\xEFV[\x81[`@Qc\xD1\xEA\n\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xD1\xEA\n\x1D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14XW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14|\x91\x90aJ\xDCV[a\x14\x99W`@QcM9\xD5\xA3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x14\xA5\x89\x89a\x0E2V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x81\x1B\x82\x16` \x84\x01R\x83\x90\x1B\x16`4\x82\x01R\x89\x15\x15`\xF8\x1B`H\x82\x01R\x91\x93P\x91P_\x90`I\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x15\xC1\x84`\x01`\x01`\xA0\x1B\x03\x16c\\`\xDA\x1B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x157W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15[\x91\x90aJ\xF7V[`@Q`8\x81\x01\x87\x90RoZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\xFF`$\x82\x01R`\x14\x81\x01\x91\x90\x91Rs=`-\x80`\n=9\x81\xF36==7===6=s\x81R`X\x81\x01\x83\x90R`7`\x0C\x82\x01 `x\x82\x01R`U`C\x90\x91\x01 `\x01`\x01`\xA0\x1B\x03\x16\x90V[\x9A\x99PPPPPPPPPPV[\x80a\x15\xD9\x81a*@V[a\x15\xEE\x85\x85_\x81\x81\x10a\tgWa\tgaJFV[_a\x15\xFA`\x01\x86aJ-V[\x90P_\x86\x86\x83\x81\x81\x10a\x16\x0FWa\x16\x0FaJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x16'\x91\x90aJ|V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16mW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x91\x91\x90aJ\xB2V[\x90Pa\x16\xEE\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16\xE3Wa\x16\xD4`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x16\xB7V[PPPPP\x86a+\xC1V[\x87\x81\x88\x88\x85\x81\x81\x10a\x17\x02Wa\x17\x02aJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x17\x1A\x91\x90aJ|V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01a\x0CmV[___a\x17W\x87\x87a\x0E2V[P\x90P__a\x17h\x89\x89\x89\x89a\x13\xB5V[`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xA3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xC7\x91\x90aK\x12V[P\x91P\x91P\x82`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17\xECW\x80\x82a\x17\xEFV[\x81\x81[\x90\x9A\x90\x99P\x97PPPPPPPPV[``\x81a\x18\x0B\x81a*@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x86\x86_\x81\x81\x10a\x18GWa\x18GaJFV[a\x18]\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x84W`@Qc \xDB\x82g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\xD54\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa\x18\xC6`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x18\xA9V[\x91P\x86\x82`\x01\x84Qa\x18\xE7\x91\x90aJ-V[\x81Q\x81\x10a\x18\xF7Wa\x18\xF7aJFV[` \x02` \x01\x01Q\x10\x15a\x19\x1EW`@QcB0\x1C#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E(\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0E\x19Wa\x19``\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x19CV[_a\x19}` \x87\x01\x87aJ|V[\x90P_a\x19\x90`@\x88\x01` \x89\x01aJ|V[\x90P_`\x01`\x01`\xA0\x1B\x03\x8A\x16s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x14a\x19\xBDW\x89a\x19\xDFV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[\x90Pa\x19\xEB\x89\x89a4qV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B3W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1AGW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ak\x91\x90aJ\xB2V[\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16\x88\x88a\x1A\x84`\x01\x82aJ-V[\x81\x81\x10a\x1A\x93Wa\x1A\x93aJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x1A\xAB\x91\x90aJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1A\xD2W`@Qc\t\xD4\x1Cg`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B3\x84\x82\x8B`\x80\x015\x8B\x8B\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1B)Wa\x1B\x1A`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x1A\xFDV[PPPPPa5\xCDV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1CpW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xB2\x91\x90aJ\xB2V[\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16\x86\x86a\x1B\xCB`\x01\x82aJ-V[\x81\x81\x10a\x1B\xDAWa\x1B\xDAaJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x1B\xF2\x91\x90aJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\x19W`@Qc2\xB2A\x03`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1Cp\x83\x82\x8B`\xA0\x015\x89\x89\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1B)Wa\x1Ca`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x1CDV[a\x1Cy\x8Ba6\xA8V[PPPPPPPPPPPV[___\x83a\x1C\x93\x81a*@V[a\x1C\xC2\x8B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C\x8C4\x8D\x8Da2\x13V[\x90\x94P\x92P_a\x1D\x14\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13\xB5V[\x90Pa\x1D\"\x8C3\x83\x88a*\xD1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x1D{W__\xFD[PZ\xF1\x15\x80\x15a\x1D\x8DW=__>=_\xFD[PP`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16`\x04\x83\x01R\x85\x16\x93PcjbxB\x92P`$\x01\x90P` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xFC\x91\x90aJ\xB2V[\x92P\x834\x11\x15a\x1E\x19Wa\x1E\x193a\x1E\x14\x864aJ-V[a8cV[PP\x97P\x97P\x97\x94PPPPPV[``\x81a\x1E4\x81a*@V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x86a\x1Ek`\x01\x82aJ-V[\x81\x81\x10a\x1EzWa\x1EzaJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x1E\x92\x91\x90aJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E\xB9W`@Qc \xDB\x82g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\n\x88\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa\x1E\xFB`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x1E\xDEV[\x91P\x86\x82`\x01\x84Qa\x1F\x1C\x91\x90aJ-V[\x81Q\x81\x10a\x1F,Wa\x1F,aJFV[` \x02` \x01\x01Q\x10\x15a\x1FSW`@QcB0\x1C#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\xA6\x86\x86_\x81\x81\x10a\x1FhWa\x1FhaJFV[a\x1F~\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[3a\x1F\x94\x89\x89_\x81\x81\x10a\r-Wa\r-aJFV[\x85_\x81Q\x81\x10a\r\xBBWa\r\xBBaJFV[a \x02\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1F\xF7Wa\x1F\xE8`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x1F\xCBV[PPPPP0a/\xCAV[a 2\x84\x83`\x01\x85Qa \x15\x91\x90aJ-V[\x81Q\x81\x10a %Wa %aJFV[` \x02` \x01\x01Qa8cV[P\x96\x95PPPPPPV[___\x84`\x01`\x01`\xA0\x1B\x03\x16cy\xBCW\xD5\x89\x89\x89`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a p\x93\x92\x91\x90aK=V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xAF\x91\x90aJ\xF7V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a \xCBW__\x92P\x92PPa!sV[__a \xD9\x8A\x8A\x8A\x8Aa\x17JV[\x91P\x91P_\x83`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!>\x91\x90aJ\xB2V[\x90P\x80a!K\x84\x89aKaV[a!U\x91\x90aKxV[\x95P\x80a!b\x83\x89aKaV[a!l\x91\x90aKxV[\x94PPPPP[\x95P\x95\x93PPPPV[``\x81a!\x89\x81a*@V[a!\xDA\x88\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa!\xCB`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a!\xAEV[\x91P\x86\x82`\x01\x84Qa!\xEC\x91\x90aJ-V[\x81Q\x81\x10a!\xFCWa!\xFCaJFV[` \x02` \x01\x01Q\x10\x15a\"#W`@QcB0\x1C#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\"8\x86\x86_\x81\x81\x10a\x1FhWa\x1FhaJFV[a 2\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0E\x19Wa\"z`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\"]V[____\x86`\x01`\x01`\xA0\x1B\x03\x16cy\xBCW\xD5\x8B\x8B\x8B`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\"\xBD\x93\x92\x91\x90aK=V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xFC\x91\x90aJ\xF7V[\x90P_\x80\x80`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a#\x84W\x83`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#LW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#p\x91\x90aJ\xB2V[\x90Pa#~\x8D\x8D\x8D\x8Da\x17JV[\x90\x93P\x91P[\x82\x15\x80\x15a#\x90WP\x81\x15[\x15a$HW\x88\x96P\x87\x95Pa\x03\xE8a$7a#\xAB\x88\x8AaKaV[p\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Ch\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cd\xFF\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Cb\xFF\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1Cb\x01\0\0\x01`\xB5`\x01\x92\x83\x1C\x1B\x02`\x12\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[a$A\x91\x90aJ-V[\x94Pa$\xDFV[_a$T\x8A\x85\x85a8\xEDV[\x90P\x88\x81\x11a$\xA2W\x89\x97P\x95P\x85a$\x9B\x84a$q\x84\x8BaKaV[a${\x91\x90aKxV[\x84a$\x86\x85\x8BaKaV[a$\x90\x91\x90aKxV[\x80\x82\x18\x90\x82\x11\x02\x18\x90V[\x95Pa$\xDDV[_a$\xAE\x8A\x85\x87a8\xEDV[\x98P\x89\x97P\x88\x90Pa$\xD9\x85a$\xC4\x85\x84aKaV[a$\xCE\x91\x90aKxV[\x85a$\x86\x86\x8CaKaV[\x96PP[P[PPPP\x96P\x96P\x96\x93PPPPV[__\x82a$\xFB\x81a*@V[a%+\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x8B\x8B\x8B0\x8Ba\x07yV[\x90\x93P\x91Pa%;\x8A\x86\x85a9TV[a%E\x85\x83a8cV[P\x97P\x97\x95PPPPPPV[__a%a\x85\x85`\x01\x86a\x13\xB5V[\x90P_\x85`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xC4\x91\x90aK\x97V[a%\xCF\x90`\naL\x9AV[\x90P_\x85`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&2\x91\x90aK\x97V[a&=\x90`\naL\x9AV[`@Qcx\xA0Q\xAD`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`$\x83\x01R\x91\x92P\x83\x91_\x91\x90\x86\x16\x90c\xF1@\xA3Z\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x90W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xB4\x91\x90aJ\xB2V[\x90P__a&\xC7\x8B\x8B`\x01\x8C\x88\x88a\"\x89V[P\x90\x92P\x90P\x85a&\xE0\x83g\r\xE0\xB6\xB3\xA7d\0\0aKaV[a&\xEA\x91\x90aKxV[\x91P\x84a&\xFF\x82g\r\xE0\xB6\xB3\xA7d\0\0aKaV[a'\t\x91\x90aKxV[\x90P\x84a'\x1E\x84g\r\xE0\xB6\xB3\xA7d\0\0aKaV[a'(\x91\x90aKxV[\x92P\x85a'=\x85g\r\xE0\xB6\xB3\xA7d\0\0aKaV[a'G\x91\x90aKxV[\x93P\x80\x82\x85a'^\x86g\r\xE0\xB6\xB3\xA7d\0\0aKaV[a'h\x91\x90aKxV[a'r\x91\x90aKaV[a'|\x91\x90aKxV[\x97Pa'\x90\x88g\r\xE0\xB6\xB3\xA7d\0\0aJ\xC9V[a'\xA2\x85g\r\xE0\xB6\xB3\xA7d\0\0aKaV[a'\xAC\x91\x90aKxV[\x9B\x9APPPPPPPPPPPV[_\x80a'\xC7\x8A\x8CaJ\xC9V[\x90P\x8B4s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xED\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a(8W\x80\x83\x14a(\x10W`@Qc8Q\xFD\xC9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa(cV[\x80\x15a(WW`@Qc\xAEmVo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a(c\x8230\x86a*\xD1V[a(s\x82\x8E\x8E\x8E\x8E\x8E\x8E\x8Ea:4V[a(|\x8Ba<\xDFV[_a(\xC6a(\x8D` \x8E\x01\x8EaJ|V[\x8D` \x01` \x81\x01\x90a(\xA0\x91\x90aJ|V[\x8E`@\x01` \x81\x01\x90a(\xB3\x91\x90aJ\x97V[\x8F``\x01` \x81\x01\x90a\x04\x85\x91\x90aJ|V[`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x91\x92P\x90\x82\x16\x90cjbxB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\x0FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)3\x91\x90aJ\xB2V[\x94Pa)>\x8Fa6\xA8V[a)Sa)N` \x8E\x01\x8EaJ|V[a6\xA8V[a)fa)N`@\x8E\x01` \x8F\x01aJ|V[PPPP\x9A\x99PPPPPPPPPPV[_\x81a)\x83\x81a*@V[a)\xB3\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x8A\x8A\x8A0\x8Aa\x07yV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x93Pa**\x91P\x8A\x90\x86\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\x01W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*%\x91\x90aJ\xB2V[a9TV[a*4\x84\x83a8cV[P\x97\x96PPPPPPPV[B\x81\x10\x15a*aW`@Qc\x04\x07\xB0[`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R`d\x82\x01\x83\x90Ra*\xCB\x91\x86\x91\x82\x16\x90c#\xB8r\xDD\x90`\x84\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa>GV[PPPPV[_\x84`\x01`\x01`\xA0\x1B\x03\x16;\x11a*\xE6W__\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90R\x91Q_\x92\x83\x92\x90\x88\x16\x91a+I\x91\x90aL\xA8V[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a+\x82W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a+\x87V[``\x91P[P\x91P\x91P\x81\x80\x15a+\xB1WP\x80Q\x15\x80a+\xB1WP\x80\x80` \x01\x90Q\x81\x01\x90a+\xB1\x91\x90aJ\xDCV[a+\xB9W__\xFD[PPPPPPV[\x81Q_[\x81\x81\x10\x15a*\xCBW_a,\x11\x85\x83\x81Q\x81\x10a+\xE3Wa+\xE3aJFV[` \x02` \x01\x01Q_\x01Q\x86\x84\x81Q\x81\x10a,\0Wa,\0aJFV[` \x02` \x01\x01Q` \x01Qa\x0E2V[P\x90P_a,\x94\x86\x84\x81Q\x81\x10a,*Wa,*aJFV[` \x02` \x01\x01Q_\x01Q\x87\x85\x81Q\x81\x10a,GWa,GaJFV[` \x02` \x01\x01Q` \x01Q\x88\x86\x81Q\x81\x10a,eWa,eaJFV[` \x02` \x01\x01Q`@\x01Q\x89\x87\x81Q\x81\x10a,\x83Wa,\x83aJFV[` \x02` \x01\x01Q``\x01Qa\x13\xB5V[\x90P___a-\x18\x89\x87\x81Q\x81\x10a,\xAEWa,\xAEaJFV[` \x02` \x01\x01Q_\x01Q\x8A\x88\x81Q\x81\x10a,\xCBWa,\xCBaJFV[` \x02` \x01\x01Q` \x01Q\x8B\x89\x81Q\x81\x10a,\xE9Wa,\xE9aJFV[` \x02` \x01\x01Q`@\x01Q\x8C\x8A\x81Q\x81\x10a-\x07Wa-\x07aJFV[` \x02` \x01\x01Q``\x01Qa\x17JV[P\x90P\x80\x89\x87\x81Q\x81\x10a-.Wa-.aJFV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\x7FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xA3\x91\x90aJ\xB2V[a-\xAD\x91\x90aJ-V[\x92PP\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF1@\xA3Z\x83\x8A\x88\x81Q\x81\x10a-\xD2Wa-\xD2aJFV[` \x02` \x01\x01Q_\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.\x0C\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.K\x91\x90aJ\xB2V[\x90P__\x85`\x01`\x01`\xA0\x1B\x03\x16\x8A\x88\x81Q\x81\x10a.kWa.kaJFV[` \x02` \x01\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a.\x8BW\x82_a.\x8EV[_\x83[\x91P\x91P_`\x01\x8BQa.\xA1\x91\x90aJ-V[\x88\x10a.\xADW\x89a/IV[a/I\x8Ba.\xBC\x8A`\x01aJ\xC9V[\x81Q\x81\x10a.\xCCWa.\xCCaJFV[` \x02` \x01\x01Q_\x01Q\x8C\x8A`\x01a.\xE5\x91\x90aJ\xC9V[\x81Q\x81\x10a.\xF5Wa.\xF5aJFV[` \x02` \x01\x01Q` \x01Q\x8D\x8B`\x01a/\x0F\x91\x90aJ\xC9V[\x81Q\x81\x10a/\x1FWa/\x1FaJFV[` \x02` \x01\x01Q`@\x01Q\x8E\x8C`\x01a/9\x91\x90aJ\xC9V[\x81Q\x81\x10a,\x83Wa,\x83aJFV[`@\x80Q_\x81R` \x81\x01\x91\x82\x90Rc\x02,\r\x9F`\xE0\x1B\x90\x91R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x02,\r\x9F\x90a/\x8A\x90\x86\x90\x86\x90\x86\x90`$\x81\x01aL\xBEV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\xA1W__\xFD[PZ\xF1\x15\x80\x15a/\xB3W=__>=_\xFD[PP`\x01\x90\x99\x01\x98Pa+\xC5\x97PPPPPPPPV[\x81Q_[\x81\x81\x10\x15a2\x0CW_a/\xEC\x85\x83\x81Q\x81\x10a+\xE3Wa+\xE3aJFV[P\x90P_\x86a/\xFC\x84`\x01aJ\xC9V[\x81Q\x81\x10a0\x0CWa0\x0CaJFV[` \x02` \x01\x01Q\x90P__\x83`\x01`\x01`\xA0\x1B\x03\x16\x88\x86\x81Q\x81\x10a04Wa04aJFV[` \x02` \x01\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a0TW\x82_a0WV[_\x83[\x91P\x91P_`\x01\x89Qa0j\x91\x90aJ-V[\x86\x10a0vW\x87a1\x02V[a1\x02\x89a0\x85\x88`\x01aJ\xC9V[\x81Q\x81\x10a0\x95Wa0\x95aJFV[` \x02` \x01\x01Q_\x01Q\x8A\x88`\x01a0\xAE\x91\x90aJ\xC9V[\x81Q\x81\x10a0\xBEWa0\xBEaJFV[` \x02` \x01\x01Q` \x01Q\x8B\x89`\x01a0\xD8\x91\x90aJ\xC9V[\x81Q\x81\x10a0\xE8Wa0\xE8aJFV[` \x02` \x01\x01Q`@\x01Q\x8C\x8A`\x01a/9\x91\x90aJ\xC9V[\x90Pa1r\x89\x87\x81Q\x81\x10a1\x19Wa1\x19aJFV[` \x02` \x01\x01Q_\x01Q\x8A\x88\x81Q\x81\x10a16Wa16aJFV[` \x02` \x01\x01Q` \x01Q\x8B\x89\x81Q\x81\x10a1TWa1TaJFV[` \x02` \x01\x01Q`@\x01Q\x8C\x8A\x81Q\x81\x10a,\x83Wa,\x83aJFV[`\x01`\x01`\xA0\x1B\x03\x16c\x02,\r\x9F\x84\x84\x84_`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a1\xAEW` \x82\x01\x81\x806\x837\x01\x90P[P`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a1\xCE\x94\x93\x92\x91\x90aL\xBEV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a1\xE5W__\xFD[PZ\xF1\x15\x80\x15a1\xF7W=__>=_\xFD[PP`\x01\x90\x97\x01\x96Pa/\xCE\x95PPPPPPV[PPPPPV[__\x83\x86\x10\x15a26W`@Qcn5\x97y`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x85\x10\x15a2WW`@Qc\xAC\xEE\x05\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcy\xBCW\xD5`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cy\xBCW\xD5\x90a2\xA9\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01aK=V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xC4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\xE8\x91\x90aJ\xF7V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a3\x8BW`@Qc\x01\xB5\xFC\xAD`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c6\xBF\x95\xA0\x90a3H\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01aK=V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a3dW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x88\x91\x90aJ\xF7V[\x90P[__a3\xB9\x8C\x8C\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x17JV[\x91P\x91P\x81_\x14\x80\x15a3\xCAWP\x80\x15[\x15a3\xDAW\x88\x94P\x87\x93Pa4bV[_a3\xE6\x8A\x84\x84a8\xEDV[\x90P\x88\x81\x11a4\x1BW\x86\x81\x10\x15a4\x10W`@Qc\r2A\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x89\x95P\x93P\x83a4`V[_a4'\x8A\x84\x86a8\xEDV[\x90P\x8A\x81\x11\x15a49Wa49aM\x10V[\x88\x81\x10\x15a4ZW`@Qc#\xD9\xBB\x05`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95P\x88\x94P[P[PPP\x97P\x97\x95PPPPPPV[_a4\x7F` \x83\x01\x83aJ|V[\x90P_a4\x92`@\x84\x01` \x85\x01aJ|V[\x90P_a4\xBA\x83\x83a4\xAA``\x88\x01`@\x89\x01aJ\x97V[a\x04\x85`\x80\x89\x01``\x8A\x01aJ|V[\x90Pa4\xD1`\x01`\x01`\xA0\x1B\x03\x82\x163\x83\x88a*dV[_a4\xDC\x84\x84a\x0E2V[P`@Qc\"k\xF2\xD1`\xE2\x1B\x81R0`\x04\x82\x01R\x90\x91P_\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x89\xAF\xCBD\x90`$\x01`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a5&W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5J\x91\x90aJZV[\x91P\x91P__\x84`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16\x14a5pW\x82\x84a5sV[\x83\x83[\x91P\x91P\x88`\xC0\x015\x82\x10\x15a5\x9CW`@Qc#\xD9\xBB\x05`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x88`\xE0\x015\x81\x10\x15a5\xC1W`@Qc\r2A\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPV[_a5\xD8\x84\x83a\x0E\xBCV[\x90P\x82\x81`\x01\x83Qa5\xEA\x91\x90aJ-V[\x81Q\x81\x10a5\xFAWa5\xFAaJFV[` \x02` \x01\x01Q\x10\x15a6!W`@QcB0\x1C#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a6\x90\x83_\x81Q\x81\x10a67Wa67aJFV[` \x02` \x01\x01Q_\x01Q\x84_\x81Q\x81\x10a6TWa6TaJFV[` \x02` \x01\x01Q` \x01Q\x85_\x81Q\x81\x10a6rWa6raJFV[` \x02` \x01\x01Q`@\x01Q\x86_\x81Q\x81\x10a,\x83Wa,\x83aJFV[\x90Pa6\x9D\x86\x82\x87a9TV[a+\xB9\x82\x840a/\xCAV[3_s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xED\x19`\x01`\x01`\xA0\x1B\x03\x84\x16\x01a7\xE1W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a71W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7U\x91\x90aJ\xB2V[\x90P\x80\x15a7\xDCW`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c.\x1A}M\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a7\xBCW__\xFD[PZ\xF1\x15\x80\x15a7\xCEW=__>=_\xFD[PPPPa7\xDC\x82\x82a8cV[PPPV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8G\x91\x90aJ\xB2V[\x90P\x80\x15a7\xDCWa7\xDC`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a>\xB7V[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Qa8\x8C\x91\x90aL\xA8V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a8\xC6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a8\xCBV[``\x91P[PP\x90P\x80a7\xDCW`@Qc\xB1-\x13\xEB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83_\x03a9\x0EW`@Qc,\xA2\xF5+`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x15\x80a9\x19WP\x81\x15[\x15a97W`@Qc\xBBU\xFD'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a9B\x83\x86aKaV[a9L\x91\x90aKxV[\x94\x93PPPPV[_\x83`\x01`\x01`\xA0\x1B\x03\x16;\x11a9iW__\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q_\x92\x83\x92\x90\x87\x16\x91a9\xC4\x91\x90aL\xA8V[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a9\xFDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a:\x02V[``\x91P[P\x91P\x91P\x81\x80\x15a:,WP\x80Q\x15\x80a:,WP\x80\x80` \x01\x90Q\x81\x01\x90a:,\x91\x90aJ\xDCV[a2\x0CW__\xFD[_a:B` \x87\x01\x87aJ|V[\x90P_a:U`@\x88\x01` \x89\x01aJ|V[\x90P_a:h``\x89\x01`@\x8A\x01aJ\x97V[\x90P_a:{`\x80\x8A\x01``\x8B\x01aJ|V[\x90P_a:\x8A\x85\x85\x85\x85a\x13\xB5V[\x90P__\x82`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\xCAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\xEE\x91\x90aK\x12V[P\x91P\x91Pa\x03\xE8\x82\x11\x15\x80a;\x06WPa\x03\xE8\x81\x11\x15[\x15a;$W`@Qc\x02r\x1E\x1F`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x84`\x01`\x01`\xA0\x1B\x03\x16\x8D`\x01`\x01`\xA0\x1B\x03\x16\x14a;\xFBW`\x01`\x01`\xA0\x1B\x03\x85\x16\x89\x89a;V`\x01\x82aJ-V[\x81\x81\x10a;eWa;eaJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a;}\x91\x90aJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a;\xA4W`@Qc\t\xD4\x1Cg`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a;\xFB\x8D\x8D\x8C`\x80\x015\x8C\x8C\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1B)Wa;\xEC`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a;\xCFV[\x83`\x01`\x01`\xA0\x1B\x03\x16\x8D`\x01`\x01`\xA0\x1B\x03\x16\x14a<\xD0W`\x01`\x01`\xA0\x1B\x03\x84\x16\x87\x87a<+`\x01\x82aJ-V[\x81\x81\x10a<:Wa<:aJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a<R\x91\x90aJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a<yW`@Qc2\xB2A\x03`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a<\xD0\x8D\x8C\x8C`\xA0\x015\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1B)Wa<\xC1`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a<\xA4V[PPPPPPPPPPPPPV[_a<\xED` \x83\x01\x83aJ|V[\x90P_a=\0`@\x84\x01` \x85\x01aJ|V[\x90P_a=\x13``\x85\x01`@\x86\x01aJ\x97V[\x90P_a=&`\x80\x86\x01``\x87\x01aJ|V[\x90P_a=5\x85\x85\x85\x85a\x13\xB5V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P_\x90\x81\x90a>#\x90\x88\x90\x88\x90\x88\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\x8AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xAE\x91\x90aJ\xB2V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8D\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\x14\x91\x90aJ\xB2V[\x8E`\xC0\x015\x8F`\xE0\x015a>\xE8V[\x91P\x91Pa>2\x87\x84\x84a9TV[a>=\x86\x84\x83a9TV[PPPPPPPPV[__` _\x84Q` \x86\x01_\x88Z\xF1\x80a>fW`@Q=_\x82>=\x81\xFD[PP_Q=\x91P\x81\x15a>}W\x80`\x01\x14\x15a>\x8AV[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a*\xCBW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90Ra7\xDC\x91\x85\x91\x82\x16\x90c\xA9\x05\x9C\xBB\x90`d\x01a*\x99V[__\x83\x86\x10\x15a?\x0BW`@Qcn5\x97y`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x85\x10\x15a?,W`@Qc\xAC\xEE\x05\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a?:\x8C\x8C\x8C\x8Ca\x17JV[\x91P\x91P\x81_\x14\x80\x15a?KWP\x80\x15[\x15a?[W\x87\x93P\x86\x92Pa?\xE3V[_a?g\x89\x84\x84a8\xEDV[\x90P\x87\x81\x11a?\x9CW\x85\x81\x10\x15a?\x91W`@Qc\r2A\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x88\x94P\x92P\x82a?\xE1V[_a?\xA8\x89\x84\x86a8\xEDV[\x90P\x89\x81\x11\x15a?\xBAWa?\xBAaM\x10V[\x87\x81\x10\x15a?\xDBW`@Qc#\xD9\xBB\x05`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x94P\x87\x93P[P[PP\x98P\x98\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a*aW__\xFD[\x805a@\x11\x81a?\xF2V[\x91\x90PV[\x80\x15\x15\x81\x14a*aW__\xFD[__\x83`\x1F\x84\x01\x12a@3W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a@IW__\xFD[` \x83\x01\x91P\x83` \x82`\x07\x1B\x85\x01\x01\x11\x15a\x0E\xB5W__\xFD[__________a\x01\0\x8B\x8D\x03\x12\x15a@}W__\xFD[\x8A5a@\x88\x81a?\xF2V[\x99P` \x8B\x015a@\x98\x81a?\xF2V[\x98P`@\x8B\x015a@\xA8\x81a@\x16V[\x97P``\x8B\x015a@\xB8\x81a?\xF2V[\x96P`\x80\x8B\x015\x95P`\xA0\x8B\x015\x94P`\xC0\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xE0W__\xFD[a@\xEC\x8D\x82\x8E\x01a@#V[\x90\x95P\x93PP`\xE0\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aA\nW__\xFD[aA\x16\x8D\x82\x8E\x01a@#V[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[________a\x01\0\x89\x8B\x03\x12\x15aAGW__\xFD[\x885aAR\x81a?\xF2V[\x97P` \x89\x015aAb\x81a?\xF2V[\x96P`@\x89\x015aAr\x81a@\x16V[\x95P``\x89\x015\x94P`\x80\x89\x015\x93P`\xA0\x89\x015\x92P`\xC0\x89\x015aA\x97\x81a?\xF2V[\x97\x9A\x96\x99P\x94\x97\x93\x96\x92\x95\x91\x94P\x91\x92`\xE0\x015\x91PV[______`\xA0\x87\x89\x03\x12\x15aA\xC4W__\xFD[\x865\x95P` \x87\x015\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xE7W__\xFD[aA\xF3\x89\x82\x8A\x01a@#V[\x90\x95P\x93PP``\x87\x015aB\x07\x81a?\xF2V[\x95\x98\x94\x97P\x92\x95\x91\x94\x93`\x80\x90\x92\x015\x92PPV[_____`\x80\x86\x88\x03\x12\x15aB0W__\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aBLW__\xFD[aBX\x88\x82\x89\x01a@#V[\x90\x95P\x93PP`@\x86\x015aBl\x81a?\xF2V[\x94\x97\x93\x96P\x91\x94``\x015\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\xB9WaB\xB9aB}V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aB\xD9WaB\xD9aB}V[P`\x05\x1B` \x01\x90V[_____`\x80\x86\x88\x03\x12\x15aB\xF7W__\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15aC\x0CW__\xFD[\x86\x01`\x1F\x81\x01\x88\x13aC\x1CW__\xFD[\x805aC/aC*\x82aB\xC1V[aB\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x8A\x83\x11\x15aCPW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aCrW\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aCWV[\x97PPPP` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aC\x8FW__\xFD[aC\x9B\x88\x82\x89\x01a@#V[\x90\x95P\x93PaBl\x90P`@\x87\x01a@\x06V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aC\xE5W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aC\xC7V[P\x90\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15aD\x01W__\xFD[\x825aD\x0C\x81a?\xF2V[\x91P` \x83\x015aD\x1C\x81a?\xF2V[\x80\x91PP\x92P\x92\x90PV[_`\x80\x82\x84\x03\x12\x15aD7W__\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aDYWaDYaB}V[`@R\x90P\x80\x825aDj\x81a?\xF2V[\x81R` \x83\x015aDz\x81a?\xF2V[` \x82\x01R`@\x83\x015aD\x8D\x81a@\x16V[`@\x82\x01R``\x83\x015aD\xA0\x81a?\xF2V[``\x91\x90\x91\x01R\x92\x91PPV[__`@\x83\x85\x03\x12\x15aD\xBEW__\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xDAW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aD\xEAW__\xFD[\x805aD\xF8aC*\x82aB\xC1V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x07\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aE\x19W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aEEWaE2\x88\x85aD'V[\x82R` \x82\x01\x91P`\x80\x84\x01\x93PaE V[\x80\x94PPPPP\x92P\x92\x90PV[_________a\x01 \x8A\x8C\x03\x12\x15aElW__\xFD[\x895aEw\x81a?\xF2V[\x98P` \x8A\x015aE\x87\x81a?\xF2V[\x97P`@\x8A\x015aE\x97\x81a@\x16V[\x96P``\x8A\x015\x95P`\x80\x8A\x015\x94P`\xA0\x8A\x015\x93P`\xC0\x8A\x015\x92P`\xE0\x8A\x015aE\xC3\x81a?\xF2V[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x93\x96\x92\x95P\x90\x93a\x01\0\x015\x91\x90PV[_________`\xE0\x8A\x8C\x03\x12\x15aE\xF5W__\xFD[\x895aF\0\x81a?\xF2V[\x98P` \x8A\x015aF\x10\x81a?\xF2V[\x97P`@\x8A\x015aF \x81a@\x16V[\x96P``\x8A\x015aF0\x81a?\xF2V[\x95P`\x80\x8A\x015\x94P`\xA0\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aFQW__\xFD[aF]\x8C\x82\x8D\x01a@#V[\x90\x95P\x93PP`\xC0\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF{W__\xFD[aF\x87\x8C\x82\x8D\x01a@#V[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[____`\x80\x85\x87\x03\x12\x15aF\xB1W__\xFD[\x845aF\xBC\x81a?\xF2V[\x93P` \x85\x015aF\xCC\x81a?\xF2V[\x92P`@\x85\x015aF\xDC\x81a@\x16V[\x91P``\x85\x015aF\xEC\x81a?\xF2V[\x93\x96\x92\x95P\x90\x93PPV[_a\x01\0\x82\x84\x03\x12\x15aG\x08W__\xFD[P\x91\x90PV[_______a\x01\x80\x88\x8A\x03\x12\x15aG%W__\xFD[\x875aG0\x81a?\xF2V[\x96P` \x88\x015\x95PaGF\x89`@\x8A\x01aF\xF7V[\x94Pa\x01@\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aGaW__\xFD[aGm\x8A\x82\x8B\x01a@#V[\x90\x95P\x93PPa\x01`\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aG\x8CW__\xFD[aG\x98\x8A\x82\x8B\x01a@#V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[_______`\xE0\x88\x8A\x03\x12\x15aG\xC1W__\xFD[\x875aG\xCC\x81a?\xF2V[\x96P` \x88\x015aG\xDC\x81a@\x16V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015\x92P`\xA0\x88\x015aH\x01\x81a?\xF2V[\x96\x99\x95\x98P\x93\x96\x92\x95\x91\x94\x91\x93PP`\xC0\x90\x91\x015\x90V[_____`\xA0\x86\x88\x03\x12\x15aH-W__\xFD[\x855aH8\x81a?\xF2V[\x94P` \x86\x015aHH\x81a?\xF2V[\x93P`@\x86\x015aHX\x81a@\x16V[\x92P``\x86\x015aHh\x81a?\xF2V[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[______`\xC0\x87\x89\x03\x12\x15aH\x8EW__\xFD[\x865aH\x99\x81a?\xF2V[\x95P` \x87\x015aH\xA9\x81a?\xF2V[\x94P`@\x87\x015aH\xB9\x81a@\x16V[\x93P``\x87\x015aH\xC9\x81a?\xF2V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[___``\x84\x86\x03\x12\x15aH\xF5W__\xFD[\x835aI\0\x81a?\xF2V[\x92P` \x84\x015aI\x10\x81a?\xF2V[\x91P`@\x84\x015aI \x81a?\xF2V[\x80\x91PP\x92P\x92P\x92V[__________a\x01\xE0\x8B\x8D\x03\x12\x15aIEW__\xFD[\x8A5aIP\x81a?\xF2V[\x99P` \x8B\x015\x98P`@\x8B\x015\x97PaIm\x8C``\x8D\x01aF\xF7V[\x96Pa\x01`\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x88W__\xFD[aI\x94\x8D\x82\x8E\x01a@#V[\x90\x97P\x95PPa\x01\x80\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xB3W__\xFD[aI\xBF\x8D\x82\x8E\x01a@#V[\x90\x95P\x93PPa\x01\xA0\x8B\x015aI\xD4\x81a?\xF2V[\x91Pa\x01\xC0\x8B\x015aI\xE5\x81a@\x16V[\x80\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[_`\x80\x82\x84\x03\x12\x15aJ\x08W__\xFD[aJ\x12\x83\x83aD'V[\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15aJ@WaJ@aJ\x19V[\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[__`@\x83\x85\x03\x12\x15aJkW__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[_` \x82\x84\x03\x12\x15aJ\x8CW__\xFD[\x815aJ\x12\x81a?\xF2V[_` \x82\x84\x03\x12\x15aJ\xA7W__\xFD[\x815aJ\x12\x81a@\x16V[_` \x82\x84\x03\x12\x15aJ\xC2W__\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15aJ@WaJ@aJ\x19V[_` \x82\x84\x03\x12\x15aJ\xECW__\xFD[\x81QaJ\x12\x81a@\x16V[_` \x82\x84\x03\x12\x15aK\x07W__\xFD[\x81QaJ\x12\x81a?\xF2V[___``\x84\x86\x03\x12\x15aK$W__\xFD[PP\x81Q` \x83\x01Q`@\x90\x93\x01Q\x90\x94\x92\x93P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R\x90\x15\x15`@\x82\x01R``\x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17aJ@WaJ@aJ\x19V[_\x82aK\x92WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15aK\xA7W__\xFD[\x81Q`\xFF\x81\x16\x81\x14aJ\x12W__\xFD[`\x01\x81[`\x01\x84\x11\x15aK\xF2W\x80\x85\x04\x81\x11\x15aK\xD6WaK\xD6aJ\x19V[`\x01\x84\x16\x15aK\xE4W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aK\xBBV[\x93P\x93\x91PPV[_\x82aL\x08WP`\x01aJ@V[\x81aL\x14WP_aJ@V[\x81`\x01\x81\x14aL*W`\x02\x81\x14aL4WaLPV[`\x01\x91PPaJ@V[`\xFF\x84\x11\x15aLEWaLEaJ\x19V[PP`\x01\x82\x1BaJ@V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aLsWP\x81\x81\naJ@V[aL\x7F_\x19\x84\x84aK\xB7V[\x80_\x19\x04\x82\x11\x15aL\x92WaL\x92aJ\x19V[\x02\x93\x92PPPV[_aJ\x12`\xFF\x84\x16\x83aK\xFAV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[\x84\x81R\x83` \x82\x01R`\x01\x80`\xA0\x1B\x03\x83\x16`@\x82\x01R`\x80``\x82\x01R_\x82Q\x80`\x80\x84\x01R\x80` \x85\x01`\xA0\x85\x01^_`\xA0\x82\x85\x01\x01R`\xA0`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static MOCKROUTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\xB2W_5`\xE0\x1C\x80c\x88\xCD\x82\x1E\x11a\0\xE7W\x80c\xCA\xC8\x8E\xA9\x11a\0\x87W\x80c\xD7\xB0\xE0\xA5\x11a\0bW\x80c\xD7\xB0\xE0\xA5\x14a\x05\xBCW\x80c\xF5\xBAS\xC7\x14a\x05\xDBW\x80c\xFBI\xBA\xFD\x14a\x06\x08W\x80c\xFEA\x1F\x14\x14a\x06\x1BW__\xFD[\x80c\xCA\xC8\x8E\xA9\x14a\x05KW\x80c\xCEp\x0C)\x14a\x05jW\x80c\xD4\xB6\x84m\x14a\x05\x89W__\xFD[\x80c\xA8\x1B\x91Y\x11a\0\xC2W\x80c\xA8\x1B\x91Y\x14a\x04\xDBW\x80c\xB7\xE0\xD4\xC0\x14a\x04\xFAW\x80c\xC6\xB7\xF1\xB6\x14a\x05\rW\x80c\xC9-\xE3\xEC\x14a\x05,W__\xFD[\x80c\x88\xCD\x82\x1E\x14a\x04\x8AW\x80c\x8C\x007\xDC\x14a\x04\xA9W\x80c\x9068\xA4\x14a\x04\xC8W__\xFD[\x80cB\xCB\x1F\xBC\x11a\x01RW\x80cU\t\xA1\xAC\x11a\x01-W\x80cU\t\xA1\xAC\x14a\x03\xF3W\x80cZG\xDD\xC3\x14a\x04\x12W\x80cu9\xD4\x13\x14a\x04LW\x80c\x87@)\xD9\x14a\x04kW__\xFD[\x80cB\xCB\x1F\xBC\x14a\x03ZW\x80cF\xC9j\xAC\x14a\x03\x81W\x80cTL\xAAV\x14a\x03\xB4W__\xFD[\x80c;\xF0\xC9\xFB\x11a\x01\x8DW\x80c;\xF0\xC9\xFB\x14a\x02\x9DW\x80c=\xA5\xAC\xBA\x14a\x02\xE8W\x80c?\xC8\xCE\xF3\x14a\x02\xFBW\x80cA\x11\xD5\x97\x14a\x03.W__\xFD[\x80c\x07\xDBP\xFA\x14a\x02\x06W\x80c\r\xED\xE6\xC4\x14a\x02JW\x80c\x12\xBC:\xCA\x14a\x02~W__\xFD[6a\x02\x02W3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\0W`@Qc\x01\xF1\x80\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\0[__\xFD[4\x80\x15a\x02\x11W__\xFD[Pa\x02%a\x02 6`\x04a@cV[a\x06:V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02UW__\xFD[Pa\x02ia\x02d6`\x04aA/V[a\x07yV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02AV[4\x80\x15a\x02\x89W__\xFD[Pa\x02\0a\x02\x986`\x04aA\xAFV[a\x08\xC3V[4\x80\x15a\x02\xA8W__\xFD[Pa\x02\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02AV[a\x02\0a\x02\xF66`\x04aB\x1CV[a\n\x91V[4\x80\x15a\x03\x06W__\xFD[Pa\x02\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x039W__\xFD[Pa\x03Ma\x03H6`\x04aB\xE3V[a\x0C\xE0V[`@Qa\x02A\x91\x90aC\xAEV[4\x80\x15a\x03eW__\xFD[Pa\x02\xD0s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x81V[4\x80\x15a\x03\x8CW__\xFD[Pa\x02\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\xBFW__\xFD[Pa\x03\xD3a\x03\xCE6`\x04aC\xF0V[a\x0E2V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x02AV[4\x80\x15a\x03\xFEW__\xFD[Pa\x03Ma\x04\r6`\x04aD\xADV[a\x0E\xBCV[4\x80\x15a\x04\x1DW__\xFD[Pa\x041a\x04,6`\x04aESV[a\x11\x9CV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02AV[4\x80\x15a\x04WW__\xFD[Pa\x02%a\x04f6`\x04aE\xDDV[a\x12\x80V[4\x80\x15a\x04vW__\xFD[Pa\x02\xD0a\x04\x856`\x04aF\x9EV[a\x13\xB5V[4\x80\x15a\x04\x95W__\xFD[Pa\x02\0a\x04\xA46`\x04aA\xAFV[a\x15\xCFV[4\x80\x15a\x04\xB4W__\xFD[Pa\x02ia\x04\xC36`\x04aF\x9EV[a\x17JV[a\x03Ma\x04\xD66`\x04aB\x1CV[a\x17\xFFV[4\x80\x15a\x04\xE6W__\xFD[Pa\x02\0a\x04\xF56`\x04aG\x0EV[a\x19oV[a\x041a\x05\x086`\x04aG\xABV[a\x1C\x86V[4\x80\x15a\x05\x18W__\xFD[Pa\x03Ma\x05'6`\x04aA\xAFV[a\x1E(V[4\x80\x15a\x057W__\xFD[Pa\x02ia\x05F6`\x04aH\x19V[a =V[4\x80\x15a\x05VW__\xFD[Pa\x03Ma\x05e6`\x04aA\xAFV[a!}V[4\x80\x15a\x05uW__\xFD[Pa\x041a\x05\x846`\x04aHyV[a\"\x89V[4\x80\x15a\x05\x94W__\xFD[Pa\x02\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\xC7W__\xFD[Pa\x02ia\x05\xD66`\x04aG\xABV[a$\xEFV[4\x80\x15a\x05\xE6W__\xFD[Pa\x05\xFAa\x05\xF56`\x04aH\xE3V[a%RV[`@Q\x90\x81R` \x01a\x02AV[a\x05\xFAa\x06\x166`\x04aI+V[a'\xBBV[4\x80\x15a\x06&W__\xFD[Pa\x05\xFAa\x0656`\x04aG\xABV[a)xV[\x85\x85_\x80``\x87\x15a\x06\xCDWa\x06\xA1\x8B\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa\x06\x88`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x06kV[PPPPPa\x0E\xBCV[\x90P\x80`\x01\x82Qa\x06\xB2\x91\x90aJ-V[\x81Q\x81\x10a\x06\xC2Wa\x06\xC2aJFV[` \x02` \x01\x01Q\x94P[\x85\x15a\x07PWa\x07$\x8A\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa\x07\x15`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x06\xF8V[\x90P\x80`\x01\x82Qa\x075\x91\x90aJ-V[\x81Q\x81\x10a\x07EWa\x07EaJFV[` \x02` \x01\x01Q\x93P[a\x07^\x8F\x8F\x8F\x8F\x89\x89a\"\x89V[P\x80\x93P\x81\x94PPPP\x9AP\x9AP\x9AP\x9A\x96PPPPPPPV[__\x82a\x07\x85\x81a*@V[_a\x07\xB2\x8C\x8C\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13\xB5V[\x90Pa\x07\xC9`\x01`\x01`\xA0\x1B\x03\x82\x163\x83\x8Ca*dV[`@Qc\"k\xF2\xD1`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R_\x91\x82\x91\x84\x16\x90c\x89\xAF\xCBD\x90`$\x01`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\x11W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x085\x91\x90aJZV[\x91P\x91P_a\x08D\x8F\x8Fa\x0E2V[P\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x8F`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08gW\x81\x83a\x08jV[\x82\x82[\x90\x97P\x95P\x8A\x87\x10\x15a\x08\x90W`@Qc#\xD9\xBB\x05`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x89\x86\x10\x15a\x08\xB1W`@Qc\r2A\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPP\x98P\x98\x96PPPPPPPV[\x80a\x08\xCD\x81a*@V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85\x85a\t\x04`\x01\x82aJ-V[\x81\x81\x10a\t\x13Wa\t\x13aJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\t+\x91\x90aJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\tRW`@Qc \xDB\x82g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n-\x85\x85_\x81\x81\x10a\tgWa\tgaJFV[a\t}\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[3a\n'\x88\x88_\x81\x81\x10a\t\x93Wa\t\x93aJFV[a\t\xA9\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[\x89\x89_\x81\x81\x10a\t\xBBWa\t\xBBaJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\t\xD3\x91\x90aJ|V[\x8A\x8A_\x81\x81\x10a\t\xE5Wa\t\xE5aJFV[\x90P`\x80\x02\x01`@\x01` \x81\x01\x90a\t\xFD\x91\x90aJ\x97V[\x8B\x8B_\x81\x81\x10a\n\x0FWa\n\x0FaJFV[\x90P`\x80\x02\x01``\x01` \x81\x01\x90a\x04\x85\x91\x90aJ|V[\x8Aa*\xD1V[a\n\x88\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\n}Wa\nn`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\nQV[PPPPP0a+\xC1V[PPPPPPPV[\x80a\n\x9B\x81a*@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x85\x85_\x81\x81\x10a\n\xD7Wa\n\xD7aJFV[a\n\xED\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\x14W`@Qc \xDB\x82g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[4_a\x0B!`\x01\x87aJ-V[\x90P_\x87\x87\x83\x81\x81\x10a\x0B6Wa\x0B6aJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x0BN\x91\x90aJ|V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xB8\x91\x90aJ\xB2V[\x90Pa\x0C\x15\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\nWa\x0B\xFB`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x0B\xDEV[PPPPP\x87a+\xC1V[\x88\x81\x89\x89\x85\x81\x81\x10a\x0C)Wa\x0C)aJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x0CA\x91\x90aJ|V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xAC\x91\x90aJ\xB2V[a\x0C\xB6\x91\x90aJ-V[\x10\x15a\x0C\xD5W`@QcB0\x1C#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPV[``\x81a\x0C\xEC\x81a*@V[a\r\xC8\x86\x86_\x81\x81\x10a\r\x01Wa\r\x01aJFV[a\r\x17\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[3a\r\xA9\x89\x89_\x81\x81\x10a\r-Wa\r-aJFV[a\rC\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[\x8A\x8A_\x81\x81\x10a\rUWa\rUaJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\rm\x91\x90aJ|V[\x8B\x8B_\x81\x81\x10a\r\x7FWa\r\x7FaJFV[\x90P`\x80\x02\x01`@\x01` \x81\x01\x90a\r\x97\x91\x90aJ\x97V[\x8C\x8C_\x81\x81\x10a\n\x0FWa\n\x0FaJFV[\x8A_\x81Q\x81\x10a\r\xBBWa\r\xBBaJFV[` \x02` \x01\x01Qa*\xD1V[a\x0E$\x87\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0E\x19Wa\x0E\n`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\r\xEDV[PPPPP\x86a/\xCAV[\x86\x91P[P\x95\x94PPPPPV[__\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0EfW`@Qc2\x95\xF3\xFD`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0E\x86W\x82\x84a\x0E\x89V[\x83\x83[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0E\xB5W`@Qc\xD9.#=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x92P\x92\x90PV[```\x01\x82Q\x10\x15a\x0E\xE1W`@Qc \xDB\x82g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Qa\x0E\xEE\x90`\x01aJ\xC9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\x05Wa\x0F\x05aB}V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F.W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x82\x81_\x81Q\x81\x10a\x0FDWa\x0FDaJFV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81Q_[\x81\x81\x10\x15a\x11\x94W__`\x01`\x01`\xA0\x1B\x03\x16\x85\x83\x81Q\x81\x10a\x0FxWa\x0FxaJFV[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\xB5W\x84\x82\x81Q\x81\x10a\x0F\xA4Wa\x0F\xA4aJFV[` \x02` \x01\x01Q``\x01Qa\x0F\xD7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[\x90P_a\x10<\x86\x84\x81Q\x81\x10a\x0F\xEFWa\x0F\xEFaJFV[` \x02` \x01\x01Q_\x01Q\x87\x85\x81Q\x81\x10a\x10\x0CWa\x10\x0CaJFV[` \x02` \x01\x01Q` \x01Q\x88\x86\x81Q\x81\x10a\x10*Wa\x10*aJFV[` \x02` \x01\x01Q`@\x01Q\x85a\x13\xB5V[`@Qc[\x16\xEB\xB7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x91\x92P\x90\x83\x16\x90c[\x16\xEB\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xA8\x91\x90aJ\xDCV[\x15a\x11\x8AW\x80`\x01`\x01`\xA0\x1B\x03\x16c\xF1@\xA3Z\x86\x85\x81Q\x81\x10a\x10\xCEWa\x10\xCEaJFV[` \x02` \x01\x01Q\x88\x86\x81Q\x81\x10a\x10\xE8Wa\x10\xE8aJFV[` \x02` \x01\x01Q_\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\"\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11a\x91\x90aJ\xB2V[\x85a\x11m\x85`\x01aJ\xC9V[\x81Q\x81\x10a\x11}Wa\x11}aJFV[` \x02` \x01\x01\x81\x81RPP[PP`\x01\x01a\x0FSV[PP\x92\x91PPV[___\x83a\x11\xA9\x81a*@V[a\x11\xB8\x8D\x8D\x8D\x8D\x8D\x8D\x8Da2\x13V[\x90\x94P\x92P_a\x11\xEA\x8E\x8E\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13\xB5V[\x90Pa\x11\xF8\x8E3\x83\x88a*\xD1V[a\x12\x04\x8D3\x83\x87a*\xD1V[`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x82\x16\x90cjbxB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12IW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12m\x91\x90aJ\xB2V[\x92PPP\x99P\x99P\x99\x96PPPPPPPV[____a\x12\x91\x8D\x8D\x8D\x8D\x8Da =V[\x90\x94P\x92P\x83\x91P\x82\x90P``\x87\x15a\x13!Wa\x12\xF5\x83\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa\x12\xE6`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x12\xC9V[\x90P\x80`\x01\x82Qa\x13\x06\x91\x90aJ-V[\x81Q\x81\x10a\x13\x16Wa\x13\x16aJFV[` \x02` \x01\x01Q\x94P[\x85\x15a\x13\xA4Wa\x13x\x82\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa\x13i`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x13LV[\x90P\x80`\x01\x82Qa\x13\x89\x91\x90aJ-V[\x81Q\x81\x10a\x13\x99Wa\x13\x99aJFV[` \x02` \x01\x01Q\x93P[P\x99P\x99P\x99P\x99\x95PPPPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x13\xEDW\x83a\x13\xEFV[\x81[`@Qc\xD1\xEA\n\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xD1\xEA\n\x1D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14XW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14|\x91\x90aJ\xDCV[a\x14\x99W`@QcM9\xD5\xA3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x14\xA5\x89\x89a\x0E2V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x81\x1B\x82\x16` \x84\x01R\x83\x90\x1B\x16`4\x82\x01R\x89\x15\x15`\xF8\x1B`H\x82\x01R\x91\x93P\x91P_\x90`I\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x15\xC1\x84`\x01`\x01`\xA0\x1B\x03\x16c\\`\xDA\x1B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x157W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15[\x91\x90aJ\xF7V[`@Q`8\x81\x01\x87\x90RoZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\xFF`$\x82\x01R`\x14\x81\x01\x91\x90\x91Rs=`-\x80`\n=9\x81\xF36==7===6=s\x81R`X\x81\x01\x83\x90R`7`\x0C\x82\x01 `x\x82\x01R`U`C\x90\x91\x01 `\x01`\x01`\xA0\x1B\x03\x16\x90V[\x9A\x99PPPPPPPPPPV[\x80a\x15\xD9\x81a*@V[a\x15\xEE\x85\x85_\x81\x81\x10a\tgWa\tgaJFV[_a\x15\xFA`\x01\x86aJ-V[\x90P_\x86\x86\x83\x81\x81\x10a\x16\x0FWa\x16\x0FaJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x16'\x91\x90aJ|V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16mW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x91\x91\x90aJ\xB2V[\x90Pa\x16\xEE\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16\xE3Wa\x16\xD4`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x16\xB7V[PPPPP\x86a+\xC1V[\x87\x81\x88\x88\x85\x81\x81\x10a\x17\x02Wa\x17\x02aJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x17\x1A\x91\x90aJ|V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01a\x0CmV[___a\x17W\x87\x87a\x0E2V[P\x90P__a\x17h\x89\x89\x89\x89a\x13\xB5V[`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xA3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xC7\x91\x90aK\x12V[P\x91P\x91P\x82`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17\xECW\x80\x82a\x17\xEFV[\x81\x81[\x90\x9A\x90\x99P\x97PPPPPPPPV[``\x81a\x18\x0B\x81a*@V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x86\x86_\x81\x81\x10a\x18GWa\x18GaJFV[a\x18]\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x84W`@Qc \xDB\x82g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\xD54\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa\x18\xC6`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x18\xA9V[\x91P\x86\x82`\x01\x84Qa\x18\xE7\x91\x90aJ-V[\x81Q\x81\x10a\x18\xF7Wa\x18\xF7aJFV[` \x02` \x01\x01Q\x10\x15a\x19\x1EW`@QcB0\x1C#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E(\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0E\x19Wa\x19``\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x19CV[_a\x19}` \x87\x01\x87aJ|V[\x90P_a\x19\x90`@\x88\x01` \x89\x01aJ|V[\x90P_`\x01`\x01`\xA0\x1B\x03\x8A\x16s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\x14a\x19\xBDW\x89a\x19\xDFV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[\x90Pa\x19\xEB\x89\x89a4qV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B3W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1AGW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ak\x91\x90aJ\xB2V[\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16\x88\x88a\x1A\x84`\x01\x82aJ-V[\x81\x81\x10a\x1A\x93Wa\x1A\x93aJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x1A\xAB\x91\x90aJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1A\xD2W`@Qc\t\xD4\x1Cg`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B3\x84\x82\x8B`\x80\x015\x8B\x8B\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1B)Wa\x1B\x1A`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x1A\xFDV[PPPPPa5\xCDV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1CpW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xB2\x91\x90aJ\xB2V[\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16\x86\x86a\x1B\xCB`\x01\x82aJ-V[\x81\x81\x10a\x1B\xDAWa\x1B\xDAaJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x1B\xF2\x91\x90aJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\x19W`@Qc2\xB2A\x03`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1Cp\x83\x82\x8B`\xA0\x015\x89\x89\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1B)Wa\x1Ca`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x1CDV[a\x1Cy\x8Ba6\xA8V[PPPPPPPPPPPV[___\x83a\x1C\x93\x81a*@V[a\x1C\xC2\x8B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8C\x8C4\x8D\x8Da2\x13V[\x90\x94P\x92P_a\x1D\x14\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13\xB5V[\x90Pa\x1D\"\x8C3\x83\x88a*\xD1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x1D{W__\xFD[PZ\xF1\x15\x80\x15a\x1D\x8DW=__>=_\xFD[PP`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16`\x04\x83\x01R\x85\x16\x93PcjbxB\x92P`$\x01\x90P` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xFC\x91\x90aJ\xB2V[\x92P\x834\x11\x15a\x1E\x19Wa\x1E\x193a\x1E\x14\x864aJ-V[a8cV[PP\x97P\x97P\x97\x94PPPPPV[``\x81a\x1E4\x81a*@V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x86a\x1Ek`\x01\x82aJ-V[\x81\x81\x10a\x1EzWa\x1EzaJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x1E\x92\x91\x90aJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E\xB9W`@Qc \xDB\x82g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\n\x88\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa\x1E\xFB`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x1E\xDEV[\x91P\x86\x82`\x01\x84Qa\x1F\x1C\x91\x90aJ-V[\x81Q\x81\x10a\x1F,Wa\x1F,aJFV[` \x02` \x01\x01Q\x10\x15a\x1FSW`@QcB0\x1C#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\xA6\x86\x86_\x81\x81\x10a\x1FhWa\x1FhaJFV[a\x1F~\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaJ|V[3a\x1F\x94\x89\x89_\x81\x81\x10a\r-Wa\r-aJFV[\x85_\x81Q\x81\x10a\r\xBBWa\r\xBBaJFV[a \x02\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1F\xF7Wa\x1F\xE8`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\x1F\xCBV[PPPPP0a/\xCAV[a 2\x84\x83`\x01\x85Qa \x15\x91\x90aJ-V[\x81Q\x81\x10a %Wa %aJFV[` \x02` \x01\x01Qa8cV[P\x96\x95PPPPPPV[___\x84`\x01`\x01`\xA0\x1B\x03\x16cy\xBCW\xD5\x89\x89\x89`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a p\x93\x92\x91\x90aK=V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xAF\x91\x90aJ\xF7V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a \xCBW__\x92P\x92PPa!sV[__a \xD9\x8A\x8A\x8A\x8Aa\x17JV[\x91P\x91P_\x83`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!>\x91\x90aJ\xB2V[\x90P\x80a!K\x84\x89aKaV[a!U\x91\x90aKxV[\x95P\x80a!b\x83\x89aKaV[a!l\x91\x90aKxV[\x94PPPPP[\x95P\x95\x93PPPPV[``\x81a!\x89\x81a*@V[a!\xDA\x88\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x06\x97Wa!\xCB`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a!\xAEV[\x91P\x86\x82`\x01\x84Qa!\xEC\x91\x90aJ-V[\x81Q\x81\x10a!\xFCWa!\xFCaJFV[` \x02` \x01\x01Q\x10\x15a\"#W`@QcB0\x1C#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\"8\x86\x86_\x81\x81\x10a\x1FhWa\x1FhaJFV[a 2\x82\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0E\x19Wa\"z`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a\"]V[____\x86`\x01`\x01`\xA0\x1B\x03\x16cy\xBCW\xD5\x8B\x8B\x8B`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\"\xBD\x93\x92\x91\x90aK=V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xFC\x91\x90aJ\xF7V[\x90P_\x80\x80`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a#\x84W\x83`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#LW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#p\x91\x90aJ\xB2V[\x90Pa#~\x8D\x8D\x8D\x8Da\x17JV[\x90\x93P\x91P[\x82\x15\x80\x15a#\x90WP\x81\x15[\x15a$HW\x88\x96P\x87\x95Pa\x03\xE8a$7a#\xAB\x88\x8AaKaV[p\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Ch\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cd\xFF\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Cb\xFF\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1Cb\x01\0\0\x01`\xB5`\x01\x92\x83\x1C\x1B\x02`\x12\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[a$A\x91\x90aJ-V[\x94Pa$\xDFV[_a$T\x8A\x85\x85a8\xEDV[\x90P\x88\x81\x11a$\xA2W\x89\x97P\x95P\x85a$\x9B\x84a$q\x84\x8BaKaV[a${\x91\x90aKxV[\x84a$\x86\x85\x8BaKaV[a$\x90\x91\x90aKxV[\x80\x82\x18\x90\x82\x11\x02\x18\x90V[\x95Pa$\xDDV[_a$\xAE\x8A\x85\x87a8\xEDV[\x98P\x89\x97P\x88\x90Pa$\xD9\x85a$\xC4\x85\x84aKaV[a$\xCE\x91\x90aKxV[\x85a$\x86\x86\x8CaKaV[\x96PP[P[PPPP\x96P\x96P\x96\x93PPPPV[__\x82a$\xFB\x81a*@V[a%+\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x8B\x8B\x8B0\x8Ba\x07yV[\x90\x93P\x91Pa%;\x8A\x86\x85a9TV[a%E\x85\x83a8cV[P\x97P\x97\x95PPPPPPV[__a%a\x85\x85`\x01\x86a\x13\xB5V[\x90P_\x85`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xC4\x91\x90aK\x97V[a%\xCF\x90`\naL\x9AV[\x90P_\x85`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&2\x91\x90aK\x97V[a&=\x90`\naL\x9AV[`@Qcx\xA0Q\xAD`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`$\x83\x01R\x91\x92P\x83\x91_\x91\x90\x86\x16\x90c\xF1@\xA3Z\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x90W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xB4\x91\x90aJ\xB2V[\x90P__a&\xC7\x8B\x8B`\x01\x8C\x88\x88a\"\x89V[P\x90\x92P\x90P\x85a&\xE0\x83g\r\xE0\xB6\xB3\xA7d\0\0aKaV[a&\xEA\x91\x90aKxV[\x91P\x84a&\xFF\x82g\r\xE0\xB6\xB3\xA7d\0\0aKaV[a'\t\x91\x90aKxV[\x90P\x84a'\x1E\x84g\r\xE0\xB6\xB3\xA7d\0\0aKaV[a'(\x91\x90aKxV[\x92P\x85a'=\x85g\r\xE0\xB6\xB3\xA7d\0\0aKaV[a'G\x91\x90aKxV[\x93P\x80\x82\x85a'^\x86g\r\xE0\xB6\xB3\xA7d\0\0aKaV[a'h\x91\x90aKxV[a'r\x91\x90aKaV[a'|\x91\x90aKxV[\x97Pa'\x90\x88g\r\xE0\xB6\xB3\xA7d\0\0aJ\xC9V[a'\xA2\x85g\r\xE0\xB6\xB3\xA7d\0\0aKaV[a'\xAC\x91\x90aKxV[\x9B\x9APPPPPPPPPPPV[_\x80a'\xC7\x8A\x8CaJ\xC9V[\x90P\x8B4s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xED\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a(8W\x80\x83\x14a(\x10W`@Qc8Q\xFD\xC9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa(cV[\x80\x15a(WW`@Qc\xAEmVo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a(c\x8230\x86a*\xD1V[a(s\x82\x8E\x8E\x8E\x8E\x8E\x8E\x8Ea:4V[a(|\x8Ba<\xDFV[_a(\xC6a(\x8D` \x8E\x01\x8EaJ|V[\x8D` \x01` \x81\x01\x90a(\xA0\x91\x90aJ|V[\x8E`@\x01` \x81\x01\x90a(\xB3\x91\x90aJ\x97V[\x8F``\x01` \x81\x01\x90a\x04\x85\x91\x90aJ|V[`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x91\x92P\x90\x82\x16\x90cjbxB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\x0FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)3\x91\x90aJ\xB2V[\x94Pa)>\x8Fa6\xA8V[a)Sa)N` \x8E\x01\x8EaJ|V[a6\xA8V[a)fa)N`@\x8E\x01` \x8F\x01aJ|V[PPPP\x9A\x99PPPPPPPPPPV[_\x81a)\x83\x81a*@V[a)\xB3\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x8A\x8A\x8A0\x8Aa\x07yV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x93Pa**\x91P\x8A\x90\x86\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\x01W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*%\x91\x90aJ\xB2V[a9TV[a*4\x84\x83a8cV[P\x97\x96PPPPPPPV[B\x81\x10\x15a*aW`@Qc\x04\x07\xB0[`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R`d\x82\x01\x83\x90Ra*\xCB\x91\x86\x91\x82\x16\x90c#\xB8r\xDD\x90`\x84\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa>GV[PPPPV[_\x84`\x01`\x01`\xA0\x1B\x03\x16;\x11a*\xE6W__\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90R\x91Q_\x92\x83\x92\x90\x88\x16\x91a+I\x91\x90aL\xA8V[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a+\x82W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a+\x87V[``\x91P[P\x91P\x91P\x81\x80\x15a+\xB1WP\x80Q\x15\x80a+\xB1WP\x80\x80` \x01\x90Q\x81\x01\x90a+\xB1\x91\x90aJ\xDCV[a+\xB9W__\xFD[PPPPPPV[\x81Q_[\x81\x81\x10\x15a*\xCBW_a,\x11\x85\x83\x81Q\x81\x10a+\xE3Wa+\xE3aJFV[` \x02` \x01\x01Q_\x01Q\x86\x84\x81Q\x81\x10a,\0Wa,\0aJFV[` \x02` \x01\x01Q` \x01Qa\x0E2V[P\x90P_a,\x94\x86\x84\x81Q\x81\x10a,*Wa,*aJFV[` \x02` \x01\x01Q_\x01Q\x87\x85\x81Q\x81\x10a,GWa,GaJFV[` \x02` \x01\x01Q` \x01Q\x88\x86\x81Q\x81\x10a,eWa,eaJFV[` \x02` \x01\x01Q`@\x01Q\x89\x87\x81Q\x81\x10a,\x83Wa,\x83aJFV[` \x02` \x01\x01Q``\x01Qa\x13\xB5V[\x90P___a-\x18\x89\x87\x81Q\x81\x10a,\xAEWa,\xAEaJFV[` \x02` \x01\x01Q_\x01Q\x8A\x88\x81Q\x81\x10a,\xCBWa,\xCBaJFV[` \x02` \x01\x01Q` \x01Q\x8B\x89\x81Q\x81\x10a,\xE9Wa,\xE9aJFV[` \x02` \x01\x01Q`@\x01Q\x8C\x8A\x81Q\x81\x10a-\x07Wa-\x07aJFV[` \x02` \x01\x01Q``\x01Qa\x17JV[P\x90P\x80\x89\x87\x81Q\x81\x10a-.Wa-.aJFV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\x7FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xA3\x91\x90aJ\xB2V[a-\xAD\x91\x90aJ-V[\x92PP\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF1@\xA3Z\x83\x8A\x88\x81Q\x81\x10a-\xD2Wa-\xD2aJFV[` \x02` \x01\x01Q_\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.\x0C\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.K\x91\x90aJ\xB2V[\x90P__\x85`\x01`\x01`\xA0\x1B\x03\x16\x8A\x88\x81Q\x81\x10a.kWa.kaJFV[` \x02` \x01\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a.\x8BW\x82_a.\x8EV[_\x83[\x91P\x91P_`\x01\x8BQa.\xA1\x91\x90aJ-V[\x88\x10a.\xADW\x89a/IV[a/I\x8Ba.\xBC\x8A`\x01aJ\xC9V[\x81Q\x81\x10a.\xCCWa.\xCCaJFV[` \x02` \x01\x01Q_\x01Q\x8C\x8A`\x01a.\xE5\x91\x90aJ\xC9V[\x81Q\x81\x10a.\xF5Wa.\xF5aJFV[` \x02` \x01\x01Q` \x01Q\x8D\x8B`\x01a/\x0F\x91\x90aJ\xC9V[\x81Q\x81\x10a/\x1FWa/\x1FaJFV[` \x02` \x01\x01Q`@\x01Q\x8E\x8C`\x01a/9\x91\x90aJ\xC9V[\x81Q\x81\x10a,\x83Wa,\x83aJFV[`@\x80Q_\x81R` \x81\x01\x91\x82\x90Rc\x02,\r\x9F`\xE0\x1B\x90\x91R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x02,\r\x9F\x90a/\x8A\x90\x86\x90\x86\x90\x86\x90`$\x81\x01aL\xBEV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\xA1W__\xFD[PZ\xF1\x15\x80\x15a/\xB3W=__>=_\xFD[PP`\x01\x90\x99\x01\x98Pa+\xC5\x97PPPPPPPPV[\x81Q_[\x81\x81\x10\x15a2\x0CW_a/\xEC\x85\x83\x81Q\x81\x10a+\xE3Wa+\xE3aJFV[P\x90P_\x86a/\xFC\x84`\x01aJ\xC9V[\x81Q\x81\x10a0\x0CWa0\x0CaJFV[` \x02` \x01\x01Q\x90P__\x83`\x01`\x01`\xA0\x1B\x03\x16\x88\x86\x81Q\x81\x10a04Wa04aJFV[` \x02` \x01\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a0TW\x82_a0WV[_\x83[\x91P\x91P_`\x01\x89Qa0j\x91\x90aJ-V[\x86\x10a0vW\x87a1\x02V[a1\x02\x89a0\x85\x88`\x01aJ\xC9V[\x81Q\x81\x10a0\x95Wa0\x95aJFV[` \x02` \x01\x01Q_\x01Q\x8A\x88`\x01a0\xAE\x91\x90aJ\xC9V[\x81Q\x81\x10a0\xBEWa0\xBEaJFV[` \x02` \x01\x01Q` \x01Q\x8B\x89`\x01a0\xD8\x91\x90aJ\xC9V[\x81Q\x81\x10a0\xE8Wa0\xE8aJFV[` \x02` \x01\x01Q`@\x01Q\x8C\x8A`\x01a/9\x91\x90aJ\xC9V[\x90Pa1r\x89\x87\x81Q\x81\x10a1\x19Wa1\x19aJFV[` \x02` \x01\x01Q_\x01Q\x8A\x88\x81Q\x81\x10a16Wa16aJFV[` \x02` \x01\x01Q` \x01Q\x8B\x89\x81Q\x81\x10a1TWa1TaJFV[` \x02` \x01\x01Q`@\x01Q\x8C\x8A\x81Q\x81\x10a,\x83Wa,\x83aJFV[`\x01`\x01`\xA0\x1B\x03\x16c\x02,\r\x9F\x84\x84\x84_`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a1\xAEW` \x82\x01\x81\x806\x837\x01\x90P[P`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a1\xCE\x94\x93\x92\x91\x90aL\xBEV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a1\xE5W__\xFD[PZ\xF1\x15\x80\x15a1\xF7W=__>=_\xFD[PP`\x01\x90\x97\x01\x96Pa/\xCE\x95PPPPPPV[PPPPPV[__\x83\x86\x10\x15a26W`@Qcn5\x97y`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x85\x10\x15a2WW`@Qc\xAC\xEE\x05\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcy\xBCW\xD5`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cy\xBCW\xD5\x90a2\xA9\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01aK=V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xC4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\xE8\x91\x90aJ\xF7V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a3\x8BW`@Qc\x01\xB5\xFC\xAD`\xE5\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c6\xBF\x95\xA0\x90a3H\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01aK=V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a3dW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x88\x91\x90aJ\xF7V[\x90P[__a3\xB9\x8C\x8C\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x17JV[\x91P\x91P\x81_\x14\x80\x15a3\xCAWP\x80\x15[\x15a3\xDAW\x88\x94P\x87\x93Pa4bV[_a3\xE6\x8A\x84\x84a8\xEDV[\x90P\x88\x81\x11a4\x1BW\x86\x81\x10\x15a4\x10W`@Qc\r2A\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x89\x95P\x93P\x83a4`V[_a4'\x8A\x84\x86a8\xEDV[\x90P\x8A\x81\x11\x15a49Wa49aM\x10V[\x88\x81\x10\x15a4ZW`@Qc#\xD9\xBB\x05`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95P\x88\x94P[P[PPP\x97P\x97\x95PPPPPPV[_a4\x7F` \x83\x01\x83aJ|V[\x90P_a4\x92`@\x84\x01` \x85\x01aJ|V[\x90P_a4\xBA\x83\x83a4\xAA``\x88\x01`@\x89\x01aJ\x97V[a\x04\x85`\x80\x89\x01``\x8A\x01aJ|V[\x90Pa4\xD1`\x01`\x01`\xA0\x1B\x03\x82\x163\x83\x88a*dV[_a4\xDC\x84\x84a\x0E2V[P`@Qc\"k\xF2\xD1`\xE2\x1B\x81R0`\x04\x82\x01R\x90\x91P_\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x89\xAF\xCBD\x90`$\x01`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a5&W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5J\x91\x90aJZV[\x91P\x91P__\x84`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16\x14a5pW\x82\x84a5sV[\x83\x83[\x91P\x91P\x88`\xC0\x015\x82\x10\x15a5\x9CW`@Qc#\xD9\xBB\x05`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x88`\xE0\x015\x81\x10\x15a5\xC1W`@Qc\r2A\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPV[_a5\xD8\x84\x83a\x0E\xBCV[\x90P\x82\x81`\x01\x83Qa5\xEA\x91\x90aJ-V[\x81Q\x81\x10a5\xFAWa5\xFAaJFV[` \x02` \x01\x01Q\x10\x15a6!W`@QcB0\x1C#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a6\x90\x83_\x81Q\x81\x10a67Wa67aJFV[` \x02` \x01\x01Q_\x01Q\x84_\x81Q\x81\x10a6TWa6TaJFV[` \x02` \x01\x01Q` \x01Q\x85_\x81Q\x81\x10a6rWa6raJFV[` \x02` \x01\x01Q`@\x01Q\x86_\x81Q\x81\x10a,\x83Wa,\x83aJFV[\x90Pa6\x9D\x86\x82\x87a9TV[a+\xB9\x82\x840a/\xCAV[3_s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xED\x19`\x01`\x01`\xA0\x1B\x03\x84\x16\x01a7\xE1W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a71W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7U\x91\x90aJ\xB2V[\x90P\x80\x15a7\xDCW`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c.\x1A}M\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a7\xBCW__\xFD[PZ\xF1\x15\x80\x15a7\xCEW=__>=_\xFD[PPPPa7\xDC\x82\x82a8cV[PPPV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8G\x91\x90aJ\xB2V[\x90P\x80\x15a7\xDCWa7\xDC`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a>\xB7V[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Qa8\x8C\x91\x90aL\xA8V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a8\xC6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a8\xCBV[``\x91P[PP\x90P\x80a7\xDCW`@Qc\xB1-\x13\xEB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83_\x03a9\x0EW`@Qc,\xA2\xF5+`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x15\x80a9\x19WP\x81\x15[\x15a97W`@Qc\xBBU\xFD'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a9B\x83\x86aKaV[a9L\x91\x90aKxV[\x94\x93PPPPV[_\x83`\x01`\x01`\xA0\x1B\x03\x16;\x11a9iW__\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q_\x92\x83\x92\x90\x87\x16\x91a9\xC4\x91\x90aL\xA8V[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a9\xFDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a:\x02V[``\x91P[P\x91P\x91P\x81\x80\x15a:,WP\x80Q\x15\x80a:,WP\x80\x80` \x01\x90Q\x81\x01\x90a:,\x91\x90aJ\xDCV[a2\x0CW__\xFD[_a:B` \x87\x01\x87aJ|V[\x90P_a:U`@\x88\x01` \x89\x01aJ|V[\x90P_a:h``\x89\x01`@\x8A\x01aJ\x97V[\x90P_a:{`\x80\x8A\x01``\x8B\x01aJ|V[\x90P_a:\x8A\x85\x85\x85\x85a\x13\xB5V[\x90P__\x82`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\xCAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\xEE\x91\x90aK\x12V[P\x91P\x91Pa\x03\xE8\x82\x11\x15\x80a;\x06WPa\x03\xE8\x81\x11\x15[\x15a;$W`@Qc\x02r\x1E\x1F`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x84`\x01`\x01`\xA0\x1B\x03\x16\x8D`\x01`\x01`\xA0\x1B\x03\x16\x14a;\xFBW`\x01`\x01`\xA0\x1B\x03\x85\x16\x89\x89a;V`\x01\x82aJ-V[\x81\x81\x10a;eWa;eaJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a;}\x91\x90aJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a;\xA4W`@Qc\t\xD4\x1Cg`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a;\xFB\x8D\x8D\x8C`\x80\x015\x8C\x8C\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1B)Wa;\xEC`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a;\xCFV[\x83`\x01`\x01`\xA0\x1B\x03\x16\x8D`\x01`\x01`\xA0\x1B\x03\x16\x14a<\xD0W`\x01`\x01`\xA0\x1B\x03\x84\x16\x87\x87a<+`\x01\x82aJ-V[\x81\x81\x10a<:Wa<:aJFV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a<R\x91\x90aJ|V[`\x01`\x01`\xA0\x1B\x03\x16\x14a<yW`@Qc2\xB2A\x03`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a<\xD0\x8D\x8C\x8C`\xA0\x015\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1B)Wa<\xC1`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\xF8V[\x81R` \x01\x90`\x01\x01\x90a<\xA4V[PPPPPPPPPPPPPV[_a<\xED` \x83\x01\x83aJ|V[\x90P_a=\0`@\x84\x01` \x85\x01aJ|V[\x90P_a=\x13``\x85\x01`@\x86\x01aJ\x97V[\x90P_a=&`\x80\x86\x01``\x87\x01aJ|V[\x90P_a=5\x85\x85\x85\x85a\x13\xB5V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P_\x90\x81\x90a>#\x90\x88\x90\x88\x90\x88\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\x8AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xAE\x91\x90aJ\xB2V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8D\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\x14\x91\x90aJ\xB2V[\x8E`\xC0\x015\x8F`\xE0\x015a>\xE8V[\x91P\x91Pa>2\x87\x84\x84a9TV[a>=\x86\x84\x83a9TV[PPPPPPPPV[__` _\x84Q` \x86\x01_\x88Z\xF1\x80a>fW`@Q=_\x82>=\x81\xFD[PP_Q=\x91P\x81\x15a>}W\x80`\x01\x14\x15a>\x8AV[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a*\xCBW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90Ra7\xDC\x91\x85\x91\x82\x16\x90c\xA9\x05\x9C\xBB\x90`d\x01a*\x99V[__\x83\x86\x10\x15a?\x0BW`@Qcn5\x97y`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x85\x10\x15a?,W`@Qc\xAC\xEE\x05\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a?:\x8C\x8C\x8C\x8Ca\x17JV[\x91P\x91P\x81_\x14\x80\x15a?KWP\x80\x15[\x15a?[W\x87\x93P\x86\x92Pa?\xE3V[_a?g\x89\x84\x84a8\xEDV[\x90P\x87\x81\x11a?\x9CW\x85\x81\x10\x15a?\x91W`@Qc\r2A\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x88\x94P\x92P\x82a?\xE1V[_a?\xA8\x89\x84\x86a8\xEDV[\x90P\x89\x81\x11\x15a?\xBAWa?\xBAaM\x10V[\x87\x81\x10\x15a?\xDBW`@Qc#\xD9\xBB\x05`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x94P\x87\x93P[P[PP\x98P\x98\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a*aW__\xFD[\x805a@\x11\x81a?\xF2V[\x91\x90PV[\x80\x15\x15\x81\x14a*aW__\xFD[__\x83`\x1F\x84\x01\x12a@3W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a@IW__\xFD[` \x83\x01\x91P\x83` \x82`\x07\x1B\x85\x01\x01\x11\x15a\x0E\xB5W__\xFD[__________a\x01\0\x8B\x8D\x03\x12\x15a@}W__\xFD[\x8A5a@\x88\x81a?\xF2V[\x99P` \x8B\x015a@\x98\x81a?\xF2V[\x98P`@\x8B\x015a@\xA8\x81a@\x16V[\x97P``\x8B\x015a@\xB8\x81a?\xF2V[\x96P`\x80\x8B\x015\x95P`\xA0\x8B\x015\x94P`\xC0\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xE0W__\xFD[a@\xEC\x8D\x82\x8E\x01a@#V[\x90\x95P\x93PP`\xE0\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aA\nW__\xFD[aA\x16\x8D\x82\x8E\x01a@#V[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[________a\x01\0\x89\x8B\x03\x12\x15aAGW__\xFD[\x885aAR\x81a?\xF2V[\x97P` \x89\x015aAb\x81a?\xF2V[\x96P`@\x89\x015aAr\x81a@\x16V[\x95P``\x89\x015\x94P`\x80\x89\x015\x93P`\xA0\x89\x015\x92P`\xC0\x89\x015aA\x97\x81a?\xF2V[\x97\x9A\x96\x99P\x94\x97\x93\x96\x92\x95\x91\x94P\x91\x92`\xE0\x015\x91PV[______`\xA0\x87\x89\x03\x12\x15aA\xC4W__\xFD[\x865\x95P` \x87\x015\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xE7W__\xFD[aA\xF3\x89\x82\x8A\x01a@#V[\x90\x95P\x93PP``\x87\x015aB\x07\x81a?\xF2V[\x95\x98\x94\x97P\x92\x95\x91\x94\x93`\x80\x90\x92\x015\x92PPV[_____`\x80\x86\x88\x03\x12\x15aB0W__\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aBLW__\xFD[aBX\x88\x82\x89\x01a@#V[\x90\x95P\x93PP`@\x86\x015aBl\x81a?\xF2V[\x94\x97\x93\x96P\x91\x94``\x015\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\xB9WaB\xB9aB}V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aB\xD9WaB\xD9aB}V[P`\x05\x1B` \x01\x90V[_____`\x80\x86\x88\x03\x12\x15aB\xF7W__\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15aC\x0CW__\xFD[\x86\x01`\x1F\x81\x01\x88\x13aC\x1CW__\xFD[\x805aC/aC*\x82aB\xC1V[aB\x91V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x8A\x83\x11\x15aCPW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aCrW\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aCWV[\x97PPPP` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aC\x8FW__\xFD[aC\x9B\x88\x82\x89\x01a@#V[\x90\x95P\x93PaBl\x90P`@\x87\x01a@\x06V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aC\xE5W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aC\xC7V[P\x90\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15aD\x01W__\xFD[\x825aD\x0C\x81a?\xF2V[\x91P` \x83\x015aD\x1C\x81a?\xF2V[\x80\x91PP\x92P\x92\x90PV[_`\x80\x82\x84\x03\x12\x15aD7W__\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aDYWaDYaB}V[`@R\x90P\x80\x825aDj\x81a?\xF2V[\x81R` \x83\x015aDz\x81a?\xF2V[` \x82\x01R`@\x83\x015aD\x8D\x81a@\x16V[`@\x82\x01R``\x83\x015aD\xA0\x81a?\xF2V[``\x91\x90\x91\x01R\x92\x91PPV[__`@\x83\x85\x03\x12\x15aD\xBEW__\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xDAW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aD\xEAW__\xFD[\x805aD\xF8aC*\x82aB\xC1V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x07\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aE\x19W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aEEWaE2\x88\x85aD'V[\x82R` \x82\x01\x91P`\x80\x84\x01\x93PaE V[\x80\x94PPPPP\x92P\x92\x90PV[_________a\x01 \x8A\x8C\x03\x12\x15aElW__\xFD[\x895aEw\x81a?\xF2V[\x98P` \x8A\x015aE\x87\x81a?\xF2V[\x97P`@\x8A\x015aE\x97\x81a@\x16V[\x96P``\x8A\x015\x95P`\x80\x8A\x015\x94P`\xA0\x8A\x015\x93P`\xC0\x8A\x015\x92P`\xE0\x8A\x015aE\xC3\x81a?\xF2V[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x93\x96\x92\x95P\x90\x93a\x01\0\x015\x91\x90PV[_________`\xE0\x8A\x8C\x03\x12\x15aE\xF5W__\xFD[\x895aF\0\x81a?\xF2V[\x98P` \x8A\x015aF\x10\x81a?\xF2V[\x97P`@\x8A\x015aF \x81a@\x16V[\x96P``\x8A\x015aF0\x81a?\xF2V[\x95P`\x80\x8A\x015\x94P`\xA0\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aFQW__\xFD[aF]\x8C\x82\x8D\x01a@#V[\x90\x95P\x93PP`\xC0\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF{W__\xFD[aF\x87\x8C\x82\x8D\x01a@#V[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[____`\x80\x85\x87\x03\x12\x15aF\xB1W__\xFD[\x845aF\xBC\x81a?\xF2V[\x93P` \x85\x015aF\xCC\x81a?\xF2V[\x92P`@\x85\x015aF\xDC\x81a@\x16V[\x91P``\x85\x015aF\xEC\x81a?\xF2V[\x93\x96\x92\x95P\x90\x93PPV[_a\x01\0\x82\x84\x03\x12\x15aG\x08W__\xFD[P\x91\x90PV[_______a\x01\x80\x88\x8A\x03\x12\x15aG%W__\xFD[\x875aG0\x81a?\xF2V[\x96P` \x88\x015\x95PaGF\x89`@\x8A\x01aF\xF7V[\x94Pa\x01@\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aGaW__\xFD[aGm\x8A\x82\x8B\x01a@#V[\x90\x95P\x93PPa\x01`\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aG\x8CW__\xFD[aG\x98\x8A\x82\x8B\x01a@#V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[_______`\xE0\x88\x8A\x03\x12\x15aG\xC1W__\xFD[\x875aG\xCC\x81a?\xF2V[\x96P` \x88\x015aG\xDC\x81a@\x16V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015\x92P`\xA0\x88\x015aH\x01\x81a?\xF2V[\x96\x99\x95\x98P\x93\x96\x92\x95\x91\x94\x91\x93PP`\xC0\x90\x91\x015\x90V[_____`\xA0\x86\x88\x03\x12\x15aH-W__\xFD[\x855aH8\x81a?\xF2V[\x94P` \x86\x015aHH\x81a?\xF2V[\x93P`@\x86\x015aHX\x81a@\x16V[\x92P``\x86\x015aHh\x81a?\xF2V[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[______`\xC0\x87\x89\x03\x12\x15aH\x8EW__\xFD[\x865aH\x99\x81a?\xF2V[\x95P` \x87\x015aH\xA9\x81a?\xF2V[\x94P`@\x87\x015aH\xB9\x81a@\x16V[\x93P``\x87\x015aH\xC9\x81a?\xF2V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[___``\x84\x86\x03\x12\x15aH\xF5W__\xFD[\x835aI\0\x81a?\xF2V[\x92P` \x84\x015aI\x10\x81a?\xF2V[\x91P`@\x84\x015aI \x81a?\xF2V[\x80\x91PP\x92P\x92P\x92V[__________a\x01\xE0\x8B\x8D\x03\x12\x15aIEW__\xFD[\x8A5aIP\x81a?\xF2V[\x99P` \x8B\x015\x98P`@\x8B\x015\x97PaIm\x8C``\x8D\x01aF\xF7V[\x96Pa\x01`\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x88W__\xFD[aI\x94\x8D\x82\x8E\x01a@#V[\x90\x97P\x95PPa\x01\x80\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xB3W__\xFD[aI\xBF\x8D\x82\x8E\x01a@#V[\x90\x95P\x93PPa\x01\xA0\x8B\x015aI\xD4\x81a?\xF2V[\x91Pa\x01\xC0\x8B\x015aI\xE5\x81a@\x16V[\x80\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[_`\x80\x82\x84\x03\x12\x15aJ\x08W__\xFD[aJ\x12\x83\x83aD'V[\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15aJ@WaJ@aJ\x19V[\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[__`@\x83\x85\x03\x12\x15aJkW__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[_` \x82\x84\x03\x12\x15aJ\x8CW__\xFD[\x815aJ\x12\x81a?\xF2V[_` \x82\x84\x03\x12\x15aJ\xA7W__\xFD[\x815aJ\x12\x81a@\x16V[_` \x82\x84\x03\x12\x15aJ\xC2W__\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15aJ@WaJ@aJ\x19V[_` \x82\x84\x03\x12\x15aJ\xECW__\xFD[\x81QaJ\x12\x81a@\x16V[_` \x82\x84\x03\x12\x15aK\x07W__\xFD[\x81QaJ\x12\x81a?\xF2V[___``\x84\x86\x03\x12\x15aK$W__\xFD[PP\x81Q` \x83\x01Q`@\x90\x93\x01Q\x90\x94\x92\x93P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R\x90\x15\x15`@\x82\x01R``\x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17aJ@WaJ@aJ\x19V[_\x82aK\x92WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15aK\xA7W__\xFD[\x81Q`\xFF\x81\x16\x81\x14aJ\x12W__\xFD[`\x01\x81[`\x01\x84\x11\x15aK\xF2W\x80\x85\x04\x81\x11\x15aK\xD6WaK\xD6aJ\x19V[`\x01\x84\x16\x15aK\xE4W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aK\xBBV[\x93P\x93\x91PPV[_\x82aL\x08WP`\x01aJ@V[\x81aL\x14WP_aJ@V[\x81`\x01\x81\x14aL*W`\x02\x81\x14aL4WaLPV[`\x01\x91PPaJ@V[`\xFF\x84\x11\x15aLEWaLEaJ\x19V[PP`\x01\x82\x1BaJ@V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aLsWP\x81\x81\naJ@V[aL\x7F_\x19\x84\x84aK\xB7V[\x80_\x19\x04\x82\x11\x15aL\x92WaL\x92aJ\x19V[\x02\x93\x92PPPV[_aJ\x12`\xFF\x84\x16\x83aK\xFAV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[\x84\x81R\x83` \x82\x01R`\x01\x80`\xA0\x1B\x03\x83\x16`@\x82\x01R`\x80``\x82\x01R_\x82Q\x80`\x80\x84\x01R\x80` \x85\x01`\xA0\x85\x01^_`\xA0\x82\x85\x01\x01R`\xA0`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static MOCKROUTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockRouter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockRouter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockRouter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockRouter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockRouter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockRouter)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockRouter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKROUTER_ABI.clone(),
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
                MOCKROUTER_ABI.clone(),
                MOCKROUTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ETHER` (0x42cb1fbc) function
        pub fn ether(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([66, 203, 31, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNSAFE_swapExactTokensForTokens` (0x4111d597) function
        pub fn unsafe_swap_exact_tokens_for_tokens(
            &self,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            routes: ::std::vec::Vec<Route>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([65, 17, 213, 151], (amounts, routes, to, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidity` (0x5a47ddc3) function
        pub fn add_liquidity(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            stable: bool,
            amount_a_desired: ::ethers::core::types::U256,
            amount_b_desired: ::ethers::core::types::U256,
            amount_a_min: ::ethers::core::types::U256,
            amount_b_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [90, 71, 221, 195],
                    (
                        token_a,
                        token_b,
                        stable,
                        amount_a_desired,
                        amount_b_desired,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidityETH` (0xb7e0d4c0) function
        pub fn add_liquidity_eth(
            &self,
            token: ::ethers::core::types::Address,
            stable: bool,
            amount_token_desired: ::ethers::core::types::U256,
            amount_token_min: ::ethers::core::types::U256,
            amount_eth_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [183, 224, 212, 192],
                    (
                        token,
                        stable,
                        amount_token_desired,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultFactory` (0xd4b6846d) function
        pub fn default_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([212, 182, 132, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factoryRegistry` (0x3bf0c9fb) function
        pub fn factory_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([59, 240, 201, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `generateZapInParams` (0x07db50fa) function
        pub fn generate_zap_in_params(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            stable: bool,
            factory: ::ethers::core::types::Address,
            amount_in_a: ::ethers::core::types::U256,
            amount_in_b: ::ethers::core::types::U256,
            routes_a: ::std::vec::Vec<Route>,
            routes_b: ::std::vec::Vec<Route>,
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
                .method_hash(
                    [7, 219, 80, 250],
                    (
                        token_a,
                        token_b,
                        stable,
                        factory,
                        amount_in_a,
                        amount_in_b,
                        routes_a,
                        routes_b,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `generateZapOutParams` (0x7539d413) function
        pub fn generate_zap_out_params(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            stable: bool,
            factory: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            routes_a: ::std::vec::Vec<Route>,
            routes_b: ::std::vec::Vec<Route>,
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
                .method_hash(
                    [117, 57, 212, 19],
                    (token_a, token_b, stable, factory, liquidity, routes_a, routes_b),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountsOut` (0x5509a1ac) function
        pub fn get_amounts_out(
            &self,
            amount_in: ::ethers::core::types::U256,
            routes: ::std::vec::Vec<Route>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([85, 9, 161, 172], (amount_in, routes))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserves` (0x8c0037dc) function
        pub fn get_reserves(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            stable: bool,
            factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([140, 0, 55, 220], (token_a, token_b, stable, factory))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolFor` (0x874029d9) function
        pub fn pool_for(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            stable: bool,
            factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([135, 64, 41, 217], (token_a, token_b, stable, factory))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteAddLiquidity` (0xce700c29) function
        pub fn quote_add_liquidity(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            stable: bool,
            factory: ::ethers::core::types::Address,
            amount_a_desired: ::ethers::core::types::U256,
            amount_b_desired: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [206, 112, 12, 41],
                    (
                        token_a,
                        token_b,
                        stable,
                        factory,
                        amount_a_desired,
                        amount_b_desired,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteRemoveLiquidity` (0xc92de3ec) function
        pub fn quote_remove_liquidity(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            stable: bool,
            factory: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [201, 45, 227, 236],
                    (token_a, token_b, stable, factory, liquidity),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteStableLiquidityRatio` (0xf5ba53c7) function
        pub fn quote_stable_liquidity_ratio(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([245, 186, 83, 199], (token_a, token_b, factory))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidity` (0x0dede6c4) function
        pub fn remove_liquidity(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            stable: bool,
            liquidity: ::ethers::core::types::U256,
            amount_a_min: ::ethers::core::types::U256,
            amount_b_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [13, 237, 230, 196],
                    (
                        token_a,
                        token_b,
                        stable,
                        liquidity,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidityETH` (0xd7b0e0a5) function
        pub fn remove_liquidity_eth(
            &self,
            token: ::ethers::core::types::Address,
            stable: bool,
            liquidity: ::ethers::core::types::U256,
            amount_token_min: ::ethers::core::types::U256,
            amount_eth_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [215, 176, 224, 165],
                    (
                        token,
                        stable,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidityETHSupportingFeeOnTransferTokens` (0xfe411f14) function
        pub fn remove_liquidity_eth_supporting_fee_on_transfer_tokens(
            &self,
            token: ::ethers::core::types::Address,
            stable: bool,
            liquidity: ::ethers::core::types::U256,
            amount_token_min: ::ethers::core::types::U256,
            amount_eth_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [254, 65, 31, 20],
                    (
                        token,
                        stable,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sortTokens` (0x544caa56) function
        pub fn sort_tokens(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::Address),
        > {
            self.0
                .method_hash([84, 76, 170, 86], (token_a, token_b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactETHForTokens` (0x903638a4) function
        pub fn swap_exact_eth_for_tokens(
            &self,
            amount_out_min: ::ethers::core::types::U256,
            routes: ::std::vec::Vec<Route>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([144, 54, 56, 164], (amount_out_min, routes, to, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactETHForTokensSupportingFeeOnTransferTokens` (0x3da5acba) function
        pub fn swap_exact_eth_for_tokens_supporting_fee_on_transfer_tokens(
            &self,
            amount_out_min: ::ethers::core::types::U256,
            routes: ::std::vec::Vec<Route>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 165, 172, 186], (amount_out_min, routes, to, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForETH` (0xc6b7f1b6) function
        pub fn swap_exact_tokens_for_eth(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            routes: ::std::vec::Vec<Route>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [198, 183, 241, 182],
                    (amount_in, amount_out_min, routes, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForETHSupportingFeeOnTransferTokens` (0x12bc3aca) function
        pub fn swap_exact_tokens_for_eth_supporting_fee_on_transfer_tokens(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            routes: ::std::vec::Vec<Route>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [18, 188, 58, 202],
                    (amount_in, amount_out_min, routes, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForTokens` (0xcac88ea9) function
        pub fn swap_exact_tokens_for_tokens(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            routes: ::std::vec::Vec<Route>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [202, 200, 142, 169],
                    (amount_in, amount_out_min, routes, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForTokensSupportingFeeOnTransferTokens` (0x88cd821e) function
        pub fn swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            routes: ::std::vec::Vec<Route>,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [136, 205, 130, 30],
                    (amount_in, amount_out_min, routes, to, deadline),
                )
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
        ///Calls the contract's `weth` (0x3fc8cef3) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([63, 200, 206, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `zapIn` (0xfb49bafd) function
        pub fn zap_in(
            &self,
            token_in: ::ethers::core::types::Address,
            amount_in_a: ::ethers::core::types::U256,
            amount_in_b: ::ethers::core::types::U256,
            zap_in_pool: Zap,
            routes_a: ::std::vec::Vec<Route>,
            routes_b: ::std::vec::Vec<Route>,
            to: ::ethers::core::types::Address,
            stake: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [251, 73, 186, 253],
                    (
                        token_in,
                        amount_in_a,
                        amount_in_b,
                        zap_in_pool,
                        routes_a,
                        routes_b,
                        to,
                        stake,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `zapOut` (0xa81b9159) function
        pub fn zap_out(
            &self,
            token_out: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            zap_out_pool: Zap,
            routes_a: ::std::vec::Vec<Route>,
            routes_b: ::std::vec::Vec<Route>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [168, 27, 145, 89],
                    (token_out, liquidity, zap_out_pool, routes_a, routes_b),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockRouter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ETHTransferFailed` with signature `ETHTransferFailed()` and selector `0xb12d13eb`
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
    #[etherror(name = "ETHTransferFailed", abi = "ETHTransferFailed()")]
    pub struct ETHTransferFailed;
    ///Custom Error type `Expired` with signature `Expired()` and selector `0x203d82d8`
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
    #[etherror(name = "Expired", abi = "Expired()")]
    pub struct Expired;
    ///Custom Error type `InsufficientAmount` with signature `InsufficientAmount()` and selector `0x5945ea56`
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
    #[etherror(name = "InsufficientAmount", abi = "InsufficientAmount()")]
    pub struct InsufficientAmount;
    ///Custom Error type `InsufficientAmountA` with signature `InsufficientAmountA()` and selector `0x8f66ec14`
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
    #[etherror(name = "InsufficientAmountA", abi = "InsufficientAmountA()")]
    pub struct InsufficientAmountA;
    ///Custom Error type `InsufficientAmountADesired` with signature `InsufficientAmountADesired()` and selector `0xdc6b2ef2`
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
        name = "InsufficientAmountADesired",
        abi = "InsufficientAmountADesired()"
    )]
    pub struct InsufficientAmountADesired;
    ///Custom Error type `InsufficientAmountAOptimal` with signature `InsufficientAmountAOptimal()` and selector `0xfe496df6`
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
        name = "InsufficientAmountAOptimal",
        abi = "InsufficientAmountAOptimal()"
    )]
    pub struct InsufficientAmountAOptimal;
    ///Custom Error type `InsufficientAmountB` with signature `InsufficientAmountB()` and selector `0x34c90624`
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
    #[etherror(name = "InsufficientAmountB", abi = "InsufficientAmountB()")]
    pub struct InsufficientAmountB;
    ///Custom Error type `InsufficientAmountBDesired` with signature `InsufficientAmountBDesired()` and selector `0xacee0513`
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
        name = "InsufficientAmountBDesired",
        abi = "InsufficientAmountBDesired()"
    )]
    pub struct InsufficientAmountBDesired;
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
    ///Custom Error type `InvalidAmountInForETHDeposit` with signature `InvalidAmountInForETHDeposit()` and selector `0x70a3fb92`
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
        name = "InvalidAmountInForETHDeposit",
        abi = "InvalidAmountInForETHDeposit()"
    )]
    pub struct InvalidAmountInForETHDeposit;
    ///Custom Error type `InvalidPath` with signature `InvalidPath()` and selector `0x20db8267`
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
    #[etherror(name = "InvalidPath", abi = "InvalidPath()")]
    pub struct InvalidPath;
    ///Custom Error type `InvalidRouteA` with signature `InvalidRouteA()` and selector `0x4ea0e338`
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
    #[etherror(name = "InvalidRouteA", abi = "InvalidRouteA()")]
    pub struct InvalidRouteA;
    ///Custom Error type `InvalidRouteB` with signature `InvalidRouteB()` and selector `0xcac9040c`
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
    #[etherror(name = "InvalidRouteB", abi = "InvalidRouteB()")]
    pub struct InvalidRouteB;
    ///Custom Error type `InvalidTokenInForETHDeposit` with signature `InvalidTokenInForETHDeposit()` and selector `0xae6d566f`
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
        name = "InvalidTokenInForETHDeposit",
        abi = "InvalidTokenInForETHDeposit()"
    )]
    pub struct InvalidTokenInForETHDeposit;
    ///Custom Error type `OnlyWETH` with signature `OnlyWETH()` and selector `0x01f180c9`
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
    #[etherror(name = "OnlyWETH", abi = "OnlyWETH()")]
    pub struct OnlyWETH;
    ///Custom Error type `PoolDoesNotExist` with signature `PoolDoesNotExist()` and selector `0x9c8787c0`
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
    #[etherror(name = "PoolDoesNotExist", abi = "PoolDoesNotExist()")]
    pub struct PoolDoesNotExist;
    ///Custom Error type `PoolFactoryDoesNotExist` with signature `PoolFactoryDoesNotExist()` and selector `0x9a73ab46`
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
    #[etherror(name = "PoolFactoryDoesNotExist", abi = "PoolFactoryDoesNotExist()")]
    pub struct PoolFactoryDoesNotExist;
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
    ///Custom Error type `SameAddresses` with signature `SameAddresses()` and selector `0xca57cff4`
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
    #[etherror(name = "SameAddresses", abi = "SameAddresses()")]
    pub struct SameAddresses;
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
    pub enum MockRouterErrors {
        ETHTransferFailed(ETHTransferFailed),
        Expired(Expired),
        InsufficientAmount(InsufficientAmount),
        InsufficientAmountA(InsufficientAmountA),
        InsufficientAmountADesired(InsufficientAmountADesired),
        InsufficientAmountAOptimal(InsufficientAmountAOptimal),
        InsufficientAmountB(InsufficientAmountB),
        InsufficientAmountBDesired(InsufficientAmountBDesired),
        InsufficientLiquidity(InsufficientLiquidity),
        InsufficientOutputAmount(InsufficientOutputAmount),
        InvalidAmountInForETHDeposit(InvalidAmountInForETHDeposit),
        InvalidPath(InvalidPath),
        InvalidRouteA(InvalidRouteA),
        InvalidRouteB(InvalidRouteB),
        InvalidTokenInForETHDeposit(InvalidTokenInForETHDeposit),
        OnlyWETH(OnlyWETH),
        PoolDoesNotExist(PoolDoesNotExist),
        PoolFactoryDoesNotExist(PoolFactoryDoesNotExist),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        SameAddresses(SameAddresses),
        ZeroAddress(ZeroAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockRouterErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ETHTransferFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ETHTransferFailed(decoded));
            }
            if let Ok(decoded) = <Expired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Expired(decoded));
            }
            if let Ok(decoded) = <InsufficientAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientAmount(decoded));
            }
            if let Ok(decoded) = <InsufficientAmountA as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientAmountA(decoded));
            }
            if let Ok(decoded) = <InsufficientAmountADesired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientAmountADesired(decoded));
            }
            if let Ok(decoded) = <InsufficientAmountAOptimal as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientAmountAOptimal(decoded));
            }
            if let Ok(decoded) = <InsufficientAmountB as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientAmountB(decoded));
            }
            if let Ok(decoded) = <InsufficientAmountBDesired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientAmountBDesired(decoded));
            }
            if let Ok(decoded) = <InsufficientLiquidity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientLiquidity(decoded));
            }
            if let Ok(decoded) = <InsufficientOutputAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientOutputAmount(decoded));
            }
            if let Ok(decoded) = <InvalidAmountInForETHDeposit as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidAmountInForETHDeposit(decoded));
            }
            if let Ok(decoded) = <InvalidPath as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidPath(decoded));
            }
            if let Ok(decoded) = <InvalidRouteA as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidRouteA(decoded));
            }
            if let Ok(decoded) = <InvalidRouteB as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidRouteB(decoded));
            }
            if let Ok(decoded) = <InvalidTokenInForETHDeposit as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidTokenInForETHDeposit(decoded));
            }
            if let Ok(decoded) = <OnlyWETH as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyWETH(decoded));
            }
            if let Ok(decoded) = <PoolDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolDoesNotExist(decoded));
            }
            if let Ok(decoded) = <PoolFactoryDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolFactoryDoesNotExist(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            if let Ok(decoded) = <SameAddresses as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SameAddresses(decoded));
            }
            if let Ok(decoded) = <ZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockRouterErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ETHTransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Expired(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InsufficientAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientAmountA(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientAmountADesired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientAmountAOptimal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientAmountB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientAmountBDesired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientOutputAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAmountInForETHDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPath(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidRouteA(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidRouteB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTokenInForETHDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyWETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolFactoryDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SameAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockRouterErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ETHTransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Expired as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InsufficientAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientAmountA as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientAmountADesired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientAmountAOptimal as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientAmountB as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientAmountBDesired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientLiquidity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientOutputAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidAmountInForETHDeposit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPath as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidRouteA as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidRouteB as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTokenInForETHDeposit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyWETH as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PoolDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolFactoryDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SameAddresses as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroAddress as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockRouterErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ETHTransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Expired(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientAmountA(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientAmountADesired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientAmountAOptimal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientAmountB(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientAmountBDesired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientOutputAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidAmountInForETHDeposit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidPath(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidRouteA(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidRouteB(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTokenInForETHDeposit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyWETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolDoesNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolFactoryDoesNotExist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SameAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockRouterErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ETHTransferFailed> for MockRouterErrors {
        fn from(value: ETHTransferFailed) -> Self {
            Self::ETHTransferFailed(value)
        }
    }
    impl ::core::convert::From<Expired> for MockRouterErrors {
        fn from(value: Expired) -> Self {
            Self::Expired(value)
        }
    }
    impl ::core::convert::From<InsufficientAmount> for MockRouterErrors {
        fn from(value: InsufficientAmount) -> Self {
            Self::InsufficientAmount(value)
        }
    }
    impl ::core::convert::From<InsufficientAmountA> for MockRouterErrors {
        fn from(value: InsufficientAmountA) -> Self {
            Self::InsufficientAmountA(value)
        }
    }
    impl ::core::convert::From<InsufficientAmountADesired> for MockRouterErrors {
        fn from(value: InsufficientAmountADesired) -> Self {
            Self::InsufficientAmountADesired(value)
        }
    }
    impl ::core::convert::From<InsufficientAmountAOptimal> for MockRouterErrors {
        fn from(value: InsufficientAmountAOptimal) -> Self {
            Self::InsufficientAmountAOptimal(value)
        }
    }
    impl ::core::convert::From<InsufficientAmountB> for MockRouterErrors {
        fn from(value: InsufficientAmountB) -> Self {
            Self::InsufficientAmountB(value)
        }
    }
    impl ::core::convert::From<InsufficientAmountBDesired> for MockRouterErrors {
        fn from(value: InsufficientAmountBDesired) -> Self {
            Self::InsufficientAmountBDesired(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidity> for MockRouterErrors {
        fn from(value: InsufficientLiquidity) -> Self {
            Self::InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<InsufficientOutputAmount> for MockRouterErrors {
        fn from(value: InsufficientOutputAmount) -> Self {
            Self::InsufficientOutputAmount(value)
        }
    }
    impl ::core::convert::From<InvalidAmountInForETHDeposit> for MockRouterErrors {
        fn from(value: InvalidAmountInForETHDeposit) -> Self {
            Self::InvalidAmountInForETHDeposit(value)
        }
    }
    impl ::core::convert::From<InvalidPath> for MockRouterErrors {
        fn from(value: InvalidPath) -> Self {
            Self::InvalidPath(value)
        }
    }
    impl ::core::convert::From<InvalidRouteA> for MockRouterErrors {
        fn from(value: InvalidRouteA) -> Self {
            Self::InvalidRouteA(value)
        }
    }
    impl ::core::convert::From<InvalidRouteB> for MockRouterErrors {
        fn from(value: InvalidRouteB) -> Self {
            Self::InvalidRouteB(value)
        }
    }
    impl ::core::convert::From<InvalidTokenInForETHDeposit> for MockRouterErrors {
        fn from(value: InvalidTokenInForETHDeposit) -> Self {
            Self::InvalidTokenInForETHDeposit(value)
        }
    }
    impl ::core::convert::From<OnlyWETH> for MockRouterErrors {
        fn from(value: OnlyWETH) -> Self {
            Self::OnlyWETH(value)
        }
    }
    impl ::core::convert::From<PoolDoesNotExist> for MockRouterErrors {
        fn from(value: PoolDoesNotExist) -> Self {
            Self::PoolDoesNotExist(value)
        }
    }
    impl ::core::convert::From<PoolFactoryDoesNotExist> for MockRouterErrors {
        fn from(value: PoolFactoryDoesNotExist) -> Self {
            Self::PoolFactoryDoesNotExist(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for MockRouterErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<SameAddresses> for MockRouterErrors {
        fn from(value: SameAddresses) -> Self {
            Self::SameAddresses(value)
        }
    }
    impl ::core::convert::From<ZeroAddress> for MockRouterErrors {
        fn from(value: ZeroAddress) -> Self {
            Self::ZeroAddress(value)
        }
    }
    ///Container type for all input parameters for the `ETHER` function with signature `ETHER()` and selector `0x42cb1fbc`
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
    #[ethcall(name = "ETHER", abi = "ETHER()")]
    pub struct EtherCall;
    ///Container type for all input parameters for the `UNSAFE_swapExactTokensForTokens` function with signature `UNSAFE_swapExactTokensForTokens(uint256[],(address,address,bool,address)[],address,uint256)` and selector `0x4111d597`
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
        name = "UNSAFE_swapExactTokensForTokens",
        abi = "UNSAFE_swapExactTokensForTokens(uint256[],(address,address,bool,address)[],address,uint256)"
    )]
    pub struct UnsafeSwapExactTokensForTokensCall {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub routes: ::std::vec::Vec<Route>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `addLiquidity` function with signature `addLiquidity(address,address,bool,uint256,uint256,uint256,uint256,address,uint256)` and selector `0x5a47ddc3`
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
        name = "addLiquidity",
        abi = "addLiquidity(address,address,bool,uint256,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct AddLiquidityCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub stable: bool,
        pub amount_a_desired: ::ethers::core::types::U256,
        pub amount_b_desired: ::ethers::core::types::U256,
        pub amount_a_min: ::ethers::core::types::U256,
        pub amount_b_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `addLiquidityETH` function with signature `addLiquidityETH(address,bool,uint256,uint256,uint256,address,uint256)` and selector `0xb7e0d4c0`
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
        name = "addLiquidityETH",
        abi = "addLiquidityETH(address,bool,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct AddLiquidityETHCall {
        pub token: ::ethers::core::types::Address,
        pub stable: bool,
        pub amount_token_desired: ::ethers::core::types::U256,
        pub amount_token_min: ::ethers::core::types::U256,
        pub amount_eth_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `defaultFactory` function with signature `defaultFactory()` and selector `0xd4b6846d`
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
    #[ethcall(name = "defaultFactory", abi = "defaultFactory()")]
    pub struct DefaultFactoryCall;
    ///Container type for all input parameters for the `factoryRegistry` function with signature `factoryRegistry()` and selector `0x3bf0c9fb`
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
    #[ethcall(name = "factoryRegistry", abi = "factoryRegistry()")]
    pub struct FactoryRegistryCall;
    ///Container type for all input parameters for the `generateZapInParams` function with signature `generateZapInParams(address,address,bool,address,uint256,uint256,(address,address,bool,address)[],(address,address,bool,address)[])` and selector `0x07db50fa`
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
        name = "generateZapInParams",
        abi = "generateZapInParams(address,address,bool,address,uint256,uint256,(address,address,bool,address)[],(address,address,bool,address)[])"
    )]
    pub struct GenerateZapInParamsCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub stable: bool,
        pub factory: ::ethers::core::types::Address,
        pub amount_in_a: ::ethers::core::types::U256,
        pub amount_in_b: ::ethers::core::types::U256,
        pub routes_a: ::std::vec::Vec<Route>,
        pub routes_b: ::std::vec::Vec<Route>,
    }
    ///Container type for all input parameters for the `generateZapOutParams` function with signature `generateZapOutParams(address,address,bool,address,uint256,(address,address,bool,address)[],(address,address,bool,address)[])` and selector `0x7539d413`
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
        name = "generateZapOutParams",
        abi = "generateZapOutParams(address,address,bool,address,uint256,(address,address,bool,address)[],(address,address,bool,address)[])"
    )]
    pub struct GenerateZapOutParamsCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub stable: bool,
        pub factory: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub routes_a: ::std::vec::Vec<Route>,
        pub routes_b: ::std::vec::Vec<Route>,
    }
    ///Container type for all input parameters for the `getAmountsOut` function with signature `getAmountsOut(uint256,(address,address,bool,address)[])` and selector `0x5509a1ac`
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
        name = "getAmountsOut",
        abi = "getAmountsOut(uint256,(address,address,bool,address)[])"
    )]
    pub struct GetAmountsOutCall {
        pub amount_in: ::ethers::core::types::U256,
        pub routes: ::std::vec::Vec<Route>,
    }
    ///Container type for all input parameters for the `getReserves` function with signature `getReserves(address,address,bool,address)` and selector `0x8c0037dc`
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
    #[ethcall(name = "getReserves", abi = "getReserves(address,address,bool,address)")]
    pub struct GetReservesCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub stable: bool,
        pub factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `poolFor` function with signature `poolFor(address,address,bool,address)` and selector `0x874029d9`
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
    #[ethcall(name = "poolFor", abi = "poolFor(address,address,bool,address)")]
    pub struct PoolForCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub stable: bool,
        pub factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `quoteAddLiquidity` function with signature `quoteAddLiquidity(address,address,bool,address,uint256,uint256)` and selector `0xce700c29`
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
        name = "quoteAddLiquidity",
        abi = "quoteAddLiquidity(address,address,bool,address,uint256,uint256)"
    )]
    pub struct QuoteAddLiquidityCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub stable: bool,
        pub factory: ::ethers::core::types::Address,
        pub amount_a_desired: ::ethers::core::types::U256,
        pub amount_b_desired: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quoteRemoveLiquidity` function with signature `quoteRemoveLiquidity(address,address,bool,address,uint256)` and selector `0xc92de3ec`
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
        name = "quoteRemoveLiquidity",
        abi = "quoteRemoveLiquidity(address,address,bool,address,uint256)"
    )]
    pub struct QuoteRemoveLiquidityCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub stable: bool,
        pub factory: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quoteStableLiquidityRatio` function with signature `quoteStableLiquidityRatio(address,address,address)` and selector `0xf5ba53c7`
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
        name = "quoteStableLiquidityRatio",
        abi = "quoteStableLiquidityRatio(address,address,address)"
    )]
    pub struct QuoteStableLiquidityRatioCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeLiquidity` function with signature `removeLiquidity(address,address,bool,uint256,uint256,uint256,address,uint256)` and selector `0x0dede6c4`
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
        name = "removeLiquidity",
        abi = "removeLiquidity(address,address,bool,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub stable: bool,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_a_min: ::ethers::core::types::U256,
        pub amount_b_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeLiquidityETH` function with signature `removeLiquidityETH(address,bool,uint256,uint256,uint256,address,uint256)` and selector `0xd7b0e0a5`
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
        name = "removeLiquidityETH",
        abi = "removeLiquidityETH(address,bool,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityETHCall {
        pub token: ::ethers::core::types::Address,
        pub stable: bool,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_token_min: ::ethers::core::types::U256,
        pub amount_eth_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeLiquidityETHSupportingFeeOnTransferTokens` function with signature `removeLiquidityETHSupportingFeeOnTransferTokens(address,bool,uint256,uint256,uint256,address,uint256)` and selector `0xfe411f14`
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
        name = "removeLiquidityETHSupportingFeeOnTransferTokens",
        abi = "removeLiquidityETHSupportingFeeOnTransferTokens(address,bool,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityETHSupportingFeeOnTransferTokensCall {
        pub token: ::ethers::core::types::Address,
        pub stable: bool,
        pub liquidity: ::ethers::core::types::U256,
        pub amount_token_min: ::ethers::core::types::U256,
        pub amount_eth_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sortTokens` function with signature `sortTokens(address,address)` and selector `0x544caa56`
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
    #[ethcall(name = "sortTokens", abi = "sortTokens(address,address)")]
    pub struct SortTokensCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `swapExactETHForTokens` function with signature `swapExactETHForTokens(uint256,(address,address,bool,address)[],address,uint256)` and selector `0x903638a4`
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
        name = "swapExactETHForTokens",
        abi = "swapExactETHForTokens(uint256,(address,address,bool,address)[],address,uint256)"
    )]
    pub struct SwapExactETHForTokensCall {
        pub amount_out_min: ::ethers::core::types::U256,
        pub routes: ::std::vec::Vec<Route>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactETHForTokensSupportingFeeOnTransferTokens` function with signature `swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,(address,address,bool,address)[],address,uint256)` and selector `0x3da5acba`
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
        name = "swapExactETHForTokensSupportingFeeOnTransferTokens",
        abi = "swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,(address,address,bool,address)[],address,uint256)"
    )]
    pub struct SwapExactETHForTokensSupportingFeeOnTransferTokensCall {
        pub amount_out_min: ::ethers::core::types::U256,
        pub routes: ::std::vec::Vec<Route>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactTokensForETH` function with signature `swapExactTokensForETH(uint256,uint256,(address,address,bool,address)[],address,uint256)` and selector `0xc6b7f1b6`
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
        name = "swapExactTokensForETH",
        abi = "swapExactTokensForETH(uint256,uint256,(address,address,bool,address)[],address,uint256)"
    )]
    pub struct SwapExactTokensForETHCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub routes: ::std::vec::Vec<Route>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactTokensForETHSupportingFeeOnTransferTokens` function with signature `swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)` and selector `0x12bc3aca`
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
        name = "swapExactTokensForETHSupportingFeeOnTransferTokens",
        abi = "swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)"
    )]
    pub struct SwapExactTokensForETHSupportingFeeOnTransferTokensCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub routes: ::std::vec::Vec<Route>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)` and selector `0xcac88ea9`
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
        name = "swapExactTokensForTokens",
        abi = "swapExactTokensForTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)"
    )]
    pub struct SwapExactTokensForTokensCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub routes: ::std::vec::Vec<Route>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactTokensForTokensSupportingFeeOnTransferTokens` function with signature `swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)` and selector `0x88cd821e`
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
        name = "swapExactTokensForTokensSupportingFeeOnTransferTokens",
        abi = "swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)"
    )]
    pub struct SwapExactTokensForTokensSupportingFeeOnTransferTokensCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub routes: ::std::vec::Vec<Route>,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `weth` function with signature `weth()` and selector `0x3fc8cef3`
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
    #[ethcall(name = "weth", abi = "weth()")]
    pub struct WethCall;
    ///Container type for all input parameters for the `zapIn` function with signature `zapIn(address,uint256,uint256,(address,address,bool,address,uint256,uint256,uint256,uint256),(address,address,bool,address)[],(address,address,bool,address)[],address,bool)` and selector `0xfb49bafd`
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
        name = "zapIn",
        abi = "zapIn(address,uint256,uint256,(address,address,bool,address,uint256,uint256,uint256,uint256),(address,address,bool,address)[],(address,address,bool,address)[],address,bool)"
    )]
    pub struct ZapInCall {
        pub token_in: ::ethers::core::types::Address,
        pub amount_in_a: ::ethers::core::types::U256,
        pub amount_in_b: ::ethers::core::types::U256,
        pub zap_in_pool: Zap,
        pub routes_a: ::std::vec::Vec<Route>,
        pub routes_b: ::std::vec::Vec<Route>,
        pub to: ::ethers::core::types::Address,
        pub stake: bool,
    }
    ///Container type for all input parameters for the `zapOut` function with signature `zapOut(address,uint256,(address,address,bool,address,uint256,uint256,uint256,uint256),(address,address,bool,address)[],(address,address,bool,address)[])` and selector `0xa81b9159`
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
        name = "zapOut",
        abi = "zapOut(address,uint256,(address,address,bool,address,uint256,uint256,uint256,uint256),(address,address,bool,address)[],(address,address,bool,address)[])"
    )]
    pub struct ZapOutCall {
        pub token_out: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub zap_out_pool: Zap,
        pub routes_a: ::std::vec::Vec<Route>,
        pub routes_b: ::std::vec::Vec<Route>,
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
    pub enum MockRouterCalls {
        Ether(EtherCall),
        UnsafeSwapExactTokensForTokens(UnsafeSwapExactTokensForTokensCall),
        AddLiquidity(AddLiquidityCall),
        AddLiquidityETH(AddLiquidityETHCall),
        DefaultFactory(DefaultFactoryCall),
        FactoryRegistry(FactoryRegistryCall),
        GenerateZapInParams(GenerateZapInParamsCall),
        GenerateZapOutParams(GenerateZapOutParamsCall),
        GetAmountsOut(GetAmountsOutCall),
        GetReserves(GetReservesCall),
        PoolFor(PoolForCall),
        QuoteAddLiquidity(QuoteAddLiquidityCall),
        QuoteRemoveLiquidity(QuoteRemoveLiquidityCall),
        QuoteStableLiquidityRatio(QuoteStableLiquidityRatioCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RemoveLiquidityETH(RemoveLiquidityETHCall),
        RemoveLiquidityETHSupportingFeeOnTransferTokens(
            RemoveLiquidityETHSupportingFeeOnTransferTokensCall,
        ),
        SortTokens(SortTokensCall),
        SwapExactETHForTokens(SwapExactETHForTokensCall),
        SwapExactETHForTokensSupportingFeeOnTransferTokens(
            SwapExactETHForTokensSupportingFeeOnTransferTokensCall,
        ),
        SwapExactTokensForETH(SwapExactTokensForETHCall),
        SwapExactTokensForETHSupportingFeeOnTransferTokens(
            SwapExactTokensForETHSupportingFeeOnTransferTokensCall,
        ),
        SwapExactTokensForTokens(SwapExactTokensForTokensCall),
        SwapExactTokensForTokensSupportingFeeOnTransferTokens(
            SwapExactTokensForTokensSupportingFeeOnTransferTokensCall,
        ),
        Voter(VoterCall),
        Weth(WethCall),
        ZapIn(ZapInCall),
        ZapOut(ZapOutCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <EtherCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ether(decoded));
            }
            if let Ok(decoded) = <UnsafeSwapExactTokensForTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnsafeSwapExactTokensForTokens(decoded));
            }
            if let Ok(decoded) = <AddLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLiquidity(decoded));
            }
            if let Ok(decoded) = <AddLiquidityETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLiquidityETH(decoded));
            }
            if let Ok(decoded) = <DefaultFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultFactory(decoded));
            }
            if let Ok(decoded) = <FactoryRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FactoryRegistry(decoded));
            }
            if let Ok(decoded) = <GenerateZapInParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GenerateZapInParams(decoded));
            }
            if let Ok(decoded) = <GenerateZapOutParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GenerateZapOutParams(decoded));
            }
            if let Ok(decoded) = <GetAmountsOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAmountsOut(decoded));
            }
            if let Ok(decoded) = <GetReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReserves(decoded));
            }
            if let Ok(decoded) = <PoolForCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolFor(decoded));
            }
            if let Ok(decoded) = <QuoteAddLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteAddLiquidity(decoded));
            }
            if let Ok(decoded) = <QuoteRemoveLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteRemoveLiquidity(decoded));
            }
            if let Ok(decoded) = <QuoteStableLiquidityRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteStableLiquidityRatio(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidityETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveLiquidityETH(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidityETHSupportingFeeOnTransferTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::RemoveLiquidityETHSupportingFeeOnTransferTokens(decoded),
                );
            }
            if let Ok(decoded) = <SortTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SortTokens(decoded));
            }
            if let Ok(decoded) = <SwapExactETHForTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapExactETHForTokens(decoded));
            }
            if let Ok(decoded) = <SwapExactETHForTokensSupportingFeeOnTransferTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::SwapExactETHForTokensSupportingFeeOnTransferTokens(decoded),
                );
            }
            if let Ok(decoded) = <SwapExactTokensForETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapExactTokensForETH(decoded));
            }
            if let Ok(decoded) = <SwapExactTokensForETHSupportingFeeOnTransferTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::SwapExactTokensForETHSupportingFeeOnTransferTokens(decoded),
                );
            }
            if let Ok(decoded) = <SwapExactTokensForTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapExactTokensForTokens(decoded));
            }
            if let Ok(decoded) = <SwapExactTokensForTokensSupportingFeeOnTransferTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::SwapExactTokensForTokensSupportingFeeOnTransferTokens(decoded),
                );
            }
            if let Ok(decoded) = <VoterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Voter(decoded));
            }
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Weth(decoded));
            }
            if let Ok(decoded) = <ZapInCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZapIn(decoded));
            }
            if let Ok(decoded) = <ZapOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZapOut(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Ether(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnsafeSwapExactTokensForTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddLiquidityETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FactoryRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GenerateZapInParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GenerateZapOutParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAmountsOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolFor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QuoteAddLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteRemoveLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteStableLiquidityRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveLiquidityETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveLiquidityETHSupportingFeeOnTransferTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SortTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactETHForTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactETHForTokensSupportingFeeOnTransferTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactTokensForETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactTokensForETHSupportingFeeOnTransferTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactTokensForTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactTokensForTokensSupportingFeeOnTransferTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZapIn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZapOut(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockRouterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Ether(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsafeSwapExactTokensForTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidityETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::FactoryRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::GenerateZapInParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GenerateZapOutParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAmountsOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolFor(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteAddLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteRemoveLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuoteStableLiquidityRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidityETH(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveLiquidityETHSupportingFeeOnTransferTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SortTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapExactETHForTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactETHForTokensSupportingFeeOnTransferTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactTokensForETH(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactTokensForETHSupportingFeeOnTransferTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactTokensForTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExactTokensForTokensSupportingFeeOnTransferTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Voter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZapIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZapOut(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EtherCall> for MockRouterCalls {
        fn from(value: EtherCall) -> Self {
            Self::Ether(value)
        }
    }
    impl ::core::convert::From<UnsafeSwapExactTokensForTokensCall> for MockRouterCalls {
        fn from(value: UnsafeSwapExactTokensForTokensCall) -> Self {
            Self::UnsafeSwapExactTokensForTokens(value)
        }
    }
    impl ::core::convert::From<AddLiquidityCall> for MockRouterCalls {
        fn from(value: AddLiquidityCall) -> Self {
            Self::AddLiquidity(value)
        }
    }
    impl ::core::convert::From<AddLiquidityETHCall> for MockRouterCalls {
        fn from(value: AddLiquidityETHCall) -> Self {
            Self::AddLiquidityETH(value)
        }
    }
    impl ::core::convert::From<DefaultFactoryCall> for MockRouterCalls {
        fn from(value: DefaultFactoryCall) -> Self {
            Self::DefaultFactory(value)
        }
    }
    impl ::core::convert::From<FactoryRegistryCall> for MockRouterCalls {
        fn from(value: FactoryRegistryCall) -> Self {
            Self::FactoryRegistry(value)
        }
    }
    impl ::core::convert::From<GenerateZapInParamsCall> for MockRouterCalls {
        fn from(value: GenerateZapInParamsCall) -> Self {
            Self::GenerateZapInParams(value)
        }
    }
    impl ::core::convert::From<GenerateZapOutParamsCall> for MockRouterCalls {
        fn from(value: GenerateZapOutParamsCall) -> Self {
            Self::GenerateZapOutParams(value)
        }
    }
    impl ::core::convert::From<GetAmountsOutCall> for MockRouterCalls {
        fn from(value: GetAmountsOutCall) -> Self {
            Self::GetAmountsOut(value)
        }
    }
    impl ::core::convert::From<GetReservesCall> for MockRouterCalls {
        fn from(value: GetReservesCall) -> Self {
            Self::GetReserves(value)
        }
    }
    impl ::core::convert::From<PoolForCall> for MockRouterCalls {
        fn from(value: PoolForCall) -> Self {
            Self::PoolFor(value)
        }
    }
    impl ::core::convert::From<QuoteAddLiquidityCall> for MockRouterCalls {
        fn from(value: QuoteAddLiquidityCall) -> Self {
            Self::QuoteAddLiquidity(value)
        }
    }
    impl ::core::convert::From<QuoteRemoveLiquidityCall> for MockRouterCalls {
        fn from(value: QuoteRemoveLiquidityCall) -> Self {
            Self::QuoteRemoveLiquidity(value)
        }
    }
    impl ::core::convert::From<QuoteStableLiquidityRatioCall> for MockRouterCalls {
        fn from(value: QuoteStableLiquidityRatioCall) -> Self {
            Self::QuoteStableLiquidityRatio(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityCall> for MockRouterCalls {
        fn from(value: RemoveLiquidityCall) -> Self {
            Self::RemoveLiquidity(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityETHCall> for MockRouterCalls {
        fn from(value: RemoveLiquidityETHCall) -> Self {
            Self::RemoveLiquidityETH(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityETHSupportingFeeOnTransferTokensCall>
    for MockRouterCalls {
        fn from(value: RemoveLiquidityETHSupportingFeeOnTransferTokensCall) -> Self {
            Self::RemoveLiquidityETHSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<SortTokensCall> for MockRouterCalls {
        fn from(value: SortTokensCall) -> Self {
            Self::SortTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactETHForTokensCall> for MockRouterCalls {
        fn from(value: SwapExactETHForTokensCall) -> Self {
            Self::SwapExactETHForTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactETHForTokensSupportingFeeOnTransferTokensCall>
    for MockRouterCalls {
        fn from(value: SwapExactETHForTokensSupportingFeeOnTransferTokensCall) -> Self {
            Self::SwapExactETHForTokensSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForETHCall> for MockRouterCalls {
        fn from(value: SwapExactTokensForETHCall) -> Self {
            Self::SwapExactTokensForETH(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForETHSupportingFeeOnTransferTokensCall>
    for MockRouterCalls {
        fn from(value: SwapExactTokensForETHSupportingFeeOnTransferTokensCall) -> Self {
            Self::SwapExactTokensForETHSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForTokensCall> for MockRouterCalls {
        fn from(value: SwapExactTokensForTokensCall) -> Self {
            Self::SwapExactTokensForTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForTokensSupportingFeeOnTransferTokensCall>
    for MockRouterCalls {
        fn from(
            value: SwapExactTokensForTokensSupportingFeeOnTransferTokensCall,
        ) -> Self {
            Self::SwapExactTokensForTokensSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<VoterCall> for MockRouterCalls {
        fn from(value: VoterCall) -> Self {
            Self::Voter(value)
        }
    }
    impl ::core::convert::From<WethCall> for MockRouterCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    impl ::core::convert::From<ZapInCall> for MockRouterCalls {
        fn from(value: ZapInCall) -> Self {
            Self::ZapIn(value)
        }
    }
    impl ::core::convert::From<ZapOutCall> for MockRouterCalls {
        fn from(value: ZapOutCall) -> Self {
            Self::ZapOut(value)
        }
    }
    ///Container type for all return fields from the `ETHER` function with signature `ETHER()` and selector `0x42cb1fbc`
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
    pub struct EtherReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `UNSAFE_swapExactTokensForTokens` function with signature `UNSAFE_swapExactTokensForTokens(uint256[],(address,address,bool,address)[],address,uint256)` and selector `0x4111d597`
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
    pub struct UnsafeSwapExactTokensForTokensReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `addLiquidity` function with signature `addLiquidity(address,address,bool,uint256,uint256,uint256,uint256,address,uint256)` and selector `0x5a47ddc3`
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
    pub struct AddLiquidityReturn {
        pub amount_a: ::ethers::core::types::U256,
        pub amount_b: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `addLiquidityETH` function with signature `addLiquidityETH(address,bool,uint256,uint256,uint256,address,uint256)` and selector `0xb7e0d4c0`
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
    pub struct AddLiquidityETHReturn {
        pub amount_token: ::ethers::core::types::U256,
        pub amount_eth: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `defaultFactory` function with signature `defaultFactory()` and selector `0xd4b6846d`
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
    pub struct DefaultFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `factoryRegistry` function with signature `factoryRegistry()` and selector `0x3bf0c9fb`
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
    pub struct FactoryRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `generateZapInParams` function with signature `generateZapInParams(address,address,bool,address,uint256,uint256,(address,address,bool,address)[],(address,address,bool,address)[])` and selector `0x07db50fa`
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
    pub struct GenerateZapInParamsReturn {
        pub amount_out_min_a: ::ethers::core::types::U256,
        pub amount_out_min_b: ::ethers::core::types::U256,
        pub amount_a_min: ::ethers::core::types::U256,
        pub amount_b_min: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `generateZapOutParams` function with signature `generateZapOutParams(address,address,bool,address,uint256,(address,address,bool,address)[],(address,address,bool,address)[])` and selector `0x7539d413`
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
    pub struct GenerateZapOutParamsReturn {
        pub amount_out_min_a: ::ethers::core::types::U256,
        pub amount_out_min_b: ::ethers::core::types::U256,
        pub amount_a_min: ::ethers::core::types::U256,
        pub amount_b_min: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getAmountsOut` function with signature `getAmountsOut(uint256,(address,address,bool,address)[])` and selector `0x5509a1ac`
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
    pub struct GetAmountsOutReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `getReserves` function with signature `getReserves(address,address,bool,address)` and selector `0x8c0037dc`
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
        pub reserve_a: ::ethers::core::types::U256,
        pub reserve_b: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `poolFor` function with signature `poolFor(address,address,bool,address)` and selector `0x874029d9`
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
    pub struct PoolForReturn {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `quoteAddLiquidity` function with signature `quoteAddLiquidity(address,address,bool,address,uint256,uint256)` and selector `0xce700c29`
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
    pub struct QuoteAddLiquidityReturn {
        pub amount_a: ::ethers::core::types::U256,
        pub amount_b: ::ethers::core::types::U256,
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `quoteRemoveLiquidity` function with signature `quoteRemoveLiquidity(address,address,bool,address,uint256)` and selector `0xc92de3ec`
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
    pub struct QuoteRemoveLiquidityReturn {
        pub amount_a: ::ethers::core::types::U256,
        pub amount_b: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `quoteStableLiquidityRatio` function with signature `quoteStableLiquidityRatio(address,address,address)` and selector `0xf5ba53c7`
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
    pub struct QuoteStableLiquidityRatioReturn {
        pub ratio: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidity` function with signature `removeLiquidity(address,address,bool,uint256,uint256,uint256,address,uint256)` and selector `0x0dede6c4`
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
    pub struct RemoveLiquidityReturn {
        pub amount_a: ::ethers::core::types::U256,
        pub amount_b: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidityETH` function with signature `removeLiquidityETH(address,bool,uint256,uint256,uint256,address,uint256)` and selector `0xd7b0e0a5`
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
    pub struct RemoveLiquidityETHReturn {
        pub amount_token: ::ethers::core::types::U256,
        pub amount_eth: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `removeLiquidityETHSupportingFeeOnTransferTokens` function with signature `removeLiquidityETHSupportingFeeOnTransferTokens(address,bool,uint256,uint256,uint256,address,uint256)` and selector `0xfe411f14`
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
    pub struct RemoveLiquidityETHSupportingFeeOnTransferTokensReturn {
        pub amount_eth: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `sortTokens` function with signature `sortTokens(address,address)` and selector `0x544caa56`
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
    pub struct SortTokensReturn {
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `swapExactETHForTokens` function with signature `swapExactETHForTokens(uint256,(address,address,bool,address)[],address,uint256)` and selector `0x903638a4`
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
    pub struct SwapExactETHForTokensReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapExactTokensForETH` function with signature `swapExactTokensForETH(uint256,uint256,(address,address,bool,address)[],address,uint256)` and selector `0xc6b7f1b6`
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
    pub struct SwapExactTokensForETHReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(uint256,uint256,(address,address,bool,address)[],address,uint256)` and selector `0xcac88ea9`
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
    pub struct SwapExactTokensForTokensReturn {
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
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
    ///Container type for all return fields from the `weth` function with signature `weth()` and selector `0x3fc8cef3`
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
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `zapIn` function with signature `zapIn(address,uint256,uint256,(address,address,bool,address,uint256,uint256,uint256,uint256),(address,address,bool,address)[],(address,address,bool,address)[],address,bool)` and selector `0xfb49bafd`
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
    pub struct ZapInReturn {
        pub liquidity: ::ethers::core::types::U256,
    }
}
