pub use address_utils::*;
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
pub mod address_utils {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "AddressUtils__InsufficientBalance",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressUtils__InsufficientBalance",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressUtils__NotContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressUtils__NotContract",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressUtils__SendValueFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressUtils__SendValueFailed",
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
    pub static ADDRESSUTILS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`,`2`\x0B\x82\x82\x829\x80Q_\x1A`s\x14`&WcNH{q`\xE0\x1B_R_`\x04R`$_\xFD[0_R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R__\xFD\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static ADDRESSUTILS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R__\xFD\xFE\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static ADDRESSUTILS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AddressUtils<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AddressUtils<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AddressUtils<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AddressUtils<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AddressUtils<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AddressUtils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AddressUtils<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ADDRESSUTILS_ABI.clone(),
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
                ADDRESSUTILS_ABI.clone(),
                ADDRESSUTILS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AddressUtils<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressUtils__InsufficientBalance` with signature `AddressUtils__InsufficientBalance()` and selector `0xc052e61b`
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
        name = "AddressUtils__InsufficientBalance",
        abi = "AddressUtils__InsufficientBalance()"
    )]
    pub struct AddressUtils__InsufficientBalance;
    ///Custom Error type `AddressUtils__NotContract` with signature `AddressUtils__NotContract()` and selector `0x89c35afc`
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
    #[etherror(name = "AddressUtils__NotContract", abi = "AddressUtils__NotContract()")]
    pub struct AddressUtils__NotContract;
    ///Custom Error type `AddressUtils__SendValueFailed` with signature `AddressUtils__SendValueFailed()` and selector `0xc6d73c55`
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
        name = "AddressUtils__SendValueFailed",
        abi = "AddressUtils__SendValueFailed()"
    )]
    pub struct AddressUtils__SendValueFailed;
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
    pub enum AddressUtilsErrors {
        AddressUtils__InsufficientBalance(AddressUtils__InsufficientBalance),
        AddressUtils__NotContract(AddressUtils__NotContract),
        AddressUtils__SendValueFailed(AddressUtils__SendValueFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AddressUtilsErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressUtils__InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressUtils__InsufficientBalance(decoded));
            }
            if let Ok(decoded) = <AddressUtils__NotContract as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressUtils__NotContract(decoded));
            }
            if let Ok(decoded) = <AddressUtils__SendValueFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressUtils__SendValueFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AddressUtilsErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressUtils__InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressUtils__NotContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressUtils__SendValueFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AddressUtilsErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressUtils__InsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressUtils__NotContract as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressUtils__SendValueFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AddressUtilsErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressUtils__InsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressUtils__NotContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressUtils__SendValueFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AddressUtilsErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressUtils__InsufficientBalance>
    for AddressUtilsErrors {
        fn from(value: AddressUtils__InsufficientBalance) -> Self {
            Self::AddressUtils__InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<AddressUtils__NotContract> for AddressUtilsErrors {
        fn from(value: AddressUtils__NotContract) -> Self {
            Self::AddressUtils__NotContract(value)
        }
    }
    impl ::core::convert::From<AddressUtils__SendValueFailed> for AddressUtilsErrors {
        fn from(value: AddressUtils__SendValueFailed) -> Self {
            Self::AddressUtils__SendValueFailed(value)
        }
    }
}
