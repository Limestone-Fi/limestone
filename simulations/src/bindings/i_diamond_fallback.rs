pub use i_diamond_fallback::*;
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
pub mod i_diamond_fallback {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getFallbackAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFallbackAddress"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackAddress"),
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
                    ::std::borrow::ToOwned::to_owned("setFallbackAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFallbackAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackAddress"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "Proxy__ImplementationIsNotContract",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Proxy__ImplementationIsNotContract",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IDIAMONDFALLBACK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IDiamondFallback<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IDiamondFallback<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IDiamondFallback<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IDiamondFallback<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IDiamondFallback<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IDiamondFallback))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IDiamondFallback<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IDIAMONDFALLBACK_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getFallbackAddress` (0x2c408059) function
        pub fn get_fallback_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([44, 64, 128, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFallbackAddress` (0x91423765) function
        pub fn set_fallback_address(
            &self,
            fallback_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 66, 55, 101], fallback_address)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IDiamondFallback<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Proxy__ImplementationIsNotContract` with signature `Proxy__ImplementationIsNotContract()` and selector `0x87c9fc34`
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
        name = "Proxy__ImplementationIsNotContract",
        abi = "Proxy__ImplementationIsNotContract()"
    )]
    pub struct Proxy__ImplementationIsNotContract;
    ///Container type for all input parameters for the `getFallbackAddress` function with signature `getFallbackAddress()` and selector `0x2c408059`
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
    #[ethcall(name = "getFallbackAddress", abi = "getFallbackAddress()")]
    pub struct GetFallbackAddressCall;
    ///Container type for all input parameters for the `setFallbackAddress` function with signature `setFallbackAddress(address)` and selector `0x91423765`
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
    #[ethcall(name = "setFallbackAddress", abi = "setFallbackAddress(address)")]
    pub struct SetFallbackAddressCall {
        pub fallback_address: ::ethers::core::types::Address,
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
    pub enum IDiamondFallbackCalls {
        GetFallbackAddress(GetFallbackAddressCall),
        SetFallbackAddress(SetFallbackAddressCall),
    }
    impl ::ethers::core::abi::AbiDecode for IDiamondFallbackCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetFallbackAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFallbackAddress(decoded));
            }
            if let Ok(decoded) = <SetFallbackAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFallbackAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IDiamondFallbackCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetFallbackAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFallbackAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IDiamondFallbackCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetFallbackAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFallbackAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetFallbackAddressCall> for IDiamondFallbackCalls {
        fn from(value: GetFallbackAddressCall) -> Self {
            Self::GetFallbackAddress(value)
        }
    }
    impl ::core::convert::From<SetFallbackAddressCall> for IDiamondFallbackCalls {
        fn from(value: SetFallbackAddressCall) -> Self {
            Self::SetFallbackAddress(value)
        }
    }
    ///Container type for all return fields from the `getFallbackAddress` function with signature `getFallbackAddress()` and selector `0x2c408059`
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
    pub struct GetFallbackAddressReturn {
        pub fallback_address: ::ethers::core::types::Address,
    }
}
