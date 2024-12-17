pub use i_gauge::*;
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
pub mod i_gauge {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
                                    name: ::std::borrow::ToOwned::to_owned("_earned"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_time"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_left"),
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
                    ::std::borrow::ToOwned::to_owned("notifyRewardAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("notifyRewardAmount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_rewardPerToken"),
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
                    ::std::borrow::ToOwned::to_owned("RewardRateTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RewardRateTooHigh"),
                            inputs: ::std::vec![],
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
    pub static IGAUGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IGauge<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IGauge<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IGauge<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IGauge<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IGauge<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IGauge)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IGauge<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IGAUGE_ABI.clone(),
                    client,
                ),
            )
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IGaugeEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IGauge<M> {
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
    pub enum IGaugeErrors {
        NotAlive(NotAlive),
        NotAuthorized(NotAuthorized),
        NotTeam(NotTeam),
        NotVoter(NotVoter),
        RewardRateTooHigh(RewardRateTooHigh),
        ZeroAmount(ZeroAmount),
        ZeroRewardRate(ZeroRewardRate),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IGaugeErrors {
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
            if let Ok(decoded) = <RewardRateTooHigh as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RewardRateTooHigh(decoded));
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
    impl ::ethers::core::abi::AbiEncode for IGaugeErrors {
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
                Self::RewardRateTooHigh(element) => {
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
    impl ::ethers::contract::ContractRevert for IGaugeErrors {
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
                    == <RewardRateTooHigh as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for IGaugeErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NotAlive(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotAuthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotTeam(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotVoter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardRateTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroRewardRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IGaugeErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<NotAlive> for IGaugeErrors {
        fn from(value: NotAlive) -> Self {
            Self::NotAlive(value)
        }
    }
    impl ::core::convert::From<NotAuthorized> for IGaugeErrors {
        fn from(value: NotAuthorized) -> Self {
            Self::NotAuthorized(value)
        }
    }
    impl ::core::convert::From<NotTeam> for IGaugeErrors {
        fn from(value: NotTeam) -> Self {
            Self::NotTeam(value)
        }
    }
    impl ::core::convert::From<NotVoter> for IGaugeErrors {
        fn from(value: NotVoter) -> Self {
            Self::NotVoter(value)
        }
    }
    impl ::core::convert::From<RewardRateTooHigh> for IGaugeErrors {
        fn from(value: RewardRateTooHigh) -> Self {
            Self::RewardRateTooHigh(value)
        }
    }
    impl ::core::convert::From<ZeroAmount> for IGaugeErrors {
        fn from(value: ZeroAmount) -> Self {
            Self::ZeroAmount(value)
        }
    }
    impl ::core::convert::From<ZeroRewardRate> for IGaugeErrors {
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
    pub enum IGaugeEvents {
        ClaimFeesFilter(ClaimFeesFilter),
        ClaimRewardsFilter(ClaimRewardsFilter),
        DepositFilter(DepositFilter),
        NotifyRewardFilter(NotifyRewardFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for IGaugeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ClaimFeesFilter::decode_log(log) {
                return Ok(IGaugeEvents::ClaimFeesFilter(decoded));
            }
            if let Ok(decoded) = ClaimRewardsFilter::decode_log(log) {
                return Ok(IGaugeEvents::ClaimRewardsFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(IGaugeEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = NotifyRewardFilter::decode_log(log) {
                return Ok(IGaugeEvents::NotifyRewardFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(IGaugeEvents::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IGaugeEvents {
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
    impl ::core::convert::From<ClaimFeesFilter> for IGaugeEvents {
        fn from(value: ClaimFeesFilter) -> Self {
            Self::ClaimFeesFilter(value)
        }
    }
    impl ::core::convert::From<ClaimRewardsFilter> for IGaugeEvents {
        fn from(value: ClaimRewardsFilter) -> Self {
            Self::ClaimRewardsFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for IGaugeEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<NotifyRewardFilter> for IGaugeEvents {
        fn from(value: NotifyRewardFilter) -> Self {
            Self::NotifyRewardFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for IGaugeEvents {
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
    pub enum IGaugeCalls {
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
    impl ::ethers::core::abi::AbiDecode for IGaugeCalls {
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
    impl ::ethers::core::abi::AbiEncode for IGaugeCalls {
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
    impl ::core::fmt::Display for IGaugeCalls {
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
    impl ::core::convert::From<BalanceOfCall> for IGaugeCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DepositWithRecipientCall> for IGaugeCalls {
        fn from(value: DepositWithRecipientCall) -> Self {
            Self::DepositWithRecipient(value)
        }
    }
    impl ::core::convert::From<DepositCall> for IGaugeCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<EarnedCall> for IGaugeCalls {
        fn from(value: EarnedCall) -> Self {
            Self::Earned(value)
        }
    }
    impl ::core::convert::From<Fees0Call> for IGaugeCalls {
        fn from(value: Fees0Call) -> Self {
            Self::Fees0(value)
        }
    }
    impl ::core::convert::From<Fees1Call> for IGaugeCalls {
        fn from(value: Fees1Call) -> Self {
            Self::Fees1(value)
        }
    }
    impl ::core::convert::From<FeesVotingRewardCall> for IGaugeCalls {
        fn from(value: FeesVotingRewardCall) -> Self {
            Self::FeesVotingReward(value)
        }
    }
    impl ::core::convert::From<GetRewardCall> for IGaugeCalls {
        fn from(value: GetRewardCall) -> Self {
            Self::GetReward(value)
        }
    }
    impl ::core::convert::From<IsPoolCall> for IGaugeCalls {
        fn from(value: IsPoolCall) -> Self {
            Self::IsPool(value)
        }
    }
    impl ::core::convert::From<LastTimeRewardApplicableCall> for IGaugeCalls {
        fn from(value: LastTimeRewardApplicableCall) -> Self {
            Self::LastTimeRewardApplicable(value)
        }
    }
    impl ::core::convert::From<LastUpdateTimeCall> for IGaugeCalls {
        fn from(value: LastUpdateTimeCall) -> Self {
            Self::LastUpdateTime(value)
        }
    }
    impl ::core::convert::From<LeftCall> for IGaugeCalls {
        fn from(value: LeftCall) -> Self {
            Self::Left(value)
        }
    }
    impl ::core::convert::From<NotifyRewardAmountCall> for IGaugeCalls {
        fn from(value: NotifyRewardAmountCall) -> Self {
            Self::NotifyRewardAmount(value)
        }
    }
    impl ::core::convert::From<NotifyRewardWithoutClaimCall> for IGaugeCalls {
        fn from(value: NotifyRewardWithoutClaimCall) -> Self {
            Self::NotifyRewardWithoutClaim(value)
        }
    }
    impl ::core::convert::From<PeriodFinishCall> for IGaugeCalls {
        fn from(value: PeriodFinishCall) -> Self {
            Self::PeriodFinish(value)
        }
    }
    impl ::core::convert::From<RewardPerTokenCall> for IGaugeCalls {
        fn from(value: RewardPerTokenCall) -> Self {
            Self::RewardPerToken(value)
        }
    }
    impl ::core::convert::From<RewardPerTokenStoredCall> for IGaugeCalls {
        fn from(value: RewardPerTokenStoredCall) -> Self {
            Self::RewardPerTokenStored(value)
        }
    }
    impl ::core::convert::From<RewardRateCall> for IGaugeCalls {
        fn from(value: RewardRateCall) -> Self {
            Self::RewardRate(value)
        }
    }
    impl ::core::convert::From<RewardRateByEpochCall> for IGaugeCalls {
        fn from(value: RewardRateByEpochCall) -> Self {
            Self::RewardRateByEpoch(value)
        }
    }
    impl ::core::convert::From<RewardTokenCall> for IGaugeCalls {
        fn from(value: RewardTokenCall) -> Self {
            Self::RewardToken(value)
        }
    }
    impl ::core::convert::From<RewardsCall> for IGaugeCalls {
        fn from(value: RewardsCall) -> Self {
            Self::Rewards(value)
        }
    }
    impl ::core::convert::From<StakingTokenCall> for IGaugeCalls {
        fn from(value: StakingTokenCall) -> Self {
            Self::StakingToken(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for IGaugeCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<UserRewardPerTokenPaidCall> for IGaugeCalls {
        fn from(value: UserRewardPerTokenPaidCall) -> Self {
            Self::UserRewardPerTokenPaid(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for IGaugeCalls {
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
    pub struct EarnedReturn {
        pub earned: ::ethers::core::types::U256,
    }
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
    pub struct LastTimeRewardApplicableReturn {
        pub time: ::ethers::core::types::U256,
    }
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
    pub struct LeftReturn {
        pub left: ::ethers::core::types::U256,
    }
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
    pub struct RewardPerTokenReturn {
        pub reward_per_token: ::ethers::core::types::U256,
    }
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
