pub use mock_worker::*;
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
pub mod mock_worker {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_token"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("health"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("health"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("healthcheck"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("healthcheck"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_id"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("token"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("work"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("work"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_debt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
    pub static MOCKWORKER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW__\xFD[P`@Qa\x02k8\x03\x80a\x02k\x839\x81\x01`@\x81\x90R`+\x91`NV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`yV[_` \x82\x84\x03\x12\x15`]W__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`rW__\xFD[\x93\x92PPPV[a\x01\xE5\x80a\0\x86_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cA_\x12@\x14a\0YW\x80cH\xDB\x8C\xC4\x14a\0lW\x80cOB\xFC\x02\x14a\0\x81W\x80cx;m`\x14a\0\x95W\x80c\xFC\x0CTj\x14a\0\xB6W[__\xFD[a\0ja\0g6`\x04a\x01&V[PV[\0[a\0ja\0z6`\x04a\x01=V[PPPPPV[`@Q`\x01\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA8a\0\xA36`\x04a\x01&V[a\0\xE0V[`@Q\x90\x81R` \x01a\0\x8CV[_Ta\0\xC8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x8CV[_\x80Ta\0\xF6\x90`\x01`\x01`\xA0\x1B\x03\x160a\0\xFCV[\x92\x91PPV[_\x81`\x14Rcp\xA0\x821``\x1B_R` \x80`$`\x10\x86Z\xFA`\x1F=\x11\x16` Q\x02\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x016W__\xFD[P5\x91\x90PV[_____`\x80\x86\x88\x03\x12\x15a\x01QW__\xFD[\x855\x94P` \x86\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01nW__\xFD[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x90W__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\x01\xA0W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xB6W__\xFD[\x88` \x82\x84\x01\x01\x11\x15a\x01\xC7W__\xFD[\x95\x98\x94\x97P\x92\x95PPP` \x01\x91\x90V\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static MOCKWORKER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cA_\x12@\x14a\0YW\x80cH\xDB\x8C\xC4\x14a\0lW\x80cOB\xFC\x02\x14a\0\x81W\x80cx;m`\x14a\0\x95W\x80c\xFC\x0CTj\x14a\0\xB6W[__\xFD[a\0ja\0g6`\x04a\x01&V[PV[\0[a\0ja\0z6`\x04a\x01=V[PPPPPV[`@Q`\x01\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA8a\0\xA36`\x04a\x01&V[a\0\xE0V[`@Q\x90\x81R` \x01a\0\x8CV[_Ta\0\xC8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x8CV[_\x80Ta\0\xF6\x90`\x01`\x01`\xA0\x1B\x03\x160a\0\xFCV[\x92\x91PPV[_\x81`\x14Rcp\xA0\x821``\x1B_R` \x80`$`\x10\x86Z\xFA`\x1F=\x11\x16` Q\x02\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x016W__\xFD[P5\x91\x90PV[_____`\x80\x86\x88\x03\x12\x15a\x01QW__\xFD[\x855\x94P` \x86\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01nW__\xFD[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x90W__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\x01\xA0W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xB6W__\xFD[\x88` \x82\x84\x01\x01\x11\x15a\x01\xC7W__\xFD[\x95\x98\x94\x97P\x92\x95PPP` \x01\x91\x90V\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static MOCKWORKER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockWorker<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockWorker<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockWorker<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockWorker<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockWorker<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockWorker)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockWorker<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKWORKER_ABI.clone(),
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
                MOCKWORKER_ABI.clone(),
                MOCKWORKER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `health` (0x783b6d60) function
        pub fn health(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([120, 59, 109, 96], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `healthcheck` (0x4f42fc02) function
        pub fn healthcheck(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([79, 66, 252, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0x415f1240) function
        pub fn liquidate(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 95, 18, 64], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token` (0xfc0c546a) function
        pub fn token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `work` (0x48db8cc4) function
        pub fn work(
            &self,
            id: ::ethers::core::types::U256,
            user: ::ethers::core::types::Address,
            debt: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 219, 140, 196], (id, user, debt, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockWorker<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `health` function with signature `health(uint256)` and selector `0x783b6d60`
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
    #[ethcall(name = "health", abi = "health(uint256)")]
    pub struct HealthCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `healthcheck` function with signature `healthcheck()` and selector `0x4f42fc02`
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
    #[ethcall(name = "healthcheck", abi = "healthcheck()")]
    pub struct HealthcheckCall;
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate(uint256)` and selector `0x415f1240`
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
    #[ethcall(name = "liquidate", abi = "liquidate(uint256)")]
    pub struct LiquidateCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `token` function with signature `token()` and selector `0xfc0c546a`
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
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    ///Container type for all input parameters for the `work` function with signature `work(uint256,address,uint256,bytes)` and selector `0x48db8cc4`
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
    #[ethcall(name = "work", abi = "work(uint256,address,uint256,bytes)")]
    pub struct WorkCall {
        pub id: ::ethers::core::types::U256,
        pub user: ::ethers::core::types::Address,
        pub debt: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum MockWorkerCalls {
        Health(HealthCall),
        Healthcheck(HealthcheckCall),
        Liquidate(LiquidateCall),
        Token(TokenCall),
        Work(WorkCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockWorkerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <HealthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Health(decoded));
            }
            if let Ok(decoded) = <HealthcheckCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Healthcheck(decoded));
            }
            if let Ok(decoded) = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Liquidate(decoded));
            }
            if let Ok(decoded) = <TokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Token(decoded));
            }
            if let Ok(decoded) = <WorkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Work(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockWorkerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Health(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Healthcheck(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Token(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Work(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockWorkerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Health(element) => ::core::fmt::Display::fmt(element, f),
                Self::Healthcheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
                Self::Work(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<HealthCall> for MockWorkerCalls {
        fn from(value: HealthCall) -> Self {
            Self::Health(value)
        }
    }
    impl ::core::convert::From<HealthcheckCall> for MockWorkerCalls {
        fn from(value: HealthcheckCall) -> Self {
            Self::Healthcheck(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for MockWorkerCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<TokenCall> for MockWorkerCalls {
        fn from(value: TokenCall) -> Self {
            Self::Token(value)
        }
    }
    impl ::core::convert::From<WorkCall> for MockWorkerCalls {
        fn from(value: WorkCall) -> Self {
            Self::Work(value)
        }
    }
    ///Container type for all return fields from the `health` function with signature `health(uint256)` and selector `0x783b6d60`
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
    pub struct HealthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `healthcheck` function with signature `healthcheck()` and selector `0x4f42fc02`
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
    pub struct HealthcheckReturn(pub bool);
    ///Container type for all return fields from the `token` function with signature `token()` and selector `0xfc0c546a`
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
    pub struct TokenReturn(pub ::ethers::core::types::Address);
}
