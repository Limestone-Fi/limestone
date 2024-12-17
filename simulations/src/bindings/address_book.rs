pub use address_book::*;
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
pub mod address_book {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AERO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("AERO"),
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
                    ::std::borrow::ToOwned::to_owned("AERO_ROUTER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("AERO_ROUTER"),
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
                    ::std::borrow::ToOwned::to_owned("AERO_USDC_GAUGE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("AERO_USDC_GAUGE"),
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
                    ::std::borrow::ToOwned::to_owned("AERO_VOTER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("AERO_VOTER"),
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
                    ::std::borrow::ToOwned::to_owned("BASE_USDC"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BASE_USDC"),
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
                    ::std::borrow::ToOwned::to_owned("ONETICKWARRIOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ONETICKWARRIOR"),
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
                    ::std::borrow::ToOwned::to_owned("WETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WETH"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ADDRESSBOOK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct AddressBook<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AddressBook<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AddressBook<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AddressBook<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AddressBook<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AddressBook))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AddressBook<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ADDRESSBOOK_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `AERO` (0x1691bd6f) function
        pub fn aero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 145, 189, 111], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `AERO_ROUTER` (0x3b9deaae) function
        pub fn aero_router(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([59, 157, 234, 174], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `AERO_USDC_GAUGE` (0xd71a7afe) function
        pub fn aero_usdc_gauge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([215, 26, 122, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `AERO_VOTER` (0xe9b037a7) function
        pub fn aero_voter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([233, 176, 55, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BASE_USDC` (0xbb9df680) function
        pub fn base_usdc(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([187, 157, 246, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ONETICKWARRIOR` (0x3d8de005) function
        pub fn onetickwarrior(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([61, 141, 224, 5], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WETH` (0xad5c4648) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AddressBook<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `AERO` function with signature `AERO()` and selector `0x1691bd6f`
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
    #[ethcall(name = "AERO", abi = "AERO()")]
    pub struct AeroCall;
    ///Container type for all input parameters for the `AERO_ROUTER` function with signature `AERO_ROUTER()` and selector `0x3b9deaae`
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
    #[ethcall(name = "AERO_ROUTER", abi = "AERO_ROUTER()")]
    pub struct AeroRouterCall;
    ///Container type for all input parameters for the `AERO_USDC_GAUGE` function with signature `AERO_USDC_GAUGE()` and selector `0xd71a7afe`
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
    #[ethcall(name = "AERO_USDC_GAUGE", abi = "AERO_USDC_GAUGE()")]
    pub struct AeroUsdcGaugeCall;
    ///Container type for all input parameters for the `AERO_VOTER` function with signature `AERO_VOTER()` and selector `0xe9b037a7`
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
    #[ethcall(name = "AERO_VOTER", abi = "AERO_VOTER()")]
    pub struct AeroVoterCall;
    ///Container type for all input parameters for the `BASE_USDC` function with signature `BASE_USDC()` and selector `0xbb9df680`
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
    #[ethcall(name = "BASE_USDC", abi = "BASE_USDC()")]
    pub struct BaseUsdcCall;
    ///Container type for all input parameters for the `ONETICKWARRIOR` function with signature `ONETICKWARRIOR()` and selector `0x3d8de005`
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
    #[ethcall(name = "ONETICKWARRIOR", abi = "ONETICKWARRIOR()")]
    pub struct OnetickwarriorCall;
    ///Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `0xad5c4648`
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
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
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
    pub enum AddressBookCalls {
        Aero(AeroCall),
        AeroRouter(AeroRouterCall),
        AeroUsdcGauge(AeroUsdcGaugeCall),
        AeroVoter(AeroVoterCall),
        BaseUsdc(BaseUsdcCall),
        Onetickwarrior(OnetickwarriorCall),
        Weth(WethCall),
    }
    impl ::ethers::core::abi::AbiDecode for AddressBookCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AeroCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Aero(decoded));
            }
            if let Ok(decoded) = <AeroRouterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AeroRouter(decoded));
            }
            if let Ok(decoded) = <AeroUsdcGaugeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AeroUsdcGauge(decoded));
            }
            if let Ok(decoded) = <AeroVoterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AeroVoter(decoded));
            }
            if let Ok(decoded) = <BaseUsdcCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseUsdc(decoded));
            }
            if let Ok(decoded) = <OnetickwarriorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Onetickwarrior(decoded));
            }
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Weth(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AddressBookCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Aero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AeroRouter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AeroUsdcGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AeroVoter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseUsdc(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Onetickwarrior(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for AddressBookCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Aero(element) => ::core::fmt::Display::fmt(element, f),
                Self::AeroRouter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AeroUsdcGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AeroVoter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseUsdc(element) => ::core::fmt::Display::fmt(element, f),
                Self::Onetickwarrior(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AeroCall> for AddressBookCalls {
        fn from(value: AeroCall) -> Self {
            Self::Aero(value)
        }
    }
    impl ::core::convert::From<AeroRouterCall> for AddressBookCalls {
        fn from(value: AeroRouterCall) -> Self {
            Self::AeroRouter(value)
        }
    }
    impl ::core::convert::From<AeroUsdcGaugeCall> for AddressBookCalls {
        fn from(value: AeroUsdcGaugeCall) -> Self {
            Self::AeroUsdcGauge(value)
        }
    }
    impl ::core::convert::From<AeroVoterCall> for AddressBookCalls {
        fn from(value: AeroVoterCall) -> Self {
            Self::AeroVoter(value)
        }
    }
    impl ::core::convert::From<BaseUsdcCall> for AddressBookCalls {
        fn from(value: BaseUsdcCall) -> Self {
            Self::BaseUsdc(value)
        }
    }
    impl ::core::convert::From<OnetickwarriorCall> for AddressBookCalls {
        fn from(value: OnetickwarriorCall) -> Self {
            Self::Onetickwarrior(value)
        }
    }
    impl ::core::convert::From<WethCall> for AddressBookCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    ///Container type for all return fields from the `AERO` function with signature `AERO()` and selector `0x1691bd6f`
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
    pub struct AeroReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `AERO_ROUTER` function with signature `AERO_ROUTER()` and selector `0x3b9deaae`
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
    pub struct AeroRouterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `AERO_USDC_GAUGE` function with signature `AERO_USDC_GAUGE()` and selector `0xd71a7afe`
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
    pub struct AeroUsdcGaugeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `AERO_VOTER` function with signature `AERO_VOTER()` and selector `0xe9b037a7`
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
    pub struct AeroVoterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `BASE_USDC` function with signature `BASE_USDC()` and selector `0xbb9df680`
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
    pub struct BaseUsdcReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ONETICKWARRIOR` function with signature `ONETICKWARRIOR()` and selector `0x3d8de005`
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
    pub struct OnetickwarriorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `WETH` function with signature `WETH()` and selector `0xad5c4648`
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
    pub struct WethReturn(pub ::ethers::core::types::Address);
}
