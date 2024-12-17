pub use i_safe_ownable_internal::*;
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
pub mod i_safe_ownable_internal {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::core::convert::From::from([
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Ownable__NotOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Ownable__NotOwner"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Ownable__NotTransitiveOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Ownable__NotTransitiveOwner",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeOwnable__NotNomineeOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeOwnable__NotNomineeOwner",
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
    pub static ISAFEOWNABLEINTERNAL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct ISafeOwnableInternal<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISafeOwnableInternal<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISafeOwnableInternal<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISafeOwnableInternal<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISafeOwnableInternal<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ISafeOwnableInternal))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISafeOwnableInternal<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISAFEOWNABLEINTERNAL_ABI.clone(),
                    client,
                ),
            )
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
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ISafeOwnableInternal<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Ownable__NotOwner` with signature `Ownable__NotOwner()` and selector `0x2f7a8ee1`
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
    #[etherror(name = "Ownable__NotOwner", abi = "Ownable__NotOwner()")]
    pub struct Ownable__NotOwner;
    ///Custom Error type `Ownable__NotTransitiveOwner` with signature `Ownable__NotTransitiveOwner()` and selector `0xd0b49847`
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
        name = "Ownable__NotTransitiveOwner",
        abi = "Ownable__NotTransitiveOwner()"
    )]
    pub struct Ownable__NotTransitiveOwner;
    ///Custom Error type `SafeOwnable__NotNomineeOwner` with signature `SafeOwnable__NotNomineeOwner()` and selector `0xefd1052d`
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
        name = "SafeOwnable__NotNomineeOwner",
        abi = "SafeOwnable__NotNomineeOwner()"
    )]
    pub struct SafeOwnable__NotNomineeOwner;
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
    pub enum ISafeOwnableInternalErrors {
        Ownable__NotOwner(Ownable__NotOwner),
        Ownable__NotTransitiveOwner(Ownable__NotTransitiveOwner),
        SafeOwnable__NotNomineeOwner(SafeOwnable__NotNomineeOwner),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ISafeOwnableInternalErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Ownable__NotOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ownable__NotOwner(decoded));
            }
            if let Ok(decoded) = <Ownable__NotTransitiveOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ownable__NotTransitiveOwner(decoded));
            }
            if let Ok(decoded) = <SafeOwnable__NotNomineeOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeOwnable__NotNomineeOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISafeOwnableInternalErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Ownable__NotOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ownable__NotTransitiveOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeOwnable__NotNomineeOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ISafeOwnableInternalErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Ownable__NotOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Ownable__NotTransitiveOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeOwnable__NotNomineeOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ISafeOwnableInternalErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Ownable__NotOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ownable__NotTransitiveOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeOwnable__NotNomineeOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ISafeOwnableInternalErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Ownable__NotOwner> for ISafeOwnableInternalErrors {
        fn from(value: Ownable__NotOwner) -> Self {
            Self::Ownable__NotOwner(value)
        }
    }
    impl ::core::convert::From<Ownable__NotTransitiveOwner>
    for ISafeOwnableInternalErrors {
        fn from(value: Ownable__NotTransitiveOwner) -> Self {
            Self::Ownable__NotTransitiveOwner(value)
        }
    }
    impl ::core::convert::From<SafeOwnable__NotNomineeOwner>
    for ISafeOwnableInternalErrors {
        fn from(value: SafeOwnable__NotNomineeOwner) -> Self {
            Self::SafeOwnable__NotNomineeOwner(value)
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
}
