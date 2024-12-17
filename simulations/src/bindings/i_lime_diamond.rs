pub use i_lime_diamond::*;
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
pub mod i_lime_diamond {
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Ownable__NotOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Ownable__NotOwner"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Ownable__NotTransitiveOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Ownable__NotTransitiveOwner",
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
    pub static ILIMEDIAMOND_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct ILimeDiamond<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ILimeDiamond<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ILimeDiamond<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ILimeDiamond<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ILimeDiamond<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ILimeDiamond))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ILimeDiamond<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ILIMEDIAMOND_ABI.clone(),
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
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], account)
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
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
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
            ILimeDiamondEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ILimeDiamond<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Ownable__NotOwner` with signature `Ownable__NotOwner()` and selector `0x2f7a8ee1`
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
    #[etherror(name = "Ownable__NotOwner", abi = "Ownable__NotOwner()")]
    pub struct Ownable__NotOwner;
    ///Custom Error type `Ownable__NotTransitiveOwner` with signature `Ownable__NotTransitiveOwner()` and selector `0xd0b49847`
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
        name = "Ownable__NotTransitiveOwner",
        abi = "Ownable__NotTransitiveOwner()"
    )]
    pub struct Ownable__NotTransitiveOwner;
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
    pub enum ILimeDiamondErrors {
        Ownable__NotOwner(Ownable__NotOwner),
        Ownable__NotTransitiveOwner(Ownable__NotTransitiveOwner),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ILimeDiamondErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Ownable__NotOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ownable__NotOwner(decoded));
            }
            if let Ok(decoded) = <Ownable__NotTransitiveOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ownable__NotTransitiveOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ILimeDiamondErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Ownable__NotOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ownable__NotTransitiveOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ILimeDiamondErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Ownable__NotOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Ownable__NotTransitiveOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ILimeDiamondErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Ownable__NotOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ownable__NotTransitiveOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ILimeDiamondErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Ownable__NotOwner> for ILimeDiamondErrors {
        fn from(value: Ownable__NotOwner) -> Self {
            Self::Ownable__NotOwner(value)
        }
    }
    impl ::core::convert::From<Ownable__NotTransitiveOwner> for ILimeDiamondErrors {
        fn from(value: Ownable__NotTransitiveOwner) -> Self {
            Self::Ownable__NotTransitiveOwner(value)
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum ILimeDiamondEvents {
        AddDebtFilter(AddDebtFilter),
        BorrowFilter(BorrowFilter),
        DelegatedBorrowFilter(DelegatedBorrowFilter),
        DepositFilter(DepositFilter),
        IncreaseCollateralFilter(IncreaseCollateralFilter),
        KillFilter(KillFilter),
        MarketCreatedFilter(MarketCreatedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PositionDivestedFilter(PositionDivestedFilter),
        PositionInvestedFilter(PositionInvestedFilter),
        RemoveDebtFilter(RemoveDebtFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for ILimeDiamondEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddDebtFilter::decode_log(log) {
                return Ok(ILimeDiamondEvents::AddDebtFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(ILimeDiamondEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = DelegatedBorrowFilter::decode_log(log) {
                return Ok(ILimeDiamondEvents::DelegatedBorrowFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(ILimeDiamondEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = IncreaseCollateralFilter::decode_log(log) {
                return Ok(ILimeDiamondEvents::IncreaseCollateralFilter(decoded));
            }
            if let Ok(decoded) = KillFilter::decode_log(log) {
                return Ok(ILimeDiamondEvents::KillFilter(decoded));
            }
            if let Ok(decoded) = MarketCreatedFilter::decode_log(log) {
                return Ok(ILimeDiamondEvents::MarketCreatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ILimeDiamondEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PositionDivestedFilter::decode_log(log) {
                return Ok(ILimeDiamondEvents::PositionDivestedFilter(decoded));
            }
            if let Ok(decoded) = PositionInvestedFilter::decode_log(log) {
                return Ok(ILimeDiamondEvents::PositionInvestedFilter(decoded));
            }
            if let Ok(decoded) = RemoveDebtFilter::decode_log(log) {
                return Ok(ILimeDiamondEvents::RemoveDebtFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(ILimeDiamondEvents::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ILimeDiamondEvents {
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
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionDivestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionInvestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveDebtFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddDebtFilter> for ILimeDiamondEvents {
        fn from(value: AddDebtFilter) -> Self {
            Self::AddDebtFilter(value)
        }
    }
    impl ::core::convert::From<BorrowFilter> for ILimeDiamondEvents {
        fn from(value: BorrowFilter) -> Self {
            Self::BorrowFilter(value)
        }
    }
    impl ::core::convert::From<DelegatedBorrowFilter> for ILimeDiamondEvents {
        fn from(value: DelegatedBorrowFilter) -> Self {
            Self::DelegatedBorrowFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for ILimeDiamondEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<IncreaseCollateralFilter> for ILimeDiamondEvents {
        fn from(value: IncreaseCollateralFilter) -> Self {
            Self::IncreaseCollateralFilter(value)
        }
    }
    impl ::core::convert::From<KillFilter> for ILimeDiamondEvents {
        fn from(value: KillFilter) -> Self {
            Self::KillFilter(value)
        }
    }
    impl ::core::convert::From<MarketCreatedFilter> for ILimeDiamondEvents {
        fn from(value: MarketCreatedFilter) -> Self {
            Self::MarketCreatedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ILimeDiamondEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PositionDivestedFilter> for ILimeDiamondEvents {
        fn from(value: PositionDivestedFilter) -> Self {
            Self::PositionDivestedFilter(value)
        }
    }
    impl ::core::convert::From<PositionInvestedFilter> for ILimeDiamondEvents {
        fn from(value: PositionInvestedFilter) -> Self {
            Self::PositionInvestedFilter(value)
        }
    }
    impl ::core::convert::From<RemoveDebtFilter> for ILimeDiamondEvents {
        fn from(value: RemoveDebtFilter) -> Self {
            Self::RemoveDebtFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for ILimeDiamondEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
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
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
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
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub account: ::ethers::core::types::Address,
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
    pub enum ILimeDiamondCalls {
        AccessAssets(AccessAssetsCall),
        AccessUserAssets(AccessUserAssetsCall),
        AuthorizedKeepers(AuthorizedKeepersCall),
        AuthorizedLiquidators(AuthorizedLiquidatorsCall),
        DebtShareToVal(DebtShareToValCall),
        DebtValToShare(DebtValToShareCall),
        Deposit(DepositCall),
        DivestFromV2LikePosition(DivestFromV2LikePositionCall),
        InvestInV2LikePosition(InvestInV2LikePositionCall),
        LiquidateV2LikePosition(LiquidateV2LikePositionCall),
        Owner(OwnerCall),
        PermissionedLiquidation(PermissionedLiquidationCall),
        Pools(PoolsCall),
        Positions(PositionsCall),
        ReinvestmentFeeNumerator(ReinvestmentFeeNumeratorCall),
        RepayV2LikeLiquidityPositionDebt(RepayV2LikeLiquidityPositionDebtCall),
        TransferOwnership(TransferOwnershipCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for ILimeDiamondCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccessAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessAssets(decoded));
            }
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
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
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
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ILimeDiamondCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccessAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::DivestFromV2LikePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvestInV2LikePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidateV2LikePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PermissionedLiquidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Positions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReinvestmentFeeNumerator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayV2LikeLiquidityPositionDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ILimeDiamondCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccessUserAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuthorizedKeepers(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuthorizedLiquidators(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DebtShareToVal(element) => ::core::fmt::Display::fmt(element, f),
                Self::DebtValToShare(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivestFromV2LikePosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvestInV2LikePosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidateV2LikePosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermissionedLiquidation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::Positions(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReinvestmentFeeNumerator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RepayV2LikeLiquidityPositionDebt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccessAssetsCall> for ILimeDiamondCalls {
        fn from(value: AccessAssetsCall) -> Self {
            Self::AccessAssets(value)
        }
    }
    impl ::core::convert::From<AccessUserAssetsCall> for ILimeDiamondCalls {
        fn from(value: AccessUserAssetsCall) -> Self {
            Self::AccessUserAssets(value)
        }
    }
    impl ::core::convert::From<AuthorizedKeepersCall> for ILimeDiamondCalls {
        fn from(value: AuthorizedKeepersCall) -> Self {
            Self::AuthorizedKeepers(value)
        }
    }
    impl ::core::convert::From<AuthorizedLiquidatorsCall> for ILimeDiamondCalls {
        fn from(value: AuthorizedLiquidatorsCall) -> Self {
            Self::AuthorizedLiquidators(value)
        }
    }
    impl ::core::convert::From<DebtShareToValCall> for ILimeDiamondCalls {
        fn from(value: DebtShareToValCall) -> Self {
            Self::DebtShareToVal(value)
        }
    }
    impl ::core::convert::From<DebtValToShareCall> for ILimeDiamondCalls {
        fn from(value: DebtValToShareCall) -> Self {
            Self::DebtValToShare(value)
        }
    }
    impl ::core::convert::From<DepositCall> for ILimeDiamondCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DivestFromV2LikePositionCall> for ILimeDiamondCalls {
        fn from(value: DivestFromV2LikePositionCall) -> Self {
            Self::DivestFromV2LikePosition(value)
        }
    }
    impl ::core::convert::From<InvestInV2LikePositionCall> for ILimeDiamondCalls {
        fn from(value: InvestInV2LikePositionCall) -> Self {
            Self::InvestInV2LikePosition(value)
        }
    }
    impl ::core::convert::From<LiquidateV2LikePositionCall> for ILimeDiamondCalls {
        fn from(value: LiquidateV2LikePositionCall) -> Self {
            Self::LiquidateV2LikePosition(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ILimeDiamondCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PermissionedLiquidationCall> for ILimeDiamondCalls {
        fn from(value: PermissionedLiquidationCall) -> Self {
            Self::PermissionedLiquidation(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for ILimeDiamondCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<PositionsCall> for ILimeDiamondCalls {
        fn from(value: PositionsCall) -> Self {
            Self::Positions(value)
        }
    }
    impl ::core::convert::From<ReinvestmentFeeNumeratorCall> for ILimeDiamondCalls {
        fn from(value: ReinvestmentFeeNumeratorCall) -> Self {
            Self::ReinvestmentFeeNumerator(value)
        }
    }
    impl ::core::convert::From<RepayV2LikeLiquidityPositionDebtCall>
    for ILimeDiamondCalls {
        fn from(value: RepayV2LikeLiquidityPositionDebtCall) -> Self {
            Self::RepayV2LikeLiquidityPositionDebt(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ILimeDiamondCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for ILimeDiamondCalls {
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
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
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
