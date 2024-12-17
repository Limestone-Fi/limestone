pub use i_factory_registry::*;
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
pub mod i_factory_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "votingRewardsFactory",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gaugeFactory"),
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
                    ::std::borrow::ToOwned::to_owned("factoriesToPoolFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "factoriesToPoolFactory",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "votingRewardsFactory",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gaugeFactory"),
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
                    ::std::borrow::ToOwned::to_owned("isPoolFactoryApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isPoolFactoryApproved",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("managedRewardsFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "managedRewardsFactory",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("poolFactories"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("poolFactories"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("poolFactoriesLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "poolFactoriesLength",
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
                    ::std::borrow::ToOwned::to_owned("setManagedRewardsFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setManagedRewardsFactory",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_newManagedRewardsFactory",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("unapprove"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unapprove"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolFactory"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "votingRewardsFactory",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gaugeFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetManagedRewardsFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SetManagedRewardsFactory",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_newRewardsFactory",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unapprove"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unapprove"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolFactory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "votingRewardsFactory",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gaugeFactory"),
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
                    ::std::borrow::ToOwned::to_owned("FallbackFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FallbackFactory"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFactoriesToPoolFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidFactoriesToPoolFactory",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PathAlreadyApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PathAlreadyApproved",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PathNotApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PathNotApproved"),
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
    pub static IFACTORYREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IFactoryRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IFactoryRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IFactoryRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IFactoryRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IFactoryRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IFactoryRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IFactoryRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IFACTORYREGISTRY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `approve` (0x9b140a85) function
        pub fn approve(
            &self,
            pool_factory: ::ethers::core::types::Address,
            voting_rewards_factory: ::ethers::core::types::Address,
            gauge_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [155, 20, 10, 133],
                    (pool_factory, voting_rewards_factory, gauge_factory),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factoriesToPoolFactory` (0x1217afdb) function
        pub fn factories_to_pool_factory(
            &self,
            pool_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::Address),
        > {
            self.0
                .method_hash([18, 23, 175, 219], pool_factory)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPoolFactoryApproved` (0xd1ea0a1d) function
        pub fn is_pool_factory_approved(
            &self,
            pool_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([209, 234, 10, 29], pool_factory)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `managedRewardsFactory` (0x0d0ae678) function
        pub fn managed_rewards_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([13, 10, 230, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolFactories` (0x06121cd5) function
        pub fn pool_factories(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([6, 18, 28, 213], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolFactoriesLength` (0x0cb299c9) function
        pub fn pool_factories_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([12, 178, 153, 201], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setManagedRewardsFactory` (0x64076939) function
        pub fn set_managed_rewards_factory(
            &self,
            new_managed_rewards_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 7, 105, 57], new_managed_rewards_factory)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unapprove` (0xfbf1f78a) function
        pub fn unapprove(
            &self,
            pool_factory: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 241, 247, 138], pool_factory)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approve` event
        pub fn approve_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApproveFilter> {
            self.0.event()
        }
        ///Gets the contract's `SetManagedRewardsFactory` event
        pub fn set_managed_rewards_factory_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetManagedRewardsFactoryFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unapprove` event
        pub fn unapprove_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnapproveFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IFactoryRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IFactoryRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `FallbackFactory` with signature `FallbackFactory()` and selector `0x1446b5be`
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
    #[etherror(name = "FallbackFactory", abi = "FallbackFactory()")]
    pub struct FallbackFactory;
    ///Custom Error type `InvalidFactoriesToPoolFactory` with signature `InvalidFactoriesToPoolFactory()` and selector `0x03580431`
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
        name = "InvalidFactoriesToPoolFactory",
        abi = "InvalidFactoriesToPoolFactory()"
    )]
    pub struct InvalidFactoriesToPoolFactory;
    ///Custom Error type `PathAlreadyApproved` with signature `PathAlreadyApproved()` and selector `0xc59dc2ea`
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
    #[etherror(name = "PathAlreadyApproved", abi = "PathAlreadyApproved()")]
    pub struct PathAlreadyApproved;
    ///Custom Error type `PathNotApproved` with signature `PathNotApproved()` and selector `0xd38afd65`
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
    #[etherror(name = "PathNotApproved", abi = "PathNotApproved()")]
    pub struct PathNotApproved;
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
    pub enum IFactoryRegistryErrors {
        FallbackFactory(FallbackFactory),
        InvalidFactoriesToPoolFactory(InvalidFactoriesToPoolFactory),
        PathAlreadyApproved(PathAlreadyApproved),
        PathNotApproved(PathNotApproved),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IFactoryRegistryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <FallbackFactory as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FallbackFactory(decoded));
            }
            if let Ok(decoded) = <InvalidFactoriesToPoolFactory as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidFactoriesToPoolFactory(decoded));
            }
            if let Ok(decoded) = <PathAlreadyApproved as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PathAlreadyApproved(decoded));
            }
            if let Ok(decoded) = <PathNotApproved as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PathNotApproved(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IFactoryRegistryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::FallbackFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFactoriesToPoolFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PathAlreadyApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PathNotApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IFactoryRegistryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <FallbackFactory as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFactoriesToPoolFactory as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PathAlreadyApproved as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PathNotApproved as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IFactoryRegistryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FallbackFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFactoriesToPoolFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PathAlreadyApproved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PathNotApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IFactoryRegistryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<FallbackFactory> for IFactoryRegistryErrors {
        fn from(value: FallbackFactory) -> Self {
            Self::FallbackFactory(value)
        }
    }
    impl ::core::convert::From<InvalidFactoriesToPoolFactory>
    for IFactoryRegistryErrors {
        fn from(value: InvalidFactoriesToPoolFactory) -> Self {
            Self::InvalidFactoriesToPoolFactory(value)
        }
    }
    impl ::core::convert::From<PathAlreadyApproved> for IFactoryRegistryErrors {
        fn from(value: PathAlreadyApproved) -> Self {
            Self::PathAlreadyApproved(value)
        }
    }
    impl ::core::convert::From<PathNotApproved> for IFactoryRegistryErrors {
        fn from(value: PathNotApproved) -> Self {
            Self::PathNotApproved(value)
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
    #[ethevent(name = "Approve", abi = "Approve(address,address,address)")]
    pub struct ApproveFilter {
        #[ethevent(indexed)]
        pub pool_factory: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub voting_rewards_factory: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge_factory: ::ethers::core::types::Address,
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
        name = "SetManagedRewardsFactory",
        abi = "SetManagedRewardsFactory(address)"
    )]
    pub struct SetManagedRewardsFactoryFilter {
        #[ethevent(indexed)]
        pub new_rewards_factory: ::ethers::core::types::Address,
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
    #[ethevent(name = "Unapprove", abi = "Unapprove(address,address,address)")]
    pub struct UnapproveFilter {
        #[ethevent(indexed)]
        pub pool_factory: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub voting_rewards_factory: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge_factory: ::ethers::core::types::Address,
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
    pub enum IFactoryRegistryEvents {
        ApproveFilter(ApproveFilter),
        SetManagedRewardsFactoryFilter(SetManagedRewardsFactoryFilter),
        UnapproveFilter(UnapproveFilter),
    }
    impl ::ethers::contract::EthLogDecode for IFactoryRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApproveFilter::decode_log(log) {
                return Ok(IFactoryRegistryEvents::ApproveFilter(decoded));
            }
            if let Ok(decoded) = SetManagedRewardsFactoryFilter::decode_log(log) {
                return Ok(
                    IFactoryRegistryEvents::SetManagedRewardsFactoryFilter(decoded),
                );
            }
            if let Ok(decoded) = UnapproveFilter::decode_log(log) {
                return Ok(IFactoryRegistryEvents::UnapproveFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IFactoryRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApproveFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetManagedRewardsFactoryFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnapproveFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveFilter> for IFactoryRegistryEvents {
        fn from(value: ApproveFilter) -> Self {
            Self::ApproveFilter(value)
        }
    }
    impl ::core::convert::From<SetManagedRewardsFactoryFilter>
    for IFactoryRegistryEvents {
        fn from(value: SetManagedRewardsFactoryFilter) -> Self {
            Self::SetManagedRewardsFactoryFilter(value)
        }
    }
    impl ::core::convert::From<UnapproveFilter> for IFactoryRegistryEvents {
        fn from(value: UnapproveFilter) -> Self {
            Self::UnapproveFilter(value)
        }
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,address,address)` and selector `0x9b140a85`
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
    #[ethcall(name = "approve", abi = "approve(address,address,address)")]
    pub struct ApproveCall {
        pub pool_factory: ::ethers::core::types::Address,
        pub voting_rewards_factory: ::ethers::core::types::Address,
        pub gauge_factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `factoriesToPoolFactory` function with signature `factoriesToPoolFactory(address)` and selector `0x1217afdb`
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
    #[ethcall(name = "factoriesToPoolFactory", abi = "factoriesToPoolFactory(address)")]
    pub struct FactoriesToPoolFactoryCall {
        pub pool_factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isPoolFactoryApproved` function with signature `isPoolFactoryApproved(address)` and selector `0xd1ea0a1d`
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
    #[ethcall(name = "isPoolFactoryApproved", abi = "isPoolFactoryApproved(address)")]
    pub struct IsPoolFactoryApprovedCall {
        pub pool_factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `managedRewardsFactory` function with signature `managedRewardsFactory()` and selector `0x0d0ae678`
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
    #[ethcall(name = "managedRewardsFactory", abi = "managedRewardsFactory()")]
    pub struct ManagedRewardsFactoryCall;
    ///Container type for all input parameters for the `poolFactories` function with signature `poolFactories()` and selector `0x06121cd5`
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
    #[ethcall(name = "poolFactories", abi = "poolFactories()")]
    pub struct PoolFactoriesCall;
    ///Container type for all input parameters for the `poolFactoriesLength` function with signature `poolFactoriesLength()` and selector `0x0cb299c9`
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
    #[ethcall(name = "poolFactoriesLength", abi = "poolFactoriesLength()")]
    pub struct PoolFactoriesLengthCall;
    ///Container type for all input parameters for the `setManagedRewardsFactory` function with signature `setManagedRewardsFactory(address)` and selector `0x64076939`
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
        name = "setManagedRewardsFactory",
        abi = "setManagedRewardsFactory(address)"
    )]
    pub struct SetManagedRewardsFactoryCall {
        pub new_managed_rewards_factory: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unapprove` function with signature `unapprove(address)` and selector `0xfbf1f78a`
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
    #[ethcall(name = "unapprove", abi = "unapprove(address)")]
    pub struct UnapproveCall {
        pub pool_factory: ::ethers::core::types::Address,
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
    pub enum IFactoryRegistryCalls {
        Approve(ApproveCall),
        FactoriesToPoolFactory(FactoriesToPoolFactoryCall),
        IsPoolFactoryApproved(IsPoolFactoryApprovedCall),
        ManagedRewardsFactory(ManagedRewardsFactoryCall),
        PoolFactories(PoolFactoriesCall),
        PoolFactoriesLength(PoolFactoriesLengthCall),
        SetManagedRewardsFactory(SetManagedRewardsFactoryCall),
        Unapprove(UnapproveCall),
    }
    impl ::ethers::core::abi::AbiDecode for IFactoryRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <FactoriesToPoolFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FactoriesToPoolFactory(decoded));
            }
            if let Ok(decoded) = <IsPoolFactoryApprovedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsPoolFactoryApproved(decoded));
            }
            if let Ok(decoded) = <ManagedRewardsFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ManagedRewardsFactory(decoded));
            }
            if let Ok(decoded) = <PoolFactoriesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolFactories(decoded));
            }
            if let Ok(decoded) = <PoolFactoriesLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolFactoriesLength(decoded));
            }
            if let Ok(decoded) = <SetManagedRewardsFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetManagedRewardsFactory(decoded));
            }
            if let Ok(decoded) = <UnapproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unapprove(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IFactoryRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FactoriesToPoolFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPoolFactoryApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ManagedRewardsFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolFactories(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolFactoriesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetManagedRewardsFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unapprove(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IFactoryRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::FactoriesToPoolFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsPoolFactoryApproved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ManagedRewardsFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolFactories(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolFactoriesLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetManagedRewardsFactory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unapprove(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveCall> for IFactoryRegistryCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<FactoriesToPoolFactoryCall> for IFactoryRegistryCalls {
        fn from(value: FactoriesToPoolFactoryCall) -> Self {
            Self::FactoriesToPoolFactory(value)
        }
    }
    impl ::core::convert::From<IsPoolFactoryApprovedCall> for IFactoryRegistryCalls {
        fn from(value: IsPoolFactoryApprovedCall) -> Self {
            Self::IsPoolFactoryApproved(value)
        }
    }
    impl ::core::convert::From<ManagedRewardsFactoryCall> for IFactoryRegistryCalls {
        fn from(value: ManagedRewardsFactoryCall) -> Self {
            Self::ManagedRewardsFactory(value)
        }
    }
    impl ::core::convert::From<PoolFactoriesCall> for IFactoryRegistryCalls {
        fn from(value: PoolFactoriesCall) -> Self {
            Self::PoolFactories(value)
        }
    }
    impl ::core::convert::From<PoolFactoriesLengthCall> for IFactoryRegistryCalls {
        fn from(value: PoolFactoriesLengthCall) -> Self {
            Self::PoolFactoriesLength(value)
        }
    }
    impl ::core::convert::From<SetManagedRewardsFactoryCall> for IFactoryRegistryCalls {
        fn from(value: SetManagedRewardsFactoryCall) -> Self {
            Self::SetManagedRewardsFactory(value)
        }
    }
    impl ::core::convert::From<UnapproveCall> for IFactoryRegistryCalls {
        fn from(value: UnapproveCall) -> Self {
            Self::Unapprove(value)
        }
    }
    ///Container type for all return fields from the `factoriesToPoolFactory` function with signature `factoriesToPoolFactory(address)` and selector `0x1217afdb`
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
    pub struct FactoriesToPoolFactoryReturn {
        pub voting_rewards_factory: ::ethers::core::types::Address,
        pub gauge_factory: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `isPoolFactoryApproved` function with signature `isPoolFactoryApproved(address)` and selector `0xd1ea0a1d`
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
    pub struct IsPoolFactoryApprovedReturn(pub bool);
    ///Container type for all return fields from the `managedRewardsFactory` function with signature `managedRewardsFactory()` and selector `0x0d0ae678`
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
    pub struct ManagedRewardsFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `poolFactories` function with signature `poolFactories()` and selector `0x06121cd5`
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
    pub struct PoolFactoriesReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `poolFactoriesLength` function with signature `poolFactoriesLength()` and selector `0x0cb299c9`
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
    pub struct PoolFactoriesLengthReturn(pub ::ethers::core::types::U256);
}
