pub use multi_modal_worker::*;
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
pub mod multi_modal_worker {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("LIMESTONE_DIAMOND"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("LIMESTONE_DIAMOND"),
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
                    ::std::borrow::ToOwned::to_owned("calculatePositionValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculatePositionValue",
                            ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cancelOwnershipHandover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "cancelOwnershipHandover",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("completeOwnershipHandover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "completeOwnershipHandover",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("divest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("divest"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pair"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rewardPool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_router"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rewards"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
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
                    ::std::borrow::ToOwned::to_owned("ownershipHandoverExpiresAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ownershipHandoverExpiresAt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                (
                    ::std::borrow::ToOwned::to_owned("requestOwnershipHandover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "requestOwnershipHandover",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipHandoverCanceled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipHandoverCanceled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipHandoverRequested"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipHandoverRequested",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                                    name: ::std::borrow::ToOwned::to_owned("oldOwner"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AlreadyInitialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AlreadyInitialized"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "Initializable__AlreadyInitialized",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Initializable__AlreadyInitialized",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewOwnerIsZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NewOwnerIsZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoHandoverRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NoHandoverRequest"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
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
    pub static MULTIMODALWORKER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct MultiModalWorker<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MultiModalWorker<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MultiModalWorker<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MultiModalWorker<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MultiModalWorker<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MultiModalWorker))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MultiModalWorker<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MULTIMODALWORKER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `LIMESTONE_DIAMOND` (0x35804023) function
        pub fn limestone_diamond(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([53, 128, 64, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculatePositionValue` (0x02a50329) function
        pub fn calculate_position_value(
            &self,
            pos_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([2, 165, 3, 41], pos_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelOwnershipHandover` (0x54d1f13d) function
        pub fn cancel_ownership_handover(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 209, 241, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completeOwnershipHandover` (0xf04e283e) function
        pub fn complete_ownership_handover(
            &self,
            pending_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 78, 40, 62], pending_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `divest` (0xa1271c4b) function
        pub fn divest(
            &self,
            ctx: V2LikePositionDivestmentContext,
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
                .method_hash([161, 39, 28, 75], (ctx,))
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
        ///Calls the contract's `initialize` (0xe6bfbfd8) function
        pub fn initialize(
            &self,
            pair: ::ethers::core::types::Address,
            reward_pool: ::ethers::core::types::Address,
            router: ::ethers::core::types::Address,
            rewards: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 191, 191, 216], (pair, reward_pool, router, rewards))
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
        ///Calls the contract's `ownershipHandoverExpiresAt` (0xfee81cf4) function
        pub fn ownership_handover_expires_at(
            &self,
            pending_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 232, 28, 244], pending_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reinvest` (0xfdb5a03e) function
        pub fn reinvest(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 181, 160, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        ///Calls the contract's `requestOwnershipHandover` (0x25692962) function
        pub fn request_ownership_handover(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 105, 41, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipHandoverCanceled` event
        pub fn ownership_handover_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipHandoverCanceledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipHandoverRequested` event
        pub fn ownership_handover_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipHandoverRequestedFilter,
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MultiModalWorkerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MultiModalWorker<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AlreadyInitialized` with signature `AlreadyInitialized()` and selector `0x0dc149f0`
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
    #[etherror(name = "AlreadyInitialized", abi = "AlreadyInitialized()")]
    pub struct AlreadyInitialized;
    ///Custom Error type `Initializable__AlreadyInitialized` with signature `Initializable__AlreadyInitialized()` and selector `0x2e8adaaa`
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
        name = "Initializable__AlreadyInitialized",
        abi = "Initializable__AlreadyInitialized()"
    )]
    pub struct Initializable__AlreadyInitialized;
    ///Custom Error type `NewOwnerIsZeroAddress` with signature `NewOwnerIsZeroAddress()` and selector `0x7448fbae`
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
    #[etherror(name = "NewOwnerIsZeroAddress", abi = "NewOwnerIsZeroAddress()")]
    pub struct NewOwnerIsZeroAddress;
    ///Custom Error type `NoHandoverRequest` with signature `NoHandoverRequest()` and selector `0x6f5e8818`
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
    #[etherror(name = "NoHandoverRequest", abi = "NoHandoverRequest()")]
    pub struct NoHandoverRequest;
    ///Custom Error type `Unauthorized` with signature `Unauthorized()` and selector `0x82b42900`
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
    #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
    pub struct Unauthorized;
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
    pub enum MultiModalWorkerErrors {
        AlreadyInitialized(AlreadyInitialized),
        Initializable__AlreadyInitialized(Initializable__AlreadyInitialized),
        NewOwnerIsZeroAddress(NewOwnerIsZeroAddress),
        NoHandoverRequest(NoHandoverRequest),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MultiModalWorkerErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AlreadyInitialized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AlreadyInitialized(decoded));
            }
            if let Ok(decoded) = <Initializable__AlreadyInitialized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initializable__AlreadyInitialized(decoded));
            }
            if let Ok(decoded) = <NewOwnerIsZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NewOwnerIsZeroAddress(decoded));
            }
            if let Ok(decoded) = <NoHandoverRequest as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NoHandoverRequest(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MultiModalWorkerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AlreadyInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initializable__AlreadyInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NewOwnerIsZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoHandoverRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MultiModalWorkerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AlreadyInitialized as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Initializable__AlreadyInitialized as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NewOwnerIsZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoHandoverRequest as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MultiModalWorkerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AlreadyInitialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initializable__AlreadyInitialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewOwnerIsZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoHandoverRequest(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MultiModalWorkerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AlreadyInitialized> for MultiModalWorkerErrors {
        fn from(value: AlreadyInitialized) -> Self {
            Self::AlreadyInitialized(value)
        }
    }
    impl ::core::convert::From<Initializable__AlreadyInitialized>
    for MultiModalWorkerErrors {
        fn from(value: Initializable__AlreadyInitialized) -> Self {
            Self::Initializable__AlreadyInitialized(value)
        }
    }
    impl ::core::convert::From<NewOwnerIsZeroAddress> for MultiModalWorkerErrors {
        fn from(value: NewOwnerIsZeroAddress) -> Self {
            Self::NewOwnerIsZeroAddress(value)
        }
    }
    impl ::core::convert::From<NoHandoverRequest> for MultiModalWorkerErrors {
        fn from(value: NoHandoverRequest) -> Self {
            Self::NoHandoverRequest(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for MultiModalWorkerErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
        name = "OwnershipHandoverCanceled",
        abi = "OwnershipHandoverCanceled(address)"
    )]
    pub struct OwnershipHandoverCanceledFilter {
        #[ethevent(indexed)]
        pub pending_owner: ::ethers::core::types::Address,
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
        name = "OwnershipHandoverRequested",
        abi = "OwnershipHandoverRequested(address)"
    )]
    pub struct OwnershipHandoverRequestedFilter {
        #[ethevent(indexed)]
        pub pending_owner: ::ethers::core::types::Address,
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
        pub old_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum MultiModalWorkerEvents {
        InitializedFilter(InitializedFilter),
        OwnershipHandoverCanceledFilter(OwnershipHandoverCanceledFilter),
        OwnershipHandoverRequestedFilter(OwnershipHandoverRequestedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for MultiModalWorkerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(MultiModalWorkerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipHandoverCanceledFilter::decode_log(log) {
                return Ok(
                    MultiModalWorkerEvents::OwnershipHandoverCanceledFilter(decoded),
                );
            }
            if let Ok(decoded) = OwnershipHandoverRequestedFilter::decode_log(log) {
                return Ok(
                    MultiModalWorkerEvents::OwnershipHandoverRequestedFilter(decoded),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MultiModalWorkerEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MultiModalWorkerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipHandoverCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipHandoverRequestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for MultiModalWorkerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverCanceledFilter>
    for MultiModalWorkerEvents {
        fn from(value: OwnershipHandoverCanceledFilter) -> Self {
            Self::OwnershipHandoverCanceledFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverRequestedFilter>
    for MultiModalWorkerEvents {
        fn from(value: OwnershipHandoverRequestedFilter) -> Self {
            Self::OwnershipHandoverRequestedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for MultiModalWorkerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `LIMESTONE_DIAMOND` function with signature `LIMESTONE_DIAMOND()` and selector `0x35804023`
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
    #[ethcall(name = "LIMESTONE_DIAMOND", abi = "LIMESTONE_DIAMOND()")]
    pub struct LimestoneDiamondCall;
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
        pub pos_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `cancelOwnershipHandover` function with signature `cancelOwnershipHandover()` and selector `0x54d1f13d`
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
    #[ethcall(name = "cancelOwnershipHandover", abi = "cancelOwnershipHandover()")]
    pub struct CancelOwnershipHandoverCall;
    ///Container type for all input parameters for the `completeOwnershipHandover` function with signature `completeOwnershipHandover(address)` and selector `0xf04e283e`
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
        name = "completeOwnershipHandover",
        abi = "completeOwnershipHandover(address)"
    )]
    pub struct CompleteOwnershipHandoverCall {
        pub pending_owner: ::ethers::core::types::Address,
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
    pub struct DivestCall {
        pub ctx: V2LikePositionDivestmentContext,
    }
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address[])` and selector `0xe6bfbfd8`
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
        name = "initialize",
        abi = "initialize(address,address,address,address[])"
    )]
    pub struct InitializeCall {
        pub pair: ::ethers::core::types::Address,
        pub reward_pool: ::ethers::core::types::Address,
        pub router: ::ethers::core::types::Address,
        pub rewards: ::std::vec::Vec<::ethers::core::types::Address>,
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
    ///Container type for all input parameters for the `ownershipHandoverExpiresAt` function with signature `ownershipHandoverExpiresAt(address)` and selector `0xfee81cf4`
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
        name = "ownershipHandoverExpiresAt",
        abi = "ownershipHandoverExpiresAt(address)"
    )]
    pub struct OwnershipHandoverExpiresAtCall {
        pub pending_owner: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
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
    ///Container type for all input parameters for the `requestOwnershipHandover` function with signature `requestOwnershipHandover()` and selector `0x25692962`
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
    #[ethcall(name = "requestOwnershipHandover", abi = "requestOwnershipHandover()")]
    pub struct RequestOwnershipHandoverCall;
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
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum MultiModalWorkerCalls {
        LimestoneDiamond(LimestoneDiamondCall),
        CalculatePositionValue(CalculatePositionValueCall),
        CancelOwnershipHandover(CancelOwnershipHandoverCall),
        CompleteOwnershipHandover(CompleteOwnershipHandoverCall),
        Divest(DivestCall),
        GetPosition(GetPositionCall),
        Initialize(InitializeCall),
        Invest(InvestCall),
        Liquidate(LiquidateCall),
        Owner(OwnerCall),
        OwnershipHandoverExpiresAt(OwnershipHandoverExpiresAtCall),
        Reinvest(ReinvestCall),
        RenounceOwnership(RenounceOwnershipCall),
        RepayDebt(RepayDebtCall),
        RequestOwnershipHandover(RequestOwnershipHandoverCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for MultiModalWorkerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <LimestoneDiamondCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LimestoneDiamond(decoded));
            }
            if let Ok(decoded) = <CalculatePositionValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculatePositionValue(decoded));
            }
            if let Ok(decoded) = <CancelOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CancelOwnershipHandover(decoded));
            }
            if let Ok(decoded) = <CompleteOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CompleteOwnershipHandover(decoded));
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
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
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
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <OwnershipHandoverExpiresAtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnershipHandoverExpiresAt(decoded));
            }
            if let Ok(decoded) = <ReinvestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Reinvest(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RepayDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RepayDebt(decoded));
            }
            if let Ok(decoded) = <RequestOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequestOwnershipHandover(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MultiModalWorkerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::LimestoneDiamond(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculatePositionValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelOwnershipHandover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteOwnershipHandover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Divest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnershipHandoverExpiresAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Reinvest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestOwnershipHandover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MultiModalWorkerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LimestoneDiamond(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculatePositionValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CancelOwnershipHandover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompleteOwnershipHandover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Divest(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invest(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipHandoverExpiresAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Reinvest(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestOwnershipHandover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LimestoneDiamondCall> for MultiModalWorkerCalls {
        fn from(value: LimestoneDiamondCall) -> Self {
            Self::LimestoneDiamond(value)
        }
    }
    impl ::core::convert::From<CalculatePositionValueCall> for MultiModalWorkerCalls {
        fn from(value: CalculatePositionValueCall) -> Self {
            Self::CalculatePositionValue(value)
        }
    }
    impl ::core::convert::From<CancelOwnershipHandoverCall> for MultiModalWorkerCalls {
        fn from(value: CancelOwnershipHandoverCall) -> Self {
            Self::CancelOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<CompleteOwnershipHandoverCall> for MultiModalWorkerCalls {
        fn from(value: CompleteOwnershipHandoverCall) -> Self {
            Self::CompleteOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<DivestCall> for MultiModalWorkerCalls {
        fn from(value: DivestCall) -> Self {
            Self::Divest(value)
        }
    }
    impl ::core::convert::From<GetPositionCall> for MultiModalWorkerCalls {
        fn from(value: GetPositionCall) -> Self {
            Self::GetPosition(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for MultiModalWorkerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InvestCall> for MultiModalWorkerCalls {
        fn from(value: InvestCall) -> Self {
            Self::Invest(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for MultiModalWorkerCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MultiModalWorkerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverExpiresAtCall>
    for MultiModalWorkerCalls {
        fn from(value: OwnershipHandoverExpiresAtCall) -> Self {
            Self::OwnershipHandoverExpiresAt(value)
        }
    }
    impl ::core::convert::From<ReinvestCall> for MultiModalWorkerCalls {
        fn from(value: ReinvestCall) -> Self {
            Self::Reinvest(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for MultiModalWorkerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RepayDebtCall> for MultiModalWorkerCalls {
        fn from(value: RepayDebtCall) -> Self {
            Self::RepayDebt(value)
        }
    }
    impl ::core::convert::From<RequestOwnershipHandoverCall> for MultiModalWorkerCalls {
        fn from(value: RequestOwnershipHandoverCall) -> Self {
            Self::RequestOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MultiModalWorkerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `LIMESTONE_DIAMOND` function with signature `LIMESTONE_DIAMOND()` and selector `0x35804023`
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
    pub struct LimestoneDiamondReturn(pub ::ethers::core::types::Address);
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
    pub struct CalculatePositionValueReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
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
    pub struct OwnerReturn {
        pub result: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `ownershipHandoverExpiresAt` function with signature `ownershipHandoverExpiresAt(address)` and selector `0xfee81cf4`
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
    pub struct OwnershipHandoverExpiresAtReturn {
        pub result: ::ethers::core::types::U256,
    }
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
