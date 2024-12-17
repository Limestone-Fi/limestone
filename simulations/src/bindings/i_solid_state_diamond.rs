pub use i_solid_state_diamond::*;
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
pub mod i_solid_state_diamond {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
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
                (
                    ::std::borrow::ToOwned::to_owned("facetAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("facetAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("selector"),
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
                                    name: ::std::borrow::ToOwned::to_owned("facet"),
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
                    ::std::borrow::ToOwned::to_owned("facetAddresses"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("facetAddresses"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addresses"),
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
                    ::std::borrow::ToOwned::to_owned("facetFunctionSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "facetFunctionSelectors",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("facet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("selectors"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("facets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("facets"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("diamondFacets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
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
                                            "struct IERC2535DiamondLoupeInternal.Facet[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("nomineeOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nomineeOwner"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ISOLIDSTATEDIAMOND_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct ISolidStateDiamond<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISolidStateDiamond<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISolidStateDiamond<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISolidStateDiamond<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISolidStateDiamond<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ISolidStateDiamond))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISolidStateDiamond<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISOLIDSTATEDIAMOND_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `facetAddress` (0xcdffacc6) function
        pub fn facet_address(
            &self,
            selector: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([205, 255, 172, 198], selector)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facetAddresses` (0x52ef6b2c) function
        pub fn facet_addresses(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([82, 239, 107, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facetFunctionSelectors` (0xadfca15e) function
        pub fn facet_function_selectors(
            &self,
            facet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 4]>> {
            self.0
                .method_hash([173, 252, 161, 94], facet)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facets` (0x7a0ed627) function
        pub fn facets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Facet>> {
            self.0
                .method_hash([122, 14, 214, 39], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `nomineeOwner` (0x8ab5150a) function
        pub fn nominee_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([138, 181, 21, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
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
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], account)
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
            ISolidStateDiamondEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ISolidStateDiamond<M> {
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
    pub enum ISolidStateDiamondErrors {
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
        Ownable__NotOwner(Ownable__NotOwner),
        Ownable__NotTransitiveOwner(Ownable__NotTransitiveOwner),
        Proxy__ImplementationIsNotContract(Proxy__ImplementationIsNotContract),
        SafeOwnable__NotNomineeOwner(SafeOwnable__NotNomineeOwner),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ISolidStateDiamondErrors {
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
            if let Ok(decoded) = <Proxy__ImplementationIsNotContract as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Proxy__ImplementationIsNotContract(decoded));
            }
            if let Ok(decoded) = <SafeOwnable__NotNomineeOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeOwnable__NotNomineeOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISolidStateDiamondErrors {
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
                Self::Ownable__NotOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ownable__NotTransitiveOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Proxy__ImplementationIsNotContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeOwnable__NotNomineeOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ISolidStateDiamondErrors {
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
                _ if selector
                    == <Ownable__NotOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Ownable__NotTransitiveOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Proxy__ImplementationIsNotContract as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for ISolidStateDiamondErrors {
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
                Self::Ownable__NotOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ownable__NotTransitiveOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Proxy__ImplementationIsNotContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeOwnable__NotNomineeOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ISolidStateDiamondErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__InvalidInitializationParameters>
    for ISolidStateDiamondErrors {
        fn from(value: DiamondWritable__InvalidInitializationParameters) -> Self {
            Self::DiamondWritable__InvalidInitializationParameters(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__RemoveTargetNotZeroAddress>
    for ISolidStateDiamondErrors {
        fn from(value: DiamondWritable__RemoveTargetNotZeroAddress) -> Self {
            Self::DiamondWritable__RemoveTargetNotZeroAddress(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__ReplaceTargetIsIdentical>
    for ISolidStateDiamondErrors {
        fn from(value: DiamondWritable__ReplaceTargetIsIdentical) -> Self {
            Self::DiamondWritable__ReplaceTargetIsIdentical(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__SelectorAlreadyAdded>
    for ISolidStateDiamondErrors {
        fn from(value: DiamondWritable__SelectorAlreadyAdded) -> Self {
            Self::DiamondWritable__SelectorAlreadyAdded(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__SelectorIsImmutable>
    for ISolidStateDiamondErrors {
        fn from(value: DiamondWritable__SelectorIsImmutable) -> Self {
            Self::DiamondWritable__SelectorIsImmutable(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__SelectorNotFound>
    for ISolidStateDiamondErrors {
        fn from(value: DiamondWritable__SelectorNotFound) -> Self {
            Self::DiamondWritable__SelectorNotFound(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__SelectorNotSpecified>
    for ISolidStateDiamondErrors {
        fn from(value: DiamondWritable__SelectorNotSpecified) -> Self {
            Self::DiamondWritable__SelectorNotSpecified(value)
        }
    }
    impl ::core::convert::From<DiamondWritable__TargetHasNoCode>
    for ISolidStateDiamondErrors {
        fn from(value: DiamondWritable__TargetHasNoCode) -> Self {
            Self::DiamondWritable__TargetHasNoCode(value)
        }
    }
    impl ::core::convert::From<Ownable__NotOwner> for ISolidStateDiamondErrors {
        fn from(value: Ownable__NotOwner) -> Self {
            Self::Ownable__NotOwner(value)
        }
    }
    impl ::core::convert::From<Ownable__NotTransitiveOwner>
    for ISolidStateDiamondErrors {
        fn from(value: Ownable__NotTransitiveOwner) -> Self {
            Self::Ownable__NotTransitiveOwner(value)
        }
    }
    impl ::core::convert::From<Proxy__ImplementationIsNotContract>
    for ISolidStateDiamondErrors {
        fn from(value: Proxy__ImplementationIsNotContract) -> Self {
            Self::Proxy__ImplementationIsNotContract(value)
        }
    }
    impl ::core::convert::From<SafeOwnable__NotNomineeOwner>
    for ISolidStateDiamondErrors {
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
        name = "DiamondCut",
        abi = "DiamondCut((address,uint8,bytes4[])[],address,bytes)"
    )]
    pub struct DiamondCutFilter {
        pub facet_cuts: ::std::vec::Vec<FacetCut>,
        pub target: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum ISolidStateDiamondEvents {
        DiamondCutFilter(DiamondCutFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for ISolidStateDiamondEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DiamondCutFilter::decode_log(log) {
                return Ok(ISolidStateDiamondEvents::DiamondCutFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ISolidStateDiamondEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ISolidStateDiamondEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DiamondCutFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DiamondCutFilter> for ISolidStateDiamondEvents {
        fn from(value: DiamondCutFilter) -> Self {
            Self::DiamondCutFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ISolidStateDiamondEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
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
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
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
    ///Container type for all input parameters for the `facetAddress` function with signature `facetAddress(bytes4)` and selector `0xcdffacc6`
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
    #[ethcall(name = "facetAddress", abi = "facetAddress(bytes4)")]
    pub struct FacetAddressCall {
        pub selector: [u8; 4],
    }
    ///Container type for all input parameters for the `facetAddresses` function with signature `facetAddresses()` and selector `0x52ef6b2c`
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
    #[ethcall(name = "facetAddresses", abi = "facetAddresses()")]
    pub struct FacetAddressesCall;
    ///Container type for all input parameters for the `facetFunctionSelectors` function with signature `facetFunctionSelectors(address)` and selector `0xadfca15e`
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
    #[ethcall(name = "facetFunctionSelectors", abi = "facetFunctionSelectors(address)")]
    pub struct FacetFunctionSelectorsCall {
        pub facet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `facets` function with signature `facets()` and selector `0x7a0ed627`
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
    #[ethcall(name = "facets", abi = "facets()")]
    pub struct FacetsCall;
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
    ///Container type for all input parameters for the `nomineeOwner` function with signature `nomineeOwner()` and selector `0x8ab5150a`
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
    #[ethcall(name = "nomineeOwner", abi = "nomineeOwner()")]
    pub struct NomineeOwnerCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
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
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub account: ::ethers::core::types::Address,
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
    pub enum ISolidStateDiamondCalls {
        AcceptOwnership(AcceptOwnershipCall),
        DiamondCut(DiamondCutCall),
        FacetAddress(FacetAddressCall),
        FacetAddresses(FacetAddressesCall),
        FacetFunctionSelectors(FacetFunctionSelectorsCall),
        Facets(FacetsCall),
        GetFallbackAddress(GetFallbackAddressCall),
        NomineeOwner(NomineeOwnerCall),
        Owner(OwnerCall),
        SetFallbackAddress(SetFallbackAddressCall),
        SupportsInterface(SupportsInterfaceCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for ISolidStateDiamondCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <DiamondCutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DiamondCut(decoded));
            }
            if let Ok(decoded) = <FacetAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FacetAddress(decoded));
            }
            if let Ok(decoded) = <FacetAddressesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FacetAddresses(decoded));
            }
            if let Ok(decoded) = <FacetFunctionSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FacetFunctionSelectors(decoded));
            }
            if let Ok(decoded) = <FacetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Facets(decoded));
            }
            if let Ok(decoded) = <GetFallbackAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFallbackAddress(decoded));
            }
            if let Ok(decoded) = <NomineeOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NomineeOwner(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <SetFallbackAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFallbackAddress(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISolidStateDiamondCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DiamondCut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FacetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FacetAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FacetFunctionSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Facets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetFallbackAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NomineeOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFallbackAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ISolidStateDiamondCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::DiamondCut(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetFunctionSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Facets(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFallbackAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NomineeOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFallbackAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for ISolidStateDiamondCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<DiamondCutCall> for ISolidStateDiamondCalls {
        fn from(value: DiamondCutCall) -> Self {
            Self::DiamondCut(value)
        }
    }
    impl ::core::convert::From<FacetAddressCall> for ISolidStateDiamondCalls {
        fn from(value: FacetAddressCall) -> Self {
            Self::FacetAddress(value)
        }
    }
    impl ::core::convert::From<FacetAddressesCall> for ISolidStateDiamondCalls {
        fn from(value: FacetAddressesCall) -> Self {
            Self::FacetAddresses(value)
        }
    }
    impl ::core::convert::From<FacetFunctionSelectorsCall> for ISolidStateDiamondCalls {
        fn from(value: FacetFunctionSelectorsCall) -> Self {
            Self::FacetFunctionSelectors(value)
        }
    }
    impl ::core::convert::From<FacetsCall> for ISolidStateDiamondCalls {
        fn from(value: FacetsCall) -> Self {
            Self::Facets(value)
        }
    }
    impl ::core::convert::From<GetFallbackAddressCall> for ISolidStateDiamondCalls {
        fn from(value: GetFallbackAddressCall) -> Self {
            Self::GetFallbackAddress(value)
        }
    }
    impl ::core::convert::From<NomineeOwnerCall> for ISolidStateDiamondCalls {
        fn from(value: NomineeOwnerCall) -> Self {
            Self::NomineeOwner(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ISolidStateDiamondCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<SetFallbackAddressCall> for ISolidStateDiamondCalls {
        fn from(value: SetFallbackAddressCall) -> Self {
            Self::SetFallbackAddress(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ISolidStateDiamondCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ISolidStateDiamondCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `facetAddress` function with signature `facetAddress(bytes4)` and selector `0xcdffacc6`
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
    pub struct FacetAddressReturn {
        pub facet: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `facetAddresses` function with signature `facetAddresses()` and selector `0x52ef6b2c`
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
    pub struct FacetAddressesReturn {
        pub addresses: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `facetFunctionSelectors` function with signature `facetFunctionSelectors(address)` and selector `0xadfca15e`
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
    pub struct FacetFunctionSelectorsReturn {
        pub selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///Container type for all return fields from the `facets` function with signature `facets()` and selector `0x7a0ed627`
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
    pub struct FacetsReturn {
        pub diamond_facets: ::std::vec::Vec<Facet>,
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
    ///Container type for all return fields from the `nomineeOwner` function with signature `nomineeOwner()` and selector `0x8ab5150a`
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
    pub struct NomineeOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
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
    pub struct SupportsInterfaceReturn(pub bool);
}
