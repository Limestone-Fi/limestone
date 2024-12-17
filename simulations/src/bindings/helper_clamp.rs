pub use helper_clamp::*;
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
pub mod helper_clamp {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static HELPERCLAMP_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct HelperClamp<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for HelperClamp<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HelperClamp<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HelperClamp<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HelperClamp<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(HelperClamp))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> HelperClamp<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    HELPERCLAMP_ABI.clone(),
                    client,
                ),
            )
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
        ///Gets the contract's `Clamped` event
        pub fn clamped_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ClampedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            HelperClampEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for HelperClamp<M> {
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
    #[ethevent(name = "Clamped", abi = "Clamped(string)")]
    pub struct ClampedFilter(pub ::std::string::String);
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
    pub enum HelperClampEvents {
        AssertEqFailFilter(AssertEqFailFilter),
        AssertFailFilter(AssertFailFilter),
        AssertGtFailFilter(AssertGtFailFilter),
        AssertGteFailFilter(AssertGteFailFilter),
        AssertLtFailFilter(AssertLtFailFilter),
        AssertLteFailFilter(AssertLteFailFilter),
        AssertNeqFailFilter(AssertNeqFailFilter),
        ClampedFilter(ClampedFilter),
    }
    impl ::ethers::contract::EthLogDecode for HelperClampEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertEqFailFilter::decode_log(log) {
                return Ok(HelperClampEvents::AssertEqFailFilter(decoded));
            }
            if let Ok(decoded) = AssertFailFilter::decode_log(log) {
                return Ok(HelperClampEvents::AssertFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGtFailFilter::decode_log(log) {
                return Ok(HelperClampEvents::AssertGtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGteFailFilter::decode_log(log) {
                return Ok(HelperClampEvents::AssertGteFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLtFailFilter::decode_log(log) {
                return Ok(HelperClampEvents::AssertLtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLteFailFilter::decode_log(log) {
                return Ok(HelperClampEvents::AssertLteFailFilter(decoded));
            }
            if let Ok(decoded) = AssertNeqFailFilter::decode_log(log) {
                return Ok(HelperClampEvents::AssertNeqFailFilter(decoded));
            }
            if let Ok(decoded) = ClampedFilter::decode_log(log) {
                return Ok(HelperClampEvents::ClampedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for HelperClampEvents {
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
                Self::ClampedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssertEqFailFilter> for HelperClampEvents {
        fn from(value: AssertEqFailFilter) -> Self {
            Self::AssertEqFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertFailFilter> for HelperClampEvents {
        fn from(value: AssertFailFilter) -> Self {
            Self::AssertFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGtFailFilter> for HelperClampEvents {
        fn from(value: AssertGtFailFilter) -> Self {
            Self::AssertGtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGteFailFilter> for HelperClampEvents {
        fn from(value: AssertGteFailFilter) -> Self {
            Self::AssertGteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLtFailFilter> for HelperClampEvents {
        fn from(value: AssertLtFailFilter) -> Self {
            Self::AssertLtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLteFailFilter> for HelperClampEvents {
        fn from(value: AssertLteFailFilter) -> Self {
            Self::AssertLteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertNeqFailFilter> for HelperClampEvents {
        fn from(value: AssertNeqFailFilter) -> Self {
            Self::AssertNeqFailFilter(value)
        }
    }
    impl ::core::convert::From<ClampedFilter> for HelperClampEvents {
        fn from(value: ClampedFilter) -> Self {
            Self::ClampedFilter(value)
        }
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
    pub enum HelperClampCalls {
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
        Lt(LtCall),
        LtWithAAndB(LtWithAAndBCall),
        Lte(LteCall),
        LteWithAAndB(LteWithAAndBCall),
        Neq(NeqCall),
        NeqWithAAndB(NeqWithAAndBCall),
        Platform(PlatformCall),
        SetPlatform(SetPlatformCall),
        T(TCall),
    }
    impl ::ethers::core::abi::AbiDecode for HelperClampCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <TCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::T(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HelperClampCalls {
        fn encode(self) -> Vec<u8> {
            match self {
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
                Self::Lt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LtWithAAndB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Lte(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LteWithAAndB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::T(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for HelperClampCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
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
                Self::Lt(element) => ::core::fmt::Display::fmt(element, f),
                Self::LtWithAAndB(element) => ::core::fmt::Display::fmt(element, f),
                Self::Lte(element) => ::core::fmt::Display::fmt(element, f),
                Self::LteWithAAndB(element) => ::core::fmt::Display::fmt(element, f),
                Self::Neq(element) => ::core::fmt::Display::fmt(element, f),
                Self::NeqWithAAndB(element) => ::core::fmt::Display::fmt(element, f),
                Self::Platform(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPlatform(element) => ::core::fmt::Display::fmt(element, f),
                Self::T(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssertRevertReasonEqual0Call> for HelperClampCalls {
        fn from(value: AssertRevertReasonEqual0Call) -> Self {
            Self::AssertRevertReasonEqual0(value)
        }
    }
    impl ::core::convert::From<AssertRevertReasonEqual3Call> for HelperClampCalls {
        fn from(value: AssertRevertReasonEqual3Call) -> Self {
            Self::AssertRevertReasonEqual3(value)
        }
    }
    impl ::core::convert::From<AssertRevertReasonEqual2Call> for HelperClampCalls {
        fn from(value: AssertRevertReasonEqual2Call) -> Self {
            Self::AssertRevertReasonEqual2(value)
        }
    }
    impl ::core::convert::From<AssertRevertReasonEqual1Call> for HelperClampCalls {
        fn from(value: AssertRevertReasonEqual1Call) -> Self {
            Self::AssertRevertReasonEqual1(value)
        }
    }
    impl ::core::convert::From<AssertRevertReasonNotEqualCall> for HelperClampCalls {
        fn from(value: AssertRevertReasonNotEqualCall) -> Self {
            Self::AssertRevertReasonNotEqual(value)
        }
    }
    impl ::core::convert::From<Clamp2Call> for HelperClampCalls {
        fn from(value: Clamp2Call) -> Self {
            Self::Clamp2(value)
        }
    }
    impl ::core::convert::From<Clamp3Call> for HelperClampCalls {
        fn from(value: Clamp3Call) -> Self {
            Self::Clamp3(value)
        }
    }
    impl ::core::convert::From<Clamp0Call> for HelperClampCalls {
        fn from(value: Clamp0Call) -> Self {
            Self::Clamp0(value)
        }
    }
    impl ::core::convert::From<Clamp1Call> for HelperClampCalls {
        fn from(value: Clamp1Call) -> Self {
            Self::Clamp1(value)
        }
    }
    impl ::core::convert::From<ClampGt2Call> for HelperClampCalls {
        fn from(value: ClampGt2Call) -> Self {
            Self::ClampGt2(value)
        }
    }
    impl ::core::convert::From<ClampGt3Call> for HelperClampCalls {
        fn from(value: ClampGt3Call) -> Self {
            Self::ClampGt3(value)
        }
    }
    impl ::core::convert::From<ClampGt0Call> for HelperClampCalls {
        fn from(value: ClampGt0Call) -> Self {
            Self::ClampGt0(value)
        }
    }
    impl ::core::convert::From<ClampGt1Call> for HelperClampCalls {
        fn from(value: ClampGt1Call) -> Self {
            Self::ClampGt1(value)
        }
    }
    impl ::core::convert::From<ClampGte2Call> for HelperClampCalls {
        fn from(value: ClampGte2Call) -> Self {
            Self::ClampGte2(value)
        }
    }
    impl ::core::convert::From<ClampGte3Call> for HelperClampCalls {
        fn from(value: ClampGte3Call) -> Self {
            Self::ClampGte3(value)
        }
    }
    impl ::core::convert::From<ClampGte0Call> for HelperClampCalls {
        fn from(value: ClampGte0Call) -> Self {
            Self::ClampGte0(value)
        }
    }
    impl ::core::convert::From<ClampGte1Call> for HelperClampCalls {
        fn from(value: ClampGte1Call) -> Self {
            Self::ClampGte1(value)
        }
    }
    impl ::core::convert::From<ClampLt2Call> for HelperClampCalls {
        fn from(value: ClampLt2Call) -> Self {
            Self::ClampLt2(value)
        }
    }
    impl ::core::convert::From<ClampLt3Call> for HelperClampCalls {
        fn from(value: ClampLt3Call) -> Self {
            Self::ClampLt3(value)
        }
    }
    impl ::core::convert::From<ClampLt0Call> for HelperClampCalls {
        fn from(value: ClampLt0Call) -> Self {
            Self::ClampLt0(value)
        }
    }
    impl ::core::convert::From<ClampLt1Call> for HelperClampCalls {
        fn from(value: ClampLt1Call) -> Self {
            Self::ClampLt1(value)
        }
    }
    impl ::core::convert::From<ClampLte2Call> for HelperClampCalls {
        fn from(value: ClampLte2Call) -> Self {
            Self::ClampLte2(value)
        }
    }
    impl ::core::convert::From<ClampLte0Call> for HelperClampCalls {
        fn from(value: ClampLte0Call) -> Self {
            Self::ClampLte0(value)
        }
    }
    impl ::core::convert::From<ClampLte1Call> for HelperClampCalls {
        fn from(value: ClampLte1Call) -> Self {
            Self::ClampLte1(value)
        }
    }
    impl ::core::convert::From<ClampLte3Call> for HelperClampCalls {
        fn from(value: ClampLte3Call) -> Self {
            Self::ClampLte3(value)
        }
    }
    impl ::core::convert::From<Eq0Call> for HelperClampCalls {
        fn from(value: Eq0Call) -> Self {
            Self::Eq0(value)
        }
    }
    impl ::core::convert::From<Eq1Call> for HelperClampCalls {
        fn from(value: Eq1Call) -> Self {
            Self::Eq1(value)
        }
    }
    impl ::core::convert::From<Eq2Call> for HelperClampCalls {
        fn from(value: Eq2Call) -> Self {
            Self::Eq2(value)
        }
    }
    impl ::core::convert::From<Eq3Call> for HelperClampCalls {
        fn from(value: Eq3Call) -> Self {
            Self::Eq3(value)
        }
    }
    impl ::core::convert::From<Eq4Call> for HelperClampCalls {
        fn from(value: Eq4Call) -> Self {
            Self::Eq4(value)
        }
    }
    impl ::core::convert::From<ErrAllowCall> for HelperClampCalls {
        fn from(value: ErrAllowCall) -> Self {
            Self::ErrAllow(value)
        }
    }
    impl ::core::convert::From<ErrsAllowCall> for HelperClampCalls {
        fn from(value: ErrsAllowCall) -> Self {
            Self::ErrsAllow(value)
        }
    }
    impl ::core::convert::From<GtCall> for HelperClampCalls {
        fn from(value: GtCall) -> Self {
            Self::Gt(value)
        }
    }
    impl ::core::convert::From<GtWithAAndBCall> for HelperClampCalls {
        fn from(value: GtWithAAndBCall) -> Self {
            Self::GtWithAAndB(value)
        }
    }
    impl ::core::convert::From<GteCall> for HelperClampCalls {
        fn from(value: GteCall) -> Self {
            Self::Gte(value)
        }
    }
    impl ::core::convert::From<GteWithAAndBCall> for HelperClampCalls {
        fn from(value: GteWithAAndBCall) -> Self {
            Self::GteWithAAndB(value)
        }
    }
    impl ::core::convert::From<LtCall> for HelperClampCalls {
        fn from(value: LtCall) -> Self {
            Self::Lt(value)
        }
    }
    impl ::core::convert::From<LtWithAAndBCall> for HelperClampCalls {
        fn from(value: LtWithAAndBCall) -> Self {
            Self::LtWithAAndB(value)
        }
    }
    impl ::core::convert::From<LteCall> for HelperClampCalls {
        fn from(value: LteCall) -> Self {
            Self::Lte(value)
        }
    }
    impl ::core::convert::From<LteWithAAndBCall> for HelperClampCalls {
        fn from(value: LteWithAAndBCall) -> Self {
            Self::LteWithAAndB(value)
        }
    }
    impl ::core::convert::From<NeqCall> for HelperClampCalls {
        fn from(value: NeqCall) -> Self {
            Self::Neq(value)
        }
    }
    impl ::core::convert::From<NeqWithAAndBCall> for HelperClampCalls {
        fn from(value: NeqWithAAndBCall) -> Self {
            Self::NeqWithAAndB(value)
        }
    }
    impl ::core::convert::From<PlatformCall> for HelperClampCalls {
        fn from(value: PlatformCall) -> Self {
            Self::Platform(value)
        }
    }
    impl ::core::convert::From<SetPlatformCall> for HelperClampCalls {
        fn from(value: SetPlatformCall) -> Self {
            Self::SetPlatform(value)
        }
    }
    impl ::core::convert::From<TCall> for HelperClampCalls {
        fn from(value: TCall) -> Self {
            Self::T(value)
        }
    }
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
