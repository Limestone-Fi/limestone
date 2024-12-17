pub use erc20_extended_internal::*;
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
pub mod erc20_extended_internal {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
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
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "ERC20Base__ApproveFromZeroAddress",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Base__ApproveFromZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20Base__ApproveToZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Base__ApproveToZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20Base__BurnExceedsBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Base__BurnExceedsBalance",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20Base__BurnFromZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Base__BurnFromZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20Base__InsufficientAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Base__InsufficientAllowance",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20Base__MintToZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Base__MintToZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ERC20Base__TransferExceedsBalance",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Base__TransferExceedsBalance",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ERC20Base__TransferFromZeroAddress",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Base__TransferFromZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20Base__TransferToZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Base__TransferToZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ERC20Extended__ExcessiveAllowance",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Extended__ExcessiveAllowance",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ERC20Extended__InsufficientAllowance",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Extended__InsufficientAllowance",
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
    pub static ERC20EXTENDEDINTERNAL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct ERC20ExtendedInternal<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC20ExtendedInternal<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC20ExtendedInternal<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC20ExtendedInternal<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC20ExtendedInternal<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC20ExtendedInternal))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC20ExtendedInternal<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC20EXTENDEDINTERNAL_ABI.clone(),
                    client,
                ),
            )
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
            ERC20ExtendedInternalEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC20ExtendedInternal<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ERC20Base__ApproveFromZeroAddress` with signature `ERC20Base__ApproveFromZeroAddress()` and selector `0x5a68b7ab`
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
        name = "ERC20Base__ApproveFromZeroAddress",
        abi = "ERC20Base__ApproveFromZeroAddress()"
    )]
    pub struct ERC20Base__ApproveFromZeroAddress;
    ///Custom Error type `ERC20Base__ApproveToZeroAddress` with signature `ERC20Base__ApproveToZeroAddress()` and selector `0xc5241600`
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
        name = "ERC20Base__ApproveToZeroAddress",
        abi = "ERC20Base__ApproveToZeroAddress()"
    )]
    pub struct ERC20Base__ApproveToZeroAddress;
    ///Custom Error type `ERC20Base__BurnExceedsBalance` with signature `ERC20Base__BurnExceedsBalance()` and selector `0xb1d35b23`
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
        name = "ERC20Base__BurnExceedsBalance",
        abi = "ERC20Base__BurnExceedsBalance()"
    )]
    pub struct ERC20Base__BurnExceedsBalance;
    ///Custom Error type `ERC20Base__BurnFromZeroAddress` with signature `ERC20Base__BurnFromZeroAddress()` and selector `0xf1e2c4ee`
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
        name = "ERC20Base__BurnFromZeroAddress",
        abi = "ERC20Base__BurnFromZeroAddress()"
    )]
    pub struct ERC20Base__BurnFromZeroAddress;
    ///Custom Error type `ERC20Base__InsufficientAllowance` with signature `ERC20Base__InsufficientAllowance()` and selector `0x01c2999e`
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
        name = "ERC20Base__InsufficientAllowance",
        abi = "ERC20Base__InsufficientAllowance()"
    )]
    pub struct ERC20Base__InsufficientAllowance;
    ///Custom Error type `ERC20Base__MintToZeroAddress` with signature `ERC20Base__MintToZeroAddress()` and selector `0xda007acd`
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
        name = "ERC20Base__MintToZeroAddress",
        abi = "ERC20Base__MintToZeroAddress()"
    )]
    pub struct ERC20Base__MintToZeroAddress;
    ///Custom Error type `ERC20Base__TransferExceedsBalance` with signature `ERC20Base__TransferExceedsBalance()` and selector `0x7183160b`
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
        name = "ERC20Base__TransferExceedsBalance",
        abi = "ERC20Base__TransferExceedsBalance()"
    )]
    pub struct ERC20Base__TransferExceedsBalance;
    ///Custom Error type `ERC20Base__TransferFromZeroAddress` with signature `ERC20Base__TransferFromZeroAddress()` and selector `0x68551d5f`
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
        name = "ERC20Base__TransferFromZeroAddress",
        abi = "ERC20Base__TransferFromZeroAddress()"
    )]
    pub struct ERC20Base__TransferFromZeroAddress;
    ///Custom Error type `ERC20Base__TransferToZeroAddress` with signature `ERC20Base__TransferToZeroAddress()` and selector `0x82899144`
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
        name = "ERC20Base__TransferToZeroAddress",
        abi = "ERC20Base__TransferToZeroAddress()"
    )]
    pub struct ERC20Base__TransferToZeroAddress;
    ///Custom Error type `ERC20Extended__ExcessiveAllowance` with signature `ERC20Extended__ExcessiveAllowance()` and selector `0xd256efb1`
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
        name = "ERC20Extended__ExcessiveAllowance",
        abi = "ERC20Extended__ExcessiveAllowance()"
    )]
    pub struct ERC20Extended__ExcessiveAllowance;
    ///Custom Error type `ERC20Extended__InsufficientAllowance` with signature `ERC20Extended__InsufficientAllowance()` and selector `0x5d2258de`
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
        name = "ERC20Extended__InsufficientAllowance",
        abi = "ERC20Extended__InsufficientAllowance()"
    )]
    pub struct ERC20Extended__InsufficientAllowance;
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
    pub enum ERC20ExtendedInternalErrors {
        ERC20Base__ApproveFromZeroAddress(ERC20Base__ApproveFromZeroAddress),
        ERC20Base__ApproveToZeroAddress(ERC20Base__ApproveToZeroAddress),
        ERC20Base__BurnExceedsBalance(ERC20Base__BurnExceedsBalance),
        ERC20Base__BurnFromZeroAddress(ERC20Base__BurnFromZeroAddress),
        ERC20Base__InsufficientAllowance(ERC20Base__InsufficientAllowance),
        ERC20Base__MintToZeroAddress(ERC20Base__MintToZeroAddress),
        ERC20Base__TransferExceedsBalance(ERC20Base__TransferExceedsBalance),
        ERC20Base__TransferFromZeroAddress(ERC20Base__TransferFromZeroAddress),
        ERC20Base__TransferToZeroAddress(ERC20Base__TransferToZeroAddress),
        ERC20Extended__ExcessiveAllowance(ERC20Extended__ExcessiveAllowance),
        ERC20Extended__InsufficientAllowance(ERC20Extended__InsufficientAllowance),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ERC20ExtendedInternalErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ERC20Base__ApproveFromZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Base__ApproveFromZeroAddress(decoded));
            }
            if let Ok(decoded) = <ERC20Base__ApproveToZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Base__ApproveToZeroAddress(decoded));
            }
            if let Ok(decoded) = <ERC20Base__BurnExceedsBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Base__BurnExceedsBalance(decoded));
            }
            if let Ok(decoded) = <ERC20Base__BurnFromZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Base__BurnFromZeroAddress(decoded));
            }
            if let Ok(decoded) = <ERC20Base__InsufficientAllowance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Base__InsufficientAllowance(decoded));
            }
            if let Ok(decoded) = <ERC20Base__MintToZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Base__MintToZeroAddress(decoded));
            }
            if let Ok(decoded) = <ERC20Base__TransferExceedsBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Base__TransferExceedsBalance(decoded));
            }
            if let Ok(decoded) = <ERC20Base__TransferFromZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Base__TransferFromZeroAddress(decoded));
            }
            if let Ok(decoded) = <ERC20Base__TransferToZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Base__TransferToZeroAddress(decoded));
            }
            if let Ok(decoded) = <ERC20Extended__ExcessiveAllowance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Extended__ExcessiveAllowance(decoded));
            }
            if let Ok(decoded) = <ERC20Extended__InsufficientAllowance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Extended__InsufficientAllowance(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC20ExtendedInternalErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC20Base__ApproveFromZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20Base__ApproveToZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20Base__BurnExceedsBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20Base__BurnFromZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20Base__InsufficientAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20Base__MintToZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20Base__TransferExceedsBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20Base__TransferFromZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20Base__TransferToZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20Extended__ExcessiveAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20Extended__InsufficientAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ERC20ExtendedInternalErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC20Base__ApproveFromZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20Base__ApproveToZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20Base__BurnExceedsBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20Base__BurnFromZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20Base__InsufficientAllowance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20Base__MintToZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20Base__TransferExceedsBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20Base__TransferFromZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20Base__TransferToZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20Extended__ExcessiveAllowance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20Extended__InsufficientAllowance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ERC20ExtendedInternalErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC20Base__ApproveFromZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20Base__ApproveToZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20Base__BurnExceedsBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20Base__BurnFromZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20Base__InsufficientAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20Base__MintToZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20Base__TransferExceedsBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20Base__TransferFromZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20Base__TransferToZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20Extended__ExcessiveAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20Extended__InsufficientAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ERC20ExtendedInternalErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC20Base__ApproveFromZeroAddress>
    for ERC20ExtendedInternalErrors {
        fn from(value: ERC20Base__ApproveFromZeroAddress) -> Self {
            Self::ERC20Base__ApproveFromZeroAddress(value)
        }
    }
    impl ::core::convert::From<ERC20Base__ApproveToZeroAddress>
    for ERC20ExtendedInternalErrors {
        fn from(value: ERC20Base__ApproveToZeroAddress) -> Self {
            Self::ERC20Base__ApproveToZeroAddress(value)
        }
    }
    impl ::core::convert::From<ERC20Base__BurnExceedsBalance>
    for ERC20ExtendedInternalErrors {
        fn from(value: ERC20Base__BurnExceedsBalance) -> Self {
            Self::ERC20Base__BurnExceedsBalance(value)
        }
    }
    impl ::core::convert::From<ERC20Base__BurnFromZeroAddress>
    for ERC20ExtendedInternalErrors {
        fn from(value: ERC20Base__BurnFromZeroAddress) -> Self {
            Self::ERC20Base__BurnFromZeroAddress(value)
        }
    }
    impl ::core::convert::From<ERC20Base__InsufficientAllowance>
    for ERC20ExtendedInternalErrors {
        fn from(value: ERC20Base__InsufficientAllowance) -> Self {
            Self::ERC20Base__InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<ERC20Base__MintToZeroAddress>
    for ERC20ExtendedInternalErrors {
        fn from(value: ERC20Base__MintToZeroAddress) -> Self {
            Self::ERC20Base__MintToZeroAddress(value)
        }
    }
    impl ::core::convert::From<ERC20Base__TransferExceedsBalance>
    for ERC20ExtendedInternalErrors {
        fn from(value: ERC20Base__TransferExceedsBalance) -> Self {
            Self::ERC20Base__TransferExceedsBalance(value)
        }
    }
    impl ::core::convert::From<ERC20Base__TransferFromZeroAddress>
    for ERC20ExtendedInternalErrors {
        fn from(value: ERC20Base__TransferFromZeroAddress) -> Self {
            Self::ERC20Base__TransferFromZeroAddress(value)
        }
    }
    impl ::core::convert::From<ERC20Base__TransferToZeroAddress>
    for ERC20ExtendedInternalErrors {
        fn from(value: ERC20Base__TransferToZeroAddress) -> Self {
            Self::ERC20Base__TransferToZeroAddress(value)
        }
    }
    impl ::core::convert::From<ERC20Extended__ExcessiveAllowance>
    for ERC20ExtendedInternalErrors {
        fn from(value: ERC20Extended__ExcessiveAllowance) -> Self {
            Self::ERC20Extended__ExcessiveAllowance(value)
        }
    }
    impl ::core::convert::From<ERC20Extended__InsufficientAllowance>
    for ERC20ExtendedInternalErrors {
        fn from(value: ERC20Extended__InsufficientAllowance) -> Self {
            Self::ERC20Extended__InsufficientAllowance(value)
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
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
        pub value: ::ethers::core::types::U256,
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
    pub enum ERC20ExtendedInternalEvents {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for ERC20ExtendedInternalEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC20ExtendedInternalEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC20ExtendedInternalEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC20ExtendedInternalEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ERC20ExtendedInternalEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ERC20ExtendedInternalEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
}
