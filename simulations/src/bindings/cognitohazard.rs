pub use cognitohazard::*;
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
pub mod cognitohazard {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_lime"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("LIME"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("LIME"),
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("cleanseFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cleanseFee"),
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
                    ::std::borrow::ToOwned::to_owned("curse"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("curse"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("victim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("markId"),
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
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getApproved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
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
                    ::std::borrow::ToOwned::to_owned("makeItStop"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("makeItStop"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mark"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mark"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_victim"),
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isApproved"),
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
                    ::std::borrow::ToOwned::to_owned("setCleanseFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setCleanseFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_cleanseFee"),
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
                    ::std::borrow::ToOwned::to_owned("setUriSource"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUriSource"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_uri"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenIds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenIds"),
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
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenURI"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                    ::std::borrow::ToOwned::to_owned("uriSource"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uriSource"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isApproved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cleansed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Cleansed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("soul"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Marked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Marked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("victim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("AccountBalanceOverflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccountBalanceOverflow",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BalanceQueryForZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BalanceQueryForZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotCursedGoAway"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotCursedGoAway"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotLime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotLime"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotOwnerNorApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotOwnerNorApproved",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Soulbound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Soulbound"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenAlreadyExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenAlreadyExists"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenDoesNotExist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenDoesNotExist"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferFromIncorrectOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TransferFromIncorrectOwner",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "TransferToNonERC721ReceiverImplementer",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TransferToNonERC721ReceiverImplementer",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferToZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TransferToZeroAddress",
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
    pub static COGNITOHAZARD_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@Rh65\xC9\xAD\xC5\xDE\xA0\0\0`\x01U4\x80\x15a\0\x1CW__\xFD[P`@Qa\x14\08\x03\x80a\x14\0\x839\x81\x01`@\x81\x90Ra\0;\x91a\x02\xCCV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R_\x80T\x90\x80a\0V\x83a\x02\xF9V[\x91\x90PUPa\0\x80s^3O\xC2\xEE\xC9xG\x8E\x84\xD1tF\xD8B\xBB\xD8\xC5\xAF}_Ta\x020` \x1B` \x1CV[_\x80Ts^3O\xC2\xEE\xC9xG\x8E\x84\xD1tF\xD8B\xBB\xD8\xC5\xAF}\x82R`\x03` R\x7F{L\x840\x02\xFE\xDB\xA6O\x99\x87\xC1\xD3\x8Eb\x82\xC7b\xA9J7\x8B\r\x04\xB6)\xD9/X\xC7\xC4]\x81\x90U\x90\x80a\0\xCE\x83a\x02\xF9V[\x91\x90PUPa\0\xF8s4\x83:\xB6w\xF5\xCC@\xA4J63\x0F\xB1\x8F\xEF*\xC4\xF0;_Ta\x020` \x1B` \x1CV[_\x80Ts4\x83:\xB6w\xF5\xCC@\xA4J63\x0F\xB1\x8F\xEF*\xC4\xF0;\x82R`\x03` R\x7F\xCEA&\x16\xD5\xF1\xB5q\x06\xF1~\n\xF4\xB8ed#\xBF\xD8\xE6\xB9\xA3\x8C0\xB2\xC6-\xE8\x0F\xCC\xA5\xF3\x81\x90U\x90\x80a\x01F\x83a\x02\xF9V[\x91\x90PUPa\x01ps\x1F+\x063\xBB\x06#\xDC\xCE\xBEW\x93-g1\xAE\x93\xF5!>_Ta\x020` \x1B` \x1CV[_\x80Ts\x1F+\x063\xBB\x06#\xDC\xCE\xBEW\x93-g1\xAE\x93\xF5!>\x82R`\x03` R\x7F\xEB\0\xBE\xCC\x02\xEBA=\x16\xA25\xDC\x91\xB3\xE0\xD7\xDE\xD6\x1A\x1D4\x86;zTw\x98o\x87\x19\xAFK\x81\x90U\x90\x80a\x01\xBE\x83a\x02\xF9V[\x91\x90PUPa\x01\xE8s\xEC\xEET\x97\xB9\xDB\xB8.\x18\x04\xE3\"Og\xD0\r\x8D\x89\x1Ci_Ta\x020` \x1B` \x1CV[P_\x80Ts\xEC\xEET\x97\xB9\xDB\xB8.\x18\x04\xE3\"Og\xD0\r\x8D\x89\x1Ci\x90\x91R`\x03` R\x7F\xA8R;\xFA\xAB0\x18\x02\xB9\xBEk\xF7\0\xEC\x94\xC6\xE2\xE5wl\x7F\x1B\x9F!\xD8!\x8A\xD7&\x95d\xD4Ua\x03\x1DV[\x81``\x1B``\x1C\x91P\x80_Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` _ \x81\x01\x81\x01\x80T\x80``\x1B\x15a\x02kWc\xC9\x91\xCB\xB1_R`\x04`\x1C\xFD[\x83\x17\x90U_\x82\x90R`\x1C`\x0C \x80T`\x01\x01c\xFF\xFF\xFF\xFF\x81\x16\x84\x02a\x02\x9FWg\xEAU;4\x013l\xEA\x84\x15`\x02\x1BR`\x04`\x1C\xFD[\x90U\x80\x82_\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x818\xA4PPV[_` \x82\x84\x03\x12\x15a\x02\xDCW__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\xF2W__\xFD[\x93\x92PPPV[_`\x01\x82\x01a\x03\x16WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V[`\x80Qa\x10\xB6a\x03J_9_\x81\x81a\x04\0\x01R\x81\x81a\x06\xD7\x01R\x81\x81a\x07\xB2\x01Ra\x07\xF8\x01Ra\x10\xB6_\xF3\xFE`\x80`@R`\x046\x10a\x011W_5`\xE0\x1C\x80cws&\r\x11a\0\xA8W\x80c\xA2,\xB4e\x11a\0mW\x80c\xA2,\xB4e\x14a\x03KW\x80c\xB8\x8DO\xDE\x14a\x03jW\x80c\xC1\xA4\x81\xD3\x14a\x03}W\x80c\xC8{V\xDD\x14a\x03\x9CW\x80c\xE9\x85\xE9\xC5\x14a\x03\xBBW\x80c\xEA\x8D+y\x14a\x03\xEFW__\xFD[\x80cws&\r\x14a\x02\xADW\x80cy\"\xBE7\x14a\x02\xD8W\x80c|\xEE\xB8\x80\x14a\x02\xECW\x80c\x86\x02\xE6\xCA\x14a\x03\x0BW\x80c\x95\xD8\x9BA\x14a\x03\x1FW__\xFD[\x80cB\x84.\x0E\x11a\0\xF9W\x80cB\x84.\x0E\x14a\x02\x06W\x80cF\xA4*\xFB\x14a\x02\x19W\x80c`SR\xE0\x14a\x02<W\x80ccR!\x1E\x14a\x02[W\x80cp\xA0\x821\x14a\x02zW\x80cqL\xFFV\x14a\x02\x99W__\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x015W\x80c\x06\xFD\xDE\x03\x14a\x01\x86W\x80c\x08\x18\x12\xFC\x14a\x01\xA7W\x80c\t^\xA7\xB3\x14a\x01\xDEW\x80c#\xB8r\xDD\x14a\x01\xF3W[__\xFD[4\x80\x15a\x01@W__\xFD[Pa\x01qa\x01O6`\x04a\x0C\xAFV[c\x01\xFF\xC9\xA7`\xE0\x91\x90\x91\x1C\x90\x81\x14c\x80\xACX\xCD\x82\x14\x17c[^\x13\x9F\x90\x91\x14\x17\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x91W__\xFD[Pa\x01\x9Aa\x04\"V[`@Qa\x01}\x91\x90a\x0C\xDDV[4\x80\x15a\x01\xB2W__\xFD[Pa\x01\xC6a\x01\xC16`\x04a\r\x12V[a\x04BV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01}V[a\x01\xF1a\x01\xEC6`\x04a\r?V[a\x04}V[\0[a\x01\xF1a\x02\x016`\x04a\rgV[a\x04\x8CV[a\x01\xF1a\x02\x146`\x04a\rgV[a\x05\x86V[4\x80\x15a\x02$W__\xFD[Pa\x02.`\x01T\x81V[`@Q\x90\x81R` \x01a\x01}V[4\x80\x15a\x02GW__\xFD[Pa\x01\xF1a\x02V6`\x04a\r\xE6V[a\x05\x9FV[4\x80\x15a\x02fW__\xFD[Pa\x01\xC6a\x02u6`\x04a\r\x12V[a\x05\xACV[4\x80\x15a\x02\x85W__\xFD[Pa\x02.a\x02\x946`\x04a\x0E%V[a\x05\xE8V[4\x80\x15a\x02\xA4W__\xFD[Pa\x02._T\x81V[4\x80\x15a\x02\xB8W__\xFD[Pa\x02.a\x02\xC76`\x04a\x0E%V[`\x03` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x02\xE3W__\xFD[Pa\x01\x9Aa\x06 V[4\x80\x15a\x02\xF7W__\xFD[Pa\x01\xF1a\x03\x066`\x04a\x0E%V[a\x06\xACV[4\x80\x15a\x03\x16W__\xFD[Pa\x01\xF1a\x07`V[4\x80\x15a\x03*W__\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81RcSPCG`\xE0\x1B` \x82\x01Ra\x01\x9AV[4\x80\x15a\x03VW__\xFD[Pa\x01\xF1a\x03e6`\x04a\x0E>V[a\x08\x85V[a\x01\xF1a\x03x6`\x04a\x0EwV[a\x08\xD8V[4\x80\x15a\x03\x88W__\xFD[Pa\x01\xF1a\x03\x976`\x04a\r\x12V[`\x01UV[4\x80\x15a\x03\xA7W__\xFD[Pa\x01\x9Aa\x03\xB66`\x04a\r\x12V[a\t2V[4\x80\x15a\x03\xC6W__\xFD[Pa\x01qa\x03\xD56`\x04a\x0E\xE1V[`\x1CRg\nZ.z\0\0\0\0`\x08R_R`0`\x0C T\x90V[4\x80\x15a\x03\xFAW__\xFD[Pa\x01\xC6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[```@Q\x80``\x01`@R\x80`)\x81R` \x01a\x10\x81`)\x919\x90P\x90V[_\x81_Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` _ \x82\x01\x82\x01\x80T``\x1Ba\x04sWc\xCE\xEA!\xB6_R`\x04`\x1C\xFD[`\x01\x01T\x92\x91PPV[a\x04\x883\x83\x83a\t\xC4V[PPV[_\x81\x81Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B3\x17`\x1CR` \x90 \x81\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x94\x93\x84\x16\x93\x81\x16\x91\x90\x82\x86\x14\x83\x02a\x04\xDDWg\xCE\xEA!\xB6\xA1\x14\x81\0\x83\x15`\x02\x1BR`\x04`\x1C\xFD[\x85_R\x81`\x01\x01T\x92P\x823\x14\x863\x14\x17a\x05\tW`0`\x0C Ta\x05\tWcKn\x7F\x18_R`\x04`\x1C\xFD[\x82\x15a\x05\x16W_\x82`\x01\x01U[\x85\x85\x18\x18\x90UP`\x1C`\x0C\x81\x81 \x80T_\x19\x01\x90U_\x84\x90R \x80T`\x01\x01c\xFF\xFF\xFF\xFF\x81\x16\x84\x02a\x05WWg\xEAU;4\x013l\xEA\x84\x15`\x02\x1BR`\x04`\x1C\xFD[\x90U\x80\x82\x84\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF_8\xA4[PPPV[`@Qc\xA4B\n\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02a\x05\x81\x82\x84\x83a\x0F\xA2V[_\x81\x81Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` \x90 \x81\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x05\xE3Wc\xCE\xEA!\xB6_R`\x04`\x1C\xFD[\x91\x90PV[_\x81a\x05\xFBWc\x8FN\xB6\x04_R`\x04`\x1C\xFD[g>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR\x81_Rc\xFF\xFF\xFF\xFF`\x1C`\x0C T\x16\x90P\x91\x90PV[`\x02\x80Ta\x06-\x90a\x0F&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06Y\x90a\x0F&V[\x80\x15a\x06\xA4W\x80`\x1F\x10a\x06{Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xA4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x87W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 T\x15a\x06\xCCWPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\x15W`@Qc\x13\xF3+\xAB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T\x90\x80a\x07#\x83a\x10\\V[\x91\x90PUPa\x073\x81_Ta\n^V[`@Q3\x90\x7Fi\xC5\xE9\xDC\xA9\x07\xCA\x02\xC0\xF3t\x18F-\xF4z\xAE\xDD\xDB\xB8X\xBC!\xA7ZVe\xBD\xC1@8\x11\x90_\x90\xA2PV[3_\x90\x81R`\x03` R`@\x81 T`\x01T\x90\x91\x82\x90\x03a\x07\x94W`@Qc\x03\x85\x1A3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x9D\x82a\n\xFAV[3_\x81\x81R`\x03` R`@\x81 Ua\x07\xE2\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x900\x84a\x0B\x07V[`@Qc\x08R\xCD\x8D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cB\x96lh\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08AW__\xFD[PZ\xF1\x15\x80\x15a\x08SW=__>=_\xFD[PP`@Q3\x92P\x7F])%\xA8\xAC\xFD\xDE\xB3\xFF\x1C7\xD2\x94'\x10\xC5r\xFA\xB4Cc%\xC7\xD6Nz\xEEe\xEE\x88\\@\x91P_\x90\xA2PPV[\x80\x15\x15\x90P\x81`\x1CRg\nZ.z\0\0\0\0`\x08R3_R\x80`0`\x0C U\x80_R\x81``\x1B``\x1C3\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` _\xA3PPV[a\x08\xE3\x85\x85\x85a\x04\x8CV[\x83;\x15a\t+Wa\t+\x85\x85\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B`\x92PPPV[PPPPPV[```\x02\x80Ta\tA\x90a\x0F&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tm\x90a\x0F&V[\x80\x15a\t\xB8W\x80`\x1F\x10a\t\x8FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xB8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x9BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[_\x19``\x1C\x82\x81\x16\x92P\x83\x81\x16\x93P\x81_R\x83g>\xC4\x12\xA9\x85-\x17=`\xC1\x1B\x17`\x1CR` _ \x82\x01\x82\x01\x80T\x82\x16\x91P\x81a\n\x07Wc\xCE\xEA!\xB6_R`\x04`\x1C\xFD[\x81\x85\x14\x85\x15\x17a\n+W\x81_R`0`\x0C Ta\n+WcKn\x7F\x18_R`\x04`\x1C\xFD[`\x01\x01\x83\x90U\x81\x83\x82\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%_8\xA4PPPPV[\x81``\x1B``\x1C\x91P\x80_Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` _ \x81\x01\x81\x01\x80T\x80``\x1B\x15a\n\x99Wc\xC9\x91\xCB\xB1_R`\x04`\x1C\xFD[\x83\x17\x90U_\x82\x90R`\x1C`\x0C \x80T`\x01\x01c\xFF\xFF\xFF\xFF\x81\x16\x84\x02a\n\xCDWg\xEAU;4\x013l\xEA\x84\x15`\x02\x1BR`\x04`\x1C\xFD[\x90U\x80\x82_\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x818\xA4PPV[a\x0B\x04_\x82a\x0B\xE9V[PV[`@Q\x81``R\x82`@R\x83``\x1B`,Rc#\xB8r\xDD``\x1B`\x0CR` _`d`\x1C_\x89Z\xF1\x80`\x01_Q\x14\x16a\x0BRW\x80=\x87;\x15\x17\x10a\x0BRWcy9\xF4$_R`\x04`\x1C\xFD[P_``R`@RPPPPV[`@Qc\x15\x0Bz\x02\x80\x82R3` \x83\x01R\x85``\x1B``\x1C`@\x83\x01R\x83``\x83\x01R`\x80\x80\x83\x01R\x82Q\x80`\xA0\x84\x01R\x80\x15a\x0B\xA7W\x80`\xC0\x84\x01\x82` \x87\x01`\x04Z\xFAP[` \x83`\xA4\x83\x01`\x1C\x86\x01_\x8AZ\xF1a\x0B\xC8W=\x15a\x0B\xC8W=_\x84>=\x83\xFD[P\x80`\xE0\x1B\x82Q\x14a\x0B\xE1Wc\xD1\xA5~\xD6_R`\x04`\x1C\xFD[PPPPPPV[_a\x0B\xF3\x82a\x05\xACV[\x90PP_\x81\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16g>\xC4\x12\xA9\x85-\x17=`\xC1\x1B\x81\x17`\x1CR` \x90\x91 \x82\x01\x82\x01\x80T\x91\x93\x82\x16\x91\x82a\x0C8Wc\xCE\xEA!\xB6_R`\x04`\x1C\xFD[\x82_R\x81`\x01\x01T\x80\x86\x14\x84\x87\x14\x17\x86\x15\x17a\x0CeW`0`\x0C Ta\x0CeWcKn\x7F\x18_R`\x04`\x1C\xFD[\x80\x15a\x0CrW_\x83`\x01\x01U[P\x82\x18\x90U`\x1C`\x0C \x80T_\x19\x01\x90U\x81_\x82\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x828\xA4PPPV[_` \x82\x84\x03\x12\x15a\x0C\xBFW__\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0C\xD6W__\xFD[\x93\x92PPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\r\"W__\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xE3W__\xFD[__`@\x83\x85\x03\x12\x15a\rPW__\xFD[a\rY\x83a\r)V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\ryW__\xFD[a\r\x82\x84a\r)V[\x92Pa\r\x90` \x85\x01a\r)V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[__\x83`\x1F\x84\x01\x12a\r\xB1W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xC8W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\r\xDFW__\xFD[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a\r\xF7W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\rW__\xFD[a\x0E\x19\x85\x82\x86\x01a\r\xA1V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15a\x0E5W__\xFD[a\x0C\xD6\x82a\r)V[__`@\x83\x85\x03\x12\x15a\x0EOW__\xFD[a\x0EX\x83a\r)V[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x0ElW__\xFD[\x80\x91PP\x92P\x92\x90PV[_____`\x80\x86\x88\x03\x12\x15a\x0E\x8BW__\xFD[a\x0E\x94\x86a\r)V[\x94Pa\x0E\xA2` \x87\x01a\r)V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xC4W__\xFD[a\x0E\xD0\x88\x82\x89\x01a\r\xA1V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x0E\xF2W__\xFD[a\x0E\xFB\x83a\r)V[\x91Pa\x0F\t` \x84\x01a\r)V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0F:W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0FXWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x05\x81W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x0F\x83WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\t+W_\x81U`\x01\x01a\x0F\x8FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x0F\xBAWa\x0F\xBAa\x0F\x12V[a\x0F\xCE\x83a\x0F\xC8\x83Ta\x0F&V[\x83a\x0F^V[_`\x1F\x84\x11`\x01\x81\x14a\x0F\xFFW_\x85\x15a\x0F\xE8WP\x83\x82\x015[_\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\t+V[_\x83\x81R` \x81 `\x1F\x19\x87\x16\x91[\x82\x81\x10\x15a\x10.W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x10\x0EV[P\x86\x82\x10\x15a\x10JW_\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[_`\x01\x82\x01a\x10yWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V\xFEThe Mark of the Solar Plexus Clown Glider\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static COGNITOHAZARD_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x011W_5`\xE0\x1C\x80cws&\r\x11a\0\xA8W\x80c\xA2,\xB4e\x11a\0mW\x80c\xA2,\xB4e\x14a\x03KW\x80c\xB8\x8DO\xDE\x14a\x03jW\x80c\xC1\xA4\x81\xD3\x14a\x03}W\x80c\xC8{V\xDD\x14a\x03\x9CW\x80c\xE9\x85\xE9\xC5\x14a\x03\xBBW\x80c\xEA\x8D+y\x14a\x03\xEFW__\xFD[\x80cws&\r\x14a\x02\xADW\x80cy\"\xBE7\x14a\x02\xD8W\x80c|\xEE\xB8\x80\x14a\x02\xECW\x80c\x86\x02\xE6\xCA\x14a\x03\x0BW\x80c\x95\xD8\x9BA\x14a\x03\x1FW__\xFD[\x80cB\x84.\x0E\x11a\0\xF9W\x80cB\x84.\x0E\x14a\x02\x06W\x80cF\xA4*\xFB\x14a\x02\x19W\x80c`SR\xE0\x14a\x02<W\x80ccR!\x1E\x14a\x02[W\x80cp\xA0\x821\x14a\x02zW\x80cqL\xFFV\x14a\x02\x99W__\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x015W\x80c\x06\xFD\xDE\x03\x14a\x01\x86W\x80c\x08\x18\x12\xFC\x14a\x01\xA7W\x80c\t^\xA7\xB3\x14a\x01\xDEW\x80c#\xB8r\xDD\x14a\x01\xF3W[__\xFD[4\x80\x15a\x01@W__\xFD[Pa\x01qa\x01O6`\x04a\x0C\xAFV[c\x01\xFF\xC9\xA7`\xE0\x91\x90\x91\x1C\x90\x81\x14c\x80\xACX\xCD\x82\x14\x17c[^\x13\x9F\x90\x91\x14\x17\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x91W__\xFD[Pa\x01\x9Aa\x04\"V[`@Qa\x01}\x91\x90a\x0C\xDDV[4\x80\x15a\x01\xB2W__\xFD[Pa\x01\xC6a\x01\xC16`\x04a\r\x12V[a\x04BV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01}V[a\x01\xF1a\x01\xEC6`\x04a\r?V[a\x04}V[\0[a\x01\xF1a\x02\x016`\x04a\rgV[a\x04\x8CV[a\x01\xF1a\x02\x146`\x04a\rgV[a\x05\x86V[4\x80\x15a\x02$W__\xFD[Pa\x02.`\x01T\x81V[`@Q\x90\x81R` \x01a\x01}V[4\x80\x15a\x02GW__\xFD[Pa\x01\xF1a\x02V6`\x04a\r\xE6V[a\x05\x9FV[4\x80\x15a\x02fW__\xFD[Pa\x01\xC6a\x02u6`\x04a\r\x12V[a\x05\xACV[4\x80\x15a\x02\x85W__\xFD[Pa\x02.a\x02\x946`\x04a\x0E%V[a\x05\xE8V[4\x80\x15a\x02\xA4W__\xFD[Pa\x02._T\x81V[4\x80\x15a\x02\xB8W__\xFD[Pa\x02.a\x02\xC76`\x04a\x0E%V[`\x03` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x02\xE3W__\xFD[Pa\x01\x9Aa\x06 V[4\x80\x15a\x02\xF7W__\xFD[Pa\x01\xF1a\x03\x066`\x04a\x0E%V[a\x06\xACV[4\x80\x15a\x03\x16W__\xFD[Pa\x01\xF1a\x07`V[4\x80\x15a\x03*W__\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x04\x81RcSPCG`\xE0\x1B` \x82\x01Ra\x01\x9AV[4\x80\x15a\x03VW__\xFD[Pa\x01\xF1a\x03e6`\x04a\x0E>V[a\x08\x85V[a\x01\xF1a\x03x6`\x04a\x0EwV[a\x08\xD8V[4\x80\x15a\x03\x88W__\xFD[Pa\x01\xF1a\x03\x976`\x04a\r\x12V[`\x01UV[4\x80\x15a\x03\xA7W__\xFD[Pa\x01\x9Aa\x03\xB66`\x04a\r\x12V[a\t2V[4\x80\x15a\x03\xC6W__\xFD[Pa\x01qa\x03\xD56`\x04a\x0E\xE1V[`\x1CRg\nZ.z\0\0\0\0`\x08R_R`0`\x0C T\x90V[4\x80\x15a\x03\xFAW__\xFD[Pa\x01\xC6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[```@Q\x80``\x01`@R\x80`)\x81R` \x01a\x10\x81`)\x919\x90P\x90V[_\x81_Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` _ \x82\x01\x82\x01\x80T``\x1Ba\x04sWc\xCE\xEA!\xB6_R`\x04`\x1C\xFD[`\x01\x01T\x92\x91PPV[a\x04\x883\x83\x83a\t\xC4V[PPV[_\x81\x81Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B3\x17`\x1CR` \x90 \x81\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x94\x93\x84\x16\x93\x81\x16\x91\x90\x82\x86\x14\x83\x02a\x04\xDDWg\xCE\xEA!\xB6\xA1\x14\x81\0\x83\x15`\x02\x1BR`\x04`\x1C\xFD[\x85_R\x81`\x01\x01T\x92P\x823\x14\x863\x14\x17a\x05\tW`0`\x0C Ta\x05\tWcKn\x7F\x18_R`\x04`\x1C\xFD[\x82\x15a\x05\x16W_\x82`\x01\x01U[\x85\x85\x18\x18\x90UP`\x1C`\x0C\x81\x81 \x80T_\x19\x01\x90U_\x84\x90R \x80T`\x01\x01c\xFF\xFF\xFF\xFF\x81\x16\x84\x02a\x05WWg\xEAU;4\x013l\xEA\x84\x15`\x02\x1BR`\x04`\x1C\xFD[\x90U\x80\x82\x84\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF_8\xA4[PPPV[`@Qc\xA4B\n\x95`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02a\x05\x81\x82\x84\x83a\x0F\xA2V[_\x81\x81Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` \x90 \x81\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x05\xE3Wc\xCE\xEA!\xB6_R`\x04`\x1C\xFD[\x91\x90PV[_\x81a\x05\xFBWc\x8FN\xB6\x04_R`\x04`\x1C\xFD[g>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR\x81_Rc\xFF\xFF\xFF\xFF`\x1C`\x0C T\x16\x90P\x91\x90PV[`\x02\x80Ta\x06-\x90a\x0F&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06Y\x90a\x0F&V[\x80\x15a\x06\xA4W\x80`\x1F\x10a\x06{Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xA4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x87W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 T\x15a\x06\xCCWPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\x15W`@Qc\x13\xF3+\xAB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T\x90\x80a\x07#\x83a\x10\\V[\x91\x90PUPa\x073\x81_Ta\n^V[`@Q3\x90\x7Fi\xC5\xE9\xDC\xA9\x07\xCA\x02\xC0\xF3t\x18F-\xF4z\xAE\xDD\xDB\xB8X\xBC!\xA7ZVe\xBD\xC1@8\x11\x90_\x90\xA2PV[3_\x90\x81R`\x03` R`@\x81 T`\x01T\x90\x91\x82\x90\x03a\x07\x94W`@Qc\x03\x85\x1A3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x9D\x82a\n\xFAV[3_\x81\x81R`\x03` R`@\x81 Ua\x07\xE2\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x900\x84a\x0B\x07V[`@Qc\x08R\xCD\x8D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cB\x96lh\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08AW__\xFD[PZ\xF1\x15\x80\x15a\x08SW=__>=_\xFD[PP`@Q3\x92P\x7F])%\xA8\xAC\xFD\xDE\xB3\xFF\x1C7\xD2\x94'\x10\xC5r\xFA\xB4Cc%\xC7\xD6Nz\xEEe\xEE\x88\\@\x91P_\x90\xA2PPV[\x80\x15\x15\x90P\x81`\x1CRg\nZ.z\0\0\0\0`\x08R3_R\x80`0`\x0C U\x80_R\x81``\x1B``\x1C3\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` _\xA3PPV[a\x08\xE3\x85\x85\x85a\x04\x8CV[\x83;\x15a\t+Wa\t+\x85\x85\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B`\x92PPPV[PPPPPV[```\x02\x80Ta\tA\x90a\x0F&V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tm\x90a\x0F&V[\x80\x15a\t\xB8W\x80`\x1F\x10a\t\x8FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xB8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x9BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[_\x19``\x1C\x82\x81\x16\x92P\x83\x81\x16\x93P\x81_R\x83g>\xC4\x12\xA9\x85-\x17=`\xC1\x1B\x17`\x1CR` _ \x82\x01\x82\x01\x80T\x82\x16\x91P\x81a\n\x07Wc\xCE\xEA!\xB6_R`\x04`\x1C\xFD[\x81\x85\x14\x85\x15\x17a\n+W\x81_R`0`\x0C Ta\n+WcKn\x7F\x18_R`\x04`\x1C\xFD[`\x01\x01\x83\x90U\x81\x83\x82\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%_8\xA4PPPPV[\x81``\x1B``\x1C\x91P\x80_Rg>\xC4\x12\xA9\x85-\x17=`\xC1\x1B`\x1CR` _ \x81\x01\x81\x01\x80T\x80``\x1B\x15a\n\x99Wc\xC9\x91\xCB\xB1_R`\x04`\x1C\xFD[\x83\x17\x90U_\x82\x90R`\x1C`\x0C \x80T`\x01\x01c\xFF\xFF\xFF\xFF\x81\x16\x84\x02a\n\xCDWg\xEAU;4\x013l\xEA\x84\x15`\x02\x1BR`\x04`\x1C\xFD[\x90U\x80\x82_\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x818\xA4PPV[a\x0B\x04_\x82a\x0B\xE9V[PV[`@Q\x81``R\x82`@R\x83``\x1B`,Rc#\xB8r\xDD``\x1B`\x0CR` _`d`\x1C_\x89Z\xF1\x80`\x01_Q\x14\x16a\x0BRW\x80=\x87;\x15\x17\x10a\x0BRWcy9\xF4$_R`\x04`\x1C\xFD[P_``R`@RPPPPV[`@Qc\x15\x0Bz\x02\x80\x82R3` \x83\x01R\x85``\x1B``\x1C`@\x83\x01R\x83``\x83\x01R`\x80\x80\x83\x01R\x82Q\x80`\xA0\x84\x01R\x80\x15a\x0B\xA7W\x80`\xC0\x84\x01\x82` \x87\x01`\x04Z\xFAP[` \x83`\xA4\x83\x01`\x1C\x86\x01_\x8AZ\xF1a\x0B\xC8W=\x15a\x0B\xC8W=_\x84>=\x83\xFD[P\x80`\xE0\x1B\x82Q\x14a\x0B\xE1Wc\xD1\xA5~\xD6_R`\x04`\x1C\xFD[PPPPPPV[_a\x0B\xF3\x82a\x05\xACV[\x90PP_\x81\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16g>\xC4\x12\xA9\x85-\x17=`\xC1\x1B\x81\x17`\x1CR` \x90\x91 \x82\x01\x82\x01\x80T\x91\x93\x82\x16\x91\x82a\x0C8Wc\xCE\xEA!\xB6_R`\x04`\x1C\xFD[\x82_R\x81`\x01\x01T\x80\x86\x14\x84\x87\x14\x17\x86\x15\x17a\x0CeW`0`\x0C Ta\x0CeWcKn\x7F\x18_R`\x04`\x1C\xFD[\x80\x15a\x0CrW_\x83`\x01\x01U[P\x82\x18\x90U`\x1C`\x0C \x80T_\x19\x01\x90U\x81_\x82\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x828\xA4PPPV[_` \x82\x84\x03\x12\x15a\x0C\xBFW__\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0C\xD6W__\xFD[\x93\x92PPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\r\"W__\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xE3W__\xFD[__`@\x83\x85\x03\x12\x15a\rPW__\xFD[a\rY\x83a\r)V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\ryW__\xFD[a\r\x82\x84a\r)V[\x92Pa\r\x90` \x85\x01a\r)V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[__\x83`\x1F\x84\x01\x12a\r\xB1W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xC8W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\r\xDFW__\xFD[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a\r\xF7W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\rW__\xFD[a\x0E\x19\x85\x82\x86\x01a\r\xA1V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15a\x0E5W__\xFD[a\x0C\xD6\x82a\r)V[__`@\x83\x85\x03\x12\x15a\x0EOW__\xFD[a\x0EX\x83a\r)V[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x0ElW__\xFD[\x80\x91PP\x92P\x92\x90PV[_____`\x80\x86\x88\x03\x12\x15a\x0E\x8BW__\xFD[a\x0E\x94\x86a\r)V[\x94Pa\x0E\xA2` \x87\x01a\r)V[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xC4W__\xFD[a\x0E\xD0\x88\x82\x89\x01a\r\xA1V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x0E\xF2W__\xFD[a\x0E\xFB\x83a\r)V[\x91Pa\x0F\t` \x84\x01a\r)V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0F:W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0FXWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x05\x81W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x0F\x83WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\t+W_\x81U`\x01\x01a\x0F\x8FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x0F\xBAWa\x0F\xBAa\x0F\x12V[a\x0F\xCE\x83a\x0F\xC8\x83Ta\x0F&V[\x83a\x0F^V[_`\x1F\x84\x11`\x01\x81\x14a\x0F\xFFW_\x85\x15a\x0F\xE8WP\x83\x82\x015[_\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\t+V[_\x83\x81R` \x81 `\x1F\x19\x87\x16\x91[\x82\x81\x10\x15a\x10.W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x10\x0EV[P\x86\x82\x10\x15a\x10JW_\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[_`\x01\x82\x01a\x10yWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V\xFEThe Mark of the Solar Plexus Clown Glider\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static COGNITOHAZARD_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Cognitohazard<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Cognitohazard<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Cognitohazard<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Cognitohazard<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Cognitohazard<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Cognitohazard))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Cognitohazard<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COGNITOHAZARD_ABI.clone(),
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
                COGNITOHAZARD_ABI.clone(),
                COGNITOHAZARD_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `LIME` (0xea8d2b79) function
        pub fn lime(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([234, 141, 43, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            account: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (account, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cleanseFee` (0x46a42afb) function
        pub fn cleanse_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([70, 164, 42, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `curse` (0x7773260d) function
        pub fn curse(
            &self,
            victim: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([119, 115, 38, 13], victim)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 24, 18, 252], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            owner: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makeItStop` (0x8602e6ca) function
        pub fn make_it_stop(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 2, 230, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mark` (0x7ceeb880) function
        pub fn mark(
            &self,
            victim: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 238, 184, 128], victim)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 82, 33, 30], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_id(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            is_approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, is_approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCleanseFee` (0xc1a481d3) function
        pub fn set_cleanse_fee(
            &self,
            cleanse_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 164, 129, 211], cleanse_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUriSource` (0x605352e0) function
        pub fn set_uri_source(
            &self,
            uri: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 83, 82, 224], uri)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenIds` (0x714cff56) function
        pub fn token_ids(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([113, 76, 255, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uriSource` (0x7922be37) function
        pub fn uri_source(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([121, 34, 190, 55], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Cleansed` event
        pub fn cleansed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CleansedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Marked` event
        pub fn marked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MarkedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CognitohazardEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Cognitohazard<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AccountBalanceOverflow` with signature `AccountBalanceOverflow()` and selector `0x01336cea`
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
    #[etherror(name = "AccountBalanceOverflow", abi = "AccountBalanceOverflow()")]
    pub struct AccountBalanceOverflow;
    ///Custom Error type `BalanceQueryForZeroAddress` with signature `BalanceQueryForZeroAddress()` and selector `0x8f4eb604`
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
        name = "BalanceQueryForZeroAddress",
        abi = "BalanceQueryForZeroAddress()"
    )]
    pub struct BalanceQueryForZeroAddress;
    ///Custom Error type `NotCursedGoAway` with signature `NotCursedGoAway()` and selector `0x3851a330`
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
    #[etherror(name = "NotCursedGoAway", abi = "NotCursedGoAway()")]
    pub struct NotCursedGoAway;
    ///Custom Error type `NotLime` with signature `NotLime()` and selector `0x4fccaeac`
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
    #[etherror(name = "NotLime", abi = "NotLime()")]
    pub struct NotLime;
    ///Custom Error type `NotOwnerNorApproved` with signature `NotOwnerNorApproved()` and selector `0x4b6e7f18`
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
    #[etherror(name = "NotOwnerNorApproved", abi = "NotOwnerNorApproved()")]
    pub struct NotOwnerNorApproved;
    ///Custom Error type `Soulbound` with signature `Soulbound()` and selector `0xa4420a95`
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
    #[etherror(name = "Soulbound", abi = "Soulbound()")]
    pub struct Soulbound;
    ///Custom Error type `TokenAlreadyExists` with signature `TokenAlreadyExists()` and selector `0xc991cbb1`
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
    #[etherror(name = "TokenAlreadyExists", abi = "TokenAlreadyExists()")]
    pub struct TokenAlreadyExists;
    ///Custom Error type `TokenDoesNotExist` with signature `TokenDoesNotExist()` and selector `0xceea21b6`
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
    #[etherror(name = "TokenDoesNotExist", abi = "TokenDoesNotExist()")]
    pub struct TokenDoesNotExist;
    ///Custom Error type `TransferFromIncorrectOwner` with signature `TransferFromIncorrectOwner()` and selector `0xa1148100`
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
        name = "TransferFromIncorrectOwner",
        abi = "TransferFromIncorrectOwner()"
    )]
    pub struct TransferFromIncorrectOwner;
    ///Custom Error type `TransferToNonERC721ReceiverImplementer` with signature `TransferToNonERC721ReceiverImplementer()` and selector `0xd1a57ed6`
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
        name = "TransferToNonERC721ReceiverImplementer",
        abi = "TransferToNonERC721ReceiverImplementer()"
    )]
    pub struct TransferToNonERC721ReceiverImplementer;
    ///Custom Error type `TransferToZeroAddress` with signature `TransferToZeroAddress()` and selector `0xea553b34`
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
    #[etherror(name = "TransferToZeroAddress", abi = "TransferToZeroAddress()")]
    pub struct TransferToZeroAddress;
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
    pub enum CognitohazardErrors {
        AccountBalanceOverflow(AccountBalanceOverflow),
        BalanceQueryForZeroAddress(BalanceQueryForZeroAddress),
        NotCursedGoAway(NotCursedGoAway),
        NotLime(NotLime),
        NotOwnerNorApproved(NotOwnerNorApproved),
        Soulbound(Soulbound),
        TokenAlreadyExists(TokenAlreadyExists),
        TokenDoesNotExist(TokenDoesNotExist),
        TransferFromIncorrectOwner(TransferFromIncorrectOwner),
        TransferToNonERC721ReceiverImplementer(TransferToNonERC721ReceiverImplementer),
        TransferToZeroAddress(TransferToZeroAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for CognitohazardErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AccountBalanceOverflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountBalanceOverflow(decoded));
            }
            if let Ok(decoded) = <BalanceQueryForZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceQueryForZeroAddress(decoded));
            }
            if let Ok(decoded) = <NotCursedGoAway as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotCursedGoAway(decoded));
            }
            if let Ok(decoded) = <NotLime as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotLime(decoded));
            }
            if let Ok(decoded) = <NotOwnerNorApproved as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotOwnerNorApproved(decoded));
            }
            if let Ok(decoded) = <Soulbound as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Soulbound(decoded));
            }
            if let Ok(decoded) = <TokenAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenAlreadyExists(decoded));
            }
            if let Ok(decoded) = <TokenDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenDoesNotExist(decoded));
            }
            if let Ok(decoded) = <TransferFromIncorrectOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFromIncorrectOwner(decoded));
            }
            if let Ok(decoded) = <TransferToNonERC721ReceiverImplementer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferToNonERC721ReceiverImplementer(decoded));
            }
            if let Ok(decoded) = <TransferToZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferToZeroAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CognitohazardErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccountBalanceOverflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceQueryForZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotCursedGoAway(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotLime(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotOwnerNorApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Soulbound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFromIncorrectOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferToNonERC721ReceiverImplementer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferToZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for CognitohazardErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AccountBalanceOverflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BalanceQueryForZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotCursedGoAway as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotLime as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotOwnerNorApproved as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Soulbound as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <TokenAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferFromIncorrectOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferToNonERC721ReceiverImplementer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferToZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for CognitohazardErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountBalanceOverflow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalanceQueryForZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotCursedGoAway(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotLime(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotOwnerNorApproved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Soulbound(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenDoesNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFromIncorrectOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferToNonERC721ReceiverImplementer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferToZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for CognitohazardErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccountBalanceOverflow> for CognitohazardErrors {
        fn from(value: AccountBalanceOverflow) -> Self {
            Self::AccountBalanceOverflow(value)
        }
    }
    impl ::core::convert::From<BalanceQueryForZeroAddress> for CognitohazardErrors {
        fn from(value: BalanceQueryForZeroAddress) -> Self {
            Self::BalanceQueryForZeroAddress(value)
        }
    }
    impl ::core::convert::From<NotCursedGoAway> for CognitohazardErrors {
        fn from(value: NotCursedGoAway) -> Self {
            Self::NotCursedGoAway(value)
        }
    }
    impl ::core::convert::From<NotLime> for CognitohazardErrors {
        fn from(value: NotLime) -> Self {
            Self::NotLime(value)
        }
    }
    impl ::core::convert::From<NotOwnerNorApproved> for CognitohazardErrors {
        fn from(value: NotOwnerNorApproved) -> Self {
            Self::NotOwnerNorApproved(value)
        }
    }
    impl ::core::convert::From<Soulbound> for CognitohazardErrors {
        fn from(value: Soulbound) -> Self {
            Self::Soulbound(value)
        }
    }
    impl ::core::convert::From<TokenAlreadyExists> for CognitohazardErrors {
        fn from(value: TokenAlreadyExists) -> Self {
            Self::TokenAlreadyExists(value)
        }
    }
    impl ::core::convert::From<TokenDoesNotExist> for CognitohazardErrors {
        fn from(value: TokenDoesNotExist) -> Self {
            Self::TokenDoesNotExist(value)
        }
    }
    impl ::core::convert::From<TransferFromIncorrectOwner> for CognitohazardErrors {
        fn from(value: TransferFromIncorrectOwner) -> Self {
            Self::TransferFromIncorrectOwner(value)
        }
    }
    impl ::core::convert::From<TransferToNonERC721ReceiverImplementer>
    for CognitohazardErrors {
        fn from(value: TransferToNonERC721ReceiverImplementer) -> Self {
            Self::TransferToNonERC721ReceiverImplementer(value)
        }
    }
    impl ::core::convert::From<TransferToZeroAddress> for CognitohazardErrors {
        fn from(value: TransferToZeroAddress) -> Self {
            Self::TransferToZeroAddress(value)
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub is_approved: bool,
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
    #[ethevent(name = "Cleansed", abi = "Cleansed(address)")]
    pub struct CleansedFilter {
        #[ethevent(indexed)]
        pub soul: ::ethers::core::types::Address,
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
    #[ethevent(name = "Marked", abi = "Marked(address)")]
    pub struct MarkedFilter {
        #[ethevent(indexed)]
        pub victim: ::ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
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
    pub enum CognitohazardEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        CleansedFilter(CleansedFilter),
        MarkedFilter(MarkedFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for CognitohazardEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CognitohazardEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(CognitohazardEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = CleansedFilter::decode_log(log) {
                return Ok(CognitohazardEvents::CleansedFilter(decoded));
            }
            if let Ok(decoded) = MarkedFilter::decode_log(log) {
                return Ok(CognitohazardEvents::MarkedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CognitohazardEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CognitohazardEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CleansedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarkedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for CognitohazardEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for CognitohazardEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<CleansedFilter> for CognitohazardEvents {
        fn from(value: CleansedFilter) -> Self {
            Self::CleansedFilter(value)
        }
    }
    impl ::core::convert::From<MarkedFilter> for CognitohazardEvents {
        fn from(value: MarkedFilter) -> Self {
            Self::MarkedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for CognitohazardEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `LIME` function with signature `LIME()` and selector `0xea8d2b79`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "LIME", abi = "LIME()")]
    pub struct LimeCall;
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub account: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
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
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `cleanseFee` function with signature `cleanseFee()` and selector `0x46a42afb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "cleanseFee", abi = "cleanseFee()")]
    pub struct CleanseFeeCall;
    ///Container type for all input parameters for the `curse` function with signature `curse(address)` and selector `0x7773260d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "curse", abi = "curse(address)")]
    pub struct CurseCall {
        pub victim: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `makeItStop` function with signature `makeItStop()` and selector `0x8602e6ca`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "makeItStop", abi = "makeItStop()")]
    pub struct MakeItStopCall;
    ///Container type for all input parameters for the `mark` function with signature `mark(address)` and selector `0x7ceeb880`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mark", abi = "mark(address)")]
    pub struct MarkCall {
        pub victim: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndIdCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub is_approved: bool,
    }
    ///Container type for all input parameters for the `setCleanseFee` function with signature `setCleanseFee(uint256)` and selector `0xc1a481d3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setCleanseFee", abi = "setCleanseFee(uint256)")]
    pub struct SetCleanseFeeCall {
        pub cleanse_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setUriSource` function with signature `setUriSource(string)` and selector `0x605352e0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setUriSource", abi = "setUriSource(string)")]
    pub struct SetUriSourceCall {
        pub uri: ::std::string::String,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `tokenIds` function with signature `tokenIds()` and selector `0x714cff56`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenIds", abi = "tokenIds()")]
    pub struct TokenIdsCall;
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `uriSource` function with signature `uriSource()` and selector `0x7922be37`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "uriSource", abi = "uriSource()")]
    pub struct UriSourceCall;
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
    pub enum CognitohazardCalls {
        Lime(LimeCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        CleanseFee(CleanseFeeCall),
        Curse(CurseCall),
        GetApproved(GetApprovedCall),
        IsApprovedForAll(IsApprovedForAllCall),
        MakeItStop(MakeItStopCall),
        Mark(MarkCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndId(SafeTransferFromWithFromAndToAndIdCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SetCleanseFee(SetCleanseFeeCall),
        SetUriSource(SetUriSourceCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenIds(TokenIdsCall),
        TokenURI(TokenURICall),
        TransferFrom(TransferFromCall),
        UriSource(UriSourceCall),
    }
    impl ::ethers::core::abi::AbiDecode for CognitohazardCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <LimeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Lime(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <CleanseFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CleanseFee(decoded));
            }
            if let Ok(decoded) = <CurseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Curse(decoded));
            }
            if let Ok(decoded) = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded) = <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <MakeItStopCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MakeItStop(decoded));
            }
            if let Ok(decoded) = <MarkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mark(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromWithFromAndToAndIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFromWithFromAndToAndId(decoded));
            }
            if let Ok(decoded) = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) = <SetCleanseFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetCleanseFee(decoded));
            }
            if let Ok(decoded) = <SetUriSourceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUriSource(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TokenIdsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenIds(decoded));
            }
            if let Ok(decoded) = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenURI(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <UriSourceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UriSource(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CognitohazardCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Lime(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CleanseFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Curse(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsApprovedForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MakeItStop(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mark(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFromWithFromAndToAndId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCleanseFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUriSource(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenIds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UriSource(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CognitohazardCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Lime(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::CleanseFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Curse(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakeItStop(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mark(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCleanseFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUriSource(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::UriSource(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LimeCall> for CognitohazardCalls {
        fn from(value: LimeCall) -> Self {
            Self::Lime(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for CognitohazardCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for CognitohazardCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<CleanseFeeCall> for CognitohazardCalls {
        fn from(value: CleanseFeeCall) -> Self {
            Self::CleanseFee(value)
        }
    }
    impl ::core::convert::From<CurseCall> for CognitohazardCalls {
        fn from(value: CurseCall) -> Self {
            Self::Curse(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for CognitohazardCalls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for CognitohazardCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<MakeItStopCall> for CognitohazardCalls {
        fn from(value: MakeItStopCall) -> Self {
            Self::MakeItStop(value)
        }
    }
    impl ::core::convert::From<MarkCall> for CognitohazardCalls {
        fn from(value: MarkCall) -> Self {
            Self::Mark(value)
        }
    }
    impl ::core::convert::From<NameCall> for CognitohazardCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for CognitohazardCalls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for CognitohazardCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndIdCall>
    for CognitohazardCalls {
        fn from(value: SafeTransferFromWithFromAndToAndIdCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndId(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for CognitohazardCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SetCleanseFeeCall> for CognitohazardCalls {
        fn from(value: SetCleanseFeeCall) -> Self {
            Self::SetCleanseFee(value)
        }
    }
    impl ::core::convert::From<SetUriSourceCall> for CognitohazardCalls {
        fn from(value: SetUriSourceCall) -> Self {
            Self::SetUriSource(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for CognitohazardCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for CognitohazardCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenIdsCall> for CognitohazardCalls {
        fn from(value: TokenIdsCall) -> Self {
            Self::TokenIds(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for CognitohazardCalls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for CognitohazardCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<UriSourceCall> for CognitohazardCalls {
        fn from(value: UriSourceCall) -> Self {
            Self::UriSource(value)
        }
    }
    ///Container type for all return fields from the `LIME` function with signature `LIME()` and selector `0xea8d2b79`
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
    pub struct LimeReturn(pub ::ethers::core::types::Address);
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
    pub struct BalanceOfReturn {
        pub result: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `cleanseFee` function with signature `cleanseFee()` and selector `0x46a42afb`
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
    pub struct CleanseFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `curse` function with signature `curse(address)` and selector `0x7773260d`
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
    pub struct CurseReturn {
        pub mark_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    pub struct GetApprovedReturn {
        pub result: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    pub struct IsApprovedForAllReturn {
        pub result: bool,
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    pub struct OwnerOfReturn {
        pub result: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn {
        pub result: bool,
    }
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `tokenIds` function with signature `tokenIds()` and selector `0x714cff56`
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
    pub struct TokenIdsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    pub struct TokenURIReturn(pub ::std::string::String);
    ///Container type for all return fields from the `uriSource` function with signature `uriSource()` and selector `0x7922be37`
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
    pub struct UriSourceReturn(pub ::std::string::String);
}
