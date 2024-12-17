pub use erc20_permit::*;
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
pub mod erc20_permit {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainSeparator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
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
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("ECDSA__InvalidS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ECDSA__InvalidS"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSA__InvalidSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSA__InvalidSignature",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSA__InvalidV"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ECDSA__InvalidV"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
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
    pub static ERC20PERMIT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct ERC20Permit<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC20Permit<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC20Permit<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC20Permit<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC20Permit<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC20Permit))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC20Permit<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC20PERMIT_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, amount, deadline, v, r, s),
                )
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
            ERC20PermitEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC20Permit<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ECDSA__InvalidS` with signature `ECDSA__InvalidS()` and selector `0xe4a61add`
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
    #[etherror(name = "ECDSA__InvalidS", abi = "ECDSA__InvalidS()")]
    pub struct ECDSA__InvalidS;
    ///Custom Error type `ECDSA__InvalidSignature` with signature `ECDSA__InvalidSignature()` and selector `0x8fe7664e`
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
    #[etherror(name = "ECDSA__InvalidSignature", abi = "ECDSA__InvalidSignature()")]
    pub struct ECDSA__InvalidSignature;
    ///Custom Error type `ECDSA__InvalidV` with signature `ECDSA__InvalidV()` and selector `0xacd54a90`
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
    #[etherror(name = "ECDSA__InvalidV", abi = "ECDSA__InvalidV()")]
    pub struct ECDSA__InvalidV;
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
    pub enum ERC20PermitErrors {
        ECDSA__InvalidS(ECDSA__InvalidS),
        ECDSA__InvalidSignature(ECDSA__InvalidSignature),
        ECDSA__InvalidV(ECDSA__InvalidV),
        ERC20Base__ApproveFromZeroAddress(ERC20Base__ApproveFromZeroAddress),
        ERC20Base__ApproveToZeroAddress(ERC20Base__ApproveToZeroAddress),
        ERC20Base__BurnExceedsBalance(ERC20Base__BurnExceedsBalance),
        ERC20Base__BurnFromZeroAddress(ERC20Base__BurnFromZeroAddress),
        ERC20Base__InsufficientAllowance(ERC20Base__InsufficientAllowance),
        ERC20Base__MintToZeroAddress(ERC20Base__MintToZeroAddress),
        ERC20Base__TransferExceedsBalance(ERC20Base__TransferExceedsBalance),
        ERC20Base__TransferFromZeroAddress(ERC20Base__TransferFromZeroAddress),
        ERC20Base__TransferToZeroAddress(ERC20Base__TransferToZeroAddress),
        ERC20Permit__ExpiredDeadline(ERC20Permit__ExpiredDeadline),
        ERC20Permit__InvalidSignature(ERC20Permit__InvalidSignature),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ERC20PermitErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ECDSA__InvalidS as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ECDSA__InvalidS(decoded));
            }
            if let Ok(decoded) = <ECDSA__InvalidSignature as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ECDSA__InvalidSignature(decoded));
            }
            if let Ok(decoded) = <ECDSA__InvalidV as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ECDSA__InvalidV(decoded));
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
    impl ::ethers::core::abi::AbiEncode for ERC20PermitErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ECDSA__InvalidS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSA__InvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSA__InvalidV(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
    impl ::ethers::contract::ContractRevert for ERC20PermitErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ECDSA__InvalidS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSA__InvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSA__InvalidV as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
    impl ::core::fmt::Display for ERC20PermitErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ECDSA__InvalidS(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSA__InvalidSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ECDSA__InvalidV(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<::std::string::String> for ERC20PermitErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ECDSA__InvalidS> for ERC20PermitErrors {
        fn from(value: ECDSA__InvalidS) -> Self {
            Self::ECDSA__InvalidS(value)
        }
    }
    impl ::core::convert::From<ECDSA__InvalidSignature> for ERC20PermitErrors {
        fn from(value: ECDSA__InvalidSignature) -> Self {
            Self::ECDSA__InvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSA__InvalidV> for ERC20PermitErrors {
        fn from(value: ECDSA__InvalidV) -> Self {
            Self::ECDSA__InvalidV(value)
        }
    }
    impl ::core::convert::From<ERC20Base__ApproveFromZeroAddress> for ERC20PermitErrors {
        fn from(value: ERC20Base__ApproveFromZeroAddress) -> Self {
            Self::ERC20Base__ApproveFromZeroAddress(value)
        }
    }
    impl ::core::convert::From<ERC20Base__ApproveToZeroAddress> for ERC20PermitErrors {
        fn from(value: ERC20Base__ApproveToZeroAddress) -> Self {
            Self::ERC20Base__ApproveToZeroAddress(value)
        }
    }
    impl ::core::convert::From<ERC20Base__BurnExceedsBalance> for ERC20PermitErrors {
        fn from(value: ERC20Base__BurnExceedsBalance) -> Self {
            Self::ERC20Base__BurnExceedsBalance(value)
        }
    }
    impl ::core::convert::From<ERC20Base__BurnFromZeroAddress> for ERC20PermitErrors {
        fn from(value: ERC20Base__BurnFromZeroAddress) -> Self {
            Self::ERC20Base__BurnFromZeroAddress(value)
        }
    }
    impl ::core::convert::From<ERC20Base__InsufficientAllowance> for ERC20PermitErrors {
        fn from(value: ERC20Base__InsufficientAllowance) -> Self {
            Self::ERC20Base__InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<ERC20Base__MintToZeroAddress> for ERC20PermitErrors {
        fn from(value: ERC20Base__MintToZeroAddress) -> Self {
            Self::ERC20Base__MintToZeroAddress(value)
        }
    }
    impl ::core::convert::From<ERC20Base__TransferExceedsBalance> for ERC20PermitErrors {
        fn from(value: ERC20Base__TransferExceedsBalance) -> Self {
            Self::ERC20Base__TransferExceedsBalance(value)
        }
    }
    impl ::core::convert::From<ERC20Base__TransferFromZeroAddress>
    for ERC20PermitErrors {
        fn from(value: ERC20Base__TransferFromZeroAddress) -> Self {
            Self::ERC20Base__TransferFromZeroAddress(value)
        }
    }
    impl ::core::convert::From<ERC20Base__TransferToZeroAddress> for ERC20PermitErrors {
        fn from(value: ERC20Base__TransferToZeroAddress) -> Self {
            Self::ERC20Base__TransferToZeroAddress(value)
        }
    }
    impl ::core::convert::From<ERC20Permit__ExpiredDeadline> for ERC20PermitErrors {
        fn from(value: ERC20Permit__ExpiredDeadline) -> Self {
            Self::ERC20Permit__ExpiredDeadline(value)
        }
    }
    impl ::core::convert::From<ERC20Permit__InvalidSignature> for ERC20PermitErrors {
        fn from(value: ERC20Permit__InvalidSignature) -> Self {
            Self::ERC20Permit__InvalidSignature(value)
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
    pub enum ERC20PermitEvents {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for ERC20PermitEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC20PermitEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC20PermitEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC20PermitEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ERC20PermitEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ERC20PermitEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
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
    pub enum ERC20PermitCalls {
        DomainSeparator(DomainSeparatorCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC20PermitCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC20PermitCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ERC20PermitCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for ERC20PermitCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for ERC20PermitCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PermitCall> for ERC20PermitCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn {
        pub domain_separator: [u8; 32],
    }
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
}
