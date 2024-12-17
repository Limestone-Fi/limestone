pub use i_diamond_writable::*;
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
pub mod i_diamond_writable {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("diamondCut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("diamondCut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("facetCuts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IERC2535DiamondCutInternal.FacetCut[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("DiamondCut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DiamondCut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("facetCuts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "DiamondWritable__InvalidInitializationParameters",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DiamondWritable__InvalidInitializationParameters",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "DiamondWritable__RemoveTargetNotZeroAddress",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DiamondWritable__RemoveTargetNotZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "DiamondWritable__ReplaceTargetIsIdentical",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DiamondWritable__ReplaceTargetIsIdentical",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "DiamondWritable__SelectorAlreadyAdded",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DiamondWritable__SelectorAlreadyAdded",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "DiamondWritable__SelectorIsImmutable",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DiamondWritable__SelectorIsImmutable",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "DiamondWritable__SelectorNotFound",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DiamondWritable__SelectorNotFound",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "DiamondWritable__SelectorNotSpecified",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DiamondWritable__SelectorNotSpecified",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DiamondWritable__TargetHasNoCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DiamondWritable__TargetHasNoCode",
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
    pub static IDIAMONDWRITABLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IDiamondWritable<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IDiamondWritable<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IDiamondWritable<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IDiamondWritable<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IDiamondWritable<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IDiamondWritable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IDiamondWritable<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IDIAMONDWRITABLE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `diamondCut` (0x1f931c1c) function
        pub fn diamond_cut(
            &self,
            facet_cuts: ::std::vec::Vec<FacetCut>,
            target: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 147, 28, 28], (facet_cuts, target, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DiamondCut` event
        pub fn diamond_cut_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DiamondCutFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DiamondCutFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IDiamondWritable<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DiamondWritable__InvalidInitializationParameters` with signature `DiamondWritable__InvalidInitializationParameters()` and selector `0x26df4ccd`
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
        name = "DiamondWritable__InvalidInitializationParameters",
        abi = "DiamondWritable__InvalidInitializationParameters()"
    )]
    pub struct DiamondWritable__InvalidInitializationParameters;
    ///Custom Error type `DiamondWritable__RemoveTargetNotZeroAddress` with signature `DiamondWritable__RemoveTargetNotZeroAddress()` and selector `0xeacd2424`
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
        name = "DiamondWritable__RemoveTargetNotZeroAddress",
        abi = "DiamondWritable__RemoveTargetNotZeroAddress()"
    )]
    pub struct DiamondWritable__RemoveTargetNotZeroAddress;
    ///Custom Error type `DiamondWritable__ReplaceTargetIsIdentical` with signature `DiamondWritable__ReplaceTargetIsIdentical()` and selector `0x617557e6`
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
        name = "DiamondWritable__ReplaceTargetIsIdentical",
        abi = "DiamondWritable__ReplaceTargetIsIdentical()"
    )]
    pub struct DiamondWritable__ReplaceTargetIsIdentical;
    ///Custom Error type `DiamondWritable__SelectorAlreadyAdded` with signature `DiamondWritable__SelectorAlreadyAdded()` and selector `0x92474ee2`
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
        name = "DiamondWritable__SelectorAlreadyAdded",
        abi = "DiamondWritable__SelectorAlreadyAdded()"
    )]
    pub struct DiamondWritable__SelectorAlreadyAdded;
    ///Custom Error type `DiamondWritable__SelectorIsImmutable` with signature `DiamondWritable__SelectorIsImmutable()` and selector `0xe9835731`
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
        name = "DiamondWritable__SelectorIsImmutable",
        abi = "DiamondWritable__SelectorIsImmutable()"
    )]
    pub struct DiamondWritable__SelectorIsImmutable;
    ///Custom Error type `DiamondWritable__SelectorNotFound` with signature `DiamondWritable__SelectorNotFound()` and selector `0x6fc4b52e`
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
        name = "DiamondWritable__SelectorNotFound",
        abi = "DiamondWritable__SelectorNotFound()"
    )]
    pub struct DiamondWritable__SelectorNotFound;
    ///Custom Error type `DiamondWritable__SelectorNotSpecified` with signature `DiamondWritable__SelectorNotSpecified()` and selector `0xeb6c3aeb`
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
        name = "DiamondWritable__SelectorNotSpecified",
        abi = "DiamondWritable__SelectorNotSpecified()"
    )]
    pub struct DiamondWritable__SelectorNotSpecified;
    ///Custom Error type `DiamondWritable__TargetHasNoCode` with signature `DiamondWritable__TargetHasNoCode()` and selector `0xf77172ac`
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
        name = "DiamondWritable__TargetHasNoCode",
        abi = "DiamondWritable__TargetHasNoCode()"
    )]
    pub struct DiamondWritable__TargetHasNoCode;
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
    pub enum IDiamondWritableErrors {
        DiamondWritable__InvalidInitializationParameters(
            DiamondWritable__InvalidInitializationParameters,
        ),
        DiamondWritable__RemoveTargetNotZeroAddress(
            DiamondWritable__RemoveTargetNotZeroAddress,
        ),
        DiamondWritable__ReplaceTargetIsIdentical(
            DiamondWritable__ReplaceTargetIsIdentical,
        ),
        DiamondWritable__SelectorAlreadyAdded(DiamondWritable__SelectorAlreadyAdded),
        DiamondWritable__SelectorIsImmutable(DiamondWritable__SelectorIsImmutable),
        DiamondWritable__SelectorNotFound(DiamondWritable__SelectorNotFound),
        DiamondWritable__SelectorNotSpecified(DiamondWritable__SelectorNotSpecified),
        DiamondWritable__TargetHasNoCode(DiamondWritable__TargetHasNoCode),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IDiamondWritableErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DiamondWritable__InvalidInitializationParameters as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::DiamondWritable__InvalidInitializationParameters(decoded),
                );
            }
            if let Ok(decoded) = <DiamondWritable__RemoveTargetNotZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DiamondWritable__RemoveTargetNotZeroAddress(decoded));
            }
            if let Ok(decoded) = <DiamondWritable__ReplaceTargetIsIdentical as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DiamondWritable__ReplaceTargetIsIdentical(decoded));
            }
            if let Ok(decoded) = <DiamondWritable__SelectorAlreadyAdded as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DiamondWritable__SelectorAlreadyAdded(decoded));
            }
            if let Ok(decoded) = <DiamondWritable__SelectorIsImmutable as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DiamondWritable__SelectorIsImmutable(decoded));
            }
            if let Ok(decoded) = <DiamondWritable__SelectorNotFound as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DiamondWritable__SelectorNotFound(decoded));
            }
            if let Ok(decoded) = <DiamondWritable__SelectorNotSpecified as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DiamondWritable__SelectorNotSpecified(decoded));
            }
            if let Ok(decoded) = <DiamondWritable__TargetHasNoCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DiamondWritable__TargetHasNoCode(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IDiamondWritableErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DiamondWritable__InvalidInitializationParameters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DiamondWritable__RemoveTargetNotZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DiamondWritable__ReplaceTargetIsIdentical(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DiamondWritable__SelectorAlreadyAdded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DiamondWritable__SelectorIsImmutable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DiamondWritable__SelectorNotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DiamondWritable__SelectorNotSpecified(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DiamondWritable__TargetHasNoCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IDiamondWritableErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DiamondWritable__InvalidInitializationParameters as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DiamondWritable__RemoveTargetNotZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DiamondWritable__ReplaceTargetIsIdentical as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DiamondWritable__SelectorAlreadyAdded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DiamondWritable__SelectorIsImmutable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DiamondWritable__SelectorNotFound as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DiamondWritable__SelectorNotSpecified as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DiamondWritable__TargetHasNoCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IDiamondWritableErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DiamondWritable__InvalidInitializationParameters(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DiamondWritable__RemoveTargetNotZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DiamondWritable__ReplaceTargetIsIdentical(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DiamondWritable__SelectorAlreadyAdded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DiamondWritable__SelectorIsImmutable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DiamondWritable__SelectorNotFound(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DiamondWritable__SelectorNotSpecified(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DiamondWritable__TargetHasNoCode(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IDiamondWritableErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__InvalidInitializationParameters>
    for IDiamondWritableErrors {
        fn from(value: DiamondWritable__InvalidInitializationParameters) -> Self {
            Self::DiamondWritable__InvalidInitializationParameters(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__RemoveTargetNotZeroAddress>
    for IDiamondWritableErrors {
        fn from(value: DiamondWritable__RemoveTargetNotZeroAddress) -> Self {
            Self::DiamondWritable__RemoveTargetNotZeroAddress(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__ReplaceTargetIsIdentical>
    for IDiamondWritableErrors {
        fn from(value: DiamondWritable__ReplaceTargetIsIdentical) -> Self {
            Self::DiamondWritable__ReplaceTargetIsIdentical(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__SelectorAlreadyAdded>
    for IDiamondWritableErrors {
        fn from(value: DiamondWritable__SelectorAlreadyAdded) -> Self {
            Self::DiamondWritable__SelectorAlreadyAdded(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__SelectorIsImmutable>
    for IDiamondWritableErrors {
        fn from(value: DiamondWritable__SelectorIsImmutable) -> Self {
            Self::DiamondWritable__SelectorIsImmutable(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__SelectorNotFound>
    for IDiamondWritableErrors {
        fn from(value: DiamondWritable__SelectorNotFound) -> Self {
            Self::DiamondWritable__SelectorNotFound(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__SelectorNotSpecified>
    for IDiamondWritableErrors {
        fn from(value: DiamondWritable__SelectorNotSpecified) -> Self {
            Self::DiamondWritable__SelectorNotSpecified(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__TargetHasNoCode>
    for IDiamondWritableErrors {
        fn from(value: DiamondWritable__TargetHasNoCode) -> Self {
            Self::DiamondWritable__TargetHasNoCode(value)
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
    #[ethevent(
        name = "DiamondCut",
        abi = "DiamondCut((address,uint8,bytes4[])[],address,bytes)"
    )]
    pub struct DiamondCutFilter {
        pub facet_cuts: ::std::vec::Vec<FacetCut>,
        pub target: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `diamondCut` function with signature `diamondCut((address,uint8,bytes4[])[],address,bytes)` and selector `0x1f931c1c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "diamondCut",
        abi = "diamondCut((address,uint8,bytes4[])[],address,bytes)"
    )]
    pub struct DiamondCutCall {
        pub facet_cuts: ::std::vec::Vec<FacetCut>,
        pub target: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
}
