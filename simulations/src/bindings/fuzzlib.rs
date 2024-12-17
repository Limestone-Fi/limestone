pub use fuzzlib::*;
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
pub mod fuzzlib {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("abs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("abs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("n"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("abs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("n"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assertRevertReasonEqual"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "assertRevertReasonEqual",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                            name: ::std::borrow::ToOwned::to_owned(
                                "assertRevertReasonEqual",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason3"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason4"),
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
                            name: ::std::borrow::ToOwned::to_owned(
                                "assertRevertReasonEqual",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason3"),
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
                            name: ::std::borrow::ToOwned::to_owned(
                                "assertRevertReasonEqual",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason2"),
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
                    ::std::borrow::ToOwned::to_owned("assertRevertReasonNotEqual"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "assertRevertReasonNotEqual",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                    ::std::borrow::ToOwned::to_owned("clamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clamp"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("low"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("high"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enableLogs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clamp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("low"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("high"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enableLogs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clamp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("low"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("high"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clamp"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("low"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("high"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clampGt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampGt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enableLogs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampGt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enableLogs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampGt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampGt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clampGte"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampGte"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enableLogs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampGte"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enableLogs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampGte"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampGte"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clampLt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampLt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enableLogs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampLt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enableLogs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampLt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampLt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clampLte"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampLte"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enableLogs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampLte"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampLte"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clampLte"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enableLogs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("diff"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("diff"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("diff"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("eq"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eq"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                            name: ::std::borrow::ToOwned::to_owned("eq"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                            name: ::std::borrow::ToOwned::to_owned("eq"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                            name: ::std::borrow::ToOwned::to_owned("eq"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                            name: ::std::borrow::ToOwned::to_owned("eq"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                    ::std::borrow::ToOwned::to_owned("errAllow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("errAllow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("errorSelector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allowedErrors"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4[]"),
                                    ),
                                },
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
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("errsAllow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("errsAllow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("errorSelector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allowedErrors"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("messages"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
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
                    ::std::borrow::ToOwned::to_owned("gt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                            name: ::std::borrow::ToOwned::to_owned("gt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                    ::std::borrow::ToOwned::to_owned("gte"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gte"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                            name: ::std::borrow::ToOwned::to_owned("gte"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                (
                    ::std::borrow::ToOwned::to_owned("lt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                            name: ::std::borrow::ToOwned::to_owned("lt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                    ::std::borrow::ToOwned::to_owned("lte"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lte"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                            name: ::std::borrow::ToOwned::to_owned("lte"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                    ::std::borrow::ToOwned::to_owned("max"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("max"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("max"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("min"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("min"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("neq"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("neq"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                            name: ::std::borrow::ToOwned::to_owned("neq"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                    ::std::borrow::ToOwned::to_owned("platform"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("platform"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPlatform"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPlatform"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPlatform"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_platform"),
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
                    ::std::borrow::ToOwned::to_owned("shuffleArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("shuffleArray"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shuffle"),
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
                                    name: ::std::borrow::ToOwned::to_owned("entropy"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("t"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("t"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AssertEqFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertEqFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssertFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssertGtFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertGtFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssertGteFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertGteFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssertLtFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertLtFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssertLteFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertLteFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssertNeqFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertNeqFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("Clamped"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Clamped"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
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
    pub static FUZZLIB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa0\x93\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x04\x01W_5`\xE0\x1C\x80c~\x1D\xDD\x80\x11a\x02\x16W\x80c\xB6\x0Er\xCC\x11a\x01*W\x80c\xE0\x1E\x92\xD2\x11a\0\xB4W\x80c\xF0\x9C\xED\x97\x11a\0\x84W\x80c\xF0\x9C\xED\x97\x14a\t\x15W\x80c\xF0\xF4\xB6R\x14a\t(W\x80c\xF5\xBC:q\x14a\t;W\x80c\xF5\xC4f\xF6\x14a\tNW\x80c\xFB\x98\x94\xB2\x14a\taW__\xFD[\x80c\xE0\x1E\x92\xD2\x14a\x08\xB6W\x80c\xE8\x94\t\r\x14a\x08\xDCW\x80c\xEE\xF2S\x89\x14a\x08\xEFW\x80c\xF0\x1F\xF6L\x14a\t\x02W__\xFD[\x80c\xC3\xB5V5\x11a\0\xFAW\x80c\xC3\xB5V5\x14a\x08bW\x80c\xC5\xDF*7\x14a\x08uW\x80c\xD2\xA5\x06\x04\x14a\x08\x88W\x80c\xDE\x9B\"2\x14a\x08\x9BW\x80c\xDF|\xD7\x7F\x14a\x08\xA3W__\xFD[\x80c\xB6\x0Er\xCC\x14a\x08\x16W\x80c\xBB5\x03\x1A\x14a\x08)W\x80c\xBC\x8DC\xA8\x14a\x08<W\x80c\xC0\xE8\xDE\xF2\x14a\x08OW__\xFD[\x80c\x94'2\xB2\x11a\x01\xABW\x80c\xA9\x82\x83\xE4\x11a\x01{W\x80c\xA9\x82\x83\xE4\x14a\x07\xB7W\x80c\xAC\x0C {\x14a\x07\xCAW\x80c\xAE\xEF\x9C!\x14a\x07\xDDW\x80c\xB0\x9CA\xB7\x14a\x07\xF0W\x80c\xB2\xF9y\x9D\x14a\x08\x03W__\xFD[\x80c\x94'2\xB2\x14a\x07kW\x80c\x95>^\x80\x14a\x07~W\x80c\x9EB$G\x14a\x07\x91W\x80c\xA1_\x9F\x07\x14a\x07\xA4W__\xFD[\x80c\x84b\x13\xA1\x11a\x01\xE6W\x80c\x84b\x13\xA1\x14a\x07\x1FW\x80c\x88\xE3b\xC6\x14a\x072W\x80c\x8CO\xB5}\x14a\x07EW\x80c\x8F^QJ\x14a\x07XW__\xFD[\x80c~\x1D\xDD\x80\x14a\x06\xD3W\x80c\x80\x10\xBA\xD3\x14a\x06\xE6W\x80c\x81\xFEW\x86\x14a\x06\xF9W\x80c\x84\x1E\xA1\x1C\x14a\x07\x0CW__\xFD[\x80c1\x9A\xF33\x11a\x03\x18W\x80cV\x8C\x13\x85\x11a\x02\xA2W\x80cl\x97d\xB3\x11a\x02rW\x80cl\x97d\xB3\x14a\x06tW\x80cmT3\xE6\x14a\x06\x87W\x80cr\xDEw\xB5\x14a\x06\x9AW\x80cz\xE2\xB5\xC7\x14a\x06\xADW\x80c{\x8D\x0F\x0C\x14a\x06\xC0W__\xFD[\x80cV\x8C\x13\x85\x14a\x06\x0CW\x80c\\+\x80\xF5\x14a\x06\x1FW\x80ciE\xC5\xEA\x14a\x062W\x80cj\xCEU,\x14a\x06aW__\xFD[\x80c<\xCB^&\x11a\x02\xE8W\x80c<\xCB^&\x14a\x05\x96W\x80cA0O\xAC\x14a\x05\xA9W\x80cB\x06\x87g\x14a\x05\xBCW\x80cK\\Bw\x14a\x05\xCFW\x80cK\xDE8\xC8\x14a\x05\xE2W__\xFD[\x80c1\x9A\xF33\x14a\x05]W\x80c6\x92\xD6\x1A\x14a\x04\xEBW\x80c;m\xDF\x03\x14a\x05pW\x80c<\xA6&\x8E\x14a\x05\x83W__\xFD[\x80c\x1BU\xD0|\x11a\x03\x99W\x80c!<\xDE,\x11a\x03iW\x80c!<\xDE,\x14a\x04\xFEW\x80c#\xCD\xD8\xE8\x14a\x05\x11W\x80c-@\x85\x1E\x14a\x05$W\x80c.\x17\xD4r\x14a\x057W\x80c/\xFD\xE6\x03\x14a\x05JW__\xFD[\x80c\x1BU\xD0|\x14a\x04\xB2W\x80c\x1BZ\xC4\xB5\x14a\x04\xC5W\x80c\x1B\xA09\x9B\x14a\x04\xD8W\x80c\x1F\xC4u^\x14a\x04\xEBW__\xFD[\x80c\x08N\xC4\x10\x11a\x03\xD4W\x80c\x08N\xC4\x10\x14a\x04fW\x80c\t\xFD8\x99\x14a\x04yW\x80c\x10Y\xBF\xC9\x14a\x04\x8CW\x80c\x14\xEC\x1C\xC6\x14a\x04\x9FW__\xFD[\x80c\x01\xB9\xE8'\x14a\x04\x05W\x80c\x04\xFB\xE1\xC5\x14a\x04\x1AW\x80c\x06\xCB\0\x11\x14a\x04-W\x80c\x06\xF8#B\x14a\x04SW[__\xFD[a\x04\x18a\x04\x136`\x04a\"IV[a\ttV[\0[a\x04\x18a\x04(6`\x04a\"\xA3V[a\nPV[a\x04@a\x04;6`\x04a\"\xEDV[a\n^V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x04\x18a\x04a6`\x04a\"IV[a\x0B-V[a\x04@a\x04t6`\x04a#)V[a\x0BJV[a\x04@a\x04\x876`\x04a\"\xEDV[a\x0B\xE3V[a\x04\x18a\x04\x9A6`\x04a#[V[a\x0C\xD0V[a\x04\x18a\x04\xAD6`\x04a\"IV[a\x0C\xE8V[a\x04@a\x04\xC06`\x04a#)V[a\r_V[a\x04@a\x04\xD36`\x04a#\xBEV[a\r\xBCV[a\x04\x18a\x04\xE66`\x04a\"IV[a\r\xDBV[a\x04\x18a\x04\xF96`\x04a#\xD5V[a\x0ERV[a\x04\x18a\x05\x0C6`\x04a#\xD5V[a\x0E\\V[a\x04\x18a\x05\x1F6`\x04a#[V[a\x0EfV[a\x04@a\x0526`\x04a#)V[a\x0EpV[a\x04@a\x05E6`\x04a#)V[a\x0E\x89V[a\x04@a\x05X6`\x04a#)V[a\x0E\xA2V[a\x04\x18a\x05k6`\x04a$,V[a\x0E\xC8V[a\x04\x18a\x05~6`\x04a\"IV[a\x0E\xD2V[a\x04\x18a\x05\x916`\x04a#\xD5V[a\x0FIV[a\x04\x18a\x05\xA46`\x04a$mV[a\x0FSV[a\x04\x18a\x05\xB76`\x04a$\xB0V[a\x0F\x82V[a\x04\x18a\x05\xCA6`\x04a#[V[a\x0F\x8EV[a\x04\x18a\x05\xDD6`\x04a#[V[a\x0F\xA5V[_Ta\x05\xF4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04JV[a\x04@a\x06\x1A6`\x04a$\xE1V[a\x0F\xAFV[a\x04\x18a\x06-6`\x04a\"IV[a\x0F\xD1V[a\x04\x18a\x06@6`\x04a%\x01V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x04\x18a\x06o6`\x04a%<V[a\x10HV[a\x04\x18a\x06\x826`\x04a%\xEBV[a\x11\x08V[a\x04@a\x06\x956`\x04a$\xE1V[a\x121V[a\x04\x18a\x06\xA86`\x04a&\x14V[a\x12FV[a\x04@a\x06\xBB6`\x04a$\xE1V[a\x12\xBFV[a\x04@a\x06\xCE6`\x04a&\xF2V[a\x12\xCDV[a\x04@a\x06\xE16`\x04a$\xE1V[a\x12\xDBV[a\x04\x18a\x06\xF46`\x04a'\x1BV[a\x12\xE8V[a\x04@a\x07\x076`\x04a$\xE1V[a\x13NV[a\x04\x18a\x07\x1A6`\x04a\"IV[a\x13\\V[a\x04\x18a\x07-6`\x04a'\xD0V[a\x13zV[a\x04\x18a\x07@6`\x04a(\xB1V[a\x13\xCDV[a\x04\x18a\x07S6`\x04a\"IV[a\x14/V[a\x04@a\x07f6`\x04a$\xE1V[a\x14\xA6V[a\x04\x18a\x07y6`\x04a#[V[a\x14\xC3V[a\x04@a\x07\x8C6`\x04a#)V[a\x14\xCDV[a\x04@a\x07\x9F6`\x04a&\xF2V[a\x14\xE1V[a\x04\x18a\x07\xB26`\x04a(\xF2V[a\x14\xEFV[a\x04\x18a\x07\xC56`\x04a#\xD5V[a\x15{V[a\x04@a\x07\xD86`\x04a$\xE1V[a\x15\x85V[a\x04\x18a\x07\xEB6`\x04a$\xB0V[a\x15\x92V[a\x04\x18a\x07\xFE6`\x04a)&V[a\x15\x9BV[a\x04@a\x08\x116`\x04a#)V[a\x16\x14V[a\x04\x18a\x08$6`\x04a#\xD5V[a\x16JV[a\x04\x18a\x0876`\x04a\"IV[a\x16TV[a\x04\x18a\x08J6`\x04a\"IV[a\x16rV[a\x04\x18a\x08]6`\x04a\"IV[a\x16\x8FV[a\x04\x18a\x08p6`\x04a\"\xA3V[a\x16\xACV[a\x04@a\x08\x836`\x04a$\xE1V[a\x16\xB6V[a\x04\x18a\x08\x966`\x04a\"IV[a\x16\xC3V[a\x04\x18a\x16\xE0V[a\x04\x18a\x08\xB16`\x04a*\x10V[a\x16\xEAV[a\x08\xC9a\x08\xC46`\x04a*9V[a\x17\xADV[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x04JV[a\x04@a\x08\xEA6`\x04a$\xE1V[a\x17\xC2V[a\x04@a\x08\xFD6`\x04a#)V[a\x17\xCFV[a\x04@a\t\x106`\x04a$\xE1V[a\x17\xEEV[a\x04@a\t#6`\x04a$\xE1V[a\x17\xFBV[a\x04\x18a\t66`\x04a$,V[a\x18\x08V[a\x04@a\tI6`\x04a$\xE1V[a\x18\x12V[a\x04\x18a\t\\6`\x04a#[V[a\x18\x1DV[a\x04@a\to6`\x04a$\xE1V[a\x18'V[\x81\x83\x10a\nKW_a\t\x85\x84a\x184V[\x90P_a\t\x91\x84a\x184V[\x90P_a\t\xBA\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a>=`\xF0\x1B\x81RP\x87a\x18wV[\x90P\x7FF\xD0\x1D\xFB\xC4\x86\x1B[\x85\x9D\x85\x8DT\x1E\x8B\0F\x17Z\x8D\xEC\xE7\x0E$\xF8\x92\xA31\x8Fp\xE2\xBB\x81`@Qa\t\xEB\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1_\x80T`@\x80Qc\t\xCC\xAE\xFB`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c\t\xCC\xAE\xFB\x92`\x04\x80\x84\x01\x93\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\n1W__\xFD[PZ\xFA\x15\x80\x15a\nCW=__>=_\xFD[PPPPPPP[PPPV[a\nZ\x82\x82a\x18\xA9V[PPV[_\x83\x85\x10\x80a\nlWP\x82\x85\x11[\x15a\x0B\"W_a\n|\x85\x85a*\xADV[a\n\x87\x90`\x01a*\xC0V[a\n\x91\x90\x87a*\xE7V[a\n\x9B\x90\x86a*\xC0V[\x90P\x82\x15a\x0B\x1BW_a\n\xAD\x87a\x184V[\x90P_a\n\xB9\x83a\x184V[\x90P_\x82\x82`@Q` \x01a\n\xCF\x92\x91\x90a+\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x7FL\xB9\xC14Q\x97P\xE0\x94P\xA0\xC3\x8D\x14o\xED.\xCE\xD0-\xD6Q\xEF2\xC9\xE3H\x8F\xAAU\xC6\xF6\x81`@Qa\x0B\x0F\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1PPP[\x90Pa\x0B%V[P\x83[\x94\x93PPPPV[\x81\x83\x12a\nKW_a\x0B>\x84a\x18\xE6V[\x90P_a\t\x91\x84a\x18\xE6V[_\x82\x84\x13\x15a\x0B\xD9W\x82\x82\x15a\x0B\xD2W_a\x0Bd\x86a\x18\xE6V[\x90P_a\x0Bp\x83a\x18\xE6V[\x90P_\x82\x82`@Q` \x01a\x0B\x86\x92\x91\x90a+\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x7FL\xB9\xC14Q\x97P\xE0\x94P\xA0\xC3\x8D\x14o\xED.\xCE\xD0-\xD6Q\xEF2\xC9\xE3H\x8F\xAAU\xC6\xF6\x81`@Qa\x0B\xC6\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1PPP[\x90Pa\x0B\xDCV[P\x82[\x93\x92PPPV[_\x83\x85\x12\x80a\x0B\xF1WP\x82\x85\x13[\x15a\x0B\"W_a\x0C\x01\x85\x85a+TV[a\x0C\x0C\x90`\x01a+sV[\x90P_\x81a\x0C\x1A\x87\x89a+TV[a\x0C$\x91\x90a+\x9AV[\x90P_\x81\x12\x15a\x0C;Wa\x0C8\x82\x82a+sV[\x90P[_a\x0CF\x82\x88a+sV[\x90P\x84\x15a\x0C\xC6W_a\x0CX\x89a\x18\xE6V[\x90P_a\x0Cd\x83a\x18\xE6V[\x90P_\x82\x82`@Q` \x01a\x0Cz\x92\x91\x90a+\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x7FL\xB9\xC14Q\x97P\xE0\x94P\xA0\xC3\x8D\x14o\xED.\xCE\xD0-\xD6Q\xEF2\xC9\xE3H\x8F\xAAU\xC6\xF6\x81`@Qa\x0C\xBA\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1PPP[\x92Pa\x0B%\x91PPV[_a\x0C\xDB\x83\x83a\x19?V[\x90Pa\nK\x81\x15\x83a\x14\xEFV[\x81\x83\x14a\nKW_a\x0C\xF9\x84a\x18\xE6V[\x90P_a\r\x05\x84a\x18\xE6V[\x90P_a\r.\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a!=`\xF0\x1B\x81RP\x87a\x18wV[\x90P\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x81`@Qa\t\xEB\x91\x90a*\x87V[_\x82\x84\x11a\r\xB5Wa\r\x8B\x83_\x19`@Q\x80`\x80\x01`@R\x80`Z\x81R` \x01a0-`Z\x919a\x14/V[_a\r\x97\x84`\x01a*\xC0V[\x90P\x82\x15a\x0B\xD2W_a\r\xA9\x86a\x184V[\x90P_a\x0Bp\x83a\x184V[P\x82a\x0B\xDCV[__\x82\x12\x15a\r\xD3Wa\r\xCE\x82a+\xADV[a\r\xD5V[\x81[\x92\x91PPV[\x81\x83\x13\x15a\nKW_a\r\xED\x84a\x18\xE6V[\x90P_a\r\xF9\x84a\x18\xE6V[\x90P_a\x0E!\x83\x83`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x1F`\xF9\x1B\x81RP\x87a\x18wV[\x90P\x7Fb\xBD\xDA\x9A\x05\xCD\xBC\xDB\xF9\x05\xCB\xAD\x99\xC6\xEB\xDC\t\x8Bo\t3\xD8\xF2\xEB<\xFA\xB7@\x0B`%\x14\x81`@Qa\t\xEB\x91\x90a*\x87V[a\nZ\x82\x82a\x19\x9FV[a\nZ\x82\x82a\x19\xD0V[a\nZ\x82\x82a\x1A\x01V[_\x82\x84\x13a\x0B\xD9W\x82\x82\x15a\x0B\xD2W_a\x0Bd\x86a\x18\xE6V[_\x82\x84\x11a\x0B\xD9W\x82\x82\x15a\x0B\xD2W_a\r\xA9\x86a\x184V[_\x82\x84\x12a\x0B\xD9W_a\x0E\xB6`\x01\x85a+TV[\x90P\x82\x15a\x0B\xD2W_a\x0Bd\x86a\x18\xE6V[a\nZ\x82\x82a\x1A2V[\x81\x83\x12\x15a\nKW_a\x0E\xE4\x84a\x18\xE6V[\x90P_a\x0E\xF0\x84a\x18\xE6V[\x90P_a\x0F\x18\x83\x83`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x0F`\xFA\x1B\x81RP\x87a\x18wV[\x90P\x7F\x94BN\xD2O\xB3\x968\xB6H\x17\xC77\xDDD?8z\xAA\x14\x86aM\xA4I\xB6hjd-ml\x81`@Qa\t\xEB\x91\x90a*\x87V[a\nZ\x82\x82a\x1AcV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14a\nKW_a\x0Fv\x84a\x1A\x94V[\x90P_a\r\x05\x84a\x1A\x94V[a\x0F\x8B\x81a\x1B\xCDV[PV[_a\x0F\x99\x83\x83a\x19?V[\x90Pa\nK\x81\x83a\x14\xEFV[a\nZ\x82\x82a\x1C\x07V[_\x81\x83\x12\x15a\x0F\xC7Wa\x0F\xC2\x83\x83a+TV[a\x0B\xDCV[a\x0B\xDC\x82\x84a+TV[\x81\x83\x11a\nKW_a\x0F\xE2\x84a\x184V[\x90P_a\x0F\xEE\x84a\x184V[\x90P_a\x10\x17\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a<=`\xF0\x1B\x81RP\x87a\x18wV[\x90P\x7Fp{\x8CV\xE4\xC2\x11\xCF\x13!\xFA\xEBAH#pb\"\x8D\xB2\xFC\xEC\xC9\xBEH~\x83\xA2h\x0E~P\x81`@Qa\t\xEB\x91\x90a*\x87V[_`\x01\x83Qa\x10W\x91\x90a*\xADV[\x90P[\x80\x15a\nKW_\x81\x84Qa\x10n\x91\x90a*\xADV[a\x10x\x90\x84a*\xE7V[\x90P_\x84\x83\x81Q\x81\x10a\x10\x8DWa\x10\x8Da+\xC7V[` \x02` \x01\x01Q\x90P_\x85\x83\x81Q\x81\x10a\x10\xAAWa\x10\xAAa+\xC7V[` \x02` \x01\x01Q\x90P\x80\x86\x85\x81Q\x81\x10a\x10\xC7Wa\x10\xC7a+\xC7V[` \x02` \x01\x01\x81\x81RPP\x81\x86\x84\x81Q\x81\x10a\x10\xE6Wa\x10\xE6a+\xC7V[` \x02` \x01\x01\x81\x81RPPPPP\x80\x80a\x11\0\x90a+\xDBV[\x91PPa\x10ZV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x81\x16\x90\x83\x16\x14a\nKW`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19\x85\x81\x16` \x83\x01R\x82Q`\x04\x81\x84\x03\x01\x81R`$\x83\x01\x84R\x90\x85\x16`D\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x90\x92\x01\x90\x92R_a\x11e\x83a\x1C8V[\x90P_a\x11q\x83a\x1C8V[\x90P_a\x11\x9A\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a!=`\xF0\x1B\x81RP\x89a\x18wV[\x90P\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x81`@Qa\x11\xCB\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1_\x80T`@\x80Qc\t\xCC\xAE\xFB`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c\t\xCC\xAE\xFB\x92`\x04\x80\x84\x01\x93\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x12\x11W__\xFD[PZ\xFA\x15\x80\x15a\x12#W=__>=_\xFD[PPPPPPPPPPPPV[_\x81\x83\x11a\x12?W\x81a\x0B\xDCV[P\x90\x91\x90PV[_a\x12Q\x86\x86a\x19?V[\x80a\x12aWPa\x12a\x86\x85a\x19?V[\x80a\x12qWPa\x12q\x86\x84a\x19?V[\x80a\x12\x81WPa\x12\x81\x86\x83a\x19?V[\x90P_\x85\x85\x85\x85`@Q` \x01a\x12\x9B\x94\x93\x92\x91\x90a+\xF0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x12\xB6\x82\x82a\x14\xEFV[PPPPPPPV[_\x81\x83\x10a\x12?W\x81a\x0B\xDCV[_a\x0B%\x84\x84\x84`\x01a\x0B\xE3V[_a\x0B\xDC\x83\x83`\x01a\x0EpV[_a\x12\xF3\x85\x85a\x19?V[\x80a\x13\x03WPa\x13\x03\x85\x84a\x19?V[\x80a\x13\x13WPa\x13\x13\x85\x83a\x19?V[\x90P_\x84\x84\x84`@Q` \x01a\x13+\x93\x92\x91\x90a,OV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x13F\x82\x82a\x14\xEFV[PPPPPPV[_\x81\x83\x13a\x12?W\x81a\x0B\xDCV[\x81\x83\x10\x15a\nKW_a\x13n\x84a\x184V[\x90P_a\x0E\xF0\x84a\x184V[_a\x13\x85\x84\x84a\x19?V[\x80a\x13\x95WPa\x13\x95\x84\x83a\x19?V[\x90P_\x83\x83`@Q` \x01a\x13\xAB\x92\x91\x90a,\x94V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x13\xC6\x82\x82a\x14\xEFV[PPPPPV[_\x80[\x83Q\x81\x10\x15a\x14\x1EW\x83\x81\x81Q\x81\x10a\x13\xEBWa\x13\xEBa+\xC7V[` \x02` \x01\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x85`\x01`\x01`\xE0\x1B\x03\x19\x16\x03a\x14\x16W`\x01\x91Pa\x14\x1EV[`\x01\x01a\x13\xD0V[Pa\x14)\x81\x83a\x14\xEFV[PPPPV[\x81\x83\x03a\nKW_a\x14@\x84a\x184V[\x90P_a\x14L\x84a\x184V[\x90P_a\x14u\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a==`\xF0\x1B\x81RP\x87a\x18wV[\x90P\x7F\xFFM\xE0\x80\xD28O#>\x1AO\xB3&\x8E\x8Fq]\x88\x02!z\xD28\"5\x90\n'M\xDEv\xDD\x81`@Qa\t\xEB\x91\x90a*\x87V[_\x81\x83\x10\x15a\x14\xB9Wa\x0F\xC2\x83\x83a*\xADV[a\x0B\xDC\x82\x84a*\xADV[a\nZ\x82\x82a\x1D\xF8V[_\x82\x84\x13a\r\xB5W_a\x0E\xB6\x84`\x01a+sV[_a\x0B%\x84\x84\x84`\x01a\n^V[\x81a\nZW\x7F\xEB\x03\xCA\x8C\x87\xC7\x84\x9B\xEF\x8FT\xCF\xDD,k\x96{'4\xFE\x87/u\x19x\xC3K\xB9\x1E\x13\xD3Q\x81`@Qa\x15#\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1_\x80T`@\x80Qc\t\xCC\xAE\xFB`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c\t\xCC\xAE\xFB\x92`\x04\x80\x84\x01\x93\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x15iW__\xFD[PZ\xFA\x15\x80\x15a\x13FW=__>=_\xFD[a\nZ\x82\x82a\x1E)V[_a\x0B\xDC\x83\x83`\x01a\x0BJV[a\x0F\x8B\x81a\x1EZV[_\x80\x80[\x84Q\x81\x10\x15a\x15\xF0W\x84\x81\x81Q\x81\x10a\x15\xBAWa\x15\xBAa+\xC7V[` \x02` \x01\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86`\x01`\x01`\xE0\x1B\x03\x19\x16\x03a\x15\xE8W`\x01\x92P\x80\x91Pa\x15\xF0V[`\x01\x01a\x15\x9FV[Pa\x13\xC6\x82\x84\x83\x81Q\x81\x10a\x16\x07Wa\x16\x07a+\xC7V[` \x02` \x01\x01Qa\x14\xEFV[_\x82\x84\x10a\x0B\xD9Wa\x16?\x83_`@Q\x80`\x80\x01`@R\x80`Q\x81R` \x01a/\xDC`Q\x919a\x14/V[_a\r\x97\x84\x86a*\xE7V[a\nZ\x82\x82a\x1E\x89V[\x81\x83\x11\x15a\nKW_a\x16f\x84a\x184V[\x90P_a\r\xF9\x84a\x184V[\x81\x83\x14a\nKW_a\x16\x83\x84a\x184V[\x90P_a\r\x05\x84a\x184V[\x81\x83\x03a\nKW_a\x16\xA0\x84a\x18\xE6V[\x90P_a\x14L\x84a\x18\xE6V[a\nZ\x82\x82a\x1E\xBAV[_a\x0B\xDC\x83\x83`\x01a\x17\xCFV[\x81\x83\x13a\nKW_a\x16\xD4\x84a\x18\xE6V[\x90P_a\x0F\xEE\x84a\x18\xE6V[a\x16\xE8a\x1E\xEBV[V[\x81\x15\x15\x83\x15\x15\x14a\nKW_\x83a\x17\x1EW`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dfalse`\xD8\x1B\x81RPa\x17<V[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01ctrue`\xE0\x1B\x81RP[\x90P_\x83a\x17gW`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dfalse`\xD8\x1B\x81RPa\r\x05V[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01ctrue`\xE0\x1B\x81RP\x90P_a\r.\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a!=`\xF0\x1B\x81RP\x87a\x18wV[__\x82`\x0F\x0B\x12\x15a\r\xD3Wa\r\xCE\x82a,\xB6V[_a\x0B\xDC\x83\x83`\x01a\x0E\x89V[_\x82\x84\x11\x15a\x0B\xD9W_a\x17\xE4\x84`\x01a*\xC0V[a\r\x97\x90\x86a*\xE7V[_a\x0B\xDC\x83\x83`\x01a\x0E\xA2V[_a\x0B\xDC\x83\x83`\x01a\x14\xCDV[a\nZ\x82\x82a\x1F\x15V[_a\x0B\xDC\x83\x83a\x18\x12V[a\nZ\x82\x82a\x1FFV[_a\x0B\xDC\x83\x83`\x01a\r_V[```\xA0`@Q\x01\x80`@R` \x81\x03\x91PP_\x81R\x80\x82[`\x01\x83\x03\x92P`\n\x81\x06`0\x01\x83S`\n\x90\x04\x80a\x18MWP\x81\x90\x03`\x1F\x19\x90\x91\x01\x90\x81R\x91\x90PV[``\x84\x83\x85\x84`@Q` \x01a\x18\x90\x94\x93\x92\x91\x90a,\xE3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[\x7F\xF9\xA6\xEE8\xCDn\x03G\xCB\x8Cjt\xFFjc\xEEw\xC6&\xBApFj\xAD\xFA\x86\xF9'/\xA3\xB8\x98\x82\x82`@Qa\x18\xDA\x92\x91\x90a-/V[`@Q\x80\x91\x03\x90\xA1PPV[``__\x83\x12\x15a\x18\xFFWa\x18\xFA\x83a+\xADV[a\x19\x01V[\x82[\x90Pa\x19\x0C\x81a\x184V[\x91P_\x83\x12\x15a\x199W\x81`@Q` \x01a\x19'\x91\x90a-RV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P[P\x91\x90PV[_\x81`@Q` \x01a\x19Q\x91\x90a-gV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x19p\x84a\x1FwV[`@Q` \x01a\x19\x80\x91\x90a-gV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14\x90P\x92\x91PPV[\x7F\x02\xD95)\xBB\xA9\xD1A\xE5\xE0g3\xC5,~o\xBC\xB1\x14\x95\x86\xAD\xB5\xC2@d\xB5\"\xAB&\xF1\xD7\x82\x82`@Qa\x18\xDA\x92\x91\x90a-rV[\x7F\x9B\xAAl\xC9oN\xF5tp\xCB\x9A\xF4\x15a\xFD9\x03/w\xCAJ\x06y0\x80{\xB7\xD0\xD38l\xBF\x82\x82`@Qa\x18\xDA\x92\x91\x90a-rV[\x7F\xE8@z\x02\t\xFA\x99\xEC:r(\xAF\xF1@\xC3\xD3\xE6\x8B\xD2y9\x179\xC7\xE0\xB6\\\xD4\x06\xCC\x93\xB5\x82\x82`@Qa\x18\xDA\x92\x91\x90a-\x93V[\x7Fb\xDD\xFF\xE5\xB5\x10\x83\x85\xF7\xA5\x90\xF1\0\xE1\xEEAJ\xD9U\x1A1\xF0\x89\xE6N\x82\x99\x84@x^\x1E\x82\x82`@Qa\x18\xDA\x92\x91\x90a-\xB7V[\x7Fj\x83\x7F\xF3\x97:\xA4\x18\x1E{\x9F\x07uo\x8B~\xE3f\xDD\x85\xA3fU\xD2\xCBB\xCDG\xF1\x0B\x968\x82\x82`@Qa\x18\xDA\x92\x91\x90a-rV[`@\x80Q`(\x80\x82R``\x82\x81\x01\x90\x93R_\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P_[`\x14\x81\x10\x15a\x1B\xC6W_a\x1A\xCE\x82`\x13a*\xADV[a\x1A\xD9\x90`\x08a-\xE0V[a\x1A\xE4\x90`\x02a.\xDAV[a\x1A\xF7\x90`\x01`\x01`\xA0\x1B\x03\x87\x16a.\xE5V[`\xF8\x1B\x90P_`\x10\x82`\xF8\x1Ca\x1B\r\x91\x90a.\xF8V[`\xF8\x1B\x90P_\x81`\xF8\x1C`\x10a\x1B#\x91\x90a/\x19V[\x83`\xF8\x1Ca\x1B1\x91\x90a/5V[`\xF8\x1B\x90Pa\x1B?\x82a!QV[\x85a\x1BK\x86`\x02a-\xE0V[\x81Q\x81\x10a\x1B[Wa\x1B[a+\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPa\x1Bz\x81a!QV[\x85a\x1B\x86\x86`\x02a-\xE0V[a\x1B\x91\x90`\x01a*\xC0V[\x81Q\x81\x10a\x1B\xA1Wa\x1B\xA1a+\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPP`\x01\x90\x92\x01\x91Pa\x1A\xB9\x90PV[P\x92\x91PPV[\x7F\xCF4\xEFSz\xC3>\xE1\xACbl\xA1Xz\n~\x8EQV\x1EU\x14\xF8\xCB6\xAF\xA1\xC5\x10+;\xAB\x81`@Qa\x1B\xFC\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1PV[\x7F'\xA8v~\xD1r\xB4\x8D|\xF6\xDB\xB2](@\xC5\xFA\x91\xEFx\xFAn\x05\xC1<\x82v=\t\x9F\x03\xCA\x82\x82`@Qa\x18\xDA\x92\x91\x90a-\x93V[``_\x82Q`\x02a\x1CI\x91\x90a-\xE0V[a\x1CT\x90`\x02a*\xC0V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1CkWa\x1Cka!\x8BV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1C\x95W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81_\x81Q\x81\x10a\x1C\xAFWa\x1C\xAFa+\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x1C\xDDWa\x1C\xDDa+\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP_[\x83Q\x81\x10\x15a\x1B\xC6W_\x84\x82\x81Q\x81\x10a\x1D\x11Wa\x1D\x11a+\xC7V[\x01` \x01Q`\xF8\x81\x90\x1C\x91Po\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x90`\xFC\x1C`\x10\x81\x10a\x1DEWa\x1DEa+\xC7V[\x1A`\xF8\x1B\x83a\x1DU\x84`\x02a-\xE0V[a\x1D`\x90`\x02a*\xC0V[\x81Q\x81\x10a\x1DpWa\x1Dpa+\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\x0F\x82\x16`\x10\x81\x10a\x1D\xAEWa\x1D\xAEa+\xC7V[\x1A`\xF8\x1B\x83a\x1D\xBE\x84`\x02a-\xE0V[a\x1D\xC9\x90`\x03a*\xC0V[\x81Q\x81\x10a\x1D\xD9Wa\x1D\xD9a+\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPP`\x01\x01a\x1C\xF5V[\x7F\xF1i'kB\x1EmkD0\xD1\x93K\xD1L\xC5Cc\x05\xAD\x11\xE2\xBA\xA6q@\x84K\xB1;~~\x82\x82`@Qa\x18\xDA\x92\x91\x90a-\x93V[\x7FN\xF9fJF\xFF\xE05q*\xA3aV\x17\xC7\xB5\xAAx#&\xD5\x8E\x89I\xDBK\xEEH\x18 A\x93\x82\x82`@Qa\x18\xDA\x92\x91\x90a-rV[\x7F\xB4&\x04\xCB\x10Z\x16\xC8\xF6\xDB\x8AA\xE6\xB0\x0C\x0C\x1BH&F^\x8B\xC5\x04\xB3\xEB>\x88\xB3\xE6\xA4\xA0\x81`@Qa\x1B\xFC\x91\x90a*\x87V[\x7F\x94\x12\x96\xA3\x9E\xA1\x07\xBD\xE6\x85R#\x18\xA4\xB6\xC2\xB5D\x90J]\xD8*Q'H\xCA,\xF89\xBE\xF7\x82\x82`@Qa\x18\xDA\x92\x91\x90a-rV[\x7FL4\xC2\xF9\xA7\x862\xF2\x9F\xA5\x9A\xAE\xD5QL\xB7B\xFD\x9F\xBC\xFD|\xCC,\x03\xC8_+\xBCb\x1CG\x82\x82`@Qa\x18\xDA\x92\x91\x90a-/V[`@Q\x7F\xF9-J\x94\xD1\xD5\x01F\x96\xDC\xFCe\xA0\xA0a\xAF\x97`\x8E\xEB\xD7\xFE\xA0Q\x9F\xF4\xFD\xBC\xA7\x1B\xAE\x9F\x90_\x90\xA1V[\x7F\xE5#+\xA1qT\xFD\xF2\xAE\x11\x84j\xF0\x9E\x8B\xD8B\n\xA2\x105\x02j\xC8\x8D\xB6\xCF\xB8\xC7\xB4\x90\x12\x82\x82`@Qa\x18\xDA\x92\x91\x90a-\xB7V[\x7F\xAD\xC8\x168\xCF\xE9\xDC/.NOu\x9B&\xF2\0j\x87\xEBa-e\xBC\xD2\x82\x8Fr\xB1L\x05\x84D\x82\x82`@Qa\x18\xDA\x92\x91\x90a-\x93V[``\x81Q`$\x03a \xF4W`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81RmPanic(uint256)`\x90\x1B` \x90\x91\x01R\x7FNH{qS\x9E\x01d\xC9\xD2\x95\x06\xCCr^I4+\xCA\xC1^\t'(+\xF3\x0F\xED\xFE\x1Crh_[`\x04\x81\x10\x15a TW\x81\x81`\x04\x81\x10a\x1F\xE6Wa\x1F\xE6a+\xC7V[\x1A`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x84\x82\x81Q\x81\x10a \x06Wa \x06a+\xC7V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x14a LWPP`@\x80Q\x80\x82\x01\x90\x91R`\x13\x81RrUndefined signature`h\x1B` \x82\x01R\x92\x91PPV[`\x01\x01a\x1F\xCBV[P_`\x04[`$\x81\x10\x15a \x90W`\x08\x82\x90\x1B\x91P\x84\x81\x81Q\x81\x10a {Wa {a+\xC7V[\x01` \x01Q`\xF8\x1C\x91\x90\x91\x17\x90`\x01\x01a YV[P\x80`\x11\x03a \xC1WPP`@\x80Q\x80\x82\x01\x90\x91R`\t\x81RhPanic(17)`\xB8\x1B` \x82\x01R\x92\x91PPV[PP`@\x80Q\x80\x82\x01\x90\x91R`\x14\x81RsUndefined panic code``\x1B` \x82\x01R\x92\x91PPV[`D\x82Q\x10\x15a!7WPP`@\x80Q\x80\x82\x01\x90\x91R`\x1D\x81R\x7FTransaction reverted silently\0\0\0` \x82\x01R\x90V[`\x04\x82\x01\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\r\xD5\x91\x90a/NV[_`\n`\xF8\x83\x90\x1C\x10\x15a!wWa!n`\xF8\x83\x90\x1C`0a/\xC2V[`\xF8\x1B\x92\x91PPV[a!n`\xF8\x83\x90\x1C`Wa/\xC2V[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a!\xC7Wa!\xC7a!\x8BV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a!\xE7Wa!\xE7a!\x8BV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a\"\x04W__\xFD[\x815` \x83\x01_a\"\x1Ca\"\x17\x84a!\xCFV[a!\x9FV[\x90P\x82\x81R\x85\x83\x83\x01\x11\x15a\"/W__\xFD[\x82\x82` \x83\x017_\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\"[W__\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"~W__\xFD[a\"\x8A\x86\x82\x87\x01a!\xF5V[\x91PP\x92P\x92P\x92V[\x805\x80\x15\x15\x81\x14a!\x86W__\xFD[__`@\x83\x85\x03\x12\x15a\"\xB4W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xC9W__\xFD[a\"\xD5\x85\x82\x86\x01a!\xF5V[\x92PPa\"\xE4` \x84\x01a\"\x94V[\x90P\x92P\x92\x90PV[____`\x80\x85\x87\x03\x12\x15a#\0W__\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa#\x1E``\x86\x01a\"\x94V[\x90P\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15a#;W__\xFD[\x835\x92P` \x84\x015\x91Pa#R`@\x85\x01a\"\x94V[\x90P\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a#lW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a#\x81W__\xFD[a#\x8D\x85\x82\x86\x01a!\xF5V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xA8W__\xFD[a#\xB4\x85\x82\x86\x01a!\xF5V[\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a#\xCEW__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a#\xE6W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xFBW__\xFD[a$\x07\x85\x82\x86\x01a!\xF5V[\x95` \x94\x90\x94\x015\x94PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\x86W__\xFD[__`@\x83\x85\x03\x12\x15a$=W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a$RW__\xFD[a$^\x85\x82\x86\x01a!\xF5V[\x92PPa\"\xE4` \x84\x01a$\x16V[___``\x84\x86\x03\x12\x15a$\x7FW__\xFD[a$\x88\x84a$\x16V[\x92Pa$\x96` \x85\x01a$\x16V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"~W__\xFD[_` \x82\x84\x03\x12\x15a$\xC0W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xD5W__\xFD[a\x0B%\x84\x82\x85\x01a!\xF5V[__`@\x83\x85\x03\x12\x15a$\xF2W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_` \x82\x84\x03\x12\x15a%\x11W__\xFD[a\x0B\xDC\x82a$\x16V[_`\x01`\x01`@\x1B\x03\x82\x11\x15a%2Wa%2a!\x8BV[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15a%MW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a%bW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a%rW__\xFD[\x805a%\x80a\"\x17\x82a%\x1AV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a%\xA1W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a%\xC3W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a%\xA8V[\x97` \x96\x90\x96\x015\x96PPPPPPV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a!\x86W__\xFD[___``\x84\x86\x03\x12\x15a%\xFDW__\xFD[a&\x06\x84a%\xD4V[\x92Pa$\x96` \x85\x01a%\xD4V[_____`\xA0\x86\x88\x03\x12\x15a&(W__\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15a&=W__\xFD[a&I\x88\x82\x89\x01a!\xF5V[\x95PP` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&dW__\xFD[a&p\x88\x82\x89\x01a!\xF5V[\x94PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&\x8BW__\xFD[a&\x97\x88\x82\x89\x01a!\xF5V[\x93PP``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xB2W__\xFD[a&\xBE\x88\x82\x89\x01a!\xF5V[\x92PP`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xD9W__\xFD[a&\xE5\x88\x82\x89\x01a!\xF5V[\x91PP\x92\x95P\x92\x95\x90\x93PV[___``\x84\x86\x03\x12\x15a'\x04W__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a'.W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a'CW__\xFD[a'O\x87\x82\x88\x01a!\xF5V[\x94PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a'jW__\xFD[a'v\x87\x82\x88\x01a!\xF5V[\x93PP`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a'\x91W__\xFD[a'\x9D\x87\x82\x88\x01a!\xF5V[\x92PP``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xB8W__\xFD[a'\xC4\x87\x82\x88\x01a!\xF5V[\x91PP\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15a'\xE2W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xF7W__\xFD[a(\x03\x86\x82\x87\x01a!\xF5V[\x93PP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\x1EW__\xFD[a(*\x86\x82\x87\x01a!\xF5V[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"~W__\xFD[_\x82`\x1F\x83\x01\x12a(TW__\xFD[\x815a(ba\"\x17\x82a%\x1AV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a(\x83W__\xFD[` \x85\x01[\x83\x81\x10\x15a(\xA7Wa(\x99\x81a%\xD4V[\x83R` \x92\x83\x01\x92\x01a(\x88V[P\x95\x94PPPPPV[___``\x84\x86\x03\x12\x15a(\xC3W__\xFD[a(\xCC\x84a%\xD4V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xE6W__\xFD[a(*\x86\x82\x87\x01a(EV[__`@\x83\x85\x03\x12\x15a)\x03W__\xFD[a)\x0C\x83a\"\x94V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xA8W__\xFD[___``\x84\x86\x03\x12\x15a)8W__\xFD[a)A\x84a%\xD4V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)[W__\xFD[a)g\x86\x82\x87\x01a(EV[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)\x82W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a)\x92W__\xFD[\x805a)\xA0a\"\x17\x82a%\x1AV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a)\xC1W__\xFD[` \x84\x01[\x83\x81\x10\x15a*\x01W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xE3W__\xFD[a)\xF2\x8B` \x83\x89\x01\x01a!\xF5V[\x84RP` \x92\x83\x01\x92\x01a)\xC6V[P\x80\x94PPPPP\x92P\x92P\x92V[___``\x84\x86\x03\x12\x15a*\"W__\xFD[a*+\x84a\"\x94V[\x92Pa$\x96` \x85\x01a\"\x94V[_` \x82\x84\x03\x12\x15a*IW__\xFD[\x815\x80`\x0F\x0B\x81\x14a\x0B\xDCW__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0B\xDC` \x83\x01\x84a*YV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\r\xD5Wa\r\xD5a*\x99V[\x80\x82\x01\x80\x82\x11\x15a\r\xD5Wa\r\xD5a*\x99V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a*\xF5Wa*\xF5a*\xD3V[P\x06\x90V[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[n\x02\x1Bc\x0Bk\x83Ks9\x03\xB3\x0Bc\xAB)`\x8D\x1B\x81R_a+4`\x0F\x83\x01\x85a*\xFAV[c\x01\x03\xA3y`\xE5\x1B\x81Ra+K`\x04\x82\x01\x85a*\xFAV[\x95\x94PPPPPV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1B\xC6Wa\x1B\xC6a*\x99V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a+\x92Wa+\x92a*\x99V[PP\x92\x91PPV[_\x82a+\xA8Wa+\xA8a*\xD3V[P\x07\x90V[_`\x01`\xFF\x1B\x82\x01a+\xC1Wa+\xC1a*\x99V[P_\x03\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x81a+\xE9Wa+\xE9a*\x99V[P_\x19\x01\x90V[_a+\xFB\x82\x87a*\xFAV[c\x01\x02z\x91`\xE5\x1B\x81Ra,\x12`\x04\x82\x01\x87a*\xFAV[\x90Pc\x01\x02z\x91`\xE5\x1B\x81Ra,+`\x04\x82\x01\x86a*\xFAV[\x90Pc\x01\x02z\x91`\xE5\x1B\x81Ra,D`\x04\x82\x01\x85a*\xFAV[\x97\x96PPPPPPPV[_a,Z\x82\x86a*\xFAV[c\x01\x02z\x91`\xE5\x1B\x81Ra,q`\x04\x82\x01\x86a*\xFAV[\x90Pc\x01\x02z\x91`\xE5\x1B\x81Ra,\x8A`\x04\x82\x01\x85a*\xFAV[\x96\x95PPPPPPV[_a,\x9F\x82\x85a*\xFAV[c\x01\x02z\x91`\xE5\x1B\x81Ra+K`\x04\x82\x01\x85a*\xFAV[_\x81`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03a,\xDBWa,\xDBa*\x99V[_\x03\x92\x91PPV[h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x81R_a-\x12a-\x0Ca-\x06`\t\x85\x01\x89a*\xFAV[\x87a*\xFAV[\x85a*\xFAV[i\x01a\x03\x93+\x0B\x9B{q\xD1`\xB5\x1B\x81Ra,D`\n\x82\x01\x85a*\xFAV[`@\x81R_a-A`@\x83\x01\x85a*YV[\x90P\x82\x15\x15` \x83\x01R\x93\x92PPPV[`-`\xF8\x1B\x81R_a\x0B\xDC`\x01\x83\x01\x84a*\xFAV[_a\x0B\xDC\x82\x84a*\xFAV[`@\x81R_a-\x84`@\x83\x01\x85a*YV[\x90P\x82` \x83\x01R\x93\x92PPPV[`@\x81R_a-\xA5`@\x83\x01\x85a*YV[\x82\x81\x03` \x84\x01Ra+K\x81\x85a*YV[`@\x81R_a-\xC9`@\x83\x01\x85a*YV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\r\xD5Wa\r\xD5a*\x99V[`\x01\x81[`\x01\x84\x11\x15a.2W\x80\x85\x04\x81\x11\x15a.\x16Wa.\x16a*\x99V[`\x01\x84\x16\x15a.$W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a-\xFBV[\x93P\x93\x91PPV[_\x82a.HWP`\x01a\r\xD5V[\x81a.TWP_a\r\xD5V[\x81`\x01\x81\x14a.jW`\x02\x81\x14a.tWa.\x90V[`\x01\x91PPa\r\xD5V[`\xFF\x84\x11\x15a.\x85Wa.\x85a*\x99V[PP`\x01\x82\x1Ba\r\xD5V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a.\xB3WP\x81\x81\na\r\xD5V[a.\xBF_\x19\x84\x84a-\xF7V[\x80_\x19\x04\x82\x11\x15a.\xD2Wa.\xD2a*\x99V[\x02\x93\x92PPPV[_a\x0B\xDC\x83\x83a.:V[_\x82a.\xF3Wa.\xF3a*\xD3V[P\x04\x90V[_`\xFF\x83\x16\x80a/\nWa/\na*\xD3V[\x80`\xFF\x84\x16\x04\x91PP\x92\x91PPV[`\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x1B\xC6Wa\x1B\xC6a*\x99V[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\r\xD5Wa\r\xD5a*\x99V[_` \x82\x84\x03\x12\x15a/^W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a/sW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a/\x83W__\xFD[\x80Qa/\x91a\"\x17\x82a!\xCFV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a/\xA5W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\r\xD5Wa\r\xD5a*\x99V\xFEclampLt cannot clamp value a to be less than zero. Check your inputs/assumptions.clampGt cannot clamp value a to be larger than uint256.max. Check your inputs/assumptions.\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static FUZZLIB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x04\x01W_5`\xE0\x1C\x80c~\x1D\xDD\x80\x11a\x02\x16W\x80c\xB6\x0Er\xCC\x11a\x01*W\x80c\xE0\x1E\x92\xD2\x11a\0\xB4W\x80c\xF0\x9C\xED\x97\x11a\0\x84W\x80c\xF0\x9C\xED\x97\x14a\t\x15W\x80c\xF0\xF4\xB6R\x14a\t(W\x80c\xF5\xBC:q\x14a\t;W\x80c\xF5\xC4f\xF6\x14a\tNW\x80c\xFB\x98\x94\xB2\x14a\taW__\xFD[\x80c\xE0\x1E\x92\xD2\x14a\x08\xB6W\x80c\xE8\x94\t\r\x14a\x08\xDCW\x80c\xEE\xF2S\x89\x14a\x08\xEFW\x80c\xF0\x1F\xF6L\x14a\t\x02W__\xFD[\x80c\xC3\xB5V5\x11a\0\xFAW\x80c\xC3\xB5V5\x14a\x08bW\x80c\xC5\xDF*7\x14a\x08uW\x80c\xD2\xA5\x06\x04\x14a\x08\x88W\x80c\xDE\x9B\"2\x14a\x08\x9BW\x80c\xDF|\xD7\x7F\x14a\x08\xA3W__\xFD[\x80c\xB6\x0Er\xCC\x14a\x08\x16W\x80c\xBB5\x03\x1A\x14a\x08)W\x80c\xBC\x8DC\xA8\x14a\x08<W\x80c\xC0\xE8\xDE\xF2\x14a\x08OW__\xFD[\x80c\x94'2\xB2\x11a\x01\xABW\x80c\xA9\x82\x83\xE4\x11a\x01{W\x80c\xA9\x82\x83\xE4\x14a\x07\xB7W\x80c\xAC\x0C {\x14a\x07\xCAW\x80c\xAE\xEF\x9C!\x14a\x07\xDDW\x80c\xB0\x9CA\xB7\x14a\x07\xF0W\x80c\xB2\xF9y\x9D\x14a\x08\x03W__\xFD[\x80c\x94'2\xB2\x14a\x07kW\x80c\x95>^\x80\x14a\x07~W\x80c\x9EB$G\x14a\x07\x91W\x80c\xA1_\x9F\x07\x14a\x07\xA4W__\xFD[\x80c\x84b\x13\xA1\x11a\x01\xE6W\x80c\x84b\x13\xA1\x14a\x07\x1FW\x80c\x88\xE3b\xC6\x14a\x072W\x80c\x8CO\xB5}\x14a\x07EW\x80c\x8F^QJ\x14a\x07XW__\xFD[\x80c~\x1D\xDD\x80\x14a\x06\xD3W\x80c\x80\x10\xBA\xD3\x14a\x06\xE6W\x80c\x81\xFEW\x86\x14a\x06\xF9W\x80c\x84\x1E\xA1\x1C\x14a\x07\x0CW__\xFD[\x80c1\x9A\xF33\x11a\x03\x18W\x80cV\x8C\x13\x85\x11a\x02\xA2W\x80cl\x97d\xB3\x11a\x02rW\x80cl\x97d\xB3\x14a\x06tW\x80cmT3\xE6\x14a\x06\x87W\x80cr\xDEw\xB5\x14a\x06\x9AW\x80cz\xE2\xB5\xC7\x14a\x06\xADW\x80c{\x8D\x0F\x0C\x14a\x06\xC0W__\xFD[\x80cV\x8C\x13\x85\x14a\x06\x0CW\x80c\\+\x80\xF5\x14a\x06\x1FW\x80ciE\xC5\xEA\x14a\x062W\x80cj\xCEU,\x14a\x06aW__\xFD[\x80c<\xCB^&\x11a\x02\xE8W\x80c<\xCB^&\x14a\x05\x96W\x80cA0O\xAC\x14a\x05\xA9W\x80cB\x06\x87g\x14a\x05\xBCW\x80cK\\Bw\x14a\x05\xCFW\x80cK\xDE8\xC8\x14a\x05\xE2W__\xFD[\x80c1\x9A\xF33\x14a\x05]W\x80c6\x92\xD6\x1A\x14a\x04\xEBW\x80c;m\xDF\x03\x14a\x05pW\x80c<\xA6&\x8E\x14a\x05\x83W__\xFD[\x80c\x1BU\xD0|\x11a\x03\x99W\x80c!<\xDE,\x11a\x03iW\x80c!<\xDE,\x14a\x04\xFEW\x80c#\xCD\xD8\xE8\x14a\x05\x11W\x80c-@\x85\x1E\x14a\x05$W\x80c.\x17\xD4r\x14a\x057W\x80c/\xFD\xE6\x03\x14a\x05JW__\xFD[\x80c\x1BU\xD0|\x14a\x04\xB2W\x80c\x1BZ\xC4\xB5\x14a\x04\xC5W\x80c\x1B\xA09\x9B\x14a\x04\xD8W\x80c\x1F\xC4u^\x14a\x04\xEBW__\xFD[\x80c\x08N\xC4\x10\x11a\x03\xD4W\x80c\x08N\xC4\x10\x14a\x04fW\x80c\t\xFD8\x99\x14a\x04yW\x80c\x10Y\xBF\xC9\x14a\x04\x8CW\x80c\x14\xEC\x1C\xC6\x14a\x04\x9FW__\xFD[\x80c\x01\xB9\xE8'\x14a\x04\x05W\x80c\x04\xFB\xE1\xC5\x14a\x04\x1AW\x80c\x06\xCB\0\x11\x14a\x04-W\x80c\x06\xF8#B\x14a\x04SW[__\xFD[a\x04\x18a\x04\x136`\x04a\"IV[a\ttV[\0[a\x04\x18a\x04(6`\x04a\"\xA3V[a\nPV[a\x04@a\x04;6`\x04a\"\xEDV[a\n^V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x04\x18a\x04a6`\x04a\"IV[a\x0B-V[a\x04@a\x04t6`\x04a#)V[a\x0BJV[a\x04@a\x04\x876`\x04a\"\xEDV[a\x0B\xE3V[a\x04\x18a\x04\x9A6`\x04a#[V[a\x0C\xD0V[a\x04\x18a\x04\xAD6`\x04a\"IV[a\x0C\xE8V[a\x04@a\x04\xC06`\x04a#)V[a\r_V[a\x04@a\x04\xD36`\x04a#\xBEV[a\r\xBCV[a\x04\x18a\x04\xE66`\x04a\"IV[a\r\xDBV[a\x04\x18a\x04\xF96`\x04a#\xD5V[a\x0ERV[a\x04\x18a\x05\x0C6`\x04a#\xD5V[a\x0E\\V[a\x04\x18a\x05\x1F6`\x04a#[V[a\x0EfV[a\x04@a\x0526`\x04a#)V[a\x0EpV[a\x04@a\x05E6`\x04a#)V[a\x0E\x89V[a\x04@a\x05X6`\x04a#)V[a\x0E\xA2V[a\x04\x18a\x05k6`\x04a$,V[a\x0E\xC8V[a\x04\x18a\x05~6`\x04a\"IV[a\x0E\xD2V[a\x04\x18a\x05\x916`\x04a#\xD5V[a\x0FIV[a\x04\x18a\x05\xA46`\x04a$mV[a\x0FSV[a\x04\x18a\x05\xB76`\x04a$\xB0V[a\x0F\x82V[a\x04\x18a\x05\xCA6`\x04a#[V[a\x0F\x8EV[a\x04\x18a\x05\xDD6`\x04a#[V[a\x0F\xA5V[_Ta\x05\xF4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04JV[a\x04@a\x06\x1A6`\x04a$\xE1V[a\x0F\xAFV[a\x04\x18a\x06-6`\x04a\"IV[a\x0F\xD1V[a\x04\x18a\x06@6`\x04a%\x01V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x04\x18a\x06o6`\x04a%<V[a\x10HV[a\x04\x18a\x06\x826`\x04a%\xEBV[a\x11\x08V[a\x04@a\x06\x956`\x04a$\xE1V[a\x121V[a\x04\x18a\x06\xA86`\x04a&\x14V[a\x12FV[a\x04@a\x06\xBB6`\x04a$\xE1V[a\x12\xBFV[a\x04@a\x06\xCE6`\x04a&\xF2V[a\x12\xCDV[a\x04@a\x06\xE16`\x04a$\xE1V[a\x12\xDBV[a\x04\x18a\x06\xF46`\x04a'\x1BV[a\x12\xE8V[a\x04@a\x07\x076`\x04a$\xE1V[a\x13NV[a\x04\x18a\x07\x1A6`\x04a\"IV[a\x13\\V[a\x04\x18a\x07-6`\x04a'\xD0V[a\x13zV[a\x04\x18a\x07@6`\x04a(\xB1V[a\x13\xCDV[a\x04\x18a\x07S6`\x04a\"IV[a\x14/V[a\x04@a\x07f6`\x04a$\xE1V[a\x14\xA6V[a\x04\x18a\x07y6`\x04a#[V[a\x14\xC3V[a\x04@a\x07\x8C6`\x04a#)V[a\x14\xCDV[a\x04@a\x07\x9F6`\x04a&\xF2V[a\x14\xE1V[a\x04\x18a\x07\xB26`\x04a(\xF2V[a\x14\xEFV[a\x04\x18a\x07\xC56`\x04a#\xD5V[a\x15{V[a\x04@a\x07\xD86`\x04a$\xE1V[a\x15\x85V[a\x04\x18a\x07\xEB6`\x04a$\xB0V[a\x15\x92V[a\x04\x18a\x07\xFE6`\x04a)&V[a\x15\x9BV[a\x04@a\x08\x116`\x04a#)V[a\x16\x14V[a\x04\x18a\x08$6`\x04a#\xD5V[a\x16JV[a\x04\x18a\x0876`\x04a\"IV[a\x16TV[a\x04\x18a\x08J6`\x04a\"IV[a\x16rV[a\x04\x18a\x08]6`\x04a\"IV[a\x16\x8FV[a\x04\x18a\x08p6`\x04a\"\xA3V[a\x16\xACV[a\x04@a\x08\x836`\x04a$\xE1V[a\x16\xB6V[a\x04\x18a\x08\x966`\x04a\"IV[a\x16\xC3V[a\x04\x18a\x16\xE0V[a\x04\x18a\x08\xB16`\x04a*\x10V[a\x16\xEAV[a\x08\xC9a\x08\xC46`\x04a*9V[a\x17\xADV[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x04JV[a\x04@a\x08\xEA6`\x04a$\xE1V[a\x17\xC2V[a\x04@a\x08\xFD6`\x04a#)V[a\x17\xCFV[a\x04@a\t\x106`\x04a$\xE1V[a\x17\xEEV[a\x04@a\t#6`\x04a$\xE1V[a\x17\xFBV[a\x04\x18a\t66`\x04a$,V[a\x18\x08V[a\x04@a\tI6`\x04a$\xE1V[a\x18\x12V[a\x04\x18a\t\\6`\x04a#[V[a\x18\x1DV[a\x04@a\to6`\x04a$\xE1V[a\x18'V[\x81\x83\x10a\nKW_a\t\x85\x84a\x184V[\x90P_a\t\x91\x84a\x184V[\x90P_a\t\xBA\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a>=`\xF0\x1B\x81RP\x87a\x18wV[\x90P\x7FF\xD0\x1D\xFB\xC4\x86\x1B[\x85\x9D\x85\x8DT\x1E\x8B\0F\x17Z\x8D\xEC\xE7\x0E$\xF8\x92\xA31\x8Fp\xE2\xBB\x81`@Qa\t\xEB\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1_\x80T`@\x80Qc\t\xCC\xAE\xFB`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c\t\xCC\xAE\xFB\x92`\x04\x80\x84\x01\x93\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\n1W__\xFD[PZ\xFA\x15\x80\x15a\nCW=__>=_\xFD[PPPPPPP[PPPV[a\nZ\x82\x82a\x18\xA9V[PPV[_\x83\x85\x10\x80a\nlWP\x82\x85\x11[\x15a\x0B\"W_a\n|\x85\x85a*\xADV[a\n\x87\x90`\x01a*\xC0V[a\n\x91\x90\x87a*\xE7V[a\n\x9B\x90\x86a*\xC0V[\x90P\x82\x15a\x0B\x1BW_a\n\xAD\x87a\x184V[\x90P_a\n\xB9\x83a\x184V[\x90P_\x82\x82`@Q` \x01a\n\xCF\x92\x91\x90a+\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x7FL\xB9\xC14Q\x97P\xE0\x94P\xA0\xC3\x8D\x14o\xED.\xCE\xD0-\xD6Q\xEF2\xC9\xE3H\x8F\xAAU\xC6\xF6\x81`@Qa\x0B\x0F\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1PPP[\x90Pa\x0B%V[P\x83[\x94\x93PPPPV[\x81\x83\x12a\nKW_a\x0B>\x84a\x18\xE6V[\x90P_a\t\x91\x84a\x18\xE6V[_\x82\x84\x13\x15a\x0B\xD9W\x82\x82\x15a\x0B\xD2W_a\x0Bd\x86a\x18\xE6V[\x90P_a\x0Bp\x83a\x18\xE6V[\x90P_\x82\x82`@Q` \x01a\x0B\x86\x92\x91\x90a+\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x7FL\xB9\xC14Q\x97P\xE0\x94P\xA0\xC3\x8D\x14o\xED.\xCE\xD0-\xD6Q\xEF2\xC9\xE3H\x8F\xAAU\xC6\xF6\x81`@Qa\x0B\xC6\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1PPP[\x90Pa\x0B\xDCV[P\x82[\x93\x92PPPV[_\x83\x85\x12\x80a\x0B\xF1WP\x82\x85\x13[\x15a\x0B\"W_a\x0C\x01\x85\x85a+TV[a\x0C\x0C\x90`\x01a+sV[\x90P_\x81a\x0C\x1A\x87\x89a+TV[a\x0C$\x91\x90a+\x9AV[\x90P_\x81\x12\x15a\x0C;Wa\x0C8\x82\x82a+sV[\x90P[_a\x0CF\x82\x88a+sV[\x90P\x84\x15a\x0C\xC6W_a\x0CX\x89a\x18\xE6V[\x90P_a\x0Cd\x83a\x18\xE6V[\x90P_\x82\x82`@Q` \x01a\x0Cz\x92\x91\x90a+\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x7FL\xB9\xC14Q\x97P\xE0\x94P\xA0\xC3\x8D\x14o\xED.\xCE\xD0-\xD6Q\xEF2\xC9\xE3H\x8F\xAAU\xC6\xF6\x81`@Qa\x0C\xBA\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1PPP[\x92Pa\x0B%\x91PPV[_a\x0C\xDB\x83\x83a\x19?V[\x90Pa\nK\x81\x15\x83a\x14\xEFV[\x81\x83\x14a\nKW_a\x0C\xF9\x84a\x18\xE6V[\x90P_a\r\x05\x84a\x18\xE6V[\x90P_a\r.\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a!=`\xF0\x1B\x81RP\x87a\x18wV[\x90P\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x81`@Qa\t\xEB\x91\x90a*\x87V[_\x82\x84\x11a\r\xB5Wa\r\x8B\x83_\x19`@Q\x80`\x80\x01`@R\x80`Z\x81R` \x01a0-`Z\x919a\x14/V[_a\r\x97\x84`\x01a*\xC0V[\x90P\x82\x15a\x0B\xD2W_a\r\xA9\x86a\x184V[\x90P_a\x0Bp\x83a\x184V[P\x82a\x0B\xDCV[__\x82\x12\x15a\r\xD3Wa\r\xCE\x82a+\xADV[a\r\xD5V[\x81[\x92\x91PPV[\x81\x83\x13\x15a\nKW_a\r\xED\x84a\x18\xE6V[\x90P_a\r\xF9\x84a\x18\xE6V[\x90P_a\x0E!\x83\x83`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x1F`\xF9\x1B\x81RP\x87a\x18wV[\x90P\x7Fb\xBD\xDA\x9A\x05\xCD\xBC\xDB\xF9\x05\xCB\xAD\x99\xC6\xEB\xDC\t\x8Bo\t3\xD8\xF2\xEB<\xFA\xB7@\x0B`%\x14\x81`@Qa\t\xEB\x91\x90a*\x87V[a\nZ\x82\x82a\x19\x9FV[a\nZ\x82\x82a\x19\xD0V[a\nZ\x82\x82a\x1A\x01V[_\x82\x84\x13a\x0B\xD9W\x82\x82\x15a\x0B\xD2W_a\x0Bd\x86a\x18\xE6V[_\x82\x84\x11a\x0B\xD9W\x82\x82\x15a\x0B\xD2W_a\r\xA9\x86a\x184V[_\x82\x84\x12a\x0B\xD9W_a\x0E\xB6`\x01\x85a+TV[\x90P\x82\x15a\x0B\xD2W_a\x0Bd\x86a\x18\xE6V[a\nZ\x82\x82a\x1A2V[\x81\x83\x12\x15a\nKW_a\x0E\xE4\x84a\x18\xE6V[\x90P_a\x0E\xF0\x84a\x18\xE6V[\x90P_a\x0F\x18\x83\x83`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x0F`\xFA\x1B\x81RP\x87a\x18wV[\x90P\x7F\x94BN\xD2O\xB3\x968\xB6H\x17\xC77\xDDD?8z\xAA\x14\x86aM\xA4I\xB6hjd-ml\x81`@Qa\t\xEB\x91\x90a*\x87V[a\nZ\x82\x82a\x1AcV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14a\nKW_a\x0Fv\x84a\x1A\x94V[\x90P_a\r\x05\x84a\x1A\x94V[a\x0F\x8B\x81a\x1B\xCDV[PV[_a\x0F\x99\x83\x83a\x19?V[\x90Pa\nK\x81\x83a\x14\xEFV[a\nZ\x82\x82a\x1C\x07V[_\x81\x83\x12\x15a\x0F\xC7Wa\x0F\xC2\x83\x83a+TV[a\x0B\xDCV[a\x0B\xDC\x82\x84a+TV[\x81\x83\x11a\nKW_a\x0F\xE2\x84a\x184V[\x90P_a\x0F\xEE\x84a\x184V[\x90P_a\x10\x17\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a<=`\xF0\x1B\x81RP\x87a\x18wV[\x90P\x7Fp{\x8CV\xE4\xC2\x11\xCF\x13!\xFA\xEBAH#pb\"\x8D\xB2\xFC\xEC\xC9\xBEH~\x83\xA2h\x0E~P\x81`@Qa\t\xEB\x91\x90a*\x87V[_`\x01\x83Qa\x10W\x91\x90a*\xADV[\x90P[\x80\x15a\nKW_\x81\x84Qa\x10n\x91\x90a*\xADV[a\x10x\x90\x84a*\xE7V[\x90P_\x84\x83\x81Q\x81\x10a\x10\x8DWa\x10\x8Da+\xC7V[` \x02` \x01\x01Q\x90P_\x85\x83\x81Q\x81\x10a\x10\xAAWa\x10\xAAa+\xC7V[` \x02` \x01\x01Q\x90P\x80\x86\x85\x81Q\x81\x10a\x10\xC7Wa\x10\xC7a+\xC7V[` \x02` \x01\x01\x81\x81RPP\x81\x86\x84\x81Q\x81\x10a\x10\xE6Wa\x10\xE6a+\xC7V[` \x02` \x01\x01\x81\x81RPPPPP\x80\x80a\x11\0\x90a+\xDBV[\x91PPa\x10ZV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x81\x16\x90\x83\x16\x14a\nKW`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19\x85\x81\x16` \x83\x01R\x82Q`\x04\x81\x84\x03\x01\x81R`$\x83\x01\x84R\x90\x85\x16`D\x83\x01R\x82Q`(\x81\x84\x03\x01\x81R`H\x90\x92\x01\x90\x92R_a\x11e\x83a\x1C8V[\x90P_a\x11q\x83a\x1C8V[\x90P_a\x11\x9A\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a!=`\xF0\x1B\x81RP\x89a\x18wV[\x90P\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x81`@Qa\x11\xCB\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1_\x80T`@\x80Qc\t\xCC\xAE\xFB`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c\t\xCC\xAE\xFB\x92`\x04\x80\x84\x01\x93\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x12\x11W__\xFD[PZ\xFA\x15\x80\x15a\x12#W=__>=_\xFD[PPPPPPPPPPPPV[_\x81\x83\x11a\x12?W\x81a\x0B\xDCV[P\x90\x91\x90PV[_a\x12Q\x86\x86a\x19?V[\x80a\x12aWPa\x12a\x86\x85a\x19?V[\x80a\x12qWPa\x12q\x86\x84a\x19?V[\x80a\x12\x81WPa\x12\x81\x86\x83a\x19?V[\x90P_\x85\x85\x85\x85`@Q` \x01a\x12\x9B\x94\x93\x92\x91\x90a+\xF0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x12\xB6\x82\x82a\x14\xEFV[PPPPPPPV[_\x81\x83\x10a\x12?W\x81a\x0B\xDCV[_a\x0B%\x84\x84\x84`\x01a\x0B\xE3V[_a\x0B\xDC\x83\x83`\x01a\x0EpV[_a\x12\xF3\x85\x85a\x19?V[\x80a\x13\x03WPa\x13\x03\x85\x84a\x19?V[\x80a\x13\x13WPa\x13\x13\x85\x83a\x19?V[\x90P_\x84\x84\x84`@Q` \x01a\x13+\x93\x92\x91\x90a,OV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x13F\x82\x82a\x14\xEFV[PPPPPPV[_\x81\x83\x13a\x12?W\x81a\x0B\xDCV[\x81\x83\x10\x15a\nKW_a\x13n\x84a\x184V[\x90P_a\x0E\xF0\x84a\x184V[_a\x13\x85\x84\x84a\x19?V[\x80a\x13\x95WPa\x13\x95\x84\x83a\x19?V[\x90P_\x83\x83`@Q` \x01a\x13\xAB\x92\x91\x90a,\x94V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x13\xC6\x82\x82a\x14\xEFV[PPPPPV[_\x80[\x83Q\x81\x10\x15a\x14\x1EW\x83\x81\x81Q\x81\x10a\x13\xEBWa\x13\xEBa+\xC7V[` \x02` \x01\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x85`\x01`\x01`\xE0\x1B\x03\x19\x16\x03a\x14\x16W`\x01\x91Pa\x14\x1EV[`\x01\x01a\x13\xD0V[Pa\x14)\x81\x83a\x14\xEFV[PPPPV[\x81\x83\x03a\nKW_a\x14@\x84a\x184V[\x90P_a\x14L\x84a\x184V[\x90P_a\x14u\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a==`\xF0\x1B\x81RP\x87a\x18wV[\x90P\x7F\xFFM\xE0\x80\xD28O#>\x1AO\xB3&\x8E\x8Fq]\x88\x02!z\xD28\"5\x90\n'M\xDEv\xDD\x81`@Qa\t\xEB\x91\x90a*\x87V[_\x81\x83\x10\x15a\x14\xB9Wa\x0F\xC2\x83\x83a*\xADV[a\x0B\xDC\x82\x84a*\xADV[a\nZ\x82\x82a\x1D\xF8V[_\x82\x84\x13a\r\xB5W_a\x0E\xB6\x84`\x01a+sV[_a\x0B%\x84\x84\x84`\x01a\n^V[\x81a\nZW\x7F\xEB\x03\xCA\x8C\x87\xC7\x84\x9B\xEF\x8FT\xCF\xDD,k\x96{'4\xFE\x87/u\x19x\xC3K\xB9\x1E\x13\xD3Q\x81`@Qa\x15#\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1_\x80T`@\x80Qc\t\xCC\xAE\xFB`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c\t\xCC\xAE\xFB\x92`\x04\x80\x84\x01\x93\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x15iW__\xFD[PZ\xFA\x15\x80\x15a\x13FW=__>=_\xFD[a\nZ\x82\x82a\x1E)V[_a\x0B\xDC\x83\x83`\x01a\x0BJV[a\x0F\x8B\x81a\x1EZV[_\x80\x80[\x84Q\x81\x10\x15a\x15\xF0W\x84\x81\x81Q\x81\x10a\x15\xBAWa\x15\xBAa+\xC7V[` \x02` \x01\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86`\x01`\x01`\xE0\x1B\x03\x19\x16\x03a\x15\xE8W`\x01\x92P\x80\x91Pa\x15\xF0V[`\x01\x01a\x15\x9FV[Pa\x13\xC6\x82\x84\x83\x81Q\x81\x10a\x16\x07Wa\x16\x07a+\xC7V[` \x02` \x01\x01Qa\x14\xEFV[_\x82\x84\x10a\x0B\xD9Wa\x16?\x83_`@Q\x80`\x80\x01`@R\x80`Q\x81R` \x01a/\xDC`Q\x919a\x14/V[_a\r\x97\x84\x86a*\xE7V[a\nZ\x82\x82a\x1E\x89V[\x81\x83\x11\x15a\nKW_a\x16f\x84a\x184V[\x90P_a\r\xF9\x84a\x184V[\x81\x83\x14a\nKW_a\x16\x83\x84a\x184V[\x90P_a\r\x05\x84a\x184V[\x81\x83\x03a\nKW_a\x16\xA0\x84a\x18\xE6V[\x90P_a\x14L\x84a\x18\xE6V[a\nZ\x82\x82a\x1E\xBAV[_a\x0B\xDC\x83\x83`\x01a\x17\xCFV[\x81\x83\x13a\nKW_a\x16\xD4\x84a\x18\xE6V[\x90P_a\x0F\xEE\x84a\x18\xE6V[a\x16\xE8a\x1E\xEBV[V[\x81\x15\x15\x83\x15\x15\x14a\nKW_\x83a\x17\x1EW`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dfalse`\xD8\x1B\x81RPa\x17<V[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01ctrue`\xE0\x1B\x81RP[\x90P_\x83a\x17gW`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dfalse`\xD8\x1B\x81RPa\r\x05V[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01ctrue`\xE0\x1B\x81RP\x90P_a\r.\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a!=`\xF0\x1B\x81RP\x87a\x18wV[__\x82`\x0F\x0B\x12\x15a\r\xD3Wa\r\xCE\x82a,\xB6V[_a\x0B\xDC\x83\x83`\x01a\x0E\x89V[_\x82\x84\x11\x15a\x0B\xD9W_a\x17\xE4\x84`\x01a*\xC0V[a\r\x97\x90\x86a*\xE7V[_a\x0B\xDC\x83\x83`\x01a\x0E\xA2V[_a\x0B\xDC\x83\x83`\x01a\x14\xCDV[a\nZ\x82\x82a\x1F\x15V[_a\x0B\xDC\x83\x83a\x18\x12V[a\nZ\x82\x82a\x1FFV[_a\x0B\xDC\x83\x83`\x01a\r_V[```\xA0`@Q\x01\x80`@R` \x81\x03\x91PP_\x81R\x80\x82[`\x01\x83\x03\x92P`\n\x81\x06`0\x01\x83S`\n\x90\x04\x80a\x18MWP\x81\x90\x03`\x1F\x19\x90\x91\x01\x90\x81R\x91\x90PV[``\x84\x83\x85\x84`@Q` \x01a\x18\x90\x94\x93\x92\x91\x90a,\xE3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x94\x93PPPPV[\x7F\xF9\xA6\xEE8\xCDn\x03G\xCB\x8Cjt\xFFjc\xEEw\xC6&\xBApFj\xAD\xFA\x86\xF9'/\xA3\xB8\x98\x82\x82`@Qa\x18\xDA\x92\x91\x90a-/V[`@Q\x80\x91\x03\x90\xA1PPV[``__\x83\x12\x15a\x18\xFFWa\x18\xFA\x83a+\xADV[a\x19\x01V[\x82[\x90Pa\x19\x0C\x81a\x184V[\x91P_\x83\x12\x15a\x199W\x81`@Q` \x01a\x19'\x91\x90a-RV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P[P\x91\x90PV[_\x81`@Q` \x01a\x19Q\x91\x90a-gV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x19p\x84a\x1FwV[`@Q` \x01a\x19\x80\x91\x90a-gV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14\x90P\x92\x91PPV[\x7F\x02\xD95)\xBB\xA9\xD1A\xE5\xE0g3\xC5,~o\xBC\xB1\x14\x95\x86\xAD\xB5\xC2@d\xB5\"\xAB&\xF1\xD7\x82\x82`@Qa\x18\xDA\x92\x91\x90a-rV[\x7F\x9B\xAAl\xC9oN\xF5tp\xCB\x9A\xF4\x15a\xFD9\x03/w\xCAJ\x06y0\x80{\xB7\xD0\xD38l\xBF\x82\x82`@Qa\x18\xDA\x92\x91\x90a-rV[\x7F\xE8@z\x02\t\xFA\x99\xEC:r(\xAF\xF1@\xC3\xD3\xE6\x8B\xD2y9\x179\xC7\xE0\xB6\\\xD4\x06\xCC\x93\xB5\x82\x82`@Qa\x18\xDA\x92\x91\x90a-\x93V[\x7Fb\xDD\xFF\xE5\xB5\x10\x83\x85\xF7\xA5\x90\xF1\0\xE1\xEEAJ\xD9U\x1A1\xF0\x89\xE6N\x82\x99\x84@x^\x1E\x82\x82`@Qa\x18\xDA\x92\x91\x90a-\xB7V[\x7Fj\x83\x7F\xF3\x97:\xA4\x18\x1E{\x9F\x07uo\x8B~\xE3f\xDD\x85\xA3fU\xD2\xCBB\xCDG\xF1\x0B\x968\x82\x82`@Qa\x18\xDA\x92\x91\x90a-rV[`@\x80Q`(\x80\x82R``\x82\x81\x01\x90\x93R_\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P_[`\x14\x81\x10\x15a\x1B\xC6W_a\x1A\xCE\x82`\x13a*\xADV[a\x1A\xD9\x90`\x08a-\xE0V[a\x1A\xE4\x90`\x02a.\xDAV[a\x1A\xF7\x90`\x01`\x01`\xA0\x1B\x03\x87\x16a.\xE5V[`\xF8\x1B\x90P_`\x10\x82`\xF8\x1Ca\x1B\r\x91\x90a.\xF8V[`\xF8\x1B\x90P_\x81`\xF8\x1C`\x10a\x1B#\x91\x90a/\x19V[\x83`\xF8\x1Ca\x1B1\x91\x90a/5V[`\xF8\x1B\x90Pa\x1B?\x82a!QV[\x85a\x1BK\x86`\x02a-\xE0V[\x81Q\x81\x10a\x1B[Wa\x1B[a+\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPa\x1Bz\x81a!QV[\x85a\x1B\x86\x86`\x02a-\xE0V[a\x1B\x91\x90`\x01a*\xC0V[\x81Q\x81\x10a\x1B\xA1Wa\x1B\xA1a+\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPP`\x01\x90\x92\x01\x91Pa\x1A\xB9\x90PV[P\x92\x91PPV[\x7F\xCF4\xEFSz\xC3>\xE1\xACbl\xA1Xz\n~\x8EQV\x1EU\x14\xF8\xCB6\xAF\xA1\xC5\x10+;\xAB\x81`@Qa\x1B\xFC\x91\x90a*\x87V[`@Q\x80\x91\x03\x90\xA1PV[\x7F'\xA8v~\xD1r\xB4\x8D|\xF6\xDB\xB2](@\xC5\xFA\x91\xEFx\xFAn\x05\xC1<\x82v=\t\x9F\x03\xCA\x82\x82`@Qa\x18\xDA\x92\x91\x90a-\x93V[``_\x82Q`\x02a\x1CI\x91\x90a-\xE0V[a\x1CT\x90`\x02a*\xC0V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1CkWa\x1Cka!\x8BV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1C\x95W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81_\x81Q\x81\x10a\x1C\xAFWa\x1C\xAFa+\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x1C\xDDWa\x1C\xDDa+\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP_[\x83Q\x81\x10\x15a\x1B\xC6W_\x84\x82\x81Q\x81\x10a\x1D\x11Wa\x1D\x11a+\xC7V[\x01` \x01Q`\xF8\x81\x90\x1C\x91Po\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x90`\xFC\x1C`\x10\x81\x10a\x1DEWa\x1DEa+\xC7V[\x1A`\xF8\x1B\x83a\x1DU\x84`\x02a-\xE0V[a\x1D`\x90`\x02a*\xC0V[\x81Q\x81\x10a\x1DpWa\x1Dpa+\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\x0F\x82\x16`\x10\x81\x10a\x1D\xAEWa\x1D\xAEa+\xC7V[\x1A`\xF8\x1B\x83a\x1D\xBE\x84`\x02a-\xE0V[a\x1D\xC9\x90`\x03a*\xC0V[\x81Q\x81\x10a\x1D\xD9Wa\x1D\xD9a+\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPP`\x01\x01a\x1C\xF5V[\x7F\xF1i'kB\x1EmkD0\xD1\x93K\xD1L\xC5Cc\x05\xAD\x11\xE2\xBA\xA6q@\x84K\xB1;~~\x82\x82`@Qa\x18\xDA\x92\x91\x90a-\x93V[\x7FN\xF9fJF\xFF\xE05q*\xA3aV\x17\xC7\xB5\xAAx#&\xD5\x8E\x89I\xDBK\xEEH\x18 A\x93\x82\x82`@Qa\x18\xDA\x92\x91\x90a-rV[\x7F\xB4&\x04\xCB\x10Z\x16\xC8\xF6\xDB\x8AA\xE6\xB0\x0C\x0C\x1BH&F^\x8B\xC5\x04\xB3\xEB>\x88\xB3\xE6\xA4\xA0\x81`@Qa\x1B\xFC\x91\x90a*\x87V[\x7F\x94\x12\x96\xA3\x9E\xA1\x07\xBD\xE6\x85R#\x18\xA4\xB6\xC2\xB5D\x90J]\xD8*Q'H\xCA,\xF89\xBE\xF7\x82\x82`@Qa\x18\xDA\x92\x91\x90a-rV[\x7FL4\xC2\xF9\xA7\x862\xF2\x9F\xA5\x9A\xAE\xD5QL\xB7B\xFD\x9F\xBC\xFD|\xCC,\x03\xC8_+\xBCb\x1CG\x82\x82`@Qa\x18\xDA\x92\x91\x90a-/V[`@Q\x7F\xF9-J\x94\xD1\xD5\x01F\x96\xDC\xFCe\xA0\xA0a\xAF\x97`\x8E\xEB\xD7\xFE\xA0Q\x9F\xF4\xFD\xBC\xA7\x1B\xAE\x9F\x90_\x90\xA1V[\x7F\xE5#+\xA1qT\xFD\xF2\xAE\x11\x84j\xF0\x9E\x8B\xD8B\n\xA2\x105\x02j\xC8\x8D\xB6\xCF\xB8\xC7\xB4\x90\x12\x82\x82`@Qa\x18\xDA\x92\x91\x90a-\xB7V[\x7F\xAD\xC8\x168\xCF\xE9\xDC/.NOu\x9B&\xF2\0j\x87\xEBa-e\xBC\xD2\x82\x8Fr\xB1L\x05\x84D\x82\x82`@Qa\x18\xDA\x92\x91\x90a-\x93V[``\x81Q`$\x03a \xF4W`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81RmPanic(uint256)`\x90\x1B` \x90\x91\x01R\x7FNH{qS\x9E\x01d\xC9\xD2\x95\x06\xCCr^I4+\xCA\xC1^\t'(+\xF3\x0F\xED\xFE\x1Crh_[`\x04\x81\x10\x15a TW\x81\x81`\x04\x81\x10a\x1F\xE6Wa\x1F\xE6a+\xC7V[\x1A`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x84\x82\x81Q\x81\x10a \x06Wa \x06a+\xC7V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x14a LWPP`@\x80Q\x80\x82\x01\x90\x91R`\x13\x81RrUndefined signature`h\x1B` \x82\x01R\x92\x91PPV[`\x01\x01a\x1F\xCBV[P_`\x04[`$\x81\x10\x15a \x90W`\x08\x82\x90\x1B\x91P\x84\x81\x81Q\x81\x10a {Wa {a+\xC7V[\x01` \x01Q`\xF8\x1C\x91\x90\x91\x17\x90`\x01\x01a YV[P\x80`\x11\x03a \xC1WPP`@\x80Q\x80\x82\x01\x90\x91R`\t\x81RhPanic(17)`\xB8\x1B` \x82\x01R\x92\x91PPV[PP`@\x80Q\x80\x82\x01\x90\x91R`\x14\x81RsUndefined panic code``\x1B` \x82\x01R\x92\x91PPV[`D\x82Q\x10\x15a!7WPP`@\x80Q\x80\x82\x01\x90\x91R`\x1D\x81R\x7FTransaction reverted silently\0\0\0` \x82\x01R\x90V[`\x04\x82\x01\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\r\xD5\x91\x90a/NV[_`\n`\xF8\x83\x90\x1C\x10\x15a!wWa!n`\xF8\x83\x90\x1C`0a/\xC2V[`\xF8\x1B\x92\x91PPV[a!n`\xF8\x83\x90\x1C`Wa/\xC2V[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a!\xC7Wa!\xC7a!\x8BV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a!\xE7Wa!\xE7a!\x8BV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a\"\x04W__\xFD[\x815` \x83\x01_a\"\x1Ca\"\x17\x84a!\xCFV[a!\x9FV[\x90P\x82\x81R\x85\x83\x83\x01\x11\x15a\"/W__\xFD[\x82\x82` \x83\x017_\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\"[W__\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"~W__\xFD[a\"\x8A\x86\x82\x87\x01a!\xF5V[\x91PP\x92P\x92P\x92V[\x805\x80\x15\x15\x81\x14a!\x86W__\xFD[__`@\x83\x85\x03\x12\x15a\"\xB4W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xC9W__\xFD[a\"\xD5\x85\x82\x86\x01a!\xF5V[\x92PPa\"\xE4` \x84\x01a\"\x94V[\x90P\x92P\x92\x90PV[____`\x80\x85\x87\x03\x12\x15a#\0W__\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa#\x1E``\x86\x01a\"\x94V[\x90P\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15a#;W__\xFD[\x835\x92P` \x84\x015\x91Pa#R`@\x85\x01a\"\x94V[\x90P\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a#lW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a#\x81W__\xFD[a#\x8D\x85\x82\x86\x01a!\xF5V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xA8W__\xFD[a#\xB4\x85\x82\x86\x01a!\xF5V[\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a#\xCEW__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a#\xE6W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xFBW__\xFD[a$\x07\x85\x82\x86\x01a!\xF5V[\x95` \x94\x90\x94\x015\x94PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\x86W__\xFD[__`@\x83\x85\x03\x12\x15a$=W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a$RW__\xFD[a$^\x85\x82\x86\x01a!\xF5V[\x92PPa\"\xE4` \x84\x01a$\x16V[___``\x84\x86\x03\x12\x15a$\x7FW__\xFD[a$\x88\x84a$\x16V[\x92Pa$\x96` \x85\x01a$\x16V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"~W__\xFD[_` \x82\x84\x03\x12\x15a$\xC0W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xD5W__\xFD[a\x0B%\x84\x82\x85\x01a!\xF5V[__`@\x83\x85\x03\x12\x15a$\xF2W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_` \x82\x84\x03\x12\x15a%\x11W__\xFD[a\x0B\xDC\x82a$\x16V[_`\x01`\x01`@\x1B\x03\x82\x11\x15a%2Wa%2a!\x8BV[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15a%MW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a%bW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a%rW__\xFD[\x805a%\x80a\"\x17\x82a%\x1AV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a%\xA1W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a%\xC3W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a%\xA8V[\x97` \x96\x90\x96\x015\x96PPPPPPV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a!\x86W__\xFD[___``\x84\x86\x03\x12\x15a%\xFDW__\xFD[a&\x06\x84a%\xD4V[\x92Pa$\x96` \x85\x01a%\xD4V[_____`\xA0\x86\x88\x03\x12\x15a&(W__\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15a&=W__\xFD[a&I\x88\x82\x89\x01a!\xF5V[\x95PP` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&dW__\xFD[a&p\x88\x82\x89\x01a!\xF5V[\x94PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&\x8BW__\xFD[a&\x97\x88\x82\x89\x01a!\xF5V[\x93PP``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xB2W__\xFD[a&\xBE\x88\x82\x89\x01a!\xF5V[\x92PP`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xD9W__\xFD[a&\xE5\x88\x82\x89\x01a!\xF5V[\x91PP\x92\x95P\x92\x95\x90\x93PV[___``\x84\x86\x03\x12\x15a'\x04W__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a'.W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a'CW__\xFD[a'O\x87\x82\x88\x01a!\xF5V[\x94PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a'jW__\xFD[a'v\x87\x82\x88\x01a!\xF5V[\x93PP`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a'\x91W__\xFD[a'\x9D\x87\x82\x88\x01a!\xF5V[\x92PP``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xB8W__\xFD[a'\xC4\x87\x82\x88\x01a!\xF5V[\x91PP\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15a'\xE2W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xF7W__\xFD[a(\x03\x86\x82\x87\x01a!\xF5V[\x93PP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\x1EW__\xFD[a(*\x86\x82\x87\x01a!\xF5V[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"~W__\xFD[_\x82`\x1F\x83\x01\x12a(TW__\xFD[\x815a(ba\"\x17\x82a%\x1AV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a(\x83W__\xFD[` \x85\x01[\x83\x81\x10\x15a(\xA7Wa(\x99\x81a%\xD4V[\x83R` \x92\x83\x01\x92\x01a(\x88V[P\x95\x94PPPPPV[___``\x84\x86\x03\x12\x15a(\xC3W__\xFD[a(\xCC\x84a%\xD4V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xE6W__\xFD[a(*\x86\x82\x87\x01a(EV[__`@\x83\x85\x03\x12\x15a)\x03W__\xFD[a)\x0C\x83a\"\x94V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xA8W__\xFD[___``\x84\x86\x03\x12\x15a)8W__\xFD[a)A\x84a%\xD4V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)[W__\xFD[a)g\x86\x82\x87\x01a(EV[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)\x82W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a)\x92W__\xFD[\x805a)\xA0a\"\x17\x82a%\x1AV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a)\xC1W__\xFD[` \x84\x01[\x83\x81\x10\x15a*\x01W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xE3W__\xFD[a)\xF2\x8B` \x83\x89\x01\x01a!\xF5V[\x84RP` \x92\x83\x01\x92\x01a)\xC6V[P\x80\x94PPPPP\x92P\x92P\x92V[___``\x84\x86\x03\x12\x15a*\"W__\xFD[a*+\x84a\"\x94V[\x92Pa$\x96` \x85\x01a\"\x94V[_` \x82\x84\x03\x12\x15a*IW__\xFD[\x815\x80`\x0F\x0B\x81\x14a\x0B\xDCW__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0B\xDC` \x83\x01\x84a*YV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\r\xD5Wa\r\xD5a*\x99V[\x80\x82\x01\x80\x82\x11\x15a\r\xD5Wa\r\xD5a*\x99V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a*\xF5Wa*\xF5a*\xD3V[P\x06\x90V[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[n\x02\x1Bc\x0Bk\x83Ks9\x03\xB3\x0Bc\xAB)`\x8D\x1B\x81R_a+4`\x0F\x83\x01\x85a*\xFAV[c\x01\x03\xA3y`\xE5\x1B\x81Ra+K`\x04\x82\x01\x85a*\xFAV[\x95\x94PPPPPV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1B\xC6Wa\x1B\xC6a*\x99V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a+\x92Wa+\x92a*\x99V[PP\x92\x91PPV[_\x82a+\xA8Wa+\xA8a*\xD3V[P\x07\x90V[_`\x01`\xFF\x1B\x82\x01a+\xC1Wa+\xC1a*\x99V[P_\x03\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x81a+\xE9Wa+\xE9a*\x99V[P_\x19\x01\x90V[_a+\xFB\x82\x87a*\xFAV[c\x01\x02z\x91`\xE5\x1B\x81Ra,\x12`\x04\x82\x01\x87a*\xFAV[\x90Pc\x01\x02z\x91`\xE5\x1B\x81Ra,+`\x04\x82\x01\x86a*\xFAV[\x90Pc\x01\x02z\x91`\xE5\x1B\x81Ra,D`\x04\x82\x01\x85a*\xFAV[\x97\x96PPPPPPPV[_a,Z\x82\x86a*\xFAV[c\x01\x02z\x91`\xE5\x1B\x81Ra,q`\x04\x82\x01\x86a*\xFAV[\x90Pc\x01\x02z\x91`\xE5\x1B\x81Ra,\x8A`\x04\x82\x01\x85a*\xFAV[\x96\x95PPPPPPV[_a,\x9F\x82\x85a*\xFAV[c\x01\x02z\x91`\xE5\x1B\x81Ra+K`\x04\x82\x01\x85a*\xFAV[_\x81`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03a,\xDBWa,\xDBa*\x99V[_\x03\x92\x91PPV[h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x81R_a-\x12a-\x0Ca-\x06`\t\x85\x01\x89a*\xFAV[\x87a*\xFAV[\x85a*\xFAV[i\x01a\x03\x93+\x0B\x9B{q\xD1`\xB5\x1B\x81Ra,D`\n\x82\x01\x85a*\xFAV[`@\x81R_a-A`@\x83\x01\x85a*YV[\x90P\x82\x15\x15` \x83\x01R\x93\x92PPPV[`-`\xF8\x1B\x81R_a\x0B\xDC`\x01\x83\x01\x84a*\xFAV[_a\x0B\xDC\x82\x84a*\xFAV[`@\x81R_a-\x84`@\x83\x01\x85a*YV[\x90P\x82` \x83\x01R\x93\x92PPPV[`@\x81R_a-\xA5`@\x83\x01\x85a*YV[\x82\x81\x03` \x84\x01Ra+K\x81\x85a*YV[`@\x81R_a-\xC9`@\x83\x01\x85a*YV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\r\xD5Wa\r\xD5a*\x99V[`\x01\x81[`\x01\x84\x11\x15a.2W\x80\x85\x04\x81\x11\x15a.\x16Wa.\x16a*\x99V[`\x01\x84\x16\x15a.$W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a-\xFBV[\x93P\x93\x91PPV[_\x82a.HWP`\x01a\r\xD5V[\x81a.TWP_a\r\xD5V[\x81`\x01\x81\x14a.jW`\x02\x81\x14a.tWa.\x90V[`\x01\x91PPa\r\xD5V[`\xFF\x84\x11\x15a.\x85Wa.\x85a*\x99V[PP`\x01\x82\x1Ba\r\xD5V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a.\xB3WP\x81\x81\na\r\xD5V[a.\xBF_\x19\x84\x84a-\xF7V[\x80_\x19\x04\x82\x11\x15a.\xD2Wa.\xD2a*\x99V[\x02\x93\x92PPPV[_a\x0B\xDC\x83\x83a.:V[_\x82a.\xF3Wa.\xF3a*\xD3V[P\x04\x90V[_`\xFF\x83\x16\x80a/\nWa/\na*\xD3V[\x80`\xFF\x84\x16\x04\x91PP\x92\x91PPV[`\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x1B\xC6Wa\x1B\xC6a*\x99V[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\r\xD5Wa\r\xD5a*\x99V[_` \x82\x84\x03\x12\x15a/^W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a/sW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a/\x83W__\xFD[\x80Qa/\x91a\"\x17\x82a!\xCFV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a/\xA5W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\r\xD5Wa\r\xD5a*\x99V\xFEclampLt cannot clamp value a to be less than zero. Check your inputs/assumptions.clampGt cannot clamp value a to be larger than uint256.max. Check your inputs/assumptions.\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static FUZZLIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Fuzzlib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Fuzzlib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Fuzzlib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Fuzzlib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Fuzzlib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Fuzzlib)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Fuzzlib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FUZZLIB_ABI.clone(),
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
                FUZZLIB_ABI.clone(),
                FUZZLIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `abs` (0x1b5ac4b5) function
        pub fn abs(
            &self,
            n: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([27, 90, 196, 181], n)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `abs` (0xe01e92d2) function
        pub fn abs_with_n(
            &self,
            n: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([224, 30, 146, 210], n)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assertRevertReasonEqual` (0x42068767) function
        pub fn assert_revert_reason_equal_0(
            &self,
            return_data: ::ethers::core::types::Bytes,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 6, 135, 103], (return_data, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assertRevertReasonEqual` (0x72de77b5) function
        pub fn assert_revert_reason_equal_3(
            &self,
            return_data: ::ethers::core::types::Bytes,
            reason_1: ::std::string::String,
            reason_2: ::std::string::String,
            reason_3: ::std::string::String,
            reason_4: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [114, 222, 119, 181],
                    (return_data, reason_1, reason_2, reason_3, reason_4),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assertRevertReasonEqual` (0x8010bad3) function
        pub fn assert_revert_reason_equal_2(
            &self,
            return_data: ::ethers::core::types::Bytes,
            reason_1: ::std::string::String,
            reason_2: ::std::string::String,
            reason_3: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [128, 16, 186, 211],
                    (return_data, reason_1, reason_2, reason_3),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assertRevertReasonEqual` (0x846213a1) function
        pub fn assert_revert_reason_equal_1(
            &self,
            return_data: ::ethers::core::types::Bytes,
            reason_1: ::std::string::String,
            reason_2: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 98, 19, 161], (return_data, reason_1, reason_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assertRevertReasonNotEqual` (0x1059bfc9) function
        pub fn assert_revert_reason_not_equal(
            &self,
            return_data: ::ethers::core::types::Bytes,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 89, 191, 201], (return_data, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clamp` (0x06cb0011) function
        pub fn clamp_2(
            &self,
            value: ::ethers::core::types::U256,
            low: ::ethers::core::types::U256,
            high: ::ethers::core::types::U256,
            enable_logs: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([6, 203, 0, 17], (value, low, high, enable_logs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clamp` (0x09fd3899) function
        pub fn clamp_3(
            &self,
            value: ::ethers::core::types::I256,
            low: ::ethers::core::types::I256,
            high: ::ethers::core::types::I256,
            enable_logs: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([9, 253, 56, 153], (value, low, high, enable_logs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clamp` (0x7b8d0f0c) function
        pub fn clamp_0(
            &self,
            value: ::ethers::core::types::I256,
            low: ::ethers::core::types::I256,
            high: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([123, 141, 15, 12], (value, low, high))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clamp` (0x9e422447) function
        pub fn clamp_1(
            &self,
            value: ::ethers::core::types::U256,
            low: ::ethers::core::types::U256,
            high: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([158, 66, 36, 71], (value, low, high))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampGt` (0x1b55d07c) function
        pub fn clamp_gt_2(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
            enable_logs: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([27, 85, 208, 124], (a, b, enable_logs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampGt` (0x953e5e80) function
        pub fn clamp_gt_3(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
            enable_logs: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([149, 62, 94, 128], (a, b, enable_logs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampGt` (0xf09ced97) function
        pub fn clamp_gt_0(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([240, 156, 237, 151], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampGt` (0xfb9894b2) function
        pub fn clamp_gt_1(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([251, 152, 148, 178], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampGte` (0x2d40851e) function
        pub fn clamp_gte_2(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
            enable_logs: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([45, 64, 133, 30], (a, b, enable_logs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampGte` (0x2e17d472) function
        pub fn clamp_gte_3(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
            enable_logs: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([46, 23, 212, 114], (a, b, enable_logs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampGte` (0x7e1ddd80) function
        pub fn clamp_gte_0(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([126, 29, 221, 128], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampGte` (0xe894090d) function
        pub fn clamp_gte_1(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([232, 148, 9, 13], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampLt` (0x2ffde603) function
        pub fn clamp_lt_2(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
            enable_logs: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([47, 253, 230, 3], (a, b, enable_logs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampLt` (0xb2f9799d) function
        pub fn clamp_lt_3(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
            enable_logs: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([178, 249, 121, 157], (a, b, enable_logs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampLt` (0xf01ff64c) function
        pub fn clamp_lt_0(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([240, 31, 246, 76], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampLt` (0xf5bc3a71) function
        pub fn clamp_lt_1(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([245, 188, 58, 113], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampLte` (0x084ec410) function
        pub fn clamp_lte_2(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
            enable_logs: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([8, 78, 196, 16], (a, b, enable_logs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampLte` (0xac0c207b) function
        pub fn clamp_lte_0(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([172, 12, 32, 123], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampLte` (0xc5df2a37) function
        pub fn clamp_lte_1(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([197, 223, 42, 55], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clampLte` (0xeef25389) function
        pub fn clamp_lte_3(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
            enable_logs: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([238, 242, 83, 137], (a, b, enable_logs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `diff` (0x568c1385) function
        pub fn diff(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([86, 140, 19, 133], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `diff` (0x8f5e514a) function
        pub fn diff_with_a_and_b(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([143, 94, 81, 74], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eq` (0x14ec1cc6) function
        pub fn eq_0(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 236, 28, 198], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eq` (0x3ccb5e26) function
        pub fn eq_1(
            &self,
            a: ::ethers::core::types::Address,
            b: ::ethers::core::types::Address,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 203, 94, 38], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eq` (0x6c9764b3) function
        pub fn eq_2(
            &self,
            a: [u8; 4],
            b: [u8; 4],
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 151, 100, 179], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eq` (0xbc8d43a8) function
        pub fn eq_3(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 141, 67, 168], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eq` (0xdf7cd77f) function
        pub fn eq_4(
            &self,
            a: bool,
            b: bool,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 124, 215, 127], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `errAllow` (0x88e362c6) function
        pub fn err_allow(
            &self,
            error_selector: [u8; 4],
            allowed_errors: ::std::vec::Vec<[u8; 4]>,
            message: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [136, 227, 98, 198],
                    (error_selector, allowed_errors, message),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `errsAllow` (0xb09c41b7) function
        pub fn errs_allow(
            &self,
            error_selector: [u8; 4],
            allowed_errors: ::std::vec::Vec<[u8; 4]>,
            messages: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [176, 156, 65, 183],
                    (error_selector, allowed_errors, messages),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gt` (0x5c2b80f5) function
        pub fn gt(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 43, 128, 245], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gt` (0xd2a50604) function
        pub fn gt_with_a_and_b(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 165, 6, 4], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gte` (0x3b6ddf03) function
        pub fn gte(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 109, 223, 3], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gte` (0x841ea11c) function
        pub fn gte_with_a_and_b(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 30, 161, 28], (a, b, reason))
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `lt` (0x01b9e827) function
        pub fn lt(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 185, 232, 39], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lt` (0x06f82342) function
        pub fn lt_with_a_and_b(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 248, 35, 66], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lte` (0x1ba0399b) function
        pub fn lte(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 160, 57, 155], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lte` (0xbb35031a) function
        pub fn lte_with_a_and_b(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 53, 3, 26], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `max` (0x6d5433e6) function
        pub fn max(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([109, 84, 51, 230], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `max` (0x81fe5786) function
        pub fn max_with_a_and_b(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([129, 254, 87, 134], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `min` (0x7ae2b5c7) function
        pub fn min(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([122, 226, 181, 199], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `neq` (0x8c4fb57d) function
        pub fn neq(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 79, 181, 125], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `neq` (0xc0e8def2) function
        pub fn neq_with_a_and_b(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 232, 222, 242], (a, b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `platform` (0x4bde38c8) function
        pub fn platform(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([75, 222, 56, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPlatform` (0x6945c5ea) function
        pub fn set_platform(
            &self,
            platform: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 69, 197, 234], platform)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shuffleArray` (0x6ace552c) function
        pub fn shuffle_array(
            &self,
            shuffle: ::std::vec::Vec<::ethers::core::types::U256>,
            entropy: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 206, 85, 44], (shuffle, entropy))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `t` (0xa15f9f07) function
        pub fn t(
            &self,
            b: bool,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 95, 159, 7], (b, reason))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AssertEqFail` event
        pub fn assert_eq_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertEqFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertFail` event
        pub fn assert_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertGtFail` event
        pub fn assert_gt_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertGtFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertGteFail` event
        pub fn assert_gte_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertGteFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertLtFail` event
        pub fn assert_lt_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertLtFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertLteFail` event
        pub fn assert_lte_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertLteFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertNeqFail` event
        pub fn assert_neq_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertNeqFailFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `Clamped` event
        pub fn clamped_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ClampedFilter> {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FuzzlibEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Fuzzlib<M> {
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
    #[ethevent(name = "AssertEqFail", abi = "AssertEqFail(string)")]
    pub struct AssertEqFailFilter(pub ::std::string::String);
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
    #[ethevent(name = "AssertFail", abi = "AssertFail(string)")]
    pub struct AssertFailFilter(pub ::std::string::String);
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
    #[ethevent(name = "AssertGtFail", abi = "AssertGtFail(string)")]
    pub struct AssertGtFailFilter(pub ::std::string::String);
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
    #[ethevent(name = "AssertGteFail", abi = "AssertGteFail(string)")]
    pub struct AssertGteFailFilter(pub ::std::string::String);
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
    #[ethevent(name = "AssertLtFail", abi = "AssertLtFail(string)")]
    pub struct AssertLtFailFilter(pub ::std::string::String);
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
    #[ethevent(name = "AssertLteFail", abi = "AssertLteFail(string)")]
    pub struct AssertLteFailFilter(pub ::std::string::String);
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
    #[ethevent(name = "AssertNeqFail", abi = "AssertNeqFail(string)")]
    pub struct AssertNeqFailFilter(pub ::std::string::String);
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
    #[ethevent(name = "Clamped", abi = "Clamped(string)")]
    pub struct ClampedFilter(pub ::std::string::String);
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
    pub enum FuzzlibEvents {
        AssertEqFailFilter(AssertEqFailFilter),
        AssertFailFilter(AssertFailFilter),
        AssertGtFailFilter(AssertGtFailFilter),
        AssertGteFailFilter(AssertGteFailFilter),
        AssertLtFailFilter(AssertLtFailFilter),
        AssertLteFailFilter(AssertLteFailFilter),
        AssertNeqFailFilter(AssertNeqFailFilter),
        AssertionFailed1Filter(AssertionFailed1Filter),
        AssertionFailed2Filter(AssertionFailed2Filter),
        AssertionFailed3Filter(AssertionFailed3Filter),
        AssertionFailed4Filter(AssertionFailed4Filter),
        AssertionFailed5Filter(AssertionFailed5Filter),
        AssertionFailed6Filter(AssertionFailed6Filter),
        AssertionFailed7Filter(AssertionFailed7Filter),
        AssertionFailed8Filter(AssertionFailed8Filter),
        ClampedFilter(ClampedFilter),
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogBoolFilter(LogBoolFilter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
    }
    impl ::ethers::contract::EthLogDecode for FuzzlibEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertEqFailFilter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertEqFailFilter(decoded));
            }
            if let Ok(decoded) = AssertFailFilter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGtFailFilter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertGtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGteFailFilter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertGteFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLtFailFilter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertLtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLteFailFilter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertLteFailFilter(decoded));
            }
            if let Ok(decoded) = AssertNeqFailFilter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertNeqFailFilter(decoded));
            }
            if let Ok(decoded) = AssertionFailed1Filter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertionFailed1Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed2Filter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertionFailed2Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed3Filter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertionFailed3Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed4Filter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertionFailed4Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed5Filter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertionFailed5Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed6Filter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertionFailed6Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed7Filter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertionFailed7Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed8Filter::decode_log(log) {
                return Ok(FuzzlibEvents::AssertionFailed8Filter(decoded));
            }
            if let Ok(decoded) = ClampedFilter::decode_log(log) {
                return Ok(FuzzlibEvents::ClampedFilter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(FuzzlibEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(FuzzlibEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogBoolFilter::decode_log(log) {
                return Ok(FuzzlibEvents::LogBoolFilter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(FuzzlibEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(FuzzlibEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(FuzzlibEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(FuzzlibEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(FuzzlibEvents::LogUintFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for FuzzlibEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssertEqFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertFailFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssertGtFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertGteFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertLtFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertLteFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertNeqFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::ClampedFilter(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<AssertEqFailFilter> for FuzzlibEvents {
        fn from(value: AssertEqFailFilter) -> Self {
            Self::AssertEqFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertFailFilter> for FuzzlibEvents {
        fn from(value: AssertFailFilter) -> Self {
            Self::AssertFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGtFailFilter> for FuzzlibEvents {
        fn from(value: AssertGtFailFilter) -> Self {
            Self::AssertGtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGteFailFilter> for FuzzlibEvents {
        fn from(value: AssertGteFailFilter) -> Self {
            Self::AssertGteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLtFailFilter> for FuzzlibEvents {
        fn from(value: AssertLtFailFilter) -> Self {
            Self::AssertLtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLteFailFilter> for FuzzlibEvents {
        fn from(value: AssertLteFailFilter) -> Self {
            Self::AssertLteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertNeqFailFilter> for FuzzlibEvents {
        fn from(value: AssertNeqFailFilter) -> Self {
            Self::AssertNeqFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed1Filter> for FuzzlibEvents {
        fn from(value: AssertionFailed1Filter) -> Self {
            Self::AssertionFailed1Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed2Filter> for FuzzlibEvents {
        fn from(value: AssertionFailed2Filter) -> Self {
            Self::AssertionFailed2Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed3Filter> for FuzzlibEvents {
        fn from(value: AssertionFailed3Filter) -> Self {
            Self::AssertionFailed3Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed4Filter> for FuzzlibEvents {
        fn from(value: AssertionFailed4Filter) -> Self {
            Self::AssertionFailed4Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed5Filter> for FuzzlibEvents {
        fn from(value: AssertionFailed5Filter) -> Self {
            Self::AssertionFailed5Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed6Filter> for FuzzlibEvents {
        fn from(value: AssertionFailed6Filter) -> Self {
            Self::AssertionFailed6Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed7Filter> for FuzzlibEvents {
        fn from(value: AssertionFailed7Filter) -> Self {
            Self::AssertionFailed7Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed8Filter> for FuzzlibEvents {
        fn from(value: AssertionFailed8Filter) -> Self {
            Self::AssertionFailed8Filter(value)
        }
    }
    impl ::core::convert::From<ClampedFilter> for FuzzlibEvents {
        fn from(value: ClampedFilter) -> Self {
            Self::ClampedFilter(value)
        }
    }
    impl ::core::convert::From<LogFilter> for FuzzlibEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for FuzzlibEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogBoolFilter> for FuzzlibEvents {
        fn from(value: LogBoolFilter) -> Self {
            Self::LogBoolFilter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for FuzzlibEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for FuzzlibEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for FuzzlibEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for FuzzlibEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for FuzzlibEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    ///Container type for all input parameters for the `abs` function with signature `abs(int256)` and selector `0x1b5ac4b5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "abs", abi = "abs(int256)")]
    pub struct AbsCall {
        pub n: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `abs` function with signature `abs(int128)` and selector `0xe01e92d2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "abs", abi = "abs(int128)")]
    pub struct AbsWithNCall {
        pub n: i128,
    }
    ///Container type for all input parameters for the `assertRevertReasonEqual` function with signature `assertRevertReasonEqual(bytes,string)` and selector `0x42068767`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "assertRevertReasonEqual",
        abi = "assertRevertReasonEqual(bytes,string)"
    )]
    pub struct AssertRevertReasonEqual0Call {
        pub return_data: ::ethers::core::types::Bytes,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `assertRevertReasonEqual` function with signature `assertRevertReasonEqual(bytes,string,string,string,string)` and selector `0x72de77b5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "assertRevertReasonEqual",
        abi = "assertRevertReasonEqual(bytes,string,string,string,string)"
    )]
    pub struct AssertRevertReasonEqual3Call {
        pub return_data: ::ethers::core::types::Bytes,
        pub reason_1: ::std::string::String,
        pub reason_2: ::std::string::String,
        pub reason_3: ::std::string::String,
        pub reason_4: ::std::string::String,
    }
    ///Container type for all input parameters for the `assertRevertReasonEqual` function with signature `assertRevertReasonEqual(bytes,string,string,string)` and selector `0x8010bad3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "assertRevertReasonEqual",
        abi = "assertRevertReasonEqual(bytes,string,string,string)"
    )]
    pub struct AssertRevertReasonEqual2Call {
        pub return_data: ::ethers::core::types::Bytes,
        pub reason_1: ::std::string::String,
        pub reason_2: ::std::string::String,
        pub reason_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `assertRevertReasonEqual` function with signature `assertRevertReasonEqual(bytes,string,string)` and selector `0x846213a1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "assertRevertReasonEqual",
        abi = "assertRevertReasonEqual(bytes,string,string)"
    )]
    pub struct AssertRevertReasonEqual1Call {
        pub return_data: ::ethers::core::types::Bytes,
        pub reason_1: ::std::string::String,
        pub reason_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `assertRevertReasonNotEqual` function with signature `assertRevertReasonNotEqual(bytes,string)` and selector `0x1059bfc9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "assertRevertReasonNotEqual",
        abi = "assertRevertReasonNotEqual(bytes,string)"
    )]
    pub struct AssertRevertReasonNotEqualCall {
        pub return_data: ::ethers::core::types::Bytes,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `clamp` function with signature `clamp(uint256,uint256,uint256,bool)` and selector `0x06cb0011`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clamp", abi = "clamp(uint256,uint256,uint256,bool)")]
    pub struct Clamp2Call {
        pub value: ::ethers::core::types::U256,
        pub low: ::ethers::core::types::U256,
        pub high: ::ethers::core::types::U256,
        pub enable_logs: bool,
    }
    ///Container type for all input parameters for the `clamp` function with signature `clamp(int256,int256,int256,bool)` and selector `0x09fd3899`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clamp", abi = "clamp(int256,int256,int256,bool)")]
    pub struct Clamp3Call {
        pub value: ::ethers::core::types::I256,
        pub low: ::ethers::core::types::I256,
        pub high: ::ethers::core::types::I256,
        pub enable_logs: bool,
    }
    ///Container type for all input parameters for the `clamp` function with signature `clamp(int256,int256,int256)` and selector `0x7b8d0f0c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clamp", abi = "clamp(int256,int256,int256)")]
    pub struct Clamp0Call {
        pub value: ::ethers::core::types::I256,
        pub low: ::ethers::core::types::I256,
        pub high: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `clamp` function with signature `clamp(uint256,uint256,uint256)` and selector `0x9e422447`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clamp", abi = "clamp(uint256,uint256,uint256)")]
    pub struct Clamp1Call {
        pub value: ::ethers::core::types::U256,
        pub low: ::ethers::core::types::U256,
        pub high: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `clampGt` function with signature `clampGt(uint256,uint256,bool)` and selector `0x1b55d07c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampGt", abi = "clampGt(uint256,uint256,bool)")]
    pub struct ClampGt2Call {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
        pub enable_logs: bool,
    }
    ///Container type for all input parameters for the `clampGt` function with signature `clampGt(int256,int256,bool)` and selector `0x953e5e80`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampGt", abi = "clampGt(int256,int256,bool)")]
    pub struct ClampGt3Call {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
        pub enable_logs: bool,
    }
    ///Container type for all input parameters for the `clampGt` function with signature `clampGt(int256,int256)` and selector `0xf09ced97`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampGt", abi = "clampGt(int256,int256)")]
    pub struct ClampGt0Call {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `clampGt` function with signature `clampGt(uint256,uint256)` and selector `0xfb9894b2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampGt", abi = "clampGt(uint256,uint256)")]
    pub struct ClampGt1Call {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `clampGte` function with signature `clampGte(int256,int256,bool)` and selector `0x2d40851e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampGte", abi = "clampGte(int256,int256,bool)")]
    pub struct ClampGte2Call {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
        pub enable_logs: bool,
    }
    ///Container type for all input parameters for the `clampGte` function with signature `clampGte(uint256,uint256,bool)` and selector `0x2e17d472`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampGte", abi = "clampGte(uint256,uint256,bool)")]
    pub struct ClampGte3Call {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
        pub enable_logs: bool,
    }
    ///Container type for all input parameters for the `clampGte` function with signature `clampGte(int256,int256)` and selector `0x7e1ddd80`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampGte", abi = "clampGte(int256,int256)")]
    pub struct ClampGte0Call {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `clampGte` function with signature `clampGte(uint256,uint256)` and selector `0xe894090d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampGte", abi = "clampGte(uint256,uint256)")]
    pub struct ClampGte1Call {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `clampLt` function with signature `clampLt(int256,int256,bool)` and selector `0x2ffde603`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampLt", abi = "clampLt(int256,int256,bool)")]
    pub struct ClampLt2Call {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
        pub enable_logs: bool,
    }
    ///Container type for all input parameters for the `clampLt` function with signature `clampLt(uint256,uint256,bool)` and selector `0xb2f9799d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampLt", abi = "clampLt(uint256,uint256,bool)")]
    pub struct ClampLt3Call {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
        pub enable_logs: bool,
    }
    ///Container type for all input parameters for the `clampLt` function with signature `clampLt(int256,int256)` and selector `0xf01ff64c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampLt", abi = "clampLt(int256,int256)")]
    pub struct ClampLt0Call {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `clampLt` function with signature `clampLt(uint256,uint256)` and selector `0xf5bc3a71`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampLt", abi = "clampLt(uint256,uint256)")]
    pub struct ClampLt1Call {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `clampLte` function with signature `clampLte(int256,int256,bool)` and selector `0x084ec410`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampLte", abi = "clampLte(int256,int256,bool)")]
    pub struct ClampLte2Call {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
        pub enable_logs: bool,
    }
    ///Container type for all input parameters for the `clampLte` function with signature `clampLte(int256,int256)` and selector `0xac0c207b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampLte", abi = "clampLte(int256,int256)")]
    pub struct ClampLte0Call {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `clampLte` function with signature `clampLte(uint256,uint256)` and selector `0xc5df2a37`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampLte", abi = "clampLte(uint256,uint256)")]
    pub struct ClampLte1Call {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `clampLte` function with signature `clampLte(uint256,uint256,bool)` and selector `0xeef25389`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clampLte", abi = "clampLte(uint256,uint256,bool)")]
    pub struct ClampLte3Call {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
        pub enable_logs: bool,
    }
    ///Container type for all input parameters for the `diff` function with signature `diff(int256,int256)` and selector `0x568c1385`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "diff", abi = "diff(int256,int256)")]
    pub struct DiffCall {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `diff` function with signature `diff(uint256,uint256)` and selector `0x8f5e514a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "diff", abi = "diff(uint256,uint256)")]
    pub struct DiffWithAAndBCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `eq` function with signature `eq(int256,int256,string)` and selector `0x14ec1cc6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "eq", abi = "eq(int256,int256,string)")]
    pub struct Eq0Call {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `eq` function with signature `eq(address,address,string)` and selector `0x3ccb5e26`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "eq", abi = "eq(address,address,string)")]
    pub struct Eq1Call {
        pub a: ::ethers::core::types::Address,
        pub b: ::ethers::core::types::Address,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `eq` function with signature `eq(bytes4,bytes4,string)` and selector `0x6c9764b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "eq", abi = "eq(bytes4,bytes4,string)")]
    pub struct Eq2Call {
        pub a: [u8; 4],
        pub b: [u8; 4],
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `eq` function with signature `eq(uint256,uint256,string)` and selector `0xbc8d43a8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "eq", abi = "eq(uint256,uint256,string)")]
    pub struct Eq3Call {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `eq` function with signature `eq(bool,bool,string)` and selector `0xdf7cd77f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "eq", abi = "eq(bool,bool,string)")]
    pub struct Eq4Call {
        pub a: bool,
        pub b: bool,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `errAllow` function with signature `errAllow(bytes4,bytes4[],string)` and selector `0x88e362c6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "errAllow", abi = "errAllow(bytes4,bytes4[],string)")]
    pub struct ErrAllowCall {
        pub error_selector: [u8; 4],
        pub allowed_errors: ::std::vec::Vec<[u8; 4]>,
        pub message: ::std::string::String,
    }
    ///Container type for all input parameters for the `errsAllow` function with signature `errsAllow(bytes4,bytes4[],string[])` and selector `0xb09c41b7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "errsAllow", abi = "errsAllow(bytes4,bytes4[],string[])")]
    pub struct ErrsAllowCall {
        pub error_selector: [u8; 4],
        pub allowed_errors: ::std::vec::Vec<[u8; 4]>,
        pub messages: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all input parameters for the `gt` function with signature `gt(uint256,uint256,string)` and selector `0x5c2b80f5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "gt", abi = "gt(uint256,uint256,string)")]
    pub struct GtCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `gt` function with signature `gt(int256,int256,string)` and selector `0xd2a50604`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "gt", abi = "gt(int256,int256,string)")]
    pub struct GtWithAAndBCall {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `gte` function with signature `gte(int256,int256,string)` and selector `0x3b6ddf03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "gte", abi = "gte(int256,int256,string)")]
    pub struct GteCall {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `gte` function with signature `gte(uint256,uint256,string)` and selector `0x841ea11c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "gte", abi = "gte(uint256,uint256,string)")]
    pub struct GteWithAAndBCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
        pub reason: ::std::string::String,
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
    ///Container type for all input parameters for the `lt` function with signature `lt(uint256,uint256,string)` and selector `0x01b9e827`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "lt", abi = "lt(uint256,uint256,string)")]
    pub struct LtCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `lt` function with signature `lt(int256,int256,string)` and selector `0x06f82342`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "lt", abi = "lt(int256,int256,string)")]
    pub struct LtWithAAndBCall {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `lte` function with signature `lte(int256,int256,string)` and selector `0x1ba0399b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "lte", abi = "lte(int256,int256,string)")]
    pub struct LteCall {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `lte` function with signature `lte(uint256,uint256,string)` and selector `0xbb35031a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "lte", abi = "lte(uint256,uint256,string)")]
    pub struct LteWithAAndBCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `max` function with signature `max(uint256,uint256)` and selector `0x6d5433e6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "max", abi = "max(uint256,uint256)")]
    pub struct MaxCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `max` function with signature `max(int256,int256)` and selector `0x81fe5786`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "max", abi = "max(int256,int256)")]
    pub struct MaxWithAAndBCall {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `min` function with signature `min(uint256,uint256)` and selector `0x7ae2b5c7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "min", abi = "min(uint256,uint256)")]
    pub struct MinCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `neq` function with signature `neq(uint256,uint256,string)` and selector `0x8c4fb57d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "neq", abi = "neq(uint256,uint256,string)")]
    pub struct NeqCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `neq` function with signature `neq(int256,int256,string)` and selector `0xc0e8def2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "neq", abi = "neq(int256,int256,string)")]
    pub struct NeqWithAAndBCall {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `platform` function with signature `platform()` and selector `0x4bde38c8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "platform", abi = "platform()")]
    pub struct PlatformCall;
    ///Container type for all input parameters for the `setPlatform` function with signature `setPlatform(address)` and selector `0x6945c5ea`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setPlatform", abi = "setPlatform(address)")]
    pub struct SetPlatformCall {
        pub platform: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `shuffleArray` function with signature `shuffleArray(uint256[],uint256)` and selector `0x6ace552c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "shuffleArray", abi = "shuffleArray(uint256[],uint256)")]
    pub struct ShuffleArrayCall {
        pub shuffle: ::std::vec::Vec<::ethers::core::types::U256>,
        pub entropy: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `t` function with signature `t(bool,string)` and selector `0xa15f9f07`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "t", abi = "t(bool,string)")]
    pub struct TCall {
        pub b: bool,
        pub reason: ::std::string::String,
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
    pub enum FuzzlibCalls {
        Abs(AbsCall),
        AbsWithN(AbsWithNCall),
        AssertRevertReasonEqual0(AssertRevertReasonEqual0Call),
        AssertRevertReasonEqual3(AssertRevertReasonEqual3Call),
        AssertRevertReasonEqual2(AssertRevertReasonEqual2Call),
        AssertRevertReasonEqual1(AssertRevertReasonEqual1Call),
        AssertRevertReasonNotEqual(AssertRevertReasonNotEqualCall),
        Clamp2(Clamp2Call),
        Clamp3(Clamp3Call),
        Clamp0(Clamp0Call),
        Clamp1(Clamp1Call),
        ClampGt2(ClampGt2Call),
        ClampGt3(ClampGt3Call),
        ClampGt0(ClampGt0Call),
        ClampGt1(ClampGt1Call),
        ClampGte2(ClampGte2Call),
        ClampGte3(ClampGte3Call),
        ClampGte0(ClampGte0Call),
        ClampGte1(ClampGte1Call),
        ClampLt2(ClampLt2Call),
        ClampLt3(ClampLt3Call),
        ClampLt0(ClampLt0Call),
        ClampLt1(ClampLt1Call),
        ClampLte2(ClampLte2Call),
        ClampLte0(ClampLte0Call),
        ClampLte1(ClampLte1Call),
        ClampLte3(ClampLte3Call),
        Diff(DiffCall),
        DiffWithAAndB(DiffWithAAndBCall),
        Eq0(Eq0Call),
        Eq1(Eq1Call),
        Eq2(Eq2Call),
        Eq3(Eq3Call),
        Eq4(Eq4Call),
        ErrAllow(ErrAllowCall),
        ErrsAllow(ErrsAllowCall),
        Gt(GtCall),
        GtWithAAndB(GtWithAAndBCall),
        Gte(GteCall),
        GteWithAAndB(GteWithAAndBCall),
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
        Lt(LtCall),
        LtWithAAndB(LtWithAAndBCall),
        Lte(LteCall),
        LteWithAAndB(LteWithAAndBCall),
        Max(MaxCall),
        MaxWithAAndB(MaxWithAAndBCall),
        Min(MinCall),
        Neq(NeqCall),
        NeqWithAAndB(NeqWithAAndBCall),
        Platform(PlatformCall),
        SetPlatform(SetPlatformCall),
        ShuffleArray(ShuffleArrayCall),
        T(TCall),
    }
    impl ::ethers::core::abi::AbiDecode for FuzzlibCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AbsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Abs(decoded));
            }
            if let Ok(decoded) = <AbsWithNCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AbsWithN(decoded));
            }
            if let Ok(decoded) = <AssertRevertReasonEqual0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssertRevertReasonEqual0(decoded));
            }
            if let Ok(decoded) = <AssertRevertReasonEqual3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssertRevertReasonEqual3(decoded));
            }
            if let Ok(decoded) = <AssertRevertReasonEqual2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssertRevertReasonEqual2(decoded));
            }
            if let Ok(decoded) = <AssertRevertReasonEqual1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssertRevertReasonEqual1(decoded));
            }
            if let Ok(decoded) = <AssertRevertReasonNotEqualCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssertRevertReasonNotEqual(decoded));
            }
            if let Ok(decoded) = <Clamp2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Clamp2(decoded));
            }
            if let Ok(decoded) = <Clamp3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Clamp3(decoded));
            }
            if let Ok(decoded) = <Clamp0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Clamp0(decoded));
            }
            if let Ok(decoded) = <Clamp1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Clamp1(decoded));
            }
            if let Ok(decoded) = <ClampGt2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampGt2(decoded));
            }
            if let Ok(decoded) = <ClampGt3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampGt3(decoded));
            }
            if let Ok(decoded) = <ClampGt0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampGt0(decoded));
            }
            if let Ok(decoded) = <ClampGt1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampGt1(decoded));
            }
            if let Ok(decoded) = <ClampGte2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampGte2(decoded));
            }
            if let Ok(decoded) = <ClampGte3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampGte3(decoded));
            }
            if let Ok(decoded) = <ClampGte0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampGte0(decoded));
            }
            if let Ok(decoded) = <ClampGte1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampGte1(decoded));
            }
            if let Ok(decoded) = <ClampLt2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampLt2(decoded));
            }
            if let Ok(decoded) = <ClampLt3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampLt3(decoded));
            }
            if let Ok(decoded) = <ClampLt0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampLt0(decoded));
            }
            if let Ok(decoded) = <ClampLt1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampLt1(decoded));
            }
            if let Ok(decoded) = <ClampLte2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampLte2(decoded));
            }
            if let Ok(decoded) = <ClampLte0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampLte0(decoded));
            }
            if let Ok(decoded) = <ClampLte1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampLte1(decoded));
            }
            if let Ok(decoded) = <ClampLte3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClampLte3(decoded));
            }
            if let Ok(decoded) = <DiffCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Diff(decoded));
            }
            if let Ok(decoded) = <DiffWithAAndBCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DiffWithAAndB(decoded));
            }
            if let Ok(decoded) = <Eq0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Eq0(decoded));
            }
            if let Ok(decoded) = <Eq1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Eq1(decoded));
            }
            if let Ok(decoded) = <Eq2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Eq2(decoded));
            }
            if let Ok(decoded) = <Eq3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Eq3(decoded));
            }
            if let Ok(decoded) = <Eq4Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Eq4(decoded));
            }
            if let Ok(decoded) = <ErrAllowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrAllow(decoded));
            }
            if let Ok(decoded) = <ErrsAllowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrsAllow(decoded));
            }
            if let Ok(decoded) = <GtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Gt(decoded));
            }
            if let Ok(decoded) = <GtWithAAndBCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GtWithAAndB(decoded));
            }
            if let Ok(decoded) = <GteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Gte(decoded));
            }
            if let Ok(decoded) = <GteWithAAndBCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GteWithAAndB(decoded));
            }
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
            if let Ok(decoded) = <LtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Lt(decoded));
            }
            if let Ok(decoded) = <LtWithAAndBCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LtWithAAndB(decoded));
            }
            if let Ok(decoded) = <LteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Lte(decoded));
            }
            if let Ok(decoded) = <LteWithAAndBCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LteWithAAndB(decoded));
            }
            if let Ok(decoded) = <MaxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Max(decoded));
            }
            if let Ok(decoded) = <MaxWithAAndBCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxWithAAndB(decoded));
            }
            if let Ok(decoded) = <MinCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NeqCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Neq(decoded));
            }
            if let Ok(decoded) = <NeqWithAAndBCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NeqWithAAndB(decoded));
            }
            if let Ok(decoded) = <PlatformCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Platform(decoded));
            }
            if let Ok(decoded) = <SetPlatformCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPlatform(decoded));
            }
            if let Ok(decoded) = <ShuffleArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ShuffleArray(decoded));
            }
            if let Ok(decoded) = <TCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::T(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FuzzlibCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Abs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AbsWithN(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssertRevertReasonEqual0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssertRevertReasonEqual3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssertRevertReasonEqual2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssertRevertReasonEqual1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssertRevertReasonNotEqual(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Clamp2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Clamp3(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Clamp0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Clamp1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClampGt2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampGt3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampGt0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampGt1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampGte2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampGte3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampGte0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampGte1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampLt2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampLt3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampLt0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampLt1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampLte2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampLte0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampLte1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClampLte3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Diff(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DiffWithAAndB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Eq0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Eq1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Eq2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Eq3(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Eq4(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ErrAllow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrsAllow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Gt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GtWithAAndB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Gte(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GteWithAAndB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::Lt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LtWithAAndB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Lte(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LteWithAAndB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Max(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxWithAAndB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Neq(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NeqWithAAndB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Platform(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPlatform(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShuffleArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::T(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for FuzzlibCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Abs(element) => ::core::fmt::Display::fmt(element, f),
                Self::AbsWithN(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssertRevertReasonEqual0(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertRevertReasonEqual3(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertRevertReasonEqual2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertRevertReasonEqual1(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertRevertReasonNotEqual(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Clamp2(element) => ::core::fmt::Display::fmt(element, f),
                Self::Clamp3(element) => ::core::fmt::Display::fmt(element, f),
                Self::Clamp0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Clamp1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampGt2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampGt3(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampGt0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampGt1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampGte2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampGte3(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampGte0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampGte1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampLt2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampLt3(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampLt0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampLt1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampLte2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampLte0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampLte1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClampLte3(element) => ::core::fmt::Display::fmt(element, f),
                Self::Diff(element) => ::core::fmt::Display::fmt(element, f),
                Self::DiffWithAAndB(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eq0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eq1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eq2(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eq3(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eq4(element) => ::core::fmt::Display::fmt(element, f),
                Self::ErrAllow(element) => ::core::fmt::Display::fmt(element, f),
                Self::ErrsAllow(element) => ::core::fmt::Display::fmt(element, f),
                Self::Gt(element) => ::core::fmt::Display::fmt(element, f),
                Self::GtWithAAndB(element) => ::core::fmt::Display::fmt(element, f),
                Self::Gte(element) => ::core::fmt::Display::fmt(element, f),
                Self::GteWithAAndB(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Lt(element) => ::core::fmt::Display::fmt(element, f),
                Self::LtWithAAndB(element) => ::core::fmt::Display::fmt(element, f),
                Self::Lte(element) => ::core::fmt::Display::fmt(element, f),
                Self::LteWithAAndB(element) => ::core::fmt::Display::fmt(element, f),
                Self::Max(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxWithAAndB(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::Neq(element) => ::core::fmt::Display::fmt(element, f),
                Self::NeqWithAAndB(element) => ::core::fmt::Display::fmt(element, f),
                Self::Platform(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPlatform(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShuffleArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::T(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AbsCall> for FuzzlibCalls {
        fn from(value: AbsCall) -> Self {
            Self::Abs(value)
        }
    }
    impl ::core::convert::From<AbsWithNCall> for FuzzlibCalls {
        fn from(value: AbsWithNCall) -> Self {
            Self::AbsWithN(value)
        }
    }
    impl ::core::convert::From<AssertRevertReasonEqual0Call> for FuzzlibCalls {
        fn from(value: AssertRevertReasonEqual0Call) -> Self {
            Self::AssertRevertReasonEqual0(value)
        }
    }
    impl ::core::convert::From<AssertRevertReasonEqual3Call> for FuzzlibCalls {
        fn from(value: AssertRevertReasonEqual3Call) -> Self {
            Self::AssertRevertReasonEqual3(value)
        }
    }
    impl ::core::convert::From<AssertRevertReasonEqual2Call> for FuzzlibCalls {
        fn from(value: AssertRevertReasonEqual2Call) -> Self {
            Self::AssertRevertReasonEqual2(value)
        }
    }
    impl ::core::convert::From<AssertRevertReasonEqual1Call> for FuzzlibCalls {
        fn from(value: AssertRevertReasonEqual1Call) -> Self {
            Self::AssertRevertReasonEqual1(value)
        }
    }
    impl ::core::convert::From<AssertRevertReasonNotEqualCall> for FuzzlibCalls {
        fn from(value: AssertRevertReasonNotEqualCall) -> Self {
            Self::AssertRevertReasonNotEqual(value)
        }
    }
    impl ::core::convert::From<Clamp2Call> for FuzzlibCalls {
        fn from(value: Clamp2Call) -> Self {
            Self::Clamp2(value)
        }
    }
    impl ::core::convert::From<Clamp3Call> for FuzzlibCalls {
        fn from(value: Clamp3Call) -> Self {
            Self::Clamp3(value)
        }
    }
    impl ::core::convert::From<Clamp0Call> for FuzzlibCalls {
        fn from(value: Clamp0Call) -> Self {
            Self::Clamp0(value)
        }
    }
    impl ::core::convert::From<Clamp1Call> for FuzzlibCalls {
        fn from(value: Clamp1Call) -> Self {
            Self::Clamp1(value)
        }
    }
    impl ::core::convert::From<ClampGt2Call> for FuzzlibCalls {
        fn from(value: ClampGt2Call) -> Self {
            Self::ClampGt2(value)
        }
    }
    impl ::core::convert::From<ClampGt3Call> for FuzzlibCalls {
        fn from(value: ClampGt3Call) -> Self {
            Self::ClampGt3(value)
        }
    }
    impl ::core::convert::From<ClampGt0Call> for FuzzlibCalls {
        fn from(value: ClampGt0Call) -> Self {
            Self::ClampGt0(value)
        }
    }
    impl ::core::convert::From<ClampGt1Call> for FuzzlibCalls {
        fn from(value: ClampGt1Call) -> Self {
            Self::ClampGt1(value)
        }
    }
    impl ::core::convert::From<ClampGte2Call> for FuzzlibCalls {
        fn from(value: ClampGte2Call) -> Self {
            Self::ClampGte2(value)
        }
    }
    impl ::core::convert::From<ClampGte3Call> for FuzzlibCalls {
        fn from(value: ClampGte3Call) -> Self {
            Self::ClampGte3(value)
        }
    }
    impl ::core::convert::From<ClampGte0Call> for FuzzlibCalls {
        fn from(value: ClampGte0Call) -> Self {
            Self::ClampGte0(value)
        }
    }
    impl ::core::convert::From<ClampGte1Call> for FuzzlibCalls {
        fn from(value: ClampGte1Call) -> Self {
            Self::ClampGte1(value)
        }
    }
    impl ::core::convert::From<ClampLt2Call> for FuzzlibCalls {
        fn from(value: ClampLt2Call) -> Self {
            Self::ClampLt2(value)
        }
    }
    impl ::core::convert::From<ClampLt3Call> for FuzzlibCalls {
        fn from(value: ClampLt3Call) -> Self {
            Self::ClampLt3(value)
        }
    }
    impl ::core::convert::From<ClampLt0Call> for FuzzlibCalls {
        fn from(value: ClampLt0Call) -> Self {
            Self::ClampLt0(value)
        }
    }
    impl ::core::convert::From<ClampLt1Call> for FuzzlibCalls {
        fn from(value: ClampLt1Call) -> Self {
            Self::ClampLt1(value)
        }
    }
    impl ::core::convert::From<ClampLte2Call> for FuzzlibCalls {
        fn from(value: ClampLte2Call) -> Self {
            Self::ClampLte2(value)
        }
    }
    impl ::core::convert::From<ClampLte0Call> for FuzzlibCalls {
        fn from(value: ClampLte0Call) -> Self {
            Self::ClampLte0(value)
        }
    }
    impl ::core::convert::From<ClampLte1Call> for FuzzlibCalls {
        fn from(value: ClampLte1Call) -> Self {
            Self::ClampLte1(value)
        }
    }
    impl ::core::convert::From<ClampLte3Call> for FuzzlibCalls {
        fn from(value: ClampLte3Call) -> Self {
            Self::ClampLte3(value)
        }
    }
    impl ::core::convert::From<DiffCall> for FuzzlibCalls {
        fn from(value: DiffCall) -> Self {
            Self::Diff(value)
        }
    }
    impl ::core::convert::From<DiffWithAAndBCall> for FuzzlibCalls {
        fn from(value: DiffWithAAndBCall) -> Self {
            Self::DiffWithAAndB(value)
        }
    }
    impl ::core::convert::From<Eq0Call> for FuzzlibCalls {
        fn from(value: Eq0Call) -> Self {
            Self::Eq0(value)
        }
    }
    impl ::core::convert::From<Eq1Call> for FuzzlibCalls {
        fn from(value: Eq1Call) -> Self {
            Self::Eq1(value)
        }
    }
    impl ::core::convert::From<Eq2Call> for FuzzlibCalls {
        fn from(value: Eq2Call) -> Self {
            Self::Eq2(value)
        }
    }
    impl ::core::convert::From<Eq3Call> for FuzzlibCalls {
        fn from(value: Eq3Call) -> Self {
            Self::Eq3(value)
        }
    }
    impl ::core::convert::From<Eq4Call> for FuzzlibCalls {
        fn from(value: Eq4Call) -> Self {
            Self::Eq4(value)
        }
    }
    impl ::core::convert::From<ErrAllowCall> for FuzzlibCalls {
        fn from(value: ErrAllowCall) -> Self {
            Self::ErrAllow(value)
        }
    }
    impl ::core::convert::From<ErrsAllowCall> for FuzzlibCalls {
        fn from(value: ErrsAllowCall) -> Self {
            Self::ErrsAllow(value)
        }
    }
    impl ::core::convert::From<GtCall> for FuzzlibCalls {
        fn from(value: GtCall) -> Self {
            Self::Gt(value)
        }
    }
    impl ::core::convert::From<GtWithAAndBCall> for FuzzlibCalls {
        fn from(value: GtWithAAndBCall) -> Self {
            Self::GtWithAAndB(value)
        }
    }
    impl ::core::convert::From<GteCall> for FuzzlibCalls {
        fn from(value: GteCall) -> Self {
            Self::Gte(value)
        }
    }
    impl ::core::convert::From<GteWithAAndBCall> for FuzzlibCalls {
        fn from(value: GteWithAAndBCall) -> Self {
            Self::GteWithAAndB(value)
        }
    }
    impl ::core::convert::From<Log1Call> for FuzzlibCalls {
        fn from(value: Log1Call) -> Self {
            Self::Log1(value)
        }
    }
    impl ::core::convert::From<Log2Call> for FuzzlibCalls {
        fn from(value: Log2Call) -> Self {
            Self::Log2(value)
        }
    }
    impl ::core::convert::From<Log3Call> for FuzzlibCalls {
        fn from(value: Log3Call) -> Self {
            Self::Log3(value)
        }
    }
    impl ::core::convert::From<Log4Call> for FuzzlibCalls {
        fn from(value: Log4Call) -> Self {
            Self::Log4(value)
        }
    }
    impl ::core::convert::From<Log0Call> for FuzzlibCalls {
        fn from(value: Log0Call) -> Self {
            Self::Log0(value)
        }
    }
    impl ::core::convert::From<Log5Call> for FuzzlibCalls {
        fn from(value: Log5Call) -> Self {
            Self::Log5(value)
        }
    }
    impl ::core::convert::From<Log6Call> for FuzzlibCalls {
        fn from(value: Log6Call) -> Self {
            Self::Log6(value)
        }
    }
    impl ::core::convert::From<Log7Call> for FuzzlibCalls {
        fn from(value: Log7Call) -> Self {
            Self::Log7(value)
        }
    }
    impl ::core::convert::From<LogFail2Call> for FuzzlibCalls {
        fn from(value: LogFail2Call) -> Self {
            Self::LogFail2(value)
        }
    }
    impl ::core::convert::From<LogFail3Call> for FuzzlibCalls {
        fn from(value: LogFail3Call) -> Self {
            Self::LogFail3(value)
        }
    }
    impl ::core::convert::From<LogFail4Call> for FuzzlibCalls {
        fn from(value: LogFail4Call) -> Self {
            Self::LogFail4(value)
        }
    }
    impl ::core::convert::From<LogFail5Call> for FuzzlibCalls {
        fn from(value: LogFail5Call) -> Self {
            Self::LogFail5(value)
        }
    }
    impl ::core::convert::From<LogFail6Call> for FuzzlibCalls {
        fn from(value: LogFail6Call) -> Self {
            Self::LogFail6(value)
        }
    }
    impl ::core::convert::From<LogFail1Call> for FuzzlibCalls {
        fn from(value: LogFail1Call) -> Self {
            Self::LogFail1(value)
        }
    }
    impl ::core::convert::From<LogFail0Call> for FuzzlibCalls {
        fn from(value: LogFail0Call) -> Self {
            Self::LogFail0(value)
        }
    }
    impl ::core::convert::From<LogFail7Call> for FuzzlibCalls {
        fn from(value: LogFail7Call) -> Self {
            Self::LogFail7(value)
        }
    }
    impl ::core::convert::From<LogFail8Call> for FuzzlibCalls {
        fn from(value: LogFail8Call) -> Self {
            Self::LogFail8(value)
        }
    }
    impl ::core::convert::From<LtCall> for FuzzlibCalls {
        fn from(value: LtCall) -> Self {
            Self::Lt(value)
        }
    }
    impl ::core::convert::From<LtWithAAndBCall> for FuzzlibCalls {
        fn from(value: LtWithAAndBCall) -> Self {
            Self::LtWithAAndB(value)
        }
    }
    impl ::core::convert::From<LteCall> for FuzzlibCalls {
        fn from(value: LteCall) -> Self {
            Self::Lte(value)
        }
    }
    impl ::core::convert::From<LteWithAAndBCall> for FuzzlibCalls {
        fn from(value: LteWithAAndBCall) -> Self {
            Self::LteWithAAndB(value)
        }
    }
    impl ::core::convert::From<MaxCall> for FuzzlibCalls {
        fn from(value: MaxCall) -> Self {
            Self::Max(value)
        }
    }
    impl ::core::convert::From<MaxWithAAndBCall> for FuzzlibCalls {
        fn from(value: MaxWithAAndBCall) -> Self {
            Self::MaxWithAAndB(value)
        }
    }
    impl ::core::convert::From<MinCall> for FuzzlibCalls {
        fn from(value: MinCall) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NeqCall> for FuzzlibCalls {
        fn from(value: NeqCall) -> Self {
            Self::Neq(value)
        }
    }
    impl ::core::convert::From<NeqWithAAndBCall> for FuzzlibCalls {
        fn from(value: NeqWithAAndBCall) -> Self {
            Self::NeqWithAAndB(value)
        }
    }
    impl ::core::convert::From<PlatformCall> for FuzzlibCalls {
        fn from(value: PlatformCall) -> Self {
            Self::Platform(value)
        }
    }
    impl ::core::convert::From<SetPlatformCall> for FuzzlibCalls {
        fn from(value: SetPlatformCall) -> Self {
            Self::SetPlatform(value)
        }
    }
    impl ::core::convert::From<ShuffleArrayCall> for FuzzlibCalls {
        fn from(value: ShuffleArrayCall) -> Self {
            Self::ShuffleArray(value)
        }
    }
    impl ::core::convert::From<TCall> for FuzzlibCalls {
        fn from(value: TCall) -> Self {
            Self::T(value)
        }
    }
    ///Container type for all return fields from the `abs` function with signature `abs(int256)` and selector `0x1b5ac4b5`
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
    pub struct AbsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `abs` function with signature `abs(int128)` and selector `0xe01e92d2`
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
    pub struct AbsWithNReturn(pub i128);
    ///Container type for all return fields from the `clamp` function with signature `clamp(uint256,uint256,uint256,bool)` and selector `0x06cb0011`
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
    pub struct Clamp2Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `clamp` function with signature `clamp(int256,int256,int256,bool)` and selector `0x09fd3899`
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
    pub struct Clamp3Return(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `clamp` function with signature `clamp(int256,int256,int256)` and selector `0x7b8d0f0c`
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
    pub struct Clamp0Return(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `clamp` function with signature `clamp(uint256,uint256,uint256)` and selector `0x9e422447`
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
    pub struct Clamp1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `clampGt` function with signature `clampGt(uint256,uint256,bool)` and selector `0x1b55d07c`
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
    pub struct ClampGt2Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `clampGt` function with signature `clampGt(int256,int256,bool)` and selector `0x953e5e80`
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
    pub struct ClampGt3Return(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `clampGt` function with signature `clampGt(int256,int256)` and selector `0xf09ced97`
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
    pub struct ClampGt0Return(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `clampGt` function with signature `clampGt(uint256,uint256)` and selector `0xfb9894b2`
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
    pub struct ClampGt1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `clampGte` function with signature `clampGte(int256,int256,bool)` and selector `0x2d40851e`
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
    pub struct ClampGte2Return(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `clampGte` function with signature `clampGte(uint256,uint256,bool)` and selector `0x2e17d472`
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
    pub struct ClampGte3Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `clampGte` function with signature `clampGte(int256,int256)` and selector `0x7e1ddd80`
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
    pub struct ClampGte0Return(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `clampGte` function with signature `clampGte(uint256,uint256)` and selector `0xe894090d`
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
    pub struct ClampGte1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `clampLt` function with signature `clampLt(int256,int256,bool)` and selector `0x2ffde603`
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
    pub struct ClampLt2Return(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `clampLt` function with signature `clampLt(uint256,uint256,bool)` and selector `0xb2f9799d`
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
    pub struct ClampLt3Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `clampLt` function with signature `clampLt(int256,int256)` and selector `0xf01ff64c`
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
    pub struct ClampLt0Return(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `clampLt` function with signature `clampLt(uint256,uint256)` and selector `0xf5bc3a71`
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
    pub struct ClampLt1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `clampLte` function with signature `clampLte(int256,int256,bool)` and selector `0x084ec410`
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
    pub struct ClampLte2Return(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `clampLte` function with signature `clampLte(int256,int256)` and selector `0xac0c207b`
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
    pub struct ClampLte0Return(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `clampLte` function with signature `clampLte(uint256,uint256)` and selector `0xc5df2a37`
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
    pub struct ClampLte1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `clampLte` function with signature `clampLte(uint256,uint256,bool)` and selector `0xeef25389`
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
    pub struct ClampLte3Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `diff` function with signature `diff(int256,int256)` and selector `0x568c1385`
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
    pub struct DiffReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `diff` function with signature `diff(uint256,uint256)` and selector `0x8f5e514a`
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
    pub struct DiffWithAAndBReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `max` function with signature `max(uint256,uint256)` and selector `0x6d5433e6`
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
    pub struct MaxReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `max` function with signature `max(int256,int256)` and selector `0x81fe5786`
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
    pub struct MaxWithAAndBReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `min` function with signature `min(uint256,uint256)` and selector `0x7ae2b5c7`
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
    pub struct MinReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `platform` function with signature `platform()` and selector `0x4bde38c8`
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
    pub struct PlatformReturn(pub ::ethers::core::types::Address);
}
