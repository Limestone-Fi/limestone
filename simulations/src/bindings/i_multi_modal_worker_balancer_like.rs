pub use i_multi_modal_worker_balancer_like::*;
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
pub mod i_multi_modal_worker_balancer_like {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("invest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BalancerLikePositionInvestmentContext",
                                        ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
    pub static IMULTIMODALWORKERBALANCERLIKE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IMultiModalWorkerBalancerLike<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IMultiModalWorkerBalancerLike<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IMultiModalWorkerBalancerLike<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IMultiModalWorkerBalancerLike<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IMultiModalWorkerBalancerLike<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IMultiModalWorkerBalancerLike))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IMultiModalWorkerBalancerLike<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IMULTIMODALWORKERBALANCERLIKE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `invest` (0x3f761a31) function
        pub fn invest(
            &self,
            p0: BalancerLikePositionInvestmentContext,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([63, 118, 26, 49], (p0,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IMultiModalWorkerBalancerLike<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `invest` function with signature `invest((uint256,address,uint256[],uint256[],uint256[],uint256))` and selector `0x3f761a31`
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
        name = "invest",
        abi = "invest((uint256,address,uint256[],uint256[],uint256[],uint256))"
    )]
    pub struct InvestCall(pub BalancerLikePositionInvestmentContext);
    ///Container type for all return fields from the `invest` function with signature `invest((uint256,address,uint256[],uint256[],uint256[],uint256))` and selector `0x3f761a31`
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
    pub struct InvestReturn(pub ::ethers::core::types::U256);
    ///`BalancerLikePositionInvestmentContext(uint256,address,uint256[],uint256[],uint256[],uint256)`
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
    pub struct BalancerLikePositionInvestmentContext {
        pub position_id: ::ethers::core::types::U256,
        pub worker: ::ethers::core::types::Address,
        pub token_amounts_in: ::std::vec::Vec<::ethers::core::types::U256>,
        pub token_pool_ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub token_borrows: ::std::vec::Vec<::ethers::core::types::U256>,
        pub min_liquidity: ::ethers::core::types::U256,
    }
}
