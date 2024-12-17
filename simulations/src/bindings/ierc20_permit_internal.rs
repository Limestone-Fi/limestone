pub use ierc20_permit_internal::*;
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
pub mod ierc20_permit_internal {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ERC20Permit__ExpiredDeadline"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Permit__ExpiredDeadline",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20Permit__InvalidSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20Permit__InvalidSignature",
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
    pub static IERC20PERMITINTERNAL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IERC20PermitInternal<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC20PermitInternal<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC20PermitInternal<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC20PermitInternal<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC20PermitInternal<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IERC20PermitInternal))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC20PermitInternal<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IERC20PERMITINTERNAL_ABI.clone(),
                    client,
                ),
            )
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IERC20PermitInternal<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ERC20Permit__ExpiredDeadline` with signature `ERC20Permit__ExpiredDeadline()` and selector `0xe13df394`
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
        name = "ERC20Permit__ExpiredDeadline",
        abi = "ERC20Permit__ExpiredDeadline()"
    )]
    pub struct ERC20Permit__ExpiredDeadline;
    ///Custom Error type `ERC20Permit__InvalidSignature` with signature `ERC20Permit__InvalidSignature()` and selector `0xeae00e56`
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
        name = "ERC20Permit__InvalidSignature",
        abi = "ERC20Permit__InvalidSignature()"
    )]
    pub struct ERC20Permit__InvalidSignature;
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
    pub enum IERC20PermitInternalErrors {
        ERC20Permit__ExpiredDeadline(ERC20Permit__ExpiredDeadline),
        ERC20Permit__InvalidSignature(ERC20Permit__InvalidSignature),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IERC20PermitInternalErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ERC20Permit__ExpiredDeadline as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Permit__ExpiredDeadline(decoded));
            }
            if let Ok(decoded) = <ERC20Permit__InvalidSignature as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20Permit__InvalidSignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IERC20PermitInternalErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC20Permit__ExpiredDeadline(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20Permit__InvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IERC20PermitInternalErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC20Permit__ExpiredDeadline as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20Permit__InvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IERC20PermitInternalErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC20Permit__ExpiredDeadline(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20Permit__InvalidSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IERC20PermitInternalErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC20Permit__ExpiredDeadline>
    for IERC20PermitInternalErrors {
        fn from(value: ERC20Permit__ExpiredDeadline) -> Self {
            Self::ERC20Permit__ExpiredDeadline(value)
        }
    }
    impl ::core::convert::From<ERC20Permit__InvalidSignature>
    for IERC20PermitInternalErrors {
        fn from(value: ERC20Permit__InvalidSignature) -> Self {
            Self::ERC20Permit__InvalidSignature(value)
        }
    }
}
