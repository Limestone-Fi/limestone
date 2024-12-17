pub use constants::*;
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
pub mod constants {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ADDRESS_CHEATS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ADDRESS_CHEATS"),
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
                    ::std::borrow::ToOwned::to_owned("PANIC_ALLOC_TOO_MUCH_MEMORY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PANIC_ALLOC_TOO_MUCH_MEMORY",
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
                    ::std::borrow::ToOwned::to_owned("PANIC_ARITHMETIC"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PANIC_ARITHMETIC"),
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
                    ::std::borrow::ToOwned::to_owned("PANIC_ARRAY_OUT_OF_BOUNDS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PANIC_ARRAY_OUT_OF_BOUNDS",
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
                    ::std::borrow::ToOwned::to_owned("PANIC_ASSERT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PANIC_ASSERT"),
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
                    ::std::borrow::ToOwned::to_owned("PANIC_DIVISION_BY_ZERO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PANIC_DIVISION_BY_ZERO",
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
                    ::std::borrow::ToOwned::to_owned("PANIC_ENUM_OUT_OF_BOUNDS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PANIC_ENUM_OUT_OF_BOUNDS",
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
                    ::std::borrow::ToOwned::to_owned("PANIC_GENERAL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PANIC_GENERAL"),
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
                    ::std::borrow::ToOwned::to_owned("PANIC_POP_EMPTY_ARRAY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PANIC_POP_EMPTY_ARRAY",
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
                    ::std::borrow::ToOwned::to_owned(
                        "PANIC_STORAGE_BYTES_ARRAY_ENCODING",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PANIC_STORAGE_BYTES_ARRAY_ENCODING",
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
                    ::std::borrow::ToOwned::to_owned(
                        "PANIC_ZERO_INIT_INTERNAL_FUNCTION",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PANIC_ZERO_INIT_INTERNAL_FUNCTION",
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CONSTANTS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01Wa\x004`\x0B\x82\x82\x829\x80Q_\x1A`s\x14`(WcNH{q`\xE0\x1B_R_`\x04R`$_\xFD[0_R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0\xB1W_5`\xE0\x1C\x80cb0\xFD\xE5\x11a\0yW\x80cb0\xFD\xE5\x14a\0\xF0W\x80cj\xA5YU\x14a\x01#W\x80cw\xD0\xB54\x14a\x01+W\x80c\x85\xE2\xC3\x19\x14a\x013W\x80c\xA7\x97\x91j\x14a\x01;W\x80c\xE2\x93(\xE4\x14a\x01BW__\xFD[\x80c,\xFA,\xAD\x14a\0\xB5W\x80c4\xC8\xA3\xD3\x14a\0\xD0W\x80cT\x91\xD3\x0F\x14a\0\xD8W\x80cWw;z\x14a\0\xE0W\x80c[\x8C\x05\x1A\x14a\0\xE8W[__\xFD[a\0\xBD`Q\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xBD`\x12\x81V[a\0\xBD`\"\x81V[a\0\xBD`A\x81V[a\0\xBD`2\x81V[a\x01\x0Bsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC7V[a\0\xBD`!\x81V[a\0\xBD`1\x81V[a\0\xBD`\x01\x81V[a\0\xBD_\x81V[a\0\xBD`\x11\x81V\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static CONSTANTS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0\xB1W_5`\xE0\x1C\x80cb0\xFD\xE5\x11a\0yW\x80cb0\xFD\xE5\x14a\0\xF0W\x80cj\xA5YU\x14a\x01#W\x80cw\xD0\xB54\x14a\x01+W\x80c\x85\xE2\xC3\x19\x14a\x013W\x80c\xA7\x97\x91j\x14a\x01;W\x80c\xE2\x93(\xE4\x14a\x01BW__\xFD[\x80c,\xFA,\xAD\x14a\0\xB5W\x80c4\xC8\xA3\xD3\x14a\0\xD0W\x80cT\x91\xD3\x0F\x14a\0\xD8W\x80cWw;z\x14a\0\xE0W\x80c[\x8C\x05\x1A\x14a\0\xE8W[__\xFD[a\0\xBD`Q\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xBD`\x12\x81V[a\0\xBD`\"\x81V[a\0\xBD`A\x81V[a\0\xBD`2\x81V[a\x01\x0Bsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC7V[a\0\xBD`!\x81V[a\0\xBD`1\x81V[a\0\xBD`\x01\x81V[a\0\xBD_\x81V[a\0\xBD`\x11\x81V\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static CONSTANTS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Constants<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Constants<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Constants<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Constants<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Constants<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Constants)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Constants<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONSTANTS_ABI.clone(),
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
                CONSTANTS_ABI.clone(),
                CONSTANTS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ADDRESS_CHEATS` (0x6230fde5) function
        pub fn address_cheats(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([98, 48, 253, 229], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PANIC_ALLOC_TOO_MUCH_MEMORY` (0x57773b7a) function
        pub fn panic_alloc_too_much_memory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([87, 119, 59, 122], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PANIC_ARITHMETIC` (0xe29328e4) function
        pub fn panic_arithmetic(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([226, 147, 40, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PANIC_ARRAY_OUT_OF_BOUNDS` (0x5b8c051a) function
        pub fn panic_array_out_of_bounds(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([91, 140, 5, 26], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PANIC_ASSERT` (0x85e2c319) function
        pub fn panic_assert(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 226, 195, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PANIC_DIVISION_BY_ZERO` (0x34c8a3d3) function
        pub fn panic_division_by_zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 200, 163, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PANIC_ENUM_OUT_OF_BOUNDS` (0x6aa55955) function
        pub fn panic_enum_out_of_bounds(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([106, 165, 89, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PANIC_GENERAL` (0xa797916a) function
        pub fn panic_general(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([167, 151, 145, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PANIC_POP_EMPTY_ARRAY` (0x77d0b534) function
        pub fn panic_pop_empty_array(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([119, 208, 181, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PANIC_STORAGE_BYTES_ARRAY_ENCODING` (0x5491d30f) function
        pub fn panic_storage_bytes_array_encoding(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 145, 211, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PANIC_ZERO_INIT_INTERNAL_FUNCTION` (0x2cfa2cad) function
        pub fn panic_zero_init_internal_function(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([44, 250, 44, 173], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Constants<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `ADDRESS_CHEATS` function with signature `ADDRESS_CHEATS()` and selector `0x6230fde5`
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
    #[ethcall(name = "ADDRESS_CHEATS", abi = "ADDRESS_CHEATS()")]
    pub struct AddressCheatsCall;
    ///Container type for all input parameters for the `PANIC_ALLOC_TOO_MUCH_MEMORY` function with signature `PANIC_ALLOC_TOO_MUCH_MEMORY()` and selector `0x57773b7a`
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
        name = "PANIC_ALLOC_TOO_MUCH_MEMORY",
        abi = "PANIC_ALLOC_TOO_MUCH_MEMORY()"
    )]
    pub struct PanicAllocTooMuchMemoryCall;
    ///Container type for all input parameters for the `PANIC_ARITHMETIC` function with signature `PANIC_ARITHMETIC()` and selector `0xe29328e4`
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
    #[ethcall(name = "PANIC_ARITHMETIC", abi = "PANIC_ARITHMETIC()")]
    pub struct PanicArithmeticCall;
    ///Container type for all input parameters for the `PANIC_ARRAY_OUT_OF_BOUNDS` function with signature `PANIC_ARRAY_OUT_OF_BOUNDS()` and selector `0x5b8c051a`
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
    #[ethcall(name = "PANIC_ARRAY_OUT_OF_BOUNDS", abi = "PANIC_ARRAY_OUT_OF_BOUNDS()")]
    pub struct PanicArrayOutOfBoundsCall;
    ///Container type for all input parameters for the `PANIC_ASSERT` function with signature `PANIC_ASSERT()` and selector `0x85e2c319`
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
    #[ethcall(name = "PANIC_ASSERT", abi = "PANIC_ASSERT()")]
    pub struct PanicAssertCall;
    ///Container type for all input parameters for the `PANIC_DIVISION_BY_ZERO` function with signature `PANIC_DIVISION_BY_ZERO()` and selector `0x34c8a3d3`
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
    #[ethcall(name = "PANIC_DIVISION_BY_ZERO", abi = "PANIC_DIVISION_BY_ZERO()")]
    pub struct PanicDivisionByZeroCall;
    ///Container type for all input parameters for the `PANIC_ENUM_OUT_OF_BOUNDS` function with signature `PANIC_ENUM_OUT_OF_BOUNDS()` and selector `0x6aa55955`
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
    #[ethcall(name = "PANIC_ENUM_OUT_OF_BOUNDS", abi = "PANIC_ENUM_OUT_OF_BOUNDS()")]
    pub struct PanicEnumOutOfBoundsCall;
    ///Container type for all input parameters for the `PANIC_GENERAL` function with signature `PANIC_GENERAL()` and selector `0xa797916a`
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
    #[ethcall(name = "PANIC_GENERAL", abi = "PANIC_GENERAL()")]
    pub struct PanicGeneralCall;
    ///Container type for all input parameters for the `PANIC_POP_EMPTY_ARRAY` function with signature `PANIC_POP_EMPTY_ARRAY()` and selector `0x77d0b534`
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
    #[ethcall(name = "PANIC_POP_EMPTY_ARRAY", abi = "PANIC_POP_EMPTY_ARRAY()")]
    pub struct PanicPopEmptyArrayCall;
    ///Container type for all input parameters for the `PANIC_STORAGE_BYTES_ARRAY_ENCODING` function with signature `PANIC_STORAGE_BYTES_ARRAY_ENCODING()` and selector `0x5491d30f`
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
        name = "PANIC_STORAGE_BYTES_ARRAY_ENCODING",
        abi = "PANIC_STORAGE_BYTES_ARRAY_ENCODING()"
    )]
    pub struct PanicStorageBytesArrayEncodingCall;
    ///Container type for all input parameters for the `PANIC_ZERO_INIT_INTERNAL_FUNCTION` function with signature `PANIC_ZERO_INIT_INTERNAL_FUNCTION()` and selector `0x2cfa2cad`
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
        name = "PANIC_ZERO_INIT_INTERNAL_FUNCTION",
        abi = "PANIC_ZERO_INIT_INTERNAL_FUNCTION()"
    )]
    pub struct PanicZeroInitInternalFunctionCall;
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
    pub enum ConstantsCalls {
        AddressCheats(AddressCheatsCall),
        PanicAllocTooMuchMemory(PanicAllocTooMuchMemoryCall),
        PanicArithmetic(PanicArithmeticCall),
        PanicArrayOutOfBounds(PanicArrayOutOfBoundsCall),
        PanicAssert(PanicAssertCall),
        PanicDivisionByZero(PanicDivisionByZeroCall),
        PanicEnumOutOfBounds(PanicEnumOutOfBoundsCall),
        PanicGeneral(PanicGeneralCall),
        PanicPopEmptyArray(PanicPopEmptyArrayCall),
        PanicStorageBytesArrayEncoding(PanicStorageBytesArrayEncodingCall),
        PanicZeroInitInternalFunction(PanicZeroInitInternalFunctionCall),
    }
    impl ::ethers::core::abi::AbiDecode for ConstantsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddressCheatsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressCheats(decoded));
            }
            if let Ok(decoded) = <PanicAllocTooMuchMemoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PanicAllocTooMuchMemory(decoded));
            }
            if let Ok(decoded) = <PanicArithmeticCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PanicArithmetic(decoded));
            }
            if let Ok(decoded) = <PanicArrayOutOfBoundsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PanicArrayOutOfBounds(decoded));
            }
            if let Ok(decoded) = <PanicAssertCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PanicAssert(decoded));
            }
            if let Ok(decoded) = <PanicDivisionByZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PanicDivisionByZero(decoded));
            }
            if let Ok(decoded) = <PanicEnumOutOfBoundsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PanicEnumOutOfBounds(decoded));
            }
            if let Ok(decoded) = <PanicGeneralCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PanicGeneral(decoded));
            }
            if let Ok(decoded) = <PanicPopEmptyArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PanicPopEmptyArray(decoded));
            }
            if let Ok(decoded) = <PanicStorageBytesArrayEncodingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PanicStorageBytesArrayEncoding(decoded));
            }
            if let Ok(decoded) = <PanicZeroInitInternalFunctionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PanicZeroInitInternalFunction(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConstantsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressCheats(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PanicAllocTooMuchMemory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PanicArithmetic(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PanicArrayOutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PanicAssert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PanicDivisionByZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PanicEnumOutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PanicGeneral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PanicPopEmptyArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PanicStorageBytesArrayEncoding(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PanicZeroInitInternalFunction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ConstantsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressCheats(element) => ::core::fmt::Display::fmt(element, f),
                Self::PanicAllocTooMuchMemory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PanicArithmetic(element) => ::core::fmt::Display::fmt(element, f),
                Self::PanicArrayOutOfBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PanicAssert(element) => ::core::fmt::Display::fmt(element, f),
                Self::PanicDivisionByZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PanicEnumOutOfBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PanicGeneral(element) => ::core::fmt::Display::fmt(element, f),
                Self::PanicPopEmptyArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PanicStorageBytesArrayEncoding(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PanicZeroInitInternalFunction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddressCheatsCall> for ConstantsCalls {
        fn from(value: AddressCheatsCall) -> Self {
            Self::AddressCheats(value)
        }
    }
    impl ::core::convert::From<PanicAllocTooMuchMemoryCall> for ConstantsCalls {
        fn from(value: PanicAllocTooMuchMemoryCall) -> Self {
            Self::PanicAllocTooMuchMemory(value)
        }
    }
    impl ::core::convert::From<PanicArithmeticCall> for ConstantsCalls {
        fn from(value: PanicArithmeticCall) -> Self {
            Self::PanicArithmetic(value)
        }
    }
    impl ::core::convert::From<PanicArrayOutOfBoundsCall> for ConstantsCalls {
        fn from(value: PanicArrayOutOfBoundsCall) -> Self {
            Self::PanicArrayOutOfBounds(value)
        }
    }
    impl ::core::convert::From<PanicAssertCall> for ConstantsCalls {
        fn from(value: PanicAssertCall) -> Self {
            Self::PanicAssert(value)
        }
    }
    impl ::core::convert::From<PanicDivisionByZeroCall> for ConstantsCalls {
        fn from(value: PanicDivisionByZeroCall) -> Self {
            Self::PanicDivisionByZero(value)
        }
    }
    impl ::core::convert::From<PanicEnumOutOfBoundsCall> for ConstantsCalls {
        fn from(value: PanicEnumOutOfBoundsCall) -> Self {
            Self::PanicEnumOutOfBounds(value)
        }
    }
    impl ::core::convert::From<PanicGeneralCall> for ConstantsCalls {
        fn from(value: PanicGeneralCall) -> Self {
            Self::PanicGeneral(value)
        }
    }
    impl ::core::convert::From<PanicPopEmptyArrayCall> for ConstantsCalls {
        fn from(value: PanicPopEmptyArrayCall) -> Self {
            Self::PanicPopEmptyArray(value)
        }
    }
    impl ::core::convert::From<PanicStorageBytesArrayEncodingCall> for ConstantsCalls {
        fn from(value: PanicStorageBytesArrayEncodingCall) -> Self {
            Self::PanicStorageBytesArrayEncoding(value)
        }
    }
    impl ::core::convert::From<PanicZeroInitInternalFunctionCall> for ConstantsCalls {
        fn from(value: PanicZeroInitInternalFunctionCall) -> Self {
            Self::PanicZeroInitInternalFunction(value)
        }
    }
    ///Container type for all return fields from the `ADDRESS_CHEATS` function with signature `ADDRESS_CHEATS()` and selector `0x6230fde5`
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
    pub struct AddressCheatsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `PANIC_ALLOC_TOO_MUCH_MEMORY` function with signature `PANIC_ALLOC_TOO_MUCH_MEMORY()` and selector `0x57773b7a`
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
    pub struct PanicAllocTooMuchMemoryReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PANIC_ARITHMETIC` function with signature `PANIC_ARITHMETIC()` and selector `0xe29328e4`
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
    pub struct PanicArithmeticReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PANIC_ARRAY_OUT_OF_BOUNDS` function with signature `PANIC_ARRAY_OUT_OF_BOUNDS()` and selector `0x5b8c051a`
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
    pub struct PanicArrayOutOfBoundsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PANIC_ASSERT` function with signature `PANIC_ASSERT()` and selector `0x85e2c319`
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
    pub struct PanicAssertReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PANIC_DIVISION_BY_ZERO` function with signature `PANIC_DIVISION_BY_ZERO()` and selector `0x34c8a3d3`
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
    pub struct PanicDivisionByZeroReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PANIC_ENUM_OUT_OF_BOUNDS` function with signature `PANIC_ENUM_OUT_OF_BOUNDS()` and selector `0x6aa55955`
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
    pub struct PanicEnumOutOfBoundsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PANIC_GENERAL` function with signature `PANIC_GENERAL()` and selector `0xa797916a`
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
    pub struct PanicGeneralReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PANIC_POP_EMPTY_ARRAY` function with signature `PANIC_POP_EMPTY_ARRAY()` and selector `0x77d0b534`
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
    pub struct PanicPopEmptyArrayReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PANIC_STORAGE_BYTES_ARRAY_ENCODING` function with signature `PANIC_STORAGE_BYTES_ARRAY_ENCODING()` and selector `0x5491d30f`
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
    pub struct PanicStorageBytesArrayEncodingReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PANIC_ZERO_INIT_INTERNAL_FUNCTION` function with signature `PANIC_ZERO_INIT_INTERNAL_FUNCTION()` and selector `0x2cfa2cad`
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
    pub struct PanicZeroInitInternalFunctionReturn(pub ::ethers::core::types::U256);
}
