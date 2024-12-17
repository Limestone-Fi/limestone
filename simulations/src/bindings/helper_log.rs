pub use helper_log::*;
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
pub mod helper_log {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                    ::std::borrow::ToOwned::to_owned("logFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logFail"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                            name: ::std::borrow::ToOwned::to_owned("logFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Log"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogBool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogBool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogBytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogBytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogInt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static HELPERLOG_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct HelperLog<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for HelperLog<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HelperLog<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HelperLog<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HelperLog<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(HelperLog)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> HelperLog<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    HELPERLOG_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `log` (0x23cdd8e8) function
        pub fn log_1(
            &self,
            message: ::std::string::String,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 205, 216, 232], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x319af333) function
        pub fn log_2(
            &self,
            message: ::std::string::String,
            data: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 154, 243, 51], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3692d61a) function
        pub fn log_3(
            &self,
            message: ::std::string::String,
            data: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 146, 214, 26], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3ca6268e) function
        pub fn log_4(
            &self,
            message: ::std::string::String,
            data: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 166, 38, 142], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x41304fac) function
        pub fn log_0(
            &self,
            message: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 48, 79, 172], message)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4b5c4277) function
        pub fn log_5(
            &self,
            message: ::std::string::String,
            data: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 92, 66, 119], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb60e72cc) function
        pub fn log_6(
            &self,
            message: ::std::string::String,
            data: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 14, 114, 204], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc3b55635) function
        pub fn log_7(
            &self,
            message: ::std::string::String,
            data: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 181, 86, 53], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logFail` (0x04fbe1c5) function
        pub fn log_fail_2(
            &self,
            message: ::std::string::String,
            data: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 251, 225, 197], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logFail` (0x1fc4755e) function
        pub fn log_fail_3(
            &self,
            message: ::std::string::String,
            data: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 196, 117, 94], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logFail` (0x213cde2c) function
        pub fn log_fail_4(
            &self,
            message: ::std::string::String,
            data: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 60, 222, 44], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logFail` (0x942732b2) function
        pub fn log_fail_5(
            &self,
            message: ::std::string::String,
            data: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 39, 50, 178], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logFail` (0xa98283e4) function
        pub fn log_fail_6(
            &self,
            message: ::std::string::String,
            data: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 130, 131, 228], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logFail` (0xaeef9c21) function
        pub fn log_fail_1(
            &self,
            message: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 239, 156, 33], message)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logFail` (0xde9b2232) function
        pub fn log_fail_0(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 155, 34, 50], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logFail` (0xf0f4b652) function
        pub fn log_fail_7(
            &self,
            message: ::std::string::String,
            data: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 244, 182, 82], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logFail` (0xf5c466f6) function
        pub fn log_fail_8(
            &self,
            message: ::std::string::String,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 196, 102, 246], (message, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_4_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed4Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_5_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed5Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_6_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed6Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_7_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed7Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_8_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed8Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Log` event
        pub fn log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogAddress` event
        pub fn log_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogBool` event
        pub fn log_bool_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogBoolFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogBytes` event
        pub fn log_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogBytes32` event
        pub fn log_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogInt` event
        pub fn log_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogIntFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogString` event
        pub fn log_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogUint` event
        pub fn log_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogUintFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            HelperLogEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for HelperLog<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed()")]
    pub struct AssertionFailed1Filter;
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string)")]
    pub struct AssertionFailed2Filter {
        pub message: ::std::string::String,
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string,string)")]
    pub struct AssertionFailed3Filter {
        pub message: ::std::string::String,
        pub data: ::std::string::String,
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string,bytes)")]
    pub struct AssertionFailed4Filter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string,uint256)")]
    pub struct AssertionFailed5Filter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::U256,
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string,int256)")]
    pub struct AssertionFailed6Filter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::I256,
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string,address)")]
    pub struct AssertionFailed7Filter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::Address,
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string,bool)")]
    pub struct AssertionFailed8Filter {
        pub message: ::std::string::String,
        pub data: bool,
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
    #[ethevent(name = "Log", abi = "Log(string)")]
    pub struct LogFilter {
        pub message: ::std::string::String,
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
    #[ethevent(name = "LogAddress", abi = "LogAddress(string,address)")]
    pub struct LogAddressFilter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::Address,
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
    #[ethevent(name = "LogBool", abi = "LogBool(string,bool)")]
    pub struct LogBoolFilter {
        pub message: ::std::string::String,
        pub data: bool,
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
    #[ethevent(name = "LogBytes", abi = "LogBytes(string,bytes)")]
    pub struct LogBytesFilter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "LogBytes32", abi = "LogBytes32(string,bytes32)")]
    pub struct LogBytes32Filter {
        pub message: ::std::string::String,
        pub data: [u8; 32],
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
    #[ethevent(name = "LogInt", abi = "LogInt(string,int256)")]
    pub struct LogIntFilter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::I256,
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
    #[ethevent(name = "LogString", abi = "LogString(string,string)")]
    pub struct LogStringFilter {
        pub message: ::std::string::String,
        pub data: ::std::string::String,
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
    #[ethevent(name = "LogUint", abi = "LogUint(string,uint256)")]
    pub struct LogUintFilter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::U256,
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
    pub enum HelperLogEvents {
        AssertionFailed1Filter(AssertionFailed1Filter),
        AssertionFailed2Filter(AssertionFailed2Filter),
        AssertionFailed3Filter(AssertionFailed3Filter),
        AssertionFailed4Filter(AssertionFailed4Filter),
        AssertionFailed5Filter(AssertionFailed5Filter),
        AssertionFailed6Filter(AssertionFailed6Filter),
        AssertionFailed7Filter(AssertionFailed7Filter),
        AssertionFailed8Filter(AssertionFailed8Filter),
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogBoolFilter(LogBoolFilter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
    }
    impl ::ethers::contract::EthLogDecode for HelperLogEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertionFailed1Filter::decode_log(log) {
                return Ok(HelperLogEvents::AssertionFailed1Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed2Filter::decode_log(log) {
                return Ok(HelperLogEvents::AssertionFailed2Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed3Filter::decode_log(log) {
                return Ok(HelperLogEvents::AssertionFailed3Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed4Filter::decode_log(log) {
                return Ok(HelperLogEvents::AssertionFailed4Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed5Filter::decode_log(log) {
                return Ok(HelperLogEvents::AssertionFailed5Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed6Filter::decode_log(log) {
                return Ok(HelperLogEvents::AssertionFailed6Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed7Filter::decode_log(log) {
                return Ok(HelperLogEvents::AssertionFailed7Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed8Filter::decode_log(log) {
                return Ok(HelperLogEvents::AssertionFailed8Filter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(HelperLogEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(HelperLogEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogBoolFilter::decode_log(log) {
                return Ok(HelperLogEvents::LogBoolFilter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(HelperLogEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(HelperLogEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(HelperLogEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(HelperLogEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(HelperLogEvents::LogUintFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for HelperLogEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssertionFailed1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed3Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed4Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed5Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed6Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed7Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed8Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBoolFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes32Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUintFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssertionFailed1Filter> for HelperLogEvents {
        fn from(value: AssertionFailed1Filter) -> Self {
            Self::AssertionFailed1Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed2Filter> for HelperLogEvents {
        fn from(value: AssertionFailed2Filter) -> Self {
            Self::AssertionFailed2Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed3Filter> for HelperLogEvents {
        fn from(value: AssertionFailed3Filter) -> Self {
            Self::AssertionFailed3Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed4Filter> for HelperLogEvents {
        fn from(value: AssertionFailed4Filter) -> Self {
            Self::AssertionFailed4Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed5Filter> for HelperLogEvents {
        fn from(value: AssertionFailed5Filter) -> Self {
            Self::AssertionFailed5Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed6Filter> for HelperLogEvents {
        fn from(value: AssertionFailed6Filter) -> Self {
            Self::AssertionFailed6Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed7Filter> for HelperLogEvents {
        fn from(value: AssertionFailed7Filter) -> Self {
            Self::AssertionFailed7Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed8Filter> for HelperLogEvents {
        fn from(value: AssertionFailed8Filter) -> Self {
            Self::AssertionFailed8Filter(value)
        }
    }
    impl ::core::convert::From<LogFilter> for HelperLogEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for HelperLogEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogBoolFilter> for HelperLogEvents {
        fn from(value: LogBoolFilter) -> Self {
            Self::LogBoolFilter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for HelperLogEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for HelperLogEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for HelperLogEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for HelperLogEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for HelperLogEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bytes)` and selector `0x23cdd8e8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bytes)")]
    pub struct Log1Call {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address)` and selector `0x319af333`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address)")]
    pub struct Log2Call {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bytes32)` and selector `0x3692d61a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bytes32)")]
    pub struct Log3Call {
        pub message: ::std::string::String,
        pub data: [u8; 32],
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,int256)` and selector `0x3ca6268e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,int256)")]
    pub struct Log4Call {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string)` and selector `0x41304fac`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string)")]
    pub struct Log0Call {
        pub message: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string)` and selector `0x4b5c4277`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string)")]
    pub struct Log5Call {
        pub message: ::std::string::String,
        pub data: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256)` and selector `0xb60e72cc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256)")]
    pub struct Log6Call {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool)` and selector `0xc3b55635`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool)")]
    pub struct Log7Call {
        pub message: ::std::string::String,
        pub data: bool,
    }
    ///Container type for all input parameters for the `logFail` function with signature `logFail(string,bool)` and selector `0x04fbe1c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logFail", abi = "logFail(string,bool)")]
    pub struct LogFail2Call {
        pub message: ::std::string::String,
        pub data: bool,
    }
    ///Container type for all input parameters for the `logFail` function with signature `logFail(string,bytes32)` and selector `0x1fc4755e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logFail", abi = "logFail(string,bytes32)")]
    pub struct LogFail3Call {
        pub message: ::std::string::String,
        pub data: [u8; 32],
    }
    ///Container type for all input parameters for the `logFail` function with signature `logFail(string,uint256)` and selector `0x213cde2c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logFail", abi = "logFail(string,uint256)")]
    pub struct LogFail4Call {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `logFail` function with signature `logFail(string,string)` and selector `0x942732b2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logFail", abi = "logFail(string,string)")]
    pub struct LogFail5Call {
        pub message: ::std::string::String,
        pub data: ::std::string::String,
    }
    ///Container type for all input parameters for the `logFail` function with signature `logFail(string,int256)` and selector `0xa98283e4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logFail", abi = "logFail(string,int256)")]
    pub struct LogFail6Call {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `logFail` function with signature `logFail(string)` and selector `0xaeef9c21`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logFail", abi = "logFail(string)")]
    pub struct LogFail1Call {
        pub message: ::std::string::String,
    }
    ///Container type for all input parameters for the `logFail` function with signature `logFail()` and selector `0xde9b2232`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logFail", abi = "logFail()")]
    pub struct LogFail0Call;
    ///Container type for all input parameters for the `logFail` function with signature `logFail(string,address)` and selector `0xf0f4b652`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logFail", abi = "logFail(string,address)")]
    pub struct LogFail7Call {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `logFail` function with signature `logFail(string,bytes)` and selector `0xf5c466f6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logFail", abi = "logFail(string,bytes)")]
    pub struct LogFail8Call {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum HelperLogCalls {
        Log1(Log1Call),
        Log2(Log2Call),
        Log3(Log3Call),
        Log4(Log4Call),
        Log0(Log0Call),
        Log5(Log5Call),
        Log6(Log6Call),
        Log7(Log7Call),
        LogFail2(LogFail2Call),
        LogFail3(LogFail3Call),
        LogFail4(LogFail4Call),
        LogFail5(LogFail5Call),
        LogFail6(LogFail6Call),
        LogFail1(LogFail1Call),
        LogFail0(LogFail0Call),
        LogFail7(LogFail7Call),
        LogFail8(LogFail8Call),
    }
    impl ::ethers::core::abi::AbiDecode for HelperLogCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Log1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log1(decoded));
            }
            if let Ok(decoded) = <Log2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log2(decoded));
            }
            if let Ok(decoded) = <Log3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log3(decoded));
            }
            if let Ok(decoded) = <Log4Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log4(decoded));
            }
            if let Ok(decoded) = <Log0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log0(decoded));
            }
            if let Ok(decoded) = <Log5Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log5(decoded));
            }
            if let Ok(decoded) = <Log6Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log6(decoded));
            }
            if let Ok(decoded) = <Log7Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log7(decoded));
            }
            if let Ok(decoded) = <LogFail2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogFail2(decoded));
            }
            if let Ok(decoded) = <LogFail3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogFail3(decoded));
            }
            if let Ok(decoded) = <LogFail4Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogFail4(decoded));
            }
            if let Ok(decoded) = <LogFail5Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogFail5(decoded));
            }
            if let Ok(decoded) = <LogFail6Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogFail6(decoded));
            }
            if let Ok(decoded) = <LogFail1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogFail1(decoded));
            }
            if let Ok(decoded) = <LogFail0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogFail0(decoded));
            }
            if let Ok(decoded) = <LogFail7Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogFail7(decoded));
            }
            if let Ok(decoded) = <LogFail8Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogFail8(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HelperLogCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Log1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Log2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Log3(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Log4(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Log0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Log5(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Log6(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Log7(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LogFail2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LogFail3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LogFail4(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LogFail5(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LogFail6(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LogFail1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LogFail0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LogFail7(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LogFail8(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for HelperLogCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Log1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log2(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log3(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log4(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log5(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log6(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log7(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogFail2(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogFail3(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogFail4(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogFail5(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogFail6(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogFail1(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogFail0(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogFail7(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogFail8(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Log1Call> for HelperLogCalls {
        fn from(value: Log1Call) -> Self {
            Self::Log1(value)
        }
    }
    impl ::core::convert::From<Log2Call> for HelperLogCalls {
        fn from(value: Log2Call) -> Self {
            Self::Log2(value)
        }
    }
    impl ::core::convert::From<Log3Call> for HelperLogCalls {
        fn from(value: Log3Call) -> Self {
            Self::Log3(value)
        }
    }
    impl ::core::convert::From<Log4Call> for HelperLogCalls {
        fn from(value: Log4Call) -> Self {
            Self::Log4(value)
        }
    }
    impl ::core::convert::From<Log0Call> for HelperLogCalls {
        fn from(value: Log0Call) -> Self {
            Self::Log0(value)
        }
    }
    impl ::core::convert::From<Log5Call> for HelperLogCalls {
        fn from(value: Log5Call) -> Self {
            Self::Log5(value)
        }
    }
    impl ::core::convert::From<Log6Call> for HelperLogCalls {
        fn from(value: Log6Call) -> Self {
            Self::Log6(value)
        }
    }
    impl ::core::convert::From<Log7Call> for HelperLogCalls {
        fn from(value: Log7Call) -> Self {
            Self::Log7(value)
        }
    }
    impl ::core::convert::From<LogFail2Call> for HelperLogCalls {
        fn from(value: LogFail2Call) -> Self {
            Self::LogFail2(value)
        }
    }
    impl ::core::convert::From<LogFail3Call> for HelperLogCalls {
        fn from(value: LogFail3Call) -> Self {
            Self::LogFail3(value)
        }
    }
    impl ::core::convert::From<LogFail4Call> for HelperLogCalls {
        fn from(value: LogFail4Call) -> Self {
            Self::LogFail4(value)
        }
    }
    impl ::core::convert::From<LogFail5Call> for HelperLogCalls {
        fn from(value: LogFail5Call) -> Self {
            Self::LogFail5(value)
        }
    }
    impl ::core::convert::From<LogFail6Call> for HelperLogCalls {
        fn from(value: LogFail6Call) -> Self {
            Self::LogFail6(value)
        }
    }
    impl ::core::convert::From<LogFail1Call> for HelperLogCalls {
        fn from(value: LogFail1Call) -> Self {
            Self::LogFail1(value)
        }
    }
    impl ::core::convert::From<LogFail0Call> for HelperLogCalls {
        fn from(value: LogFail0Call) -> Self {
            Self::LogFail0(value)
        }
    }
    impl ::core::convert::From<LogFail7Call> for HelperLogCalls {
        fn from(value: LogFail7Call) -> Self {
            Self::LogFail7(value)
        }
    }
    impl ::core::convert::From<LogFail8Call> for HelperLogCalls {
        fn from(value: LogFail8Call) -> Self {
            Self::LogFail8(value)
        }
    }
}
