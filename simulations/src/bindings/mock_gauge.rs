pub use mock_gauge::*;
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
pub mod mock_gauge {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stakingToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_feesVotingReward"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_rewardToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_isPool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
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
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("earned"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("earned"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_account"),
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
                    ::std::borrow::ToOwned::to_owned("fees0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fees0"),
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
                    ::std::borrow::ToOwned::to_owned("fees1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fees1"),
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
                    ::std::borrow::ToOwned::to_owned("feesVotingReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feesVotingReward"),
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
                    ::std::borrow::ToOwned::to_owned("getReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_account"),
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
                    ::std::borrow::ToOwned::to_owned("isPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPool"),
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
                    ::std::borrow::ToOwned::to_owned("lastTimeRewardApplicable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastTimeRewardApplicable",
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
                    ::std::borrow::ToOwned::to_owned("lastUpdateTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastUpdateTime"),
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
                    ::std::borrow::ToOwned::to_owned("left"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("left"),
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
                    ::std::borrow::ToOwned::to_owned("mutateTotal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mutateTotal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_newTotal"),
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
                    ::std::borrow::ToOwned::to_owned("notifyRewardAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("notifyRewardAmount"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("notifyRewardWithoutClaim"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "notifyRewardWithoutClaim",
                            ),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("periodFinish"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("periodFinish"),
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
                    ::std::borrow::ToOwned::to_owned("rewardPerToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardPerToken"),
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
                    ::std::borrow::ToOwned::to_owned("rewardPerTokenStored"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "rewardPerTokenStored",
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
                    ::std::borrow::ToOwned::to_owned("rewardRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardRate"),
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
                    ::std::borrow::ToOwned::to_owned("rewardRateByEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardRateByEpoch"),
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
                    ::std::borrow::ToOwned::to_owned("rewardToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardToken"),
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
                    ::std::borrow::ToOwned::to_owned("rewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewards"),
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
                    ::std::borrow::ToOwned::to_owned("stakingToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stakingToken"),
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
                    ::std::borrow::ToOwned::to_owned("userRewardPerTokenPaid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "userRewardPerTokenPaid",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("ClaimFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ClaimFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("claimed0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("claimed1"),
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
                    ::std::borrow::ToOwned::to_owned("ClaimRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ClaimRewards"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
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
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deposit"),
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
                    ::std::borrow::ToOwned::to_owned("NotifyReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NotifyReward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
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
                    ::std::borrow::ToOwned::to_owned("Withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NotAlive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotAlive"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotAuthorized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotAuthorized"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotTeam"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotTeam"),
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
                    ::std::borrow::ToOwned::to_owned("RewardRateTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RewardRateTooHigh"),
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
                    ::std::borrow::ToOwned::to_owned("ZeroAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroAmount"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroRewardRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroRewardRate"),
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
    pub static MOCKGAUGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@Qa\r\xF68\x03\x80a\r\xF6\x839\x81\x01`@\x81\x90Ra\0/\x91a\0rV[`\x01_U`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x80R\x91\x83\x16`\xC0R\x90\x91\x16`\xA0R\x15\x15`\xE0Ra\0\xC9V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0mW__\xFD[\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a\0\x85W__\xFD[a\0\x8E\x85a\0WV[\x93Pa\0\x9C` \x86\x01a\0WV[\x92Pa\0\xAA`@\x86\x01a\0WV[\x91P``\x85\x01Q\x80\x15\x15\x81\x14a\0\xBEW__\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x0C\xD0a\x01&_9_a\x03Y\x01R_a\x01\xD4\x01R_\x81\x81a\x03\x99\x01R\x81\x81a\x06)\x01R\x81\x81a\x08\x1D\x01R\x81\x81a\x08\x86\x01Ra\t\x19\x01R_\x81\x81a\x02\x87\x01R\x81\x81a\x04\xCC\x01Ra\nN\x01Ra\x0C\xD0_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x86W_5`\xE0\x1C\x80c\x8B\x87cG\x11a\0\xD9W\x80c\xCD\x1D\xE4\xB8\x11a\0\x93W\x80c\xDF\x13me\x11a\0nW\x80c\xDF\x13me\x14a\x03KW\x80c\xE2\xE1\xC6\xDB\x14a\x03TW\x80c\xEB\xE2\xB1+\x14a\x03\x8BW\x80c\xF7\xC6\x18\xC1\x14a\x03\x94W__\xFD[\x80c\xCD\x1D\xE4\xB8\x14a\x030W\x80c\xCD=\xAF\x9D\x14a\x03CW\x80c\xDC\xDC\x18\xDC\x14a\x024W__\xFD[\x80c\x8B\x87cG\x14a\x02\xBAW\x80c\x93\xF1\xC4B\x14a\x02\xD9W\x80c\x94\xAF[c\x14a\x02\xE2W\x80c\xB6\xB5_%\x14a\x03\x01W\x80c\xC0\0\x07\xB0\x14a\x03\x14W\x80c\xC8\xF3<\x91\x14a\x03'W__\xFD[\x80c<k\x16\xAB\x11a\x01DW\x80cp\xA0\x821\x11a\x01\x1FW\x80cp\xA0\x821\x14a\x02cW\x80cr\xF7\x02\xF3\x14a\x02\x82W\x80c{\nG\xEE\x14a\x02\xA9W\x80c\x80\xFA\xA5}\x14a\x02\xB2W__\xFD[\x80c<k\x16\xAB\x14a\x024W\x80cL\x02\xA2\x1C\x14a\x02GW\x80cnU?e\x14a\x02PW__\xFD[\x80b\x8C\xC2b\x14a\x01\x8AW\x80c\x07\0\x03}\x14a\x01\xB0W\x80c\x0F\xE2\xF7\x11\x14a\x01\xCFW\x80c\x16\xE6@H\x14a\x02\x0EW\x80c\x18\x16\r\xDD\x14a\x02\x16W\x80c.\x1A}M\x14a\x02\x1FW[__\xFD[a\x01\x9Da\x01\x986`\x04a\x0B\xDBV[a\x03\xBBV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x9Da\x01\xBE6`\x04a\x0B\xDBV[`\x08` R_\x90\x81R`@\x90 T\x81V[a\x01\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xA7V[a\x01\x9Da\x046V[a\x01\x9D`\x05T\x81V[a\x022a\x02-6`\x04a\x0B\xFBV[a\x04jV[\0[a\x022a\x02B6`\x04a\x0B\xFBV[a\x05CV[a\x01\x9D`\x0BT\x81V[a\x022a\x02^6`\x04a\x0C\x12V[a\x05\x81V[a\x01\x9Da\x02q6`\x04a\x0B\xDBV[`\x06` R_\x90\x81R`@\x90 T\x81V[a\x01\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x9D`\x02T\x81V[a\x01\x9Da\x05\x8FV[a\x01\x9Da\x02\xC86`\x04a\x0B\xDBV[`\x07` R_\x90\x81R`@\x90 T\x81V[a\x01\x9D`\nT\x81V[a\x01\x9Da\x02\xF06`\x04a\x0B\xFBV[`\t` R_\x90\x81R`@\x90 T\x81V[a\x022a\x03\x0F6`\x04a\x0B\xFBV[a\x05\xA7V[a\x022a\x03\"6`\x04a\x0B\xDBV[a\x05\xB1V[a\x01\x9D`\x03T\x81V[a\x022a\x03>6`\x04a\x0B\xFBV[a\x06\x9FV[a\x01\x9Da\x06\xADV[a\x01\x9D`\x04T\x81V[a\x03{\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xA7V[a\x01\x9D`\x01T\x81V[a\x01\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x08` \x90\x81R`@\x80\x83 T`\x07\x90\x92R\x82 Tg\r\xE0\xB6\xB3\xA7d\0\0\x90a\x03\xF0a\x06\xADV[a\x03\xFA\x91\x90a\x0CPV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x06` R`@\x90 Ta\x04\x1C\x91\x90a\x0CcV[a\x04&\x91\x90a\x0CzV[a\x040\x91\x90a\x0C\x99V[\x92\x91PPV[_`\x01TB\x10a\x04EWP_\x90V[_B`\x01Ta\x04T\x91\x90a\x0CPV[\x90P`\x02T\x81a\x04d\x91\x90a\x0CcV[\x91PP\x90V[a\x04ra\x07\rV[3a\x04|\x81a\x075V[\x81`\x05_\x82\x82Ta\x04\x8D\x91\x90a\x0CPV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x06` R`@\x81 \x80T\x84\x92\x90a\x04\xB9\x90\x84\x90a\x0CPV[\x90\x91UPa\x04\xF3\x90P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82\x84a\x07\x81V[\x80`\x01`\x01`\xA0\x1B\x03\x16\x7F\x88N\xDA\xD9\xCEo\xA2D\r\x8AT\xCC\x124\x90\xEB\x96\xD2v\x84y\xD4\x9F\xF9\xC76a%\xA9BCd\x83`@Qa\x05.\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2Pa\x05@`\x01_UV[PV[a\x05Ka\x07\rV[3_\x82\x90\x03a\x05mW`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05w\x81\x83a\x07\xE5V[Pa\x05@`\x01_UV[a\x05\x8B\x82\x82a\n\x0FV[PPV[_a\x05\xA2B`\x01T\x80\x82\x18\x90\x82\x11\x02\x18\x90V[\x90P\x90V[a\x05@\x813a\n\x0FV[a\x05\xB9a\x07\rV[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x81\x14a\x05\xE3W`@Qc\xEA\x8EN\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xEC\x82a\x075V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x08` R`@\x90 T\x80\x15a\x06\x94W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x08` R`@\x81 Ua\x06P\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x83a\x07\x81V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1F\x89\xF9c3\xD3\x130\0\xEEDts\x15\x1F\xA9`eC6\x8F\x02'\x1C\x9D\x95\xAE\x14\xF1;\xCCg\x82`@Qa\x06\x8B\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2[PPa\x05@`\x01_UV[a\x06\xA8_a\x075V[`\x05UV[_`\x05T_\x03a\x06\xBEWP`\x04T\x90V[`\x05Tg\r\xE0\xB6\xB3\xA7d\0\0`\x02T`\x03Ta\x06\xD8a\x05\x8FV[a\x06\xE2\x91\x90a\x0CPV[a\x06\xEC\x91\x90a\x0CcV[a\x06\xF6\x91\x90a\x0CcV[a\x07\0\x91\x90a\x0CzV[`\x04Ta\x05\xA2\x91\x90a\x0C\x99V[`\x02_T\x03a\x07/W`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_UV[a\x07=a\x06\xADV[`\x04Ua\x07Ha\x05\x8FV[`\x03Ua\x07T\x81a\x03\xBBV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16_\x90\x81R`\x08` \x90\x81R`@\x80\x83 \x93\x90\x93U`\x04T`\x07\x90\x91R\x91\x90 UV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90Ra\x07\xE0\x91\x85\x91\x82\x16\x90c\xA9\x05\x9C\xBB\x90`d\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa\x0B\x11V[PPPV[a\x07\xEDa\x06\xADV[`\x04UB_a\x08\x05\x82b\t:\x80\x80\x82\x06\x82\x03\x01a\x0CPV[\x90P`\x01T\x82\x10a\x08WWa\x08E`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x850\x86a\x0B\x87V[a\x08O\x81\x84a\x0CzV[`\x02Ua\x08\xC9V[_\x82`\x01Ta\x08f\x91\x90a\x0CPV[\x90P_`\x02T\x82a\x08w\x91\x90a\x0CcV[\x90Pa\x08\xAE`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x870\x88a\x0B\x87V[\x82a\x08\xB9\x82\x87a\x0C\x99V[a\x08\xC3\x91\x90a\x0CzV[`\x02UPP[`\x02Tb\t:\x80\x83\x06\x83\x03_\x90\x81R`\t` R`@\x81 \x82\x90U\x03a\t\x02W`@Qc\x07\xCE\xD7\xAF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tfW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x8A\x91\x90a\x0C\xACV[\x90Pa\t\x96\x82\x82a\x0CzV[`\x02T\x11\x15a\t\xB8W`@Qc<k\xE1\xB3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x83\x90Ua\t\xC7\x82\x84a\x0C\x99V[`\x01U`@Q\x84\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x7F\tVgu)WqC\x06\xE1\xA6\xAD\x83IT\x04A-\xF6\xFD\xB92\xFC\xA6\xDC\x84\x9A~\xE9\x10\xD4\xC1\x90` \x01`@Q\x80\x91\x03\x90\xA2PPPPPV[a\n\x17a\x07\rV[\x81_\x03a\n7W`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\nA\x82a\x075V[a\nv`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x820\x86a\x0B\x87V[\x82`\x05_\x82\x82Ta\n\x87\x91\x90a\x0C\x99V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x06` R`@\x81 \x80T\x85\x92\x90a\n\xB3\x90\x84\x90a\x0C\x99V[\x92PP\x81\x90UP\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7FUH\xC87\xAB\x06\x8C\xF5j,$y\xDF\x08\x82\xA4\x92/\xD2\x03\xED\xB7Qs!\x83\x1D\x95\x07\x8C_b\x85`@Qa\n\xFF\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3Pa\x05\x8B`\x01_UV[__` _\x84Q` \x86\x01_\x88Z\xF1\x80a\x0B0W`@Q=_\x82>=\x81\xFD[PP_Q=\x91P\x81\x15a\x0BGW\x80`\x01\x14\x15a\x0BTV[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x0B\x81W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R`d\x82\x01\x83\x90Ra\x0B\x81\x91\x86\x91\x82\x16\x90c#\xB8r\xDD\x90`\x84\x01a\x07\xAEV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xD6W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x0B\xEBW__\xFD[a\x0B\xF4\x82a\x0B\xC0V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x0C\x0BW__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x0C#W__\xFD[\x825\x91Pa\x0C3` \x84\x01a\x0B\xC0V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x040Wa\x040a\x0C<V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x040Wa\x040a\x0C<V[_\x82a\x0C\x94WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x040Wa\x040a\x0C<V[_` \x82\x84\x03\x12\x15a\x0C\xBCW__\xFD[PQ\x91\x90PV\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static MOCKGAUGE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x86W_5`\xE0\x1C\x80c\x8B\x87cG\x11a\0\xD9W\x80c\xCD\x1D\xE4\xB8\x11a\0\x93W\x80c\xDF\x13me\x11a\0nW\x80c\xDF\x13me\x14a\x03KW\x80c\xE2\xE1\xC6\xDB\x14a\x03TW\x80c\xEB\xE2\xB1+\x14a\x03\x8BW\x80c\xF7\xC6\x18\xC1\x14a\x03\x94W__\xFD[\x80c\xCD\x1D\xE4\xB8\x14a\x030W\x80c\xCD=\xAF\x9D\x14a\x03CW\x80c\xDC\xDC\x18\xDC\x14a\x024W__\xFD[\x80c\x8B\x87cG\x14a\x02\xBAW\x80c\x93\xF1\xC4B\x14a\x02\xD9W\x80c\x94\xAF[c\x14a\x02\xE2W\x80c\xB6\xB5_%\x14a\x03\x01W\x80c\xC0\0\x07\xB0\x14a\x03\x14W\x80c\xC8\xF3<\x91\x14a\x03'W__\xFD[\x80c<k\x16\xAB\x11a\x01DW\x80cp\xA0\x821\x11a\x01\x1FW\x80cp\xA0\x821\x14a\x02cW\x80cr\xF7\x02\xF3\x14a\x02\x82W\x80c{\nG\xEE\x14a\x02\xA9W\x80c\x80\xFA\xA5}\x14a\x02\xB2W__\xFD[\x80c<k\x16\xAB\x14a\x024W\x80cL\x02\xA2\x1C\x14a\x02GW\x80cnU?e\x14a\x02PW__\xFD[\x80b\x8C\xC2b\x14a\x01\x8AW\x80c\x07\0\x03}\x14a\x01\xB0W\x80c\x0F\xE2\xF7\x11\x14a\x01\xCFW\x80c\x16\xE6@H\x14a\x02\x0EW\x80c\x18\x16\r\xDD\x14a\x02\x16W\x80c.\x1A}M\x14a\x02\x1FW[__\xFD[a\x01\x9Da\x01\x986`\x04a\x0B\xDBV[a\x03\xBBV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x9Da\x01\xBE6`\x04a\x0B\xDBV[`\x08` R_\x90\x81R`@\x90 T\x81V[a\x01\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xA7V[a\x01\x9Da\x046V[a\x01\x9D`\x05T\x81V[a\x022a\x02-6`\x04a\x0B\xFBV[a\x04jV[\0[a\x022a\x02B6`\x04a\x0B\xFBV[a\x05CV[a\x01\x9D`\x0BT\x81V[a\x022a\x02^6`\x04a\x0C\x12V[a\x05\x81V[a\x01\x9Da\x02q6`\x04a\x0B\xDBV[`\x06` R_\x90\x81R`@\x90 T\x81V[a\x01\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x9D`\x02T\x81V[a\x01\x9Da\x05\x8FV[a\x01\x9Da\x02\xC86`\x04a\x0B\xDBV[`\x07` R_\x90\x81R`@\x90 T\x81V[a\x01\x9D`\nT\x81V[a\x01\x9Da\x02\xF06`\x04a\x0B\xFBV[`\t` R_\x90\x81R`@\x90 T\x81V[a\x022a\x03\x0F6`\x04a\x0B\xFBV[a\x05\xA7V[a\x022a\x03\"6`\x04a\x0B\xDBV[a\x05\xB1V[a\x01\x9D`\x03T\x81V[a\x022a\x03>6`\x04a\x0B\xFBV[a\x06\x9FV[a\x01\x9Da\x06\xADV[a\x01\x9D`\x04T\x81V[a\x03{\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xA7V[a\x01\x9D`\x01T\x81V[a\x01\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x08` \x90\x81R`@\x80\x83 T`\x07\x90\x92R\x82 Tg\r\xE0\xB6\xB3\xA7d\0\0\x90a\x03\xF0a\x06\xADV[a\x03\xFA\x91\x90a\x0CPV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x06` R`@\x90 Ta\x04\x1C\x91\x90a\x0CcV[a\x04&\x91\x90a\x0CzV[a\x040\x91\x90a\x0C\x99V[\x92\x91PPV[_`\x01TB\x10a\x04EWP_\x90V[_B`\x01Ta\x04T\x91\x90a\x0CPV[\x90P`\x02T\x81a\x04d\x91\x90a\x0CcV[\x91PP\x90V[a\x04ra\x07\rV[3a\x04|\x81a\x075V[\x81`\x05_\x82\x82Ta\x04\x8D\x91\x90a\x0CPV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x06` R`@\x81 \x80T\x84\x92\x90a\x04\xB9\x90\x84\x90a\x0CPV[\x90\x91UPa\x04\xF3\x90P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82\x84a\x07\x81V[\x80`\x01`\x01`\xA0\x1B\x03\x16\x7F\x88N\xDA\xD9\xCEo\xA2D\r\x8AT\xCC\x124\x90\xEB\x96\xD2v\x84y\xD4\x9F\xF9\xC76a%\xA9BCd\x83`@Qa\x05.\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2Pa\x05@`\x01_UV[PV[a\x05Ka\x07\rV[3_\x82\x90\x03a\x05mW`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05w\x81\x83a\x07\xE5V[Pa\x05@`\x01_UV[a\x05\x8B\x82\x82a\n\x0FV[PPV[_a\x05\xA2B`\x01T\x80\x82\x18\x90\x82\x11\x02\x18\x90V[\x90P\x90V[a\x05@\x813a\n\x0FV[a\x05\xB9a\x07\rV[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x81\x14a\x05\xE3W`@Qc\xEA\x8EN\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xEC\x82a\x075V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x08` R`@\x90 T\x80\x15a\x06\x94W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x08` R`@\x81 Ua\x06P\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x83a\x07\x81V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1F\x89\xF9c3\xD3\x130\0\xEEDts\x15\x1F\xA9`eC6\x8F\x02'\x1C\x9D\x95\xAE\x14\xF1;\xCCg\x82`@Qa\x06\x8B\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2[PPa\x05@`\x01_UV[a\x06\xA8_a\x075V[`\x05UV[_`\x05T_\x03a\x06\xBEWP`\x04T\x90V[`\x05Tg\r\xE0\xB6\xB3\xA7d\0\0`\x02T`\x03Ta\x06\xD8a\x05\x8FV[a\x06\xE2\x91\x90a\x0CPV[a\x06\xEC\x91\x90a\x0CcV[a\x06\xF6\x91\x90a\x0CcV[a\x07\0\x91\x90a\x0CzV[`\x04Ta\x05\xA2\x91\x90a\x0C\x99V[`\x02_T\x03a\x07/W`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_UV[a\x07=a\x06\xADV[`\x04Ua\x07Ha\x05\x8FV[`\x03Ua\x07T\x81a\x03\xBBV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16_\x90\x81R`\x08` \x90\x81R`@\x80\x83 \x93\x90\x93U`\x04T`\x07\x90\x91R\x91\x90 UV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90Ra\x07\xE0\x91\x85\x91\x82\x16\x90c\xA9\x05\x9C\xBB\x90`d\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa\x0B\x11V[PPPV[a\x07\xEDa\x06\xADV[`\x04UB_a\x08\x05\x82b\t:\x80\x80\x82\x06\x82\x03\x01a\x0CPV[\x90P`\x01T\x82\x10a\x08WWa\x08E`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x850\x86a\x0B\x87V[a\x08O\x81\x84a\x0CzV[`\x02Ua\x08\xC9V[_\x82`\x01Ta\x08f\x91\x90a\x0CPV[\x90P_`\x02T\x82a\x08w\x91\x90a\x0CcV[\x90Pa\x08\xAE`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x870\x88a\x0B\x87V[\x82a\x08\xB9\x82\x87a\x0C\x99V[a\x08\xC3\x91\x90a\x0CzV[`\x02UPP[`\x02Tb\t:\x80\x83\x06\x83\x03_\x90\x81R`\t` R`@\x81 \x82\x90U\x03a\t\x02W`@Qc\x07\xCE\xD7\xAF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tfW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x8A\x91\x90a\x0C\xACV[\x90Pa\t\x96\x82\x82a\x0CzV[`\x02T\x11\x15a\t\xB8W`@Qc<k\xE1\xB3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x83\x90Ua\t\xC7\x82\x84a\x0C\x99V[`\x01U`@Q\x84\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x7F\tVgu)WqC\x06\xE1\xA6\xAD\x83IT\x04A-\xF6\xFD\xB92\xFC\xA6\xDC\x84\x9A~\xE9\x10\xD4\xC1\x90` \x01`@Q\x80\x91\x03\x90\xA2PPPPPV[a\n\x17a\x07\rV[\x81_\x03a\n7W`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\nA\x82a\x075V[a\nv`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x820\x86a\x0B\x87V[\x82`\x05_\x82\x82Ta\n\x87\x91\x90a\x0C\x99V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x06` R`@\x81 \x80T\x85\x92\x90a\n\xB3\x90\x84\x90a\x0C\x99V[\x92PP\x81\x90UP\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7FUH\xC87\xAB\x06\x8C\xF5j,$y\xDF\x08\x82\xA4\x92/\xD2\x03\xED\xB7Qs!\x83\x1D\x95\x07\x8C_b\x85`@Qa\n\xFF\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3Pa\x05\x8B`\x01_UV[__` _\x84Q` \x86\x01_\x88Z\xF1\x80a\x0B0W`@Q=_\x82>=\x81\xFD[PP_Q=\x91P\x81\x15a\x0BGW\x80`\x01\x14\x15a\x0BTV[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x0B\x81W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R\x83\x81\x16`D\x83\x01R`d\x82\x01\x83\x90Ra\x0B\x81\x91\x86\x91\x82\x16\x90c#\xB8r\xDD\x90`\x84\x01a\x07\xAEV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xD6W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x0B\xEBW__\xFD[a\x0B\xF4\x82a\x0B\xC0V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x0C\x0BW__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x0C#W__\xFD[\x825\x91Pa\x0C3` \x84\x01a\x0B\xC0V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x040Wa\x040a\x0C<V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x040Wa\x040a\x0C<V[_\x82a\x0C\x94WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x040Wa\x040a\x0C<V[_` \x82\x84\x03\x12\x15a\x0C\xBCW__\xFD[PQ\x91\x90PV\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static MOCKGAUGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockGauge<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockGauge<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockGauge<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockGauge<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockGauge<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockGauge)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockGauge<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKGAUGE_ABI.clone(),
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
                MOCKGAUGE_ABI.clone(),
                MOCKGAUGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x6e553f65) function
        pub fn deposit_with_recipient(
            &self,
            amount: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 85, 63, 101], (amount, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xb6b55f25) function
        pub fn deposit(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 181, 95, 37], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `earned` (0x008cc262) function
        pub fn earned(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 140, 194, 98], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fees0` (0x93f1c442) function
        pub fn fees_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([147, 241, 196, 66], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fees1` (0x4c02a21c) function
        pub fn fees_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([76, 2, 162, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feesVotingReward` (0x0fe2f711) function
        pub fn fees_voting_reward(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([15, 226, 247, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReward` (0xc00007b0) function
        pub fn get_reward(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 0, 7, 176], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPool` (0xe2e1c6db) function
        pub fn is_pool(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([226, 225, 198, 219], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastTimeRewardApplicable` (0x80faa57d) function
        pub fn last_time_reward_applicable(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([128, 250, 165, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastUpdateTime` (0xc8f33c91) function
        pub fn last_update_time(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([200, 243, 60, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `left` (0x16e64048) function
        pub fn left(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([22, 230, 64, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mutateTotal` (0xcd1de4b8) function
        pub fn mutate_total(
            &self,
            new_total: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 29, 228, 184], new_total)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `notifyRewardAmount` (0x3c6b16ab) function
        pub fn notify_reward_amount(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 107, 22, 171], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `notifyRewardWithoutClaim` (0xdcdc18dc) function
        pub fn notify_reward_without_claim(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([220, 220, 24, 220], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `periodFinish` (0xebe2b12b) function
        pub fn period_finish(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([235, 226, 177, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardPerToken` (0xcd3daf9d) function
        pub fn reward_per_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([205, 61, 175, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardPerTokenStored` (0xdf136d65) function
        pub fn reward_per_token_stored(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([223, 19, 109, 101], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardRate` (0x7b0a47ee) function
        pub fn reward_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([123, 10, 71, 238], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardRateByEpoch` (0x94af5b63) function
        pub fn reward_rate_by_epoch(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([148, 175, 91, 99], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardToken` (0xf7c618c1) function
        pub fn reward_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([247, 198, 24, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewards` (0x0700037d) function
        pub fn rewards(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([7, 0, 3, 125], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakingToken` (0x72f702f3) function
        pub fn staking_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([114, 247, 2, 243], ())
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
        ///Calls the contract's `userRewardPerTokenPaid` (0x8b876347) function
        pub fn user_reward_per_token_paid(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([139, 135, 99, 71], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amount)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ClaimFees` event
        pub fn claim_fees_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ClaimFeesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ClaimRewards` event
        pub fn claim_rewards_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ClaimRewardsFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `NotifyReward` event
        pub fn notify_reward_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NotifyRewardFilter,
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
            MockGaugeEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockGauge<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotAlive` with signature `NotAlive()` and selector `0x10f3d9c9`
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
    #[etherror(name = "NotAlive", abi = "NotAlive()")]
    pub struct NotAlive;
    ///Custom Error type `NotAuthorized` with signature `NotAuthorized()` and selector `0xea8e4eb5`
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
    #[etherror(name = "NotAuthorized", abi = "NotAuthorized()")]
    pub struct NotAuthorized;
    ///Custom Error type `NotTeam` with signature `NotTeam()` and selector `0xe9f3e974`
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
    #[etherror(name = "NotTeam", abi = "NotTeam()")]
    pub struct NotTeam;
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
    ///Custom Error type `RewardRateTooHigh` with signature `RewardRateTooHigh()` and selector `0x3c6be1b3`
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
    #[etherror(name = "RewardRateTooHigh", abi = "RewardRateTooHigh()")]
    pub struct RewardRateTooHigh;
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
    ///Custom Error type `ZeroAmount` with signature `ZeroAmount()` and selector `0x1f2a2005`
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
    #[etherror(name = "ZeroAmount", abi = "ZeroAmount()")]
    pub struct ZeroAmount;
    ///Custom Error type `ZeroRewardRate` with signature `ZeroRewardRate()` and selector `0x07ced7af`
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
    #[etherror(name = "ZeroRewardRate", abi = "ZeroRewardRate()")]
    pub struct ZeroRewardRate;
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
    pub enum MockGaugeErrors {
        NotAlive(NotAlive),
        NotAuthorized(NotAuthorized),
        NotTeam(NotTeam),
        NotVoter(NotVoter),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        RewardRateTooHigh(RewardRateTooHigh),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        ZeroAmount(ZeroAmount),
        ZeroRewardRate(ZeroRewardRate),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockGaugeErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <NotAlive as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotAlive(decoded));
            }
            if let Ok(decoded) = <NotAuthorized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotAuthorized(decoded));
            }
            if let Ok(decoded) = <NotTeam as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotTeam(decoded));
            }
            if let Ok(decoded) = <NotVoter as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotVoter(decoded));
            }
            if let Ok(decoded) = <ReentrancyGuardReentrantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReentrancyGuardReentrantCall(decoded));
            }
            if let Ok(decoded) = <RewardRateTooHigh as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RewardRateTooHigh(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            if let Ok(decoded) = <ZeroAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroAmount(decoded));
            }
            if let Ok(decoded) = <ZeroRewardRate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroRewardRate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockGaugeErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::NotAlive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotAuthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotTeam(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotVoter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardRateTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroRewardRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockGaugeErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <NotAlive as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotAuthorized as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotTeam as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotVoter as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ReentrancyGuardReentrantCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RewardRateTooHigh as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroAmount as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ZeroRewardRate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockGaugeErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NotAlive(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotAuthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotTeam(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotVoter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardRateTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ZeroAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroRewardRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockGaugeErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<NotAlive> for MockGaugeErrors {
        fn from(value: NotAlive) -> Self {
            Self::NotAlive(value)
        }
    }
    impl ::core::convert::From<NotAuthorized> for MockGaugeErrors {
        fn from(value: NotAuthorized) -> Self {
            Self::NotAuthorized(value)
        }
    }
    impl ::core::convert::From<NotTeam> for MockGaugeErrors {
        fn from(value: NotTeam) -> Self {
            Self::NotTeam(value)
        }
    }
    impl ::core::convert::From<NotVoter> for MockGaugeErrors {
        fn from(value: NotVoter) -> Self {
            Self::NotVoter(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for MockGaugeErrors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<RewardRateTooHigh> for MockGaugeErrors {
        fn from(value: RewardRateTooHigh) -> Self {
            Self::RewardRateTooHigh(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for MockGaugeErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    impl ::core::convert::From<ZeroAmount> for MockGaugeErrors {
        fn from(value: ZeroAmount) -> Self {
            Self::ZeroAmount(value)
        }
    }
    impl ::core::convert::From<ZeroRewardRate> for MockGaugeErrors {
        fn from(value: ZeroRewardRate) -> Self {
            Self::ZeroRewardRate(value)
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
    #[ethevent(name = "ClaimFees", abi = "ClaimFees(address,uint256,uint256)")]
    pub struct ClaimFeesFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub claimed_0: ::ethers::core::types::U256,
        pub claimed_1: ::ethers::core::types::U256,
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
    #[ethevent(name = "ClaimRewards", abi = "ClaimRewards(address,uint256)")]
    pub struct ClaimRewardsFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
    #[ethevent(name = "NotifyReward", abi = "NotifyReward(address,uint256)")]
    pub struct NotifyRewardFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
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
    #[ethevent(name = "Withdraw", abi = "Withdraw(address,uint256)")]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub enum MockGaugeEvents {
        ClaimFeesFilter(ClaimFeesFilter),
        ClaimRewardsFilter(ClaimRewardsFilter),
        DepositFilter(DepositFilter),
        NotifyRewardFilter(NotifyRewardFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockGaugeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ClaimFeesFilter::decode_log(log) {
                return Ok(MockGaugeEvents::ClaimFeesFilter(decoded));
            }
            if let Ok(decoded) = ClaimRewardsFilter::decode_log(log) {
                return Ok(MockGaugeEvents::ClaimRewardsFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(MockGaugeEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = NotifyRewardFilter::decode_log(log) {
                return Ok(MockGaugeEvents::NotifyRewardFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(MockGaugeEvents::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockGaugeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClaimFeesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimRewardsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotifyRewardFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClaimFeesFilter> for MockGaugeEvents {
        fn from(value: ClaimFeesFilter) -> Self {
            Self::ClaimFeesFilter(value)
        }
    }
    impl ::core::convert::From<ClaimRewardsFilter> for MockGaugeEvents {
        fn from(value: ClaimRewardsFilter) -> Self {
            Self::ClaimRewardsFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for MockGaugeEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<NotifyRewardFilter> for MockGaugeEvents {
        fn from(value: NotifyRewardFilter) -> Self {
            Self::NotifyRewardFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for MockGaugeEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
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
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `deposit` function with signature `deposit(uint256,address)` and selector `0x6e553f65`
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
    #[ethcall(name = "deposit", abi = "deposit(uint256,address)")]
    pub struct DepositWithRecipientCall {
        pub amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(uint256)` and selector `0xb6b55f25`
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
    #[ethcall(name = "deposit", abi = "deposit(uint256)")]
    pub struct DepositCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `earned` function with signature `earned(address)` and selector `0x008cc262`
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
    #[ethcall(name = "earned", abi = "earned(address)")]
    pub struct EarnedCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `fees0` function with signature `fees0()` and selector `0x93f1c442`
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
    #[ethcall(name = "fees0", abi = "fees0()")]
    pub struct Fees0Call;
    ///Container type for all input parameters for the `fees1` function with signature `fees1()` and selector `0x4c02a21c`
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
    #[ethcall(name = "fees1", abi = "fees1()")]
    pub struct Fees1Call;
    ///Container type for all input parameters for the `feesVotingReward` function with signature `feesVotingReward()` and selector `0x0fe2f711`
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
    #[ethcall(name = "feesVotingReward", abi = "feesVotingReward()")]
    pub struct FeesVotingRewardCall;
    ///Container type for all input parameters for the `getReward` function with signature `getReward(address)` and selector `0xc00007b0`
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
    #[ethcall(name = "getReward", abi = "getReward(address)")]
    pub struct GetRewardCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isPool` function with signature `isPool()` and selector `0xe2e1c6db`
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
    #[ethcall(name = "isPool", abi = "isPool()")]
    pub struct IsPoolCall;
    ///Container type for all input parameters for the `lastTimeRewardApplicable` function with signature `lastTimeRewardApplicable()` and selector `0x80faa57d`
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
    #[ethcall(name = "lastTimeRewardApplicable", abi = "lastTimeRewardApplicable()")]
    pub struct LastTimeRewardApplicableCall;
    ///Container type for all input parameters for the `lastUpdateTime` function with signature `lastUpdateTime()` and selector `0xc8f33c91`
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
    #[ethcall(name = "lastUpdateTime", abi = "lastUpdateTime()")]
    pub struct LastUpdateTimeCall;
    ///Container type for all input parameters for the `left` function with signature `left()` and selector `0x16e64048`
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
    #[ethcall(name = "left", abi = "left()")]
    pub struct LeftCall;
    ///Container type for all input parameters for the `mutateTotal` function with signature `mutateTotal(uint256)` and selector `0xcd1de4b8`
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
    #[ethcall(name = "mutateTotal", abi = "mutateTotal(uint256)")]
    pub struct MutateTotalCall {
        pub new_total: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `notifyRewardAmount` function with signature `notifyRewardAmount(uint256)` and selector `0x3c6b16ab`
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
    #[ethcall(name = "notifyRewardAmount", abi = "notifyRewardAmount(uint256)")]
    pub struct NotifyRewardAmountCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `notifyRewardWithoutClaim` function with signature `notifyRewardWithoutClaim(uint256)` and selector `0xdcdc18dc`
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
        name = "notifyRewardWithoutClaim",
        abi = "notifyRewardWithoutClaim(uint256)"
    )]
    pub struct NotifyRewardWithoutClaimCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `periodFinish` function with signature `periodFinish()` and selector `0xebe2b12b`
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
    #[ethcall(name = "periodFinish", abi = "periodFinish()")]
    pub struct PeriodFinishCall;
    ///Container type for all input parameters for the `rewardPerToken` function with signature `rewardPerToken()` and selector `0xcd3daf9d`
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
    #[ethcall(name = "rewardPerToken", abi = "rewardPerToken()")]
    pub struct RewardPerTokenCall;
    ///Container type for all input parameters for the `rewardPerTokenStored` function with signature `rewardPerTokenStored()` and selector `0xdf136d65`
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
    #[ethcall(name = "rewardPerTokenStored", abi = "rewardPerTokenStored()")]
    pub struct RewardPerTokenStoredCall;
    ///Container type for all input parameters for the `rewardRate` function with signature `rewardRate()` and selector `0x7b0a47ee`
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
    #[ethcall(name = "rewardRate", abi = "rewardRate()")]
    pub struct RewardRateCall;
    ///Container type for all input parameters for the `rewardRateByEpoch` function with signature `rewardRateByEpoch(uint256)` and selector `0x94af5b63`
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
    #[ethcall(name = "rewardRateByEpoch", abi = "rewardRateByEpoch(uint256)")]
    pub struct RewardRateByEpochCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `rewardToken` function with signature `rewardToken()` and selector `0xf7c618c1`
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
    #[ethcall(name = "rewardToken", abi = "rewardToken()")]
    pub struct RewardTokenCall;
    ///Container type for all input parameters for the `rewards` function with signature `rewards(address)` and selector `0x0700037d`
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
    #[ethcall(name = "rewards", abi = "rewards(address)")]
    pub struct RewardsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `stakingToken` function with signature `stakingToken()` and selector `0x72f702f3`
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
    #[ethcall(name = "stakingToken", abi = "stakingToken()")]
    pub struct StakingTokenCall;
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
    ///Container type for all input parameters for the `userRewardPerTokenPaid` function with signature `userRewardPerTokenPaid(address)` and selector `0x8b876347`
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
    #[ethcall(name = "userRewardPerTokenPaid", abi = "userRewardPerTokenPaid(address)")]
    pub struct UserRewardPerTokenPaidCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `0x2e1a7d4d`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
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
    pub enum MockGaugeCalls {
        BalanceOf(BalanceOfCall),
        DepositWithRecipient(DepositWithRecipientCall),
        Deposit(DepositCall),
        Earned(EarnedCall),
        Fees0(Fees0Call),
        Fees1(Fees1Call),
        FeesVotingReward(FeesVotingRewardCall),
        GetReward(GetRewardCall),
        IsPool(IsPoolCall),
        LastTimeRewardApplicable(LastTimeRewardApplicableCall),
        LastUpdateTime(LastUpdateTimeCall),
        Left(LeftCall),
        MutateTotal(MutateTotalCall),
        NotifyRewardAmount(NotifyRewardAmountCall),
        NotifyRewardWithoutClaim(NotifyRewardWithoutClaimCall),
        PeriodFinish(PeriodFinishCall),
        RewardPerToken(RewardPerTokenCall),
        RewardPerTokenStored(RewardPerTokenStoredCall),
        RewardRate(RewardRateCall),
        RewardRateByEpoch(RewardRateByEpochCall),
        RewardToken(RewardTokenCall),
        Rewards(RewardsCall),
        StakingToken(StakingTokenCall),
        TotalSupply(TotalSupplyCall),
        UserRewardPerTokenPaid(UserRewardPerTokenPaidCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockGaugeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <DepositWithRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositWithRecipient(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <EarnedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Earned(decoded));
            }
            if let Ok(decoded) = <Fees0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Fees0(decoded));
            }
            if let Ok(decoded) = <Fees1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Fees1(decoded));
            }
            if let Ok(decoded) = <FeesVotingRewardCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeesVotingReward(decoded));
            }
            if let Ok(decoded) = <GetRewardCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReward(decoded));
            }
            if let Ok(decoded) = <IsPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsPool(decoded));
            }
            if let Ok(decoded) = <LastTimeRewardApplicableCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastTimeRewardApplicable(decoded));
            }
            if let Ok(decoded) = <LastUpdateTimeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastUpdateTime(decoded));
            }
            if let Ok(decoded) = <LeftCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Left(decoded));
            }
            if let Ok(decoded) = <MutateTotalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MutateTotal(decoded));
            }
            if let Ok(decoded) = <NotifyRewardAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotifyRewardAmount(decoded));
            }
            if let Ok(decoded) = <NotifyRewardWithoutClaimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotifyRewardWithoutClaim(decoded));
            }
            if let Ok(decoded) = <PeriodFinishCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PeriodFinish(decoded));
            }
            if let Ok(decoded) = <RewardPerTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RewardPerToken(decoded));
            }
            if let Ok(decoded) = <RewardPerTokenStoredCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RewardPerTokenStored(decoded));
            }
            if let Ok(decoded) = <RewardRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RewardRate(decoded));
            }
            if let Ok(decoded) = <RewardRateByEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RewardRateByEpoch(decoded));
            }
            if let Ok(decoded) = <RewardTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RewardToken(decoded));
            }
            if let Ok(decoded) = <RewardsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Rewards(decoded));
            }
            if let Ok(decoded) = <StakingTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakingToken(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <UserRewardPerTokenPaidCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserRewardPerTokenPaid(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockGaugeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositWithRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Earned(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fees0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fees1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeesVotingReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastTimeRewardApplicable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastUpdateTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Left(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MutateTotal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotifyRewardAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotifyRewardWithoutClaim(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PeriodFinish(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardPerToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardPerTokenStored(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardRateByEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rewards(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakingToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserRewardPerTokenPaid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockGaugeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositWithRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Earned(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fees0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fees1(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeesVotingReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastTimeRewardApplicable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastUpdateTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::Left(element) => ::core::fmt::Display::fmt(element, f),
                Self::MutateTotal(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotifyRewardAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotifyRewardWithoutClaim(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PeriodFinish(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardPerToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardPerTokenStored(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardRateByEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakingToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserRewardPerTokenPaid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockGaugeCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DepositWithRecipientCall> for MockGaugeCalls {
        fn from(value: DepositWithRecipientCall) -> Self {
            Self::DepositWithRecipient(value)
        }
    }
    impl ::core::convert::From<DepositCall> for MockGaugeCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<EarnedCall> for MockGaugeCalls {
        fn from(value: EarnedCall) -> Self {
            Self::Earned(value)
        }
    }
    impl ::core::convert::From<Fees0Call> for MockGaugeCalls {
        fn from(value: Fees0Call) -> Self {
            Self::Fees0(value)
        }
    }
    impl ::core::convert::From<Fees1Call> for MockGaugeCalls {
        fn from(value: Fees1Call) -> Self {
            Self::Fees1(value)
        }
    }
    impl ::core::convert::From<FeesVotingRewardCall> for MockGaugeCalls {
        fn from(value: FeesVotingRewardCall) -> Self {
            Self::FeesVotingReward(value)
        }
    }
    impl ::core::convert::From<GetRewardCall> for MockGaugeCalls {
        fn from(value: GetRewardCall) -> Self {
            Self::GetReward(value)
        }
    }
    impl ::core::convert::From<IsPoolCall> for MockGaugeCalls {
        fn from(value: IsPoolCall) -> Self {
            Self::IsPool(value)
        }
    }
    impl ::core::convert::From<LastTimeRewardApplicableCall> for MockGaugeCalls {
        fn from(value: LastTimeRewardApplicableCall) -> Self {
            Self::LastTimeRewardApplicable(value)
        }
    }
    impl ::core::convert::From<LastUpdateTimeCall> for MockGaugeCalls {
        fn from(value: LastUpdateTimeCall) -> Self {
            Self::LastUpdateTime(value)
        }
    }
    impl ::core::convert::From<LeftCall> for MockGaugeCalls {
        fn from(value: LeftCall) -> Self {
            Self::Left(value)
        }
    }
    impl ::core::convert::From<MutateTotalCall> for MockGaugeCalls {
        fn from(value: MutateTotalCall) -> Self {
            Self::MutateTotal(value)
        }
    }
    impl ::core::convert::From<NotifyRewardAmountCall> for MockGaugeCalls {
        fn from(value: NotifyRewardAmountCall) -> Self {
            Self::NotifyRewardAmount(value)
        }
    }
    impl ::core::convert::From<NotifyRewardWithoutClaimCall> for MockGaugeCalls {
        fn from(value: NotifyRewardWithoutClaimCall) -> Self {
            Self::NotifyRewardWithoutClaim(value)
        }
    }
    impl ::core::convert::From<PeriodFinishCall> for MockGaugeCalls {
        fn from(value: PeriodFinishCall) -> Self {
            Self::PeriodFinish(value)
        }
    }
    impl ::core::convert::From<RewardPerTokenCall> for MockGaugeCalls {
        fn from(value: RewardPerTokenCall) -> Self {
            Self::RewardPerToken(value)
        }
    }
    impl ::core::convert::From<RewardPerTokenStoredCall> for MockGaugeCalls {
        fn from(value: RewardPerTokenStoredCall) -> Self {
            Self::RewardPerTokenStored(value)
        }
    }
    impl ::core::convert::From<RewardRateCall> for MockGaugeCalls {
        fn from(value: RewardRateCall) -> Self {
            Self::RewardRate(value)
        }
    }
    impl ::core::convert::From<RewardRateByEpochCall> for MockGaugeCalls {
        fn from(value: RewardRateByEpochCall) -> Self {
            Self::RewardRateByEpoch(value)
        }
    }
    impl ::core::convert::From<RewardTokenCall> for MockGaugeCalls {
        fn from(value: RewardTokenCall) -> Self {
            Self::RewardToken(value)
        }
    }
    impl ::core::convert::From<RewardsCall> for MockGaugeCalls {
        fn from(value: RewardsCall) -> Self {
            Self::Rewards(value)
        }
    }
    impl ::core::convert::From<StakingTokenCall> for MockGaugeCalls {
        fn from(value: StakingTokenCall) -> Self {
            Self::StakingToken(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for MockGaugeCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<UserRewardPerTokenPaidCall> for MockGaugeCalls {
        fn from(value: UserRewardPerTokenPaidCall) -> Self {
            Self::UserRewardPerTokenPaid(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for MockGaugeCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
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
    ///Container type for all return fields from the `earned` function with signature `earned(address)` and selector `0x008cc262`
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
    pub struct EarnedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `fees0` function with signature `fees0()` and selector `0x93f1c442`
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
    pub struct Fees0Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `fees1` function with signature `fees1()` and selector `0x4c02a21c`
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
    pub struct Fees1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `feesVotingReward` function with signature `feesVotingReward()` and selector `0x0fe2f711`
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
    pub struct FeesVotingRewardReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isPool` function with signature `isPool()` and selector `0xe2e1c6db`
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
    ///Container type for all return fields from the `lastTimeRewardApplicable` function with signature `lastTimeRewardApplicable()` and selector `0x80faa57d`
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
    pub struct LastTimeRewardApplicableReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lastUpdateTime` function with signature `lastUpdateTime()` and selector `0xc8f33c91`
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
    pub struct LastUpdateTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `left` function with signature `left()` and selector `0x16e64048`
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
    pub struct LeftReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `periodFinish` function with signature `periodFinish()` and selector `0xebe2b12b`
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
    pub struct PeriodFinishReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardPerToken` function with signature `rewardPerToken()` and selector `0xcd3daf9d`
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
    pub struct RewardPerTokenReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardPerTokenStored` function with signature `rewardPerTokenStored()` and selector `0xdf136d65`
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
    pub struct RewardPerTokenStoredReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardRate` function with signature `rewardRate()` and selector `0x7b0a47ee`
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
    pub struct RewardRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardRateByEpoch` function with signature `rewardRateByEpoch(uint256)` and selector `0x94af5b63`
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
    pub struct RewardRateByEpochReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardToken` function with signature `rewardToken()` and selector `0xf7c618c1`
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
    pub struct RewardTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `rewards` function with signature `rewards(address)` and selector `0x0700037d`
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
    pub struct RewardsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stakingToken` function with signature `stakingToken()` and selector `0x72f702f3`
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
    pub struct StakingTokenReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `userRewardPerTokenPaid` function with signature `userRewardPerTokenPaid(address)` and selector `0x8b876347`
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
    pub struct UserRewardPerTokenPaidReturn(pub ::ethers::core::types::U256);
}
