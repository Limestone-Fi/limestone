pub use mock_factory_registry::*;
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
pub mod mock_factory_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_fallbackPoolFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_fallbackVotingRewardsFactory",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_fallbackGaugeFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_newManagedRewardsFactory",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "votingRewardsFactory",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gaugeFactory"),
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
                    ::std::borrow::ToOwned::to_owned("factoriesToPoolFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "factoriesToPoolFactory",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "votingRewardsFactory",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gaugeFactory"),
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
                    ::std::borrow::ToOwned::to_owned("fallbackPoolFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "fallbackPoolFactory",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("isPoolFactoryApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isPoolFactoryApproved",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolFactory"),
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
                    ::std::borrow::ToOwned::to_owned("managedRewardsFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "managedRewardsFactory",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("poolFactories"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("poolFactories"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("poolFactoriesLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "poolFactoriesLength",
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setManagedRewardsFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setManagedRewardsFactory",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_newManagedRewardsFactory",
                                    ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unapprove"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unapprove"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolFactory"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "votingRewardsFactory",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gaugeFactory"),
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
                    ::std::borrow::ToOwned::to_owned("SetManagedRewardsFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SetManagedRewardsFactory",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_newRewardsFactory",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unapprove"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unapprove"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "votingRewardsFactory",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gaugeFactory"),
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
                    ::std::borrow::ToOwned::to_owned("FallbackFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FallbackFactory"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFactoriesToPoolFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidFactoriesToPoolFactory",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableInvalidOwner",
                            ),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableUnauthorizedAccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("PathAlreadyApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PathAlreadyApproved",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PathNotApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PathNotApproved"),
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
    pub static MOCKFACTORYREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x0C\xC78\x03\x80a\x0C\xC7\x839\x81\x01`@\x81\x90Ra\0.\x91a\x03lV[3\x80a\0TW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\0]\x81a\0\x88V[P`\x01`\x01`\xA0\x1B\x03\x84\x16`\x80Ra\0v\x84\x84\x84a\0\xD7V[a\0\x7F\x81a\x02QV[PPPPa\x03\xBDV[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\0\xDFa\x02\xA2V[a\0\xEA`\x02\x84a\x02\xD0V[\x15a\x01\x08W`@Qcb\xCE\xE1u`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x84\x16\x80\x84R`\x01\x90\x91\x01T\x90\x93\x16\x90\x82\x01R\x90a\x01\xA5W`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x82R\x84\x81\x16` \x80\x84\x01\x91\x82R\x88\x83\x16_\x90\x81R`\x04\x90\x91R\x93\x90\x93 \x91Q\x82T\x90\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U\x92Q`\x01\x90\x92\x01\x80T\x92\x90\x91\x16\x91\x90\x92\x16\x17\x90Ua\x01\xF5V[\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14\x15\x80a\x01\xD7WP\x80` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x01\xF5W`@Qc\x03X\x041`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02\0`\x02\x85a\x02\xF6V[P\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x7FZ\xBEw\x02\xACH)\x9E\xF7dwU\xD7\xAFmjk\xEE\xCD\x1CXK\xBBo\xA5[z\x88$\x90\xEF\xC7`@Q`@Q\x80\x91\x03\x90\xA4PPPPV[a\x02Ya\x02\xA2V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F1#\xBA\xE5\x15*\x99\x9D\xEC\xBB[i0j\xDB0\xFA\x19\x88\\\xF9\x83\xC4\x94'\xFD[E\x94\xDC\xB07\x90_\x90\xA2PV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xCEW`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\0KV[V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15[\x90P[\x92\x91PPV[_a\x02\xED\x83`\x01`\x01`\xA0\x1B\x03\x84\x16_\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x03JWP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x02\xF0V[P_a\x02\xF0V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03gW__\xFD[\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a\x03\x7FW__\xFD[a\x03\x88\x85a\x03QV[\x93Pa\x03\x96` \x86\x01a\x03QV[\x92Pa\x03\xA4`@\x86\x01a\x03QV[\x91Pa\x03\xB2``\x86\x01a\x03QV[\x90P\x92\x95\x91\x94P\x92PV[`\x80Qa\x08\xEBa\x03\xDC_9_\x81\x81a\x01F\x01Ra\x04\xA2\x01Ra\x08\xEB_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xB1W_5`\xE0\x1C\x80cqP\x18\xA6\x11a\0nW\x80cqP\x18\xA6\x14a\x01}W\x80c\x8D\xA5\xCB[\x14a\x01\x85W\x80c\x9B\x14\n\x85\x14a\x01\x95W\x80c\xD1\xEA\n\x1D\x14a\x01\xA8W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xCBW\x80c\xFB\xF1\xF7\x8A\x14a\x01\xDEW__\xFD[\x80c\x06\x12\x1C\xD5\x14a\0\xB5W\x80c\x0C\xB2\x99\xC9\x14a\0\xD3W\x80c\r\n\xE6x\x14a\0\xE9W\x80c\x12\x17\xAF\xDB\x14a\x01\x0EW\x80cV\xD9\xCBd\x14a\x01AW\x80cd\x07i9\x14a\x01hW[__\xFD[a\0\xBDa\x01\xF1V[`@Qa\0\xCA\x91\x90a\x07\xD8V[`@Q\x80\x91\x03\x90\xF3[a\0\xDBa\x02\x02V[`@Q\x90\x81R` \x01a\0\xCAV[`\x01T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xCAV[a\x01!a\x01\x1C6`\x04a\x08>V[a\x02\rV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\0\xCAV[a\0\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01{a\x01v6`\x04a\x08>V[a\x02JV[\0[a\x01{a\x02\x9BV[_T`\x01`\x01`\xA0\x1B\x03\x16a\0\xF6V[a\x01{a\x01\xA36`\x04a\x08WV[a\x02\xAEV[a\x01\xBBa\x01\xB66`\x04a\x08>V[a\x045V[`@Q\x90\x15\x15\x81R` \x01a\0\xCAV[a\x01{a\x01\xD96`\x04a\x08>V[a\x04VV[a\x01{a\x01\xEC6`\x04a\x08>V[a\x04\x98V[``a\x01\xFD`\x02a\x05\x91V[\x90P\x90V[_a\x01\xFD`\x02a\x05\xA4V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x84\x16\x80\x84R`\x01\x90\x91\x01T\x90\x93\x16\x91\x01\x81\x90R\x90\x91V[a\x02Ra\x05\xADV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F1#\xBA\xE5\x15*\x99\x9D\xEC\xBB[i0j\xDB0\xFA\x19\x88\\\xF9\x83\xC4\x94'\xFD[E\x94\xDC\xB07\x90_\x90\xA2PV[a\x02\xA3a\x05\xADV[a\x02\xAC_a\x05\xD9V[V[a\x02\xB6a\x05\xADV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x03` R`@\x90 T\x15a\x02\xECW`@Qcb\xCE\xE1u`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x84\x16\x80\x84R`\x01\x90\x91\x01T\x90\x93\x16\x90\x82\x01R\x90a\x03\x89W`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x82R\x84\x81\x16` \x80\x84\x01\x91\x82R\x88\x83\x16_\x90\x81R`\x04\x90\x91R\x93\x90\x93 \x91Q\x82T\x90\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U\x92Q`\x01\x90\x92\x01\x80T\x92\x90\x91\x16\x91\x90\x92\x16\x17\x90Ua\x03\xD9V[\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14\x15\x80a\x03\xBBWP\x80` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x03\xD9W`@Qc\x03X\x041`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xE4`\x02\x85a\x06(V[P\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x7FZ\xBEw\x02\xACH)\x9E\xF7dwU\xD7\xAFmjk\xEE\xCD\x1CXK\xBBo\xA5[z\x88$\x90\xEF\xC7`@Q`@Q\x80\x91\x03\x90\xA4PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x81 T\x15\x15[\x92\x91PPV[a\x04^a\x05\xADV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04\x8CW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x04\x95\x81a\x05\xD9V[PV[a\x04\xA0a\x05\xADV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a\x04\xF2W`@Qc\n#Z\xDF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 Ta\x05'W`@Qc\xD3\x8A\xFDe`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x052`\x02\x82a\x06<V[P__a\x05>\x83a\x02\rV[\x91P\x91P\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xBB\xBF\x86\t\xBC\xCD$io}-\x865}\xBD\x1AU\xFF\x9By\x85:r\xEA\x11\xB1\xC0\x96\x8A\xDA\x17v`@Q`@Q\x80\x91\x03\x90\xA4PPPV[``_a\x05\x9D\x83a\x06PV[\x93\x92PPPV[_a\x04P\x82T\x90V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xACW`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x83V[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[_a\x05\x9D\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x06\xA9V[_a\x05\x9D\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x06\xF5V[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06\x9DW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x06\x89W[PPPPP\x90P\x91\x90PV[_\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x06\xEEWP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x04PV[P_a\x04PV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x07\xCFW_a\x07\x17`\x01\x83a\x08\x97V[\x85T\x90\x91P_\x90a\x07*\x90`\x01\x90a\x08\x97V[\x90P\x80\x82\x14a\x07\x89W_\x86_\x01\x82\x81T\x81\x10a\x07HWa\x07Ha\x08\xB6V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a\x07hWa\x07ha\x08\xB6V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x07\x9AWa\x07\x9Aa\x08\xCAV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x04PV[_\x91PPa\x04PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x08\x18W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x07\xF1V[P\x90\x95\x94PPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x089W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x08NW__\xFD[a\x05\x9D\x82a\x08#V[___``\x84\x86\x03\x12\x15a\x08iW__\xFD[a\x08r\x84a\x08#V[\x92Pa\x08\x80` \x85\x01a\x08#V[\x91Pa\x08\x8E`@\x85\x01a\x08#V[\x90P\x92P\x92P\x92V[\x81\x81\x03\x81\x81\x11\x15a\x04PWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static MOCKFACTORYREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xB1W_5`\xE0\x1C\x80cqP\x18\xA6\x11a\0nW\x80cqP\x18\xA6\x14a\x01}W\x80c\x8D\xA5\xCB[\x14a\x01\x85W\x80c\x9B\x14\n\x85\x14a\x01\x95W\x80c\xD1\xEA\n\x1D\x14a\x01\xA8W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xCBW\x80c\xFB\xF1\xF7\x8A\x14a\x01\xDEW__\xFD[\x80c\x06\x12\x1C\xD5\x14a\0\xB5W\x80c\x0C\xB2\x99\xC9\x14a\0\xD3W\x80c\r\n\xE6x\x14a\0\xE9W\x80c\x12\x17\xAF\xDB\x14a\x01\x0EW\x80cV\xD9\xCBd\x14a\x01AW\x80cd\x07i9\x14a\x01hW[__\xFD[a\0\xBDa\x01\xF1V[`@Qa\0\xCA\x91\x90a\x07\xD8V[`@Q\x80\x91\x03\x90\xF3[a\0\xDBa\x02\x02V[`@Q\x90\x81R` \x01a\0\xCAV[`\x01T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xCAV[a\x01!a\x01\x1C6`\x04a\x08>V[a\x02\rV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\0\xCAV[a\0\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01{a\x01v6`\x04a\x08>V[a\x02JV[\0[a\x01{a\x02\x9BV[_T`\x01`\x01`\xA0\x1B\x03\x16a\0\xF6V[a\x01{a\x01\xA36`\x04a\x08WV[a\x02\xAEV[a\x01\xBBa\x01\xB66`\x04a\x08>V[a\x045V[`@Q\x90\x15\x15\x81R` \x01a\0\xCAV[a\x01{a\x01\xD96`\x04a\x08>V[a\x04VV[a\x01{a\x01\xEC6`\x04a\x08>V[a\x04\x98V[``a\x01\xFD`\x02a\x05\x91V[\x90P\x90V[_a\x01\xFD`\x02a\x05\xA4V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x84\x16\x80\x84R`\x01\x90\x91\x01T\x90\x93\x16\x91\x01\x81\x90R\x90\x91V[a\x02Ra\x05\xADV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F1#\xBA\xE5\x15*\x99\x9D\xEC\xBB[i0j\xDB0\xFA\x19\x88\\\xF9\x83\xC4\x94'\xFD[E\x94\xDC\xB07\x90_\x90\xA2PV[a\x02\xA3a\x05\xADV[a\x02\xAC_a\x05\xD9V[V[a\x02\xB6a\x05\xADV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x03` R`@\x90 T\x15a\x02\xECW`@Qcb\xCE\xE1u`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x84\x16\x80\x84R`\x01\x90\x91\x01T\x90\x93\x16\x90\x82\x01R\x90a\x03\x89W`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x82R\x84\x81\x16` \x80\x84\x01\x91\x82R\x88\x83\x16_\x90\x81R`\x04\x90\x91R\x93\x90\x93 \x91Q\x82T\x90\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U\x92Q`\x01\x90\x92\x01\x80T\x92\x90\x91\x16\x91\x90\x92\x16\x17\x90Ua\x03\xD9V[\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14\x15\x80a\x03\xBBWP\x80` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x03\xD9W`@Qc\x03X\x041`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xE4`\x02\x85a\x06(V[P\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x7FZ\xBEw\x02\xACH)\x9E\xF7dwU\xD7\xAFmjk\xEE\xCD\x1CXK\xBBo\xA5[z\x88$\x90\xEF\xC7`@Q`@Q\x80\x91\x03\x90\xA4PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x81 T\x15\x15[\x92\x91PPV[a\x04^a\x05\xADV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04\x8CW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x04\x95\x81a\x05\xD9V[PV[a\x04\xA0a\x05\xADV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a\x04\xF2W`@Qc\n#Z\xDF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 Ta\x05'W`@Qc\xD3\x8A\xFDe`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x052`\x02\x82a\x06<V[P__a\x05>\x83a\x02\rV[\x91P\x91P\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xBB\xBF\x86\t\xBC\xCD$io}-\x865}\xBD\x1AU\xFF\x9By\x85:r\xEA\x11\xB1\xC0\x96\x8A\xDA\x17v`@Q`@Q\x80\x91\x03\x90\xA4PPPV[``_a\x05\x9D\x83a\x06PV[\x93\x92PPPV[_a\x04P\x82T\x90V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xACW`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x83V[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[_a\x05\x9D\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x06\xA9V[_a\x05\x9D\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x06\xF5V[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06\x9DW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x06\x89W[PPPPP\x90P\x91\x90PV[_\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x06\xEEWP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x04PV[P_a\x04PV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x07\xCFW_a\x07\x17`\x01\x83a\x08\x97V[\x85T\x90\x91P_\x90a\x07*\x90`\x01\x90a\x08\x97V[\x90P\x80\x82\x14a\x07\x89W_\x86_\x01\x82\x81T\x81\x10a\x07HWa\x07Ha\x08\xB6V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a\x07hWa\x07ha\x08\xB6V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x07\x9AWa\x07\x9Aa\x08\xCAV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x04PV[_\x91PPa\x04PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x08\x18W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x07\xF1V[P\x90\x95\x94PPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x089W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x08NW__\xFD[a\x05\x9D\x82a\x08#V[___``\x84\x86\x03\x12\x15a\x08iW__\xFD[a\x08r\x84a\x08#V[\x92Pa\x08\x80` \x85\x01a\x08#V[\x91Pa\x08\x8E`@\x85\x01a\x08#V[\x90P\x92P\x92P\x92V[\x81\x81\x03\x81\x81\x11\x15a\x04PWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static MOCKFACTORYREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockFactoryRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockFactoryRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockFactoryRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockFactoryRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockFactoryRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockFactoryRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockFactoryRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKFACTORYREGISTRY_ABI.clone(),
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
                MOCKFACTORYREGISTRY_ABI.clone(),
                MOCKFACTORYREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `approve` (0x9b140a85) function
        pub fn approve(
            &self,
            pool_factory: ::ethers::core::types::Address,
            voting_rewards_factory: ::ethers::core::types::Address,
            gauge_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [155, 20, 10, 133],
                    (pool_factory, voting_rewards_factory, gauge_factory),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factoriesToPoolFactory` (0x1217afdb) function
        pub fn factories_to_pool_factory(
            &self,
            pool_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::Address),
        > {
            self.0
                .method_hash([18, 23, 175, 219], pool_factory)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fallbackPoolFactory` (0x56d9cb64) function
        pub fn fallback_pool_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([86, 217, 203, 100], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPoolFactoryApproved` (0xd1ea0a1d) function
        pub fn is_pool_factory_approved(
            &self,
            pool_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([209, 234, 10, 29], pool_factory)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `managedRewardsFactory` (0x0d0ae678) function
        pub fn managed_rewards_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([13, 10, 230, 120], ())
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
        ///Calls the contract's `poolFactories` (0x06121cd5) function
        pub fn pool_factories(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([6, 18, 28, 213], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolFactoriesLength` (0x0cb299c9) function
        pub fn pool_factories_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([12, 178, 153, 201], ())
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
        ///Calls the contract's `setManagedRewardsFactory` (0x64076939) function
        pub fn set_managed_rewards_factory(
            &self,
            new_managed_rewards_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 7, 105, 57], new_managed_rewards_factory)
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
        ///Calls the contract's `unapprove` (0xfbf1f78a) function
        pub fn unapprove(
            &self,
            pool_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 241, 247, 138], pool_factory)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approve` event
        pub fn approve_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApproveFilter> {
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
        ///Gets the contract's `SetManagedRewardsFactory` event
        pub fn set_managed_rewards_factory_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetManagedRewardsFactoryFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unapprove` event
        pub fn unapprove_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnapproveFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MockFactoryRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockFactoryRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `FallbackFactory` with signature `FallbackFactory()` and selector `0x1446b5be`
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
    #[etherror(name = "FallbackFactory", abi = "FallbackFactory()")]
    pub struct FallbackFactory;
    ///Custom Error type `InvalidFactoriesToPoolFactory` with signature `InvalidFactoriesToPoolFactory()` and selector `0x03580431`
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
        name = "InvalidFactoriesToPoolFactory",
        abi = "InvalidFactoriesToPoolFactory()"
    )]
    pub struct InvalidFactoriesToPoolFactory;
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
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
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
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
        name = "OwnableUnauthorizedAccount",
        abi = "OwnableUnauthorizedAccount(address)"
    )]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `PathAlreadyApproved` with signature `PathAlreadyApproved()` and selector `0xc59dc2ea`
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
    #[etherror(name = "PathAlreadyApproved", abi = "PathAlreadyApproved()")]
    pub struct PathAlreadyApproved;
    ///Custom Error type `PathNotApproved` with signature `PathNotApproved()` and selector `0xd38afd65`
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
    #[etherror(name = "PathNotApproved", abi = "PathNotApproved()")]
    pub struct PathNotApproved;
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
    pub enum MockFactoryRegistryErrors {
        FallbackFactory(FallbackFactory),
        InvalidFactoriesToPoolFactory(InvalidFactoriesToPoolFactory),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        PathAlreadyApproved(PathAlreadyApproved),
        PathNotApproved(PathNotApproved),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockFactoryRegistryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <FallbackFactory as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FallbackFactory(decoded));
            }
            if let Ok(decoded) = <InvalidFactoriesToPoolFactory as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidFactoriesToPoolFactory(decoded));
            }
            if let Ok(decoded) = <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) = <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) = <PathAlreadyApproved as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PathAlreadyApproved(decoded));
            }
            if let Ok(decoded) = <PathNotApproved as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PathNotApproved(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockFactoryRegistryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::FallbackFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFactoriesToPoolFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PathAlreadyApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PathNotApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockFactoryRegistryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <FallbackFactory as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFactoriesToPoolFactory as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PathAlreadyApproved as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PathNotApproved as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockFactoryRegistryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FallbackFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFactoriesToPoolFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PathAlreadyApproved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PathNotApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockFactoryRegistryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<FallbackFactory> for MockFactoryRegistryErrors {
        fn from(value: FallbackFactory) -> Self {
            Self::FallbackFactory(value)
        }
    }
    impl ::core::convert::From<InvalidFactoriesToPoolFactory>
    for MockFactoryRegistryErrors {
        fn from(value: InvalidFactoriesToPoolFactory) -> Self {
            Self::InvalidFactoriesToPoolFactory(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for MockFactoryRegistryErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount>
    for MockFactoryRegistryErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<PathAlreadyApproved> for MockFactoryRegistryErrors {
        fn from(value: PathAlreadyApproved) -> Self {
            Self::PathAlreadyApproved(value)
        }
    }
    impl ::core::convert::From<PathNotApproved> for MockFactoryRegistryErrors {
        fn from(value: PathNotApproved) -> Self {
            Self::PathNotApproved(value)
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
    #[ethevent(name = "Approve", abi = "Approve(address,address,address)")]
    pub struct ApproveFilter {
        #[ethevent(indexed)]
        pub pool_factory: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub voting_rewards_factory: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge_factory: ::ethers::core::types::Address,
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
        name = "SetManagedRewardsFactory",
        abi = "SetManagedRewardsFactory(address)"
    )]
    pub struct SetManagedRewardsFactoryFilter {
        #[ethevent(indexed)]
        pub new_rewards_factory: ::ethers::core::types::Address,
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
    #[ethevent(name = "Unapprove", abi = "Unapprove(address,address,address)")]
    pub struct UnapproveFilter {
        #[ethevent(indexed)]
        pub pool_factory: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub voting_rewards_factory: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge_factory: ::ethers::core::types::Address,
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
    pub enum MockFactoryRegistryEvents {
        ApproveFilter(ApproveFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        SetManagedRewardsFactoryFilter(SetManagedRewardsFactoryFilter),
        UnapproveFilter(UnapproveFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockFactoryRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApproveFilter::decode_log(log) {
                return Ok(MockFactoryRegistryEvents::ApproveFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    MockFactoryRegistryEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = SetManagedRewardsFactoryFilter::decode_log(log) {
                return Ok(
                    MockFactoryRegistryEvents::SetManagedRewardsFactoryFilter(decoded),
                );
            }
            if let Ok(decoded) = UnapproveFilter::decode_log(log) {
                return Ok(MockFactoryRegistryEvents::UnapproveFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockFactoryRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApproveFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetManagedRewardsFactoryFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnapproveFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveFilter> for MockFactoryRegistryEvents {
        fn from(value: ApproveFilter) -> Self {
            Self::ApproveFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for MockFactoryRegistryEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<SetManagedRewardsFactoryFilter>
    for MockFactoryRegistryEvents {
        fn from(value: SetManagedRewardsFactoryFilter) -> Self {
            Self::SetManagedRewardsFactoryFilter(value)
        }
    }
    impl ::core::convert::From<UnapproveFilter> for MockFactoryRegistryEvents {
        fn from(value: UnapproveFilter) -> Self {
            Self::UnapproveFilter(value)
        }
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,address,address)` and selector `0x9b140a85`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,address,address)")]
    pub struct ApproveCall {
        pub pool_factory: ::ethers::core::types::Address,
        pub voting_rewards_factory: ::ethers::core::types::Address,
        pub gauge_factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `factoriesToPoolFactory` function with signature `factoriesToPoolFactory(address)` and selector `0x1217afdb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "factoriesToPoolFactory", abi = "factoriesToPoolFactory(address)")]
    pub struct FactoriesToPoolFactoryCall {
        pub pool_factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `fallbackPoolFactory` function with signature `fallbackPoolFactory()` and selector `0x56d9cb64`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "fallbackPoolFactory", abi = "fallbackPoolFactory()")]
    pub struct FallbackPoolFactoryCall;
    ///Container type for all input parameters for the `isPoolFactoryApproved` function with signature `isPoolFactoryApproved(address)` and selector `0xd1ea0a1d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isPoolFactoryApproved", abi = "isPoolFactoryApproved(address)")]
    pub struct IsPoolFactoryApprovedCall {
        pub pool_factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `managedRewardsFactory` function with signature `managedRewardsFactory()` and selector `0x0d0ae678`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "managedRewardsFactory", abi = "managedRewardsFactory()")]
    pub struct ManagedRewardsFactoryCall;
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
    ///Container type for all input parameters for the `poolFactories` function with signature `poolFactories()` and selector `0x06121cd5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "poolFactories", abi = "poolFactories()")]
    pub struct PoolFactoriesCall;
    ///Container type for all input parameters for the `poolFactoriesLength` function with signature `poolFactoriesLength()` and selector `0x0cb299c9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "poolFactoriesLength", abi = "poolFactoriesLength()")]
    pub struct PoolFactoriesLengthCall;
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
    ///Container type for all input parameters for the `setManagedRewardsFactory` function with signature `setManagedRewardsFactory(address)` and selector `0x64076939`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "setManagedRewardsFactory",
        abi = "setManagedRewardsFactory(address)"
    )]
    pub struct SetManagedRewardsFactoryCall {
        pub new_managed_rewards_factory: ::ethers::core::types::Address,
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
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unapprove` function with signature `unapprove(address)` and selector `0xfbf1f78a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unapprove", abi = "unapprove(address)")]
    pub struct UnapproveCall {
        pub pool_factory: ::ethers::core::types::Address,
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
    pub enum MockFactoryRegistryCalls {
        Approve(ApproveCall),
        FactoriesToPoolFactory(FactoriesToPoolFactoryCall),
        FallbackPoolFactory(FallbackPoolFactoryCall),
        IsPoolFactoryApproved(IsPoolFactoryApprovedCall),
        ManagedRewardsFactory(ManagedRewardsFactoryCall),
        Owner(OwnerCall),
        PoolFactories(PoolFactoriesCall),
        PoolFactoriesLength(PoolFactoriesLengthCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetManagedRewardsFactory(SetManagedRewardsFactoryCall),
        TransferOwnership(TransferOwnershipCall),
        Unapprove(UnapproveCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockFactoryRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <FactoriesToPoolFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FactoriesToPoolFactory(decoded));
            }
            if let Ok(decoded) = <FallbackPoolFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FallbackPoolFactory(decoded));
            }
            if let Ok(decoded) = <IsPoolFactoryApprovedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsPoolFactoryApproved(decoded));
            }
            if let Ok(decoded) = <ManagedRewardsFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ManagedRewardsFactory(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PoolFactoriesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolFactories(decoded));
            }
            if let Ok(decoded) = <PoolFactoriesLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolFactoriesLength(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetManagedRewardsFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetManagedRewardsFactory(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnapproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unapprove(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockFactoryRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FactoriesToPoolFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FallbackPoolFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPoolFactoryApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ManagedRewardsFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PoolFactories(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolFactoriesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetManagedRewardsFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unapprove(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockFactoryRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::FactoriesToPoolFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FallbackPoolFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsPoolFactoryApproved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ManagedRewardsFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolFactories(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolFactoriesLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetManagedRewardsFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unapprove(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveCall> for MockFactoryRegistryCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<FactoriesToPoolFactoryCall> for MockFactoryRegistryCalls {
        fn from(value: FactoriesToPoolFactoryCall) -> Self {
            Self::FactoriesToPoolFactory(value)
        }
    }
    impl ::core::convert::From<FallbackPoolFactoryCall> for MockFactoryRegistryCalls {
        fn from(value: FallbackPoolFactoryCall) -> Self {
            Self::FallbackPoolFactory(value)
        }
    }
    impl ::core::convert::From<IsPoolFactoryApprovedCall> for MockFactoryRegistryCalls {
        fn from(value: IsPoolFactoryApprovedCall) -> Self {
            Self::IsPoolFactoryApproved(value)
        }
    }
    impl ::core::convert::From<ManagedRewardsFactoryCall> for MockFactoryRegistryCalls {
        fn from(value: ManagedRewardsFactoryCall) -> Self {
            Self::ManagedRewardsFactory(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MockFactoryRegistryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PoolFactoriesCall> for MockFactoryRegistryCalls {
        fn from(value: PoolFactoriesCall) -> Self {
            Self::PoolFactories(value)
        }
    }
    impl ::core::convert::From<PoolFactoriesLengthCall> for MockFactoryRegistryCalls {
        fn from(value: PoolFactoriesLengthCall) -> Self {
            Self::PoolFactoriesLength(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for MockFactoryRegistryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetManagedRewardsFactoryCall>
    for MockFactoryRegistryCalls {
        fn from(value: SetManagedRewardsFactoryCall) -> Self {
            Self::SetManagedRewardsFactory(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MockFactoryRegistryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnapproveCall> for MockFactoryRegistryCalls {
        fn from(value: UnapproveCall) -> Self {
            Self::Unapprove(value)
        }
    }
    ///Container type for all return fields from the `factoriesToPoolFactory` function with signature `factoriesToPoolFactory(address)` and selector `0x1217afdb`
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
    pub struct FactoriesToPoolFactoryReturn {
        pub voting_rewards_factory: ::ethers::core::types::Address,
        pub gauge_factory: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `fallbackPoolFactory` function with signature `fallbackPoolFactory()` and selector `0x56d9cb64`
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
    pub struct FallbackPoolFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isPoolFactoryApproved` function with signature `isPoolFactoryApproved(address)` and selector `0xd1ea0a1d`
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
    pub struct IsPoolFactoryApprovedReturn(pub bool);
    ///Container type for all return fields from the `managedRewardsFactory` function with signature `managedRewardsFactory()` and selector `0x0d0ae678`
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
    pub struct ManagedRewardsFactoryReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `poolFactories` function with signature `poolFactories()` and selector `0x06121cd5`
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
    pub struct PoolFactoriesReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `poolFactoriesLength` function with signature `poolFactoriesLength()` and selector `0x0cb299c9`
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
    pub struct PoolFactoriesLengthReturn(pub ::ethers::core::types::U256);
}
