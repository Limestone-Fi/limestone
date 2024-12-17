pub use i_drome_router::*;
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
pub mod i_drome_router {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
                                    name: ::std::borrow::ToOwned::to_owned("factory"),
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IDROMEROUTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IDromeRouter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IDromeRouter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IDromeRouter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IDromeRouter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IDromeRouter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IDromeRouter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IDromeRouter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IDROMEROUTER_ABI.clone(),
                    client,
                ),
            )
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
    for IDromeRouter<M> {
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
    pub enum IDromeRouterErrors {
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
        SameAddresses(SameAddresses),
        ZeroAddress(ZeroAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IDromeRouterErrors {
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
    impl ::ethers::core::abi::AbiEncode for IDromeRouterErrors {
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
    impl ::ethers::contract::ContractRevert for IDromeRouterErrors {
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
                    == <SameAddresses as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroAddress as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IDromeRouterErrors {
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
                Self::SameAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IDromeRouterErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ETHTransferFailed> for IDromeRouterErrors {
        fn from(value: ETHTransferFailed) -> Self {
            Self::ETHTransferFailed(value)
        }
    }
    impl ::core::convert::From<Expired> for IDromeRouterErrors {
        fn from(value: Expired) -> Self {
            Self::Expired(value)
        }
    }
    impl ::core::convert::From<InsufficientAmount> for IDromeRouterErrors {
        fn from(value: InsufficientAmount) -> Self {
            Self::InsufficientAmount(value)
        }
    }
    impl ::core::convert::From<InsufficientAmountA> for IDromeRouterErrors {
        fn from(value: InsufficientAmountA) -> Self {
            Self::InsufficientAmountA(value)
        }
    }
    impl ::core::convert::From<InsufficientAmountADesired> for IDromeRouterErrors {
        fn from(value: InsufficientAmountADesired) -> Self {
            Self::InsufficientAmountADesired(value)
        }
    }
    impl ::core::convert::From<InsufficientAmountAOptimal> for IDromeRouterErrors {
        fn from(value: InsufficientAmountAOptimal) -> Self {
            Self::InsufficientAmountAOptimal(value)
        }
    }
    impl ::core::convert::From<InsufficientAmountB> for IDromeRouterErrors {
        fn from(value: InsufficientAmountB) -> Self {
            Self::InsufficientAmountB(value)
        }
    }
    impl ::core::convert::From<InsufficientAmountBDesired> for IDromeRouterErrors {
        fn from(value: InsufficientAmountBDesired) -> Self {
            Self::InsufficientAmountBDesired(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidity> for IDromeRouterErrors {
        fn from(value: InsufficientLiquidity) -> Self {
            Self::InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<InsufficientOutputAmount> for IDromeRouterErrors {
        fn from(value: InsufficientOutputAmount) -> Self {
            Self::InsufficientOutputAmount(value)
        }
    }
    impl ::core::convert::From<InvalidAmountInForETHDeposit> for IDromeRouterErrors {
        fn from(value: InvalidAmountInForETHDeposit) -> Self {
            Self::InvalidAmountInForETHDeposit(value)
        }
    }
    impl ::core::convert::From<InvalidPath> for IDromeRouterErrors {
        fn from(value: InvalidPath) -> Self {
            Self::InvalidPath(value)
        }
    }
    impl ::core::convert::From<InvalidRouteA> for IDromeRouterErrors {
        fn from(value: InvalidRouteA) -> Self {
            Self::InvalidRouteA(value)
        }
    }
    impl ::core::convert::From<InvalidRouteB> for IDromeRouterErrors {
        fn from(value: InvalidRouteB) -> Self {
            Self::InvalidRouteB(value)
        }
    }
    impl ::core::convert::From<InvalidTokenInForETHDeposit> for IDromeRouterErrors {
        fn from(value: InvalidTokenInForETHDeposit) -> Self {
            Self::InvalidTokenInForETHDeposit(value)
        }
    }
    impl ::core::convert::From<OnlyWETH> for IDromeRouterErrors {
        fn from(value: OnlyWETH) -> Self {
            Self::OnlyWETH(value)
        }
    }
    impl ::core::convert::From<PoolDoesNotExist> for IDromeRouterErrors {
        fn from(value: PoolDoesNotExist) -> Self {
            Self::PoolDoesNotExist(value)
        }
    }
    impl ::core::convert::From<PoolFactoryDoesNotExist> for IDromeRouterErrors {
        fn from(value: PoolFactoryDoesNotExist) -> Self {
            Self::PoolFactoryDoesNotExist(value)
        }
    }
    impl ::core::convert::From<SameAddresses> for IDromeRouterErrors {
        fn from(value: SameAddresses) -> Self {
            Self::SameAddresses(value)
        }
    }
    impl ::core::convert::From<ZeroAddress> for IDromeRouterErrors {
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
    pub enum IDromeRouterCalls {
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
    impl ::ethers::core::abi::AbiDecode for IDromeRouterCalls {
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
    impl ::ethers::core::abi::AbiEncode for IDromeRouterCalls {
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
    impl ::core::fmt::Display for IDromeRouterCalls {
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
    impl ::core::convert::From<EtherCall> for IDromeRouterCalls {
        fn from(value: EtherCall) -> Self {
            Self::Ether(value)
        }
    }
    impl ::core::convert::From<UnsafeSwapExactTokensForTokensCall>
    for IDromeRouterCalls {
        fn from(value: UnsafeSwapExactTokensForTokensCall) -> Self {
            Self::UnsafeSwapExactTokensForTokens(value)
        }
    }
    impl ::core::convert::From<AddLiquidityCall> for IDromeRouterCalls {
        fn from(value: AddLiquidityCall) -> Self {
            Self::AddLiquidity(value)
        }
    }
    impl ::core::convert::From<AddLiquidityETHCall> for IDromeRouterCalls {
        fn from(value: AddLiquidityETHCall) -> Self {
            Self::AddLiquidityETH(value)
        }
    }
    impl ::core::convert::From<DefaultFactoryCall> for IDromeRouterCalls {
        fn from(value: DefaultFactoryCall) -> Self {
            Self::DefaultFactory(value)
        }
    }
    impl ::core::convert::From<FactoryRegistryCall> for IDromeRouterCalls {
        fn from(value: FactoryRegistryCall) -> Self {
            Self::FactoryRegistry(value)
        }
    }
    impl ::core::convert::From<GenerateZapInParamsCall> for IDromeRouterCalls {
        fn from(value: GenerateZapInParamsCall) -> Self {
            Self::GenerateZapInParams(value)
        }
    }
    impl ::core::convert::From<GenerateZapOutParamsCall> for IDromeRouterCalls {
        fn from(value: GenerateZapOutParamsCall) -> Self {
            Self::GenerateZapOutParams(value)
        }
    }
    impl ::core::convert::From<GetAmountsOutCall> for IDromeRouterCalls {
        fn from(value: GetAmountsOutCall) -> Self {
            Self::GetAmountsOut(value)
        }
    }
    impl ::core::convert::From<GetReservesCall> for IDromeRouterCalls {
        fn from(value: GetReservesCall) -> Self {
            Self::GetReserves(value)
        }
    }
    impl ::core::convert::From<PoolForCall> for IDromeRouterCalls {
        fn from(value: PoolForCall) -> Self {
            Self::PoolFor(value)
        }
    }
    impl ::core::convert::From<QuoteAddLiquidityCall> for IDromeRouterCalls {
        fn from(value: QuoteAddLiquidityCall) -> Self {
            Self::QuoteAddLiquidity(value)
        }
    }
    impl ::core::convert::From<QuoteRemoveLiquidityCall> for IDromeRouterCalls {
        fn from(value: QuoteRemoveLiquidityCall) -> Self {
            Self::QuoteRemoveLiquidity(value)
        }
    }
    impl ::core::convert::From<QuoteStableLiquidityRatioCall> for IDromeRouterCalls {
        fn from(value: QuoteStableLiquidityRatioCall) -> Self {
            Self::QuoteStableLiquidityRatio(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityCall> for IDromeRouterCalls {
        fn from(value: RemoveLiquidityCall) -> Self {
            Self::RemoveLiquidity(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityETHCall> for IDromeRouterCalls {
        fn from(value: RemoveLiquidityETHCall) -> Self {
            Self::RemoveLiquidityETH(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityETHSupportingFeeOnTransferTokensCall>
    for IDromeRouterCalls {
        fn from(value: RemoveLiquidityETHSupportingFeeOnTransferTokensCall) -> Self {
            Self::RemoveLiquidityETHSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<SortTokensCall> for IDromeRouterCalls {
        fn from(value: SortTokensCall) -> Self {
            Self::SortTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactETHForTokensCall> for IDromeRouterCalls {
        fn from(value: SwapExactETHForTokensCall) -> Self {
            Self::SwapExactETHForTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactETHForTokensSupportingFeeOnTransferTokensCall>
    for IDromeRouterCalls {
        fn from(value: SwapExactETHForTokensSupportingFeeOnTransferTokensCall) -> Self {
            Self::SwapExactETHForTokensSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForETHCall> for IDromeRouterCalls {
        fn from(value: SwapExactTokensForETHCall) -> Self {
            Self::SwapExactTokensForETH(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForETHSupportingFeeOnTransferTokensCall>
    for IDromeRouterCalls {
        fn from(value: SwapExactTokensForETHSupportingFeeOnTransferTokensCall) -> Self {
            Self::SwapExactTokensForETHSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForTokensCall> for IDromeRouterCalls {
        fn from(value: SwapExactTokensForTokensCall) -> Self {
            Self::SwapExactTokensForTokens(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForTokensSupportingFeeOnTransferTokensCall>
    for IDromeRouterCalls {
        fn from(
            value: SwapExactTokensForTokensSupportingFeeOnTransferTokensCall,
        ) -> Self {
            Self::SwapExactTokensForTokensSupportingFeeOnTransferTokens(value)
        }
    }
    impl ::core::convert::From<VoterCall> for IDromeRouterCalls {
        fn from(value: VoterCall) -> Self {
            Self::Voter(value)
        }
    }
    impl ::core::convert::From<WethCall> for IDromeRouterCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    impl ::core::convert::From<ZapInCall> for IDromeRouterCalls {
        fn from(value: ZapInCall) -> Self {
            Self::ZapIn(value)
        }
    }
    impl ::core::convert::From<ZapOutCall> for IDromeRouterCalls {
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
