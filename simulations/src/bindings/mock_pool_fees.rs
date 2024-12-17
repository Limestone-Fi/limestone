pub use mock_pool_fees::*;
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
pub mod mock_pool_fees {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_token0"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_token1"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("claimFeesFor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimFeesFor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount1"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NotPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotPool"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeERC20FailedOperation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKPOOLFEES_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x02\xCF8\x03\x80a\x02\xCF\x839\x81\x01`@\x81\x90Ra\0.\x91a\0dV[3`\x80R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xA0R\x16`\xC0Ra\0\x95V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0_W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\0uW__\xFD[a\0~\x83a\0IV[\x91Pa\0\x8C` \x84\x01a\0IV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Qa\x02\x13a\0\xBC_9_`\xD8\x01R_`\x9E\x01R_`M\x01Ra\x02\x13_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80cS<\xF5\xCE\x14a\0-W[__\xFD[a\0@a\0;6`\x04a\x01\xC9V[a\0BV[\0[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\x8BW`@Qcoa\xF6A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15a\0\xC5Wa\0\xC5`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a\x01\x04V[\x80\x15a\0\xFFWa\0\xFF`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x83a\x01\x04V[PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x80\x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x82Qa\0\xFF\x93\x87\x93\x90\x92_\x92\x83\x92\x91\x83\x91\x90\x82\x88Z\xF1\x80a\x01rW`@Q=_\x82>=\x81\xFD[PP_Q=\x91P\x81\x15a\x01\x89W\x80`\x01\x14\x15a\x01\x96V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x01\xC3W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[___``\x84\x86\x03\x12\x15a\x01\xDBW__\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xF1W__\xFD[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static MOCKPOOLFEES_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80cS<\xF5\xCE\x14a\0-W[__\xFD[a\0@a\0;6`\x04a\x01\xC9V[a\0BV[\0[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\x8BW`@Qcoa\xF6A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15a\0\xC5Wa\0\xC5`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a\x01\x04V[\x80\x15a\0\xFFWa\0\xFF`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x83a\x01\x04V[PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x80\x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x82Qa\0\xFF\x93\x87\x93\x90\x92_\x92\x83\x92\x91\x83\x91\x90\x82\x88Z\xF1\x80a\x01rW`@Q=_\x82>=\x81\xFD[PP_Q=\x91P\x81\x15a\x01\x89W\x80`\x01\x14\x15a\x01\x96V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x01\xC3W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[___``\x84\x86\x03\x12\x15a\x01\xDBW__\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xF1W__\xFD[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static MOCKPOOLFEES_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockPoolFees<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockPoolFees<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockPoolFees<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockPoolFees<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockPoolFees<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockPoolFees))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockPoolFees<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKPOOLFEES_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MOCKPOOLFEES_ABI.clone(),
                MOCKPOOLFEES_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `claimFeesFor` (0x533cf5ce) function
        pub fn claim_fees_for(
            &self,
            recipient: ::ethers::core::types::Address,
            amount_0: ::ethers::core::types::U256,
            amount_1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 60, 245, 206], (recipient, amount_0, amount_1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockPoolFees<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotPool` with signature `NotPool()` and selector `0x6f61f641`
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
    #[etherror(name = "NotPool", abi = "NotPool()")]
    pub struct NotPool;
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
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
        name = "SafeERC20FailedOperation",
        abi = "SafeERC20FailedOperation(address)"
    )]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
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
    pub enum MockPoolFeesErrors {
        NotPool(NotPool),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockPoolFeesErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <NotPool as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotPool(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockPoolFeesErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::NotPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockPoolFeesErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <NotPool as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockPoolFeesErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NotPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockPoolFeesErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<NotPool> for MockPoolFeesErrors {
        fn from(value: NotPool) -> Self {
            Self::NotPool(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for MockPoolFeesErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    ///Container type for all input parameters for the `claimFeesFor` function with signature `claimFeesFor(address,uint256,uint256)` and selector `0x533cf5ce`
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
    #[ethcall(name = "claimFeesFor", abi = "claimFeesFor(address,uint256,uint256)")]
    pub struct ClaimFeesForCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
}
