pub use lib_log::*;
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
pub mod lib_log {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
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
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertionFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Log"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogBool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogBool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogBytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogBytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
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
                    ::std::borrow::ToOwned::to_owned("LogBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogInt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
    pub static LIBLOG_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`,`2`\x0B\x82\x82\x829\x80Q_\x1A`s\x14`&WcNH{q`\xE0\x1B_R_`\x04R`$_\xFD[0_R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R__\xFD\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static LIBLOG_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R__\xFD\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static LIBLOG_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LibLog<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LibLog<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LibLog<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LibLog<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LibLog<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LibLog)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LibLog<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIBLOG_ABI.clone(),
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
                LIBLOG_ABI.clone(),
                LIBLOG_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_4_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed4Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_5_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed5Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_6_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed6Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_7_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed7Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertionFailed` event
        pub fn assertion_failed_8_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertionFailed8Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Log` event
        pub fn log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogAddress` event
        pub fn log_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogBool` event
        pub fn log_bool_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogBoolFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogBytes` event
        pub fn log_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogBytes32` event
        pub fn log_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogInt` event
        pub fn log_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogIntFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogString` event
        pub fn log_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogUint` event
        pub fn log_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogUintFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LibLogEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LibLog<M> {
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed()")]
    pub struct AssertionFailed1Filter;
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string)")]
    pub struct AssertionFailed2Filter {
        pub message: ::std::string::String,
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string,string)")]
    pub struct AssertionFailed3Filter {
        pub message: ::std::string::String,
        pub data: ::std::string::String,
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string,bytes)")]
    pub struct AssertionFailed4Filter {
        pub message: ::std::string::String,
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string,uint256)")]
    pub struct AssertionFailed5Filter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::U256,
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string,int256)")]
    pub struct AssertionFailed6Filter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::I256,
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string,address)")]
    pub struct AssertionFailed7Filter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::Address,
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
    #[ethevent(name = "AssertionFailed", abi = "AssertionFailed(string,bool)")]
    pub struct AssertionFailed8Filter {
        pub message: ::std::string::String,
        pub data: bool,
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
    #[ethevent(name = "Log", abi = "Log(string)")]
    pub struct LogFilter {
        pub message: ::std::string::String,
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
    #[ethevent(name = "LogAddress", abi = "LogAddress(string,address)")]
    pub struct LogAddressFilter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::Address,
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
    #[ethevent(name = "LogBool", abi = "LogBool(string,bool)")]
    pub struct LogBoolFilter {
        pub message: ::std::string::String,
        pub data: bool,
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
    #[ethevent(name = "LogBytes", abi = "LogBytes(string,bytes)")]
    pub struct LogBytesFilter {
        pub message: ::std::string::String,
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
    #[ethevent(name = "LogBytes32", abi = "LogBytes32(string,bytes32)")]
    pub struct LogBytes32Filter {
        pub message: ::std::string::String,
        pub data: [u8; 32],
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
    #[ethevent(name = "LogInt", abi = "LogInt(string,int256)")]
    pub struct LogIntFilter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::I256,
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
    #[ethevent(name = "LogString", abi = "LogString(string,string)")]
    pub struct LogStringFilter {
        pub message: ::std::string::String,
        pub data: ::std::string::String,
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
    #[ethevent(name = "LogUint", abi = "LogUint(string,uint256)")]
    pub struct LogUintFilter {
        pub message: ::std::string::String,
        pub data: ::ethers::core::types::U256,
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
    pub enum LibLogEvents {
        AssertionFailed1Filter(AssertionFailed1Filter),
        AssertionFailed2Filter(AssertionFailed2Filter),
        AssertionFailed3Filter(AssertionFailed3Filter),
        AssertionFailed4Filter(AssertionFailed4Filter),
        AssertionFailed5Filter(AssertionFailed5Filter),
        AssertionFailed6Filter(AssertionFailed6Filter),
        AssertionFailed7Filter(AssertionFailed7Filter),
        AssertionFailed8Filter(AssertionFailed8Filter),
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogBoolFilter(LogBoolFilter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
    }
    impl ::ethers::contract::EthLogDecode for LibLogEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertionFailed1Filter::decode_log(log) {
                return Ok(LibLogEvents::AssertionFailed1Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed2Filter::decode_log(log) {
                return Ok(LibLogEvents::AssertionFailed2Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed3Filter::decode_log(log) {
                return Ok(LibLogEvents::AssertionFailed3Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed4Filter::decode_log(log) {
                return Ok(LibLogEvents::AssertionFailed4Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed5Filter::decode_log(log) {
                return Ok(LibLogEvents::AssertionFailed5Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed6Filter::decode_log(log) {
                return Ok(LibLogEvents::AssertionFailed6Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed7Filter::decode_log(log) {
                return Ok(LibLogEvents::AssertionFailed7Filter(decoded));
            }
            if let Ok(decoded) = AssertionFailed8Filter::decode_log(log) {
                return Ok(LibLogEvents::AssertionFailed8Filter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(LibLogEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(LibLogEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogBoolFilter::decode_log(log) {
                return Ok(LibLogEvents::LogBoolFilter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(LibLogEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(LibLogEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(LibLogEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(LibLogEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(LibLogEvents::LogUintFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LibLogEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssertionFailed1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed3Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed4Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed5Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed6Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed7Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertionFailed8Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBoolFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes32Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUintFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssertionFailed1Filter> for LibLogEvents {
        fn from(value: AssertionFailed1Filter) -> Self {
            Self::AssertionFailed1Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed2Filter> for LibLogEvents {
        fn from(value: AssertionFailed2Filter) -> Self {
            Self::AssertionFailed2Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed3Filter> for LibLogEvents {
        fn from(value: AssertionFailed3Filter) -> Self {
            Self::AssertionFailed3Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed4Filter> for LibLogEvents {
        fn from(value: AssertionFailed4Filter) -> Self {
            Self::AssertionFailed4Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed5Filter> for LibLogEvents {
        fn from(value: AssertionFailed5Filter) -> Self {
            Self::AssertionFailed5Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed6Filter> for LibLogEvents {
        fn from(value: AssertionFailed6Filter) -> Self {
            Self::AssertionFailed6Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed7Filter> for LibLogEvents {
        fn from(value: AssertionFailed7Filter) -> Self {
            Self::AssertionFailed7Filter(value)
        }
    }
    impl ::core::convert::From<AssertionFailed8Filter> for LibLogEvents {
        fn from(value: AssertionFailed8Filter) -> Self {
            Self::AssertionFailed8Filter(value)
        }
    }
    impl ::core::convert::From<LogFilter> for LibLogEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for LibLogEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogBoolFilter> for LibLogEvents {
        fn from(value: LogBoolFilter) -> Self {
            Self::LogBoolFilter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for LibLogEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for LibLogEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for LibLogEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for LibLogEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for LibLogEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
}
