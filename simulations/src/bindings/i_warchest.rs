pub use i_warchest::*;
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
pub mod i_warchest {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                    ::std::borrow::ToOwned::to_owned("underlyingBalanceWithInvestment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "underlyingBalanceWithInvestment",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("withdrawReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawReserves"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_destination"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountToWithdraw"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CircuitBreakerActivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CircuitBreakerActivated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                    ::std::borrow::ToOwned::to_owned("Invested"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Invested"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
    pub static IWARCHEST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IWarchest<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IWarchest<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IWarchest<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IWarchest<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IWarchest<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IWarchest)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IWarchest<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IWARCHEST_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `burn` (0x9dc29fac) function
        pub fn burn(
            &self,
            from: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 194, 159, 172], (from, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x40c10f19) function
        pub fn mint(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `underlyingBalanceWithInvestment` (0x1bf8e7be) function
        pub fn underlying_balance_with_investment(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([27, 248, 231, 190], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawReserves` (0xe478795d) function
        pub fn withdraw_reserves(
            &self,
            destination: ::ethers::core::types::Address,
            amount_to_withdraw: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 120, 121, 93], (destination, amount_to_withdraw))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CircuitBreakerActivated` event
        pub fn circuit_breaker_activated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CircuitBreakerActivatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Invested` event
        pub fn invested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InvestedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IWarchestEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IWarchest<M> {
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
    #[ethevent(
        name = "CircuitBreakerActivated",
        abi = "CircuitBreakerActivated(uint256)"
    )]
    pub struct CircuitBreakerActivatedFilter {
        pub timestamp: ::ethers::core::types::U256,
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
    #[ethevent(name = "Invested", abi = "Invested(address,uint256)")]
    pub struct InvestedFilter {
        #[ethevent(indexed)]
        pub strategy: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub enum IWarchestEvents {
        CircuitBreakerActivatedFilter(CircuitBreakerActivatedFilter),
        InvestedFilter(InvestedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IWarchestEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CircuitBreakerActivatedFilter::decode_log(log) {
                return Ok(IWarchestEvents::CircuitBreakerActivatedFilter(decoded));
            }
            if let Ok(decoded) = InvestedFilter::decode_log(log) {
                return Ok(IWarchestEvents::InvestedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IWarchestEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CircuitBreakerActivatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvestedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CircuitBreakerActivatedFilter> for IWarchestEvents {
        fn from(value: CircuitBreakerActivatedFilter) -> Self {
            Self::CircuitBreakerActivatedFilter(value)
        }
    }
    impl ::core::convert::From<InvestedFilter> for IWarchestEvents {
        fn from(value: InvestedFilter) -> Self {
            Self::InvestedFilter(value)
        }
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(address,uint256)` and selector `0x9dc29fac`
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
    #[ethcall(name = "burn", abi = "burn(address,uint256)")]
    pub struct BurnCall {
        pub from: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `0x40c10f19`
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
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `underlyingBalanceWithInvestment` function with signature `underlyingBalanceWithInvestment()` and selector `0x1bf8e7be`
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
        name = "underlyingBalanceWithInvestment",
        abi = "underlyingBalanceWithInvestment()"
    )]
    pub struct UnderlyingBalanceWithInvestmentCall;
    ///Container type for all input parameters for the `withdrawReserves` function with signature `withdrawReserves(address,uint256)` and selector `0xe478795d`
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
    #[ethcall(name = "withdrawReserves", abi = "withdrawReserves(address,uint256)")]
    pub struct WithdrawReservesCall {
        pub destination: ::ethers::core::types::Address,
        pub amount_to_withdraw: ::ethers::core::types::U256,
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
    pub enum IWarchestCalls {
        Burn(BurnCall),
        Mint(MintCall),
        UnderlyingBalanceWithInvestment(UnderlyingBalanceWithInvestmentCall),
        WithdrawReserves(WithdrawReservesCall),
    }
    impl ::ethers::core::abi::AbiDecode for IWarchestCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) = <UnderlyingBalanceWithInvestmentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnderlyingBalanceWithInvestment(decoded));
            }
            if let Ok(decoded) = <WithdrawReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawReserves(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IWarchestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnderlyingBalanceWithInvestment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IWarchestCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyingBalanceWithInvestment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawReserves(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BurnCall> for IWarchestCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<MintCall> for IWarchestCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<UnderlyingBalanceWithInvestmentCall> for IWarchestCalls {
        fn from(value: UnderlyingBalanceWithInvestmentCall) -> Self {
            Self::UnderlyingBalanceWithInvestment(value)
        }
    }
    impl ::core::convert::From<WithdrawReservesCall> for IWarchestCalls {
        fn from(value: WithdrawReservesCall) -> Self {
            Self::WithdrawReserves(value)
        }
    }
    ///Container type for all return fields from the `underlyingBalanceWithInvestment` function with signature `underlyingBalanceWithInvestment()` and selector `0x1bf8e7be`
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
    pub struct UnderlyingBalanceWithInvestmentReturn(pub ::ethers::core::types::U256);
}
