pub use helper_math::*;
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
pub mod helper_math {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("abs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("abs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("n"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("abs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("n"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("diff"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("diff"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("diff"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("max"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("max"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("max"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("min"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("min"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
    pub static HELPERMATH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct HelperMath<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for HelperMath<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HelperMath<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HelperMath<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HelperMath<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(HelperMath)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> HelperMath<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    HELPERMATH_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `abs` (0x1b5ac4b5) function
        pub fn abs(
            &self,
            n: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([27, 90, 196, 181], n)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `abs` (0xe01e92d2) function
        pub fn abs_with_n(
            &self,
            n: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([224, 30, 146, 210], n)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `diff` (0x568c1385) function
        pub fn diff(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([86, 140, 19, 133], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `diff` (0x8f5e514a) function
        pub fn diff_with_a_and_b(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([143, 94, 81, 74], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `max` (0x6d5433e6) function
        pub fn max(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([109, 84, 51, 230], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `max` (0x81fe5786) function
        pub fn max_with_a_and_b(
            &self,
            a: ::ethers::core::types::I256,
            b: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([129, 254, 87, 134], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `min` (0x7ae2b5c7) function
        pub fn min(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([122, 226, 181, 199], (a, b))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for HelperMath<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `abs` function with signature `abs(int256)` and selector `0x1b5ac4b5`
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
    #[ethcall(name = "abs", abi = "abs(int256)")]
    pub struct AbsCall {
        pub n: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `abs` function with signature `abs(int128)` and selector `0xe01e92d2`
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
    #[ethcall(name = "abs", abi = "abs(int128)")]
    pub struct AbsWithNCall {
        pub n: i128,
    }
    ///Container type for all input parameters for the `diff` function with signature `diff(int256,int256)` and selector `0x568c1385`
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
    #[ethcall(name = "diff", abi = "diff(int256,int256)")]
    pub struct DiffCall {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `diff` function with signature `diff(uint256,uint256)` and selector `0x8f5e514a`
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
    #[ethcall(name = "diff", abi = "diff(uint256,uint256)")]
    pub struct DiffWithAAndBCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `max` function with signature `max(uint256,uint256)` and selector `0x6d5433e6`
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
    #[ethcall(name = "max", abi = "max(uint256,uint256)")]
    pub struct MaxCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `max` function with signature `max(int256,int256)` and selector `0x81fe5786`
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
    #[ethcall(name = "max", abi = "max(int256,int256)")]
    pub struct MaxWithAAndBCall {
        pub a: ::ethers::core::types::I256,
        pub b: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `min` function with signature `min(uint256,uint256)` and selector `0x7ae2b5c7`
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
    #[ethcall(name = "min", abi = "min(uint256,uint256)")]
    pub struct MinCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
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
    pub enum HelperMathCalls {
        Abs(AbsCall),
        AbsWithN(AbsWithNCall),
        Diff(DiffCall),
        DiffWithAAndB(DiffWithAAndBCall),
        Max(MaxCall),
        MaxWithAAndB(MaxWithAAndBCall),
        Min(MinCall),
    }
    impl ::ethers::core::abi::AbiDecode for HelperMathCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AbsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Abs(decoded));
            }
            if let Ok(decoded) = <AbsWithNCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AbsWithN(decoded));
            }
            if let Ok(decoded) = <DiffCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Diff(decoded));
            }
            if let Ok(decoded) = <DiffWithAAndBCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DiffWithAAndB(decoded));
            }
            if let Ok(decoded) = <MaxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Max(decoded));
            }
            if let Ok(decoded) = <MaxWithAAndBCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxWithAAndB(decoded));
            }
            if let Ok(decoded) = <MinCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Min(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HelperMathCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Abs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AbsWithN(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Diff(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DiffWithAAndB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Max(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxWithAAndB(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for HelperMathCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Abs(element) => ::core::fmt::Display::fmt(element, f),
                Self::AbsWithN(element) => ::core::fmt::Display::fmt(element, f),
                Self::Diff(element) => ::core::fmt::Display::fmt(element, f),
                Self::DiffWithAAndB(element) => ::core::fmt::Display::fmt(element, f),
                Self::Max(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxWithAAndB(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AbsCall> for HelperMathCalls {
        fn from(value: AbsCall) -> Self {
            Self::Abs(value)
        }
    }
    impl ::core::convert::From<AbsWithNCall> for HelperMathCalls {
        fn from(value: AbsWithNCall) -> Self {
            Self::AbsWithN(value)
        }
    }
    impl ::core::convert::From<DiffCall> for HelperMathCalls {
        fn from(value: DiffCall) -> Self {
            Self::Diff(value)
        }
    }
    impl ::core::convert::From<DiffWithAAndBCall> for HelperMathCalls {
        fn from(value: DiffWithAAndBCall) -> Self {
            Self::DiffWithAAndB(value)
        }
    }
    impl ::core::convert::From<MaxCall> for HelperMathCalls {
        fn from(value: MaxCall) -> Self {
            Self::Max(value)
        }
    }
    impl ::core::convert::From<MaxWithAAndBCall> for HelperMathCalls {
        fn from(value: MaxWithAAndBCall) -> Self {
            Self::MaxWithAAndB(value)
        }
    }
    impl ::core::convert::From<MinCall> for HelperMathCalls {
        fn from(value: MinCall) -> Self {
            Self::Min(value)
        }
    }
    ///Container type for all return fields from the `abs` function with signature `abs(int256)` and selector `0x1b5ac4b5`
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
    pub struct AbsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `abs` function with signature `abs(int128)` and selector `0xe01e92d2`
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
    pub struct AbsWithNReturn(pub i128);
    ///Container type for all return fields from the `diff` function with signature `diff(int256,int256)` and selector `0x568c1385`
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
    pub struct DiffReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `diff` function with signature `diff(uint256,uint256)` and selector `0x8f5e514a`
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
    pub struct DiffWithAAndBReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `max` function with signature `max(uint256,uint256)` and selector `0x6d5433e6`
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
    pub struct MaxReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `max` function with signature `max(int256,int256)` and selector `0x81fe5786`
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
    pub struct MaxWithAAndBReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `min` function with signature `min(uint256,uint256)` and selector `0x7ae2b5c7`
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
    pub struct MinReturn(pub ::ethers::core::types::U256);
}
