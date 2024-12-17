pub use position_coordinator::*;
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
pub mod position_coordinator {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("accessAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accessAssets"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                    ::std::borrow::ToOwned::to_owned("divestFromV2LikePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "divestFromV2LikePosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ctx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct V2LikePositionDivestmentContext",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("investInV2LikePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "investInV2LikePosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ctx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct V2LikePositionInvestmentContext",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("liquidateV2LikePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "liquidateV2LikePosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ctx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct V2LikePositionLiquidationContext",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("reinvestmentFeeNumerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "reinvestmentFeeNumerator",
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
                    ::std::borrow::ToOwned::to_owned("repayV2LikeLiquidityPositionDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "repayV2LikeLiquidityPositionDebt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_positionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_worker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_repayToken0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_repayToken1"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PositionDivested"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PositionDivested"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("positionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("worker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityBurnt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token0Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token1Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token0Repaid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token1Repaid"),
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
                    ::std::borrow::ToOwned::to_owned("PositionInvested"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PositionInvested"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("positionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("worker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token0In"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token1In"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token0Borrowed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token1Borrowed"),
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
    pub static POSITIONCOORDINATOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa \xE1\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0_W_5`\xE0\x1C\x80bi\x8B7\x14a\0cW\x80cS\x8D\xBB\x9A\x14a\0xW\x80ca4`q\x14a\0\x8BW\x80cu\xDF\x04\x9E\x14a\0\xA0W\x80c\xE5\xF2f\xE6\x14a\0\xB3W\x80c\xF3\xE2\x15\xA6\x14a\0\xC6W[__\xFD[a\0va\0q6`\x04a\x19\xF3V[a\0\xD9V[\0[a\0va\0\x866`\x04a\x1A4V[a\x05cV[a\x03 `@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0va\0\xAE6`\x04a\x1A\xB6V[a\x07\xCFV[a\0va\0\xC16`\x04a\x1B8V[a\x08\x92V[a\0va\0\xD46`\x04a\x1BIV[a\x0B\x8CV[`\x80\x81\x81\x015_\x90\x81R_Q` a \xB5_9_Q\x90_R` \x90\x81R`@\x80\x83 `\xA0\x80\x87\x015\x85R\x82\x85 \x83Q`\xC0\x80\x82\x01\x86R\x87\x82R\x95\x81\x01\x87\x90R\x93\x84\x01\x86\x90R``\x84\x01\x86\x90R\x95\x83\x01\x85\x90R\x82\x01\x93\x90\x93R_Q` a \x95_9_Q\x90_R\x93\x91\x85\x015\x15a\x02DWa\x01V\x85`\x80\x015a\x0F\xF8V[a\x01\x85`\x80\x86\x015`\xC0\x87\x015_a\x01t`@\x8A\x01` \x8B\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x10\xD4V[a\x01\xB2`\x80\x86\x015`\xC0\x87\x015a\x01\xA2`@\x89\x01` \x8A\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x11\xA4V[`\x01`\x01`p\x1B\x03\x16\x81R`\x02\x83\x01Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16c\xE4xy]a\x01\xE5`@\x88\x01` \x89\x01a\x1B[V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`\xC0\x88\x015`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02-W__\xFD[PZ\xF1\x15\x80\x15a\x02?W=__>=_\xFD[PPPP[`\xE0\x85\x015\x15a\x031Wa\x02[\x85`\xA0\x015a\x0F\xF8V[a\x02y`\xA0\x86\x015`\xE0\x87\x015_a\x01t`@\x8A\x01` \x8B\x01a\x1B[V[a\x02\x96`\xA0\x86\x015`\xE0\x87\x015a\x01\xA2`@\x89\x01` \x8A\x01a\x1B[V[`\x01`\x01`p\x1B\x03\x16` \x82\x81\x01\x91\x90\x91R`\x02\x83\x01Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE4xy]\x90a\x02\xD2\x90`@\x89\x01\x90\x89\x01a\x1B[V[`@Q`\xE0\x83\x81\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x82\x01R\x90\x88\x015`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03\x1AW__\xFD[PZ\xF1\x15\x80\x15a\x03,W=__>=_\xFD[PPPP[a\x03J3a\x03E`@\x88\x01` \x89\x01a\x1B[V[a\x12rV[`@\x80\x83\x01\x91\x90\x91Ra\x03m\x90_\x90a\x03h\x90\x88\x01` \x89\x01a\x1B[V[a\x12\xBAV[a\x03}`@\x86\x01` \x87\x01a\x1B[V[\x81Q` \x83\x01Q`@Qco\xE8d\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92co\xE8d\xC5\x92a\x03\xB6\x92\x8A\x923\x92\x91\x90`\x04\x01a\x1B\x85V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xF6\x91\x90a\x1C5V[``\x82\x01Ra\x04\x0B`@\x86\x01` \x87\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16c\x02\xA5\x03)\x82``\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04<\x91\x81R` \x01\x90V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04z\x91\x90a\x1CLV[`\xA0\x83\x01\x81\x90R`\x80\x83\x01\x82\x90Ra\x04\xD1\x91\x11\x80\x15a\x04\xCAWP`\x80\x82\x01Qa\x04\xA4\x90`da\x1C\x82V[\x82`@\x01Q\x83`\xA0\x01Q\x84`\x80\x01Qa\x04\xBD\x91\x90a\x1C\x99V[a\x04\xC7\x91\x90a\x1C\x82V[\x10\x15[`4a\x133V[a\x04\xE1`@\x86\x01` \x87\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x82``\x01Q\x7F\x03\xCF>e\x83#\xC4\x0CO\x8C\xCF\xCC\x9E\xE1G\xD2\x8D\x05\x13\xB6\xEB\xA2\xF7' Pk#7Ho9\x88`@\x015\x89``\x015\x8A`\xC0\x015\x8B`\xE0\x015`@Qa\x05T\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPPV[`@Qc\xEB\x02\xC3\x01`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R_\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xEB\x02\xC3\x01\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xCC\x91\x90a\x1C\xD5V[\x90Pa\x05\xE1\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16a\x0F\xF8V[a\x05\xF4\x81``\x01Qc\xFF\xFF\xFF\xFF\x16a\x0F\xF8V[`@Qc\x02\xA5\x03)`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R_\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x02\xA5\x03)\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x068W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\\\x91\x90a\x1CLV[`@Qc\x80Y@\t`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x87\x90R`D\x81\x01\x86\x90R\x90\x92P_\x91P\x81\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\x80Y@\t\x90`d\x01`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD9\x91\x90a\x1D\x82V[\x90\x92P\x90P`\x01`\x01`p\x1B\x03\x82\x16\x15a\x07\x15Wa\x07\x15\x84`@\x01Qc\xFF\xFF\xFF\xFF\x16\x83\x89`\x01`\x01`\xA0\x1B\x03\x16a\x13A\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01`\x01`p\x1B\x03\x81\x16\x15a\x07LWa\x07L\x84``\x01Qc\xFF\xFF\xFF\xFF\x16\x82\x89`\x01`\x01`\xA0\x1B\x03\x16a\x13A\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@Qc\x02\xA5\x03)`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R_\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x02\xA5\x03)\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x90W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xB4\x91\x90a\x1CLV[\x91PPa\x07\xC4\x84\x82\x10`;a\x133V[PPPPPPPPPV[_a\x07\xD8a\x14VV[\x90Pa\x07\xFD\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14`*a\x133V[_[\x84\x81\x10\x15a\x08\x89W_\x84\x84\x83\x81\x81\x10a\x08\x1AWa\x08\x1Aa\x1D\xB3V[\x90P` \x02\x015\x11\x15a\x08\x81Wa\x08\x81\x87\x83` \x01Q\x86\x86\x85\x81\x81\x10a\x08BWa\x08Ba\x1D\xB3V[\x90P` \x02\x015\x89\x89\x86\x81\x81\x10a\x08[Wa\x08[a\x1D\xB3V[\x90P` \x02\x01` \x81\x01\x90a\x08p\x91\x90a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x14\xACV[`\x01\x01a\x07\xFFV[PPPPPPPV[_Q` a \x95_9_Q\x90_R_a\x08\xB1`@\x84\x01` \x85\x01a\x1B[V[`@Qc\xEB\x02\xC3\x01`\xE0\x1B\x81R\x845`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xEB\x02\xC3\x01\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x1A\x91\x90a\x1C\xD5V[\x90Pa\t/\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16a\x0F\xF8V[a\tB\x81``\x01Qc\xFF\xFF\xFF\xFF\x16a\x0F\xF8V[_\x80a\tT`@\x86\x01` \x87\x01a\x1B[V[`@Qc\x02\xA5\x03)`\xE0\x1B\x81R\x865`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x02\xA5\x03)\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x98W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xBC\x91\x90a\x1CLV[\x90\x92P\x90Pa\n\x1Ca\t\xD0\x82a'\x10a\x1C\x82V[`\x05\x86\x01_a\t\xE5`@\x8A\x01` \x8B\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ Ta\n\x14\x90d\x01\0\0\0\0\x90\x04a\xFF\xFF\x16\x85a\x1C\x82V[\x10`7a\x133V[a\n0_a\x03h`@\x88\x01` \x89\x01a\x1B[V[a\n@`@\x86\x01` \x87\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16c\xE9\rLw3\x87`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\nm\x92\x91\x90a\x1D\xD8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\x84W__\xFD[PZ\xF1\x15\x80\x15a\n\x96W=__>=_\xFD[Pa\n\xAB\x92PPP`@\x86\x01` \x87\x01a\x1B[V[`@Qc\x02\xA5\x03)`\xE0\x1B\x81R\x865`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x02\xA5\x03)\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xEFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x13\x91\x90a\x1CLV[\x90\x92P\x90Pa\x0B\x85a\x0B'\x82a'\x10a\x1C\x82V[a\x03\xE8`\x05\x87\x01_a\x0B?`@\x8B\x01` \x8C\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ Ta\x0Bn\x91\x90d\x01\0\0\0\0\x90\x04a\xFF\xFF\x16a\x1E_V[a\x0B|\x90a\xFF\xFF\x16\x85a\x1C\x82V[\x10\x15`<a\x133V[PPPPPV[_Q` a \x95_9_Q\x90_R_a\x0B\xAB`@\x84\x01` \x85\x01a\x1B[V[`@Qc\xEB\x02\xC3\x01`\xE0\x1B\x81R\x845`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xEB\x02\xC3\x01\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x14\x91\x90a\x1C\xD5V[\x90Pa\x0C\\`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a\x0Co\x82`@\x01Qc\xFF\xFF\xFF\xFF\x16a\x0F\xF8V[a\x0C\x82\x82``\x01Qc\xFF\xFF\xFF\xFF\x16a\x0F\xF8V[a\x0C\x963a\x03E`@\x87\x01` \x88\x01a\x1B[V[\x81Ra\x0C\xA8`@\x85\x01` \x86\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16c\xA1'\x1CK\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xD3\x91\x90a\x1E\x89V[`\x80`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xEFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x13\x91\x90a\x1F\x1DV[`\x80\x85\x81\x01\x82\x90R``\x86\x01\x83\x90R`@\x86\x01\x93\x90\x93R` \x85\x01\x93\x90\x93R\x90\x84\x01Q`\x01`\x01`p\x1B\x03\x90\x81\x16\x80\x83\x03\x92\x10_\x81\x81\x03\x93\x90\x93\x18\x01`\xA0\x80\x86\x01\x82\x90R\x86\x01Q\x90\x91\x16\x80\x84\x03\x93\x10\x91\x82\x90\x03\x92\x90\x92\x18\x01`\xC0\x83\x01R\x15a\r\xB0Wa\r\xB0\x82`@\x01Qc\xFF\xFF\xFF\xFF\x16a\r\x90\x83`\xA0\x01Qa\x15\x05V[a\r\xA0`@\x88\x01` \x89\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x13AV[`\xC0\x81\x01Q\x15a\r\xD5Wa\r\xD5\x82``\x01Qc\xFF\xFF\xFF\xFF\x16a\r\x90\x83`\xC0\x01Qa\x15\x05V[_\x81``\x01Q\x11\x80a\r\xEAWP_\x81`\x80\x01Q\x11[\x15a\x0E\xA9Wa\r\xFF`@\x85\x01` \x86\x01a\x1B[V[`@Qc\x02\xA5\x03)`\xE0\x1B\x81R\x855`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x02\xA5\x03)\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0ECW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Eg\x91\x90a\x1CLV[a\x01\0\x83\x01\x81\x90R`\xE0\x83\x01\x82\x90Ra\x0E\xA9\x91\x11\x80\x15a\x04\xCAWP`\xE0\x82\x01Qa\x0E\x92\x90`da\x1C\x82V[\x82Qa\x01\0\x84\x01Q`\xE0\x85\x01Qa\x04\xBD\x91\x90a\x1C\x99V[`@\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`\x03\x80\x87\x01` R\x83\x82 ``\x87\x01Q\x84\x16\x83R\x93\x90\x91 `\x02\x84\x01T\x91\x84\x01T\x90\x92a\x0F'\x92`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16\x92a\x0F\x0B\x92`\x01``\x1B\x90\x91\x04\x90\x91\x16\x900\x90a\x15\x19\x16V[`\x03\x85\x01T`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x15CV[`\x02\x81\x01T`\x03\x82\x01Ta\x0Fu\x91`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16\x91a\x0FY\x91`\x01``\x1B\x90\x91\x04\x160a\x15\x19V[`\x03\x84\x01T`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x15CV[a\x0F\x85`@\x87\x01` \x88\x01a\x1B[V[` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q_\x81R\x93\x84\x01\x92\x90\x92R\x82\x01R`\xA0\x80\x89\x015``\x83\x01R`\xC0\x89\x015`\x80\x83\x01R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x885\x91\x7F\x01i\xAF\x99m\xCD\"N\xDB\x94\n\xFB\x8E\x8B\xBD\xB0\x19\xB4\xA5\xAB\xAFd\x9C\xCAn7\xFF[\x13{x\xEE\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPV[c\xFF\xFF\xFF\xFE\x19\x81\x01a\x10\x07WPV[_\x81\x81R_Q` a \xB5_9_Q\x90_R` R`@\x90 \x80T`\x01`p\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16B\x11\x15a\x10\xD0W_a\x10@\x83a\x15\x8DV[`\x01\x83\x01T\x90\x91P_\x90a'\x10\x90a\x10c\x90`\x01`p\x1B\x90\x04a\xFF\xFF\x16\x84a\x1FPV[a\x10m\x91\x90a\x1F\x86V[`\x01\x84\x01\x80T`\x01`\x01`\x90\x1B\x03\x81\x16`\x01`\x90\x1B\x91\x82\x90\x04`\x01`\x01`p\x1B\x03\x90\x81\x16\x94\x90\x94\x01\x84\x16\x82\x02\x17\x90\x91U\x84T\x80\x83\x16\x90\x82\x90\x04\x83\x16\x94\x90\x94\x01\x90\x91\x16\x02c\xFF\xFF\xFF\xFF`p\x1B\x19\x16\x91\x90\x91\x17`\x01`p\x1BBc\xFF\xFF\xFF\xFF\x16\x02\x17\x82UP[PPV[_\x83\x81R_Q` a \xB5_9_Q\x90_R` R`@\x90 _Q` a \x95_9_Q\x90_R\x90\x82\x15a\x11QW\x83\x81`\x02\x01`\x15\x82\x82\x82\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`X\x1B\x03\x16a\x11(\x91\x90a\x1F\xB3V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`X\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`X\x1B\x03\x16\x02\x17\x90UPa\x11\x9CV[\x83\x81`\x02\x01`\x15\x82\x82\x82\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`X\x1B\x03\x16a\x11w\x91\x90a\x1F\xD2V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`X\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`X\x1B\x03\x16\x02\x17\x90UP[PPPPPPV[_\x82\x81R_Q` a \xB5_9_Q\x90_R` R`@\x81 _Q` a \x95_9_Q\x90_R\x90a\x11\xDE\x85a\x11\xD9\x86a\x15\x05V[a\x16\xE0V[\x92Pa\x11\xE9\x84a\x15\x05V[\x81T`\x01`\x01`p\x1B\x03`\x01`\x90\x1B\x80\x83\x04\x82\x16\x90\x93\x01\x81\x16\x90\x92\x02`\x01`\x01`\x90\x1B\x03\x90\x91\x16\x17\x82U`\x01\x90\x91\x01\x80T\x80\x83\x16\x85\x01\x83\x16`\x01`\x01`p\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x90\x96\x16_\x90\x81R`\t\x90\x92\x01` \x90\x81R`@\x80\x84 \x96\x84R\x95\x90R\x93\x90 \x80T\x80\x85\x16\x83\x01\x90\x94\x16\x93\x90\x94\x16\x92\x90\x92\x17\x90\x92U\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R\x7F\x98\xE7\xA3\xEEyht6\xFF\xE8\\\xF3\xA9\x99\xA4\xA8E\xB4\xA7\xC6\xDD-wy\xAAS|\xE4\x84\xAF-R` R`@\x90 Tb\x01\0\0\x90\x04a\xFF\xFF\x16[\x92\x91PPV[`@\x80Q\x80\x82\x01\x82Rc\xFF\xFF\xFF\xFF\x84\x16\x80\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16` \x93\x84\x01\x90\x81R\x84Q\x93\x84\x01\x92\x90\x92R\x90Q\x16\x81\x83\x01R\x81Q\x80\x82\x03\x83\x01\x81R``\x90\x91\x01\x90\x91R\x7F%\xACH\xEB.\x9D\xA4h\x18\xEF\xCE\xB7\xF5\x16\xCC\xED}\xAE\x8D.(\xDE\\\xD6Jy\xCDA\xF1\xE4\x8F>\x90a\x13.\x90\x82\x90a\x17mV[PPPV[\x81a\x10\xD0Wa\x10\xD0\x81a\x17\xB0V[_\x82\x81R_Q` a \xB5_9_Q\x90_R` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x7F\x98\xE7\xA3\xEEyht6\xFF\xE8\\\xF3\xA9\x99\xA4\xA8E\xB4\xA7\xC6\xDD-wy\xAAS|\xE4\x84\xAF-V\x83R\x81\x84 \x86\x85R\x90\x92R\x90\x91 T_Q` a \x95_9_Q\x90_R\x91\x90`\x01`\x01`p\x1B\x03\x90\x81\x16\x90\x84\x16\x81\x10\x15a\x13\xC2W\x80\x93P[_a\x13\xCD\x86\x86a\x18\x08V[\x83T`\x01`\x01`p\x1B\x03`\x01`\x90\x1B\x80\x83\x04\x82\x16\x93\x90\x93\x03\x81\x16\x90\x92\x02`\x01`\x01`\x90\x1B\x03\x90\x91\x16\x17\x84U`\x01\x90\x93\x01\x80T\x80\x85\x16\x87\x90\x03\x85\x16`\x01`\x01`p\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x90\x97\x16_\x90\x81R`\t\x90\x94\x01` \x90\x81R`@\x80\x86 \x97\x86R\x96\x90RPP\x92\x90 \x80T\x80\x84\x16\x92\x90\x92\x03\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x7F%\xACH\xEB.\x9D\xA4h\x18\xEF\xCE\xB7\xF5\x16\xCC\xED}\xAE\x8D.(\xDE\\\xD6Jy\xCDA\xF1\xE4\x8F>a\x14\x93\x81a\x18\x83V[\x80` \x01\x90Q\x81\x01\x90a\x14\xA6\x91\x90a\x1F\xF1V[\x91PP\x90V[`@Q\x81``R\x82`@R\x83``\x1B`,Rc#\xB8r\xDD``\x1B`\x0CR` _`d`\x1C_\x89Z\xF1\x80`\x01_Q\x14\x16a\x14\xF7W\x80=\x87;\x15\x17\x10a\x14\xF7Wcy9\xF4$_R`\x04`\x1C\xFD[P_``R`@RPPPPV[_`\x01`p\x1B\x82\x10a\x15\x15W__\xFD[P\x90V[_\x81`\x14Rcp\xA0\x821``\x1B_R` \x80`$`\x10\x86Z\xFA`\x1F=\x11\x16` Q\x02\x90P\x92\x91PPV[\x81`\x14R\x80`4Rc\xA9\x05\x9C\xBB``\x1B_R` _`D`\x10_\x87Z\xF1\x80`\x01_Q\x14\x16a\x15\x83W\x80=\x85;\x15\x17\x10a\x15\x83Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[P_`4RPPPV[_\x81\x81R_Q` a \xB5_9_Q\x90_R` R`@\x81 \x80T`\x01`p\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16B\x11\x15a\x16\xD8W_\x81_\x01`\x0E\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16B\x03\x90P_a\x16[\x83`\x02\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1B\xF8\xE7\xBE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x162W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16V\x91\x90a\x1C5V[a\x15\x05V[`\x02\x84\x01T\x84T\x91\x92P_\x91a\x16\x8F\x91a\x16V\x91`\xFF\x90\x91\x16\x90`\x01`\x01`p\x1B\x03`\x01`\x90\x1B\x90\x91\x04\x81\x16\x90\x86\x16a\x18\xD0V[\x84T\x90\x91Pg\r\xE0\xB6\xB3\xA7d\0\0\x90\x84\x90a\x16\xBA\x90`\x01`\x90\x1B\x90\x04`\x01`\x01`p\x1B\x03\x16\x84a\x1FPV[a\x16\xC4\x91\x90a\x1FPV[a\x16\xCE\x91\x90a\x1F\x86V[\x96\x95PPPPPPV[P_\x92\x91PPV[_c\xFF\xFF\xFF\xFE\x19\x83\x01a\x16\xF4WP_a\x12\xB4V[_\x83\x81R_Q` a \xB5_9_Q\x90_R` R`@\x81 `\x01\x81\x01T\x90\x91`\x01`\x01`p\x1B\x03\x90\x91\x16\x90\x03a\x17.W\x82\x91PPa\x12\xB4V[\x80T`\x01\x82\x01Ta\x17e\x91`\x01`\x01`p\x1B\x03`\x01`\x90\x1B\x90\x91\x04\x81\x16\x91a\x17[\x91\x90\x81\x16\x90\x87\x16a\x1C\x82V[a\x16V\x91\x90a ZV[\x94\x93PPPPV[`\x1C\x81\x01Q\x82]`\x1D\x81Q\x10a\x10\xD0W\x81_R\x80Q` \x82\x01\x01\x81\x82Q` \x1C_\x03` \x17_ \x03`<\x83\x01[\x80Q\x82\x82\x01]` \x01\x82\x81\x10\x15a\x0B\x85Wa\x17\x9AV[`0`\n\x82\x06\x01`\n\x82\x04\x91P`0`\n\x83\x06\x01`\n\x83\x04\x92P`0`\n\x84\x06\x01\x80`\x10\x1B\x82`\x08\x1B\x84\x01\x01fIM\0\0\0\0\0\x01`\xC8\x1B\x92PPPbF\x1B\xCD`\xE5\x1B_R` `\x04R`\x07`$R\x80`DR`d_\xFD[_c\xFF\xFF\xFF\xFE\x19\x83\x01a\x18\x1CWP_a\x12\xB4V[_\x83\x81R_Q` a \xB5_9_Q\x90_R` R`@\x81 `\x01\x81\x01T\x90\x91`\x01`\x01`p\x1B\x03\x90\x91\x16\x90\x03a\x18VW\x82\x91PPa\x12\xB4V[`\x01\x81\x01T\x81Ta\x17e\x91`\x01`\x01`p\x1B\x03\x90\x81\x16\x91a\x17[\x91`\x01`\x90\x1B\x90\x91\x04\x81\x16\x90\x87\x16a\x1C\x82V[`@Q_\x81R\x81\\`\x1C\x82\x01R\x80Q\x80\x82\x01` \x01`\x1D\x82\x10a\x18\xC1W\x83_R\x82` _ \x03`<\x84\x01[\x80\x82\x01\\\x81R` \x01\x82\x81\x10a\x18\xAEWPP[_\x81R` \x01`@RP\x91\x90PV[_\x80\x84\x80\x15a\x18\xE1Wa\x18\xE1a mV[\x03a\x19\xE9W_a\x18\xF1\x83\x85a \x81V[a\x18\xFD\x85a'\x10a\x1C\x82V[a\x19\x07\x91\x90a ZV[\x90Pa\x13\x88\x81\x10\x15a\x191Wa\x19)c\x01\xE13\x80g\x01cEx]\x8A\0\0a ZV[\x91PPa\x19\xECV[a%\x1C\x81\x10\x15a\x19\x87Wc\x01\xE13\x80a'\x10a\x19Oa\x13\x88\x84a\x1C\x99V[a\x19a\x90g\x02\x14\xE84\x8CO\0\0a\x1C\x82V[a\x19k\x91\x90a ZV[a\x19}\x90g\x01cEx]\x8A\0\0a \x81V[a\x19)\x91\x90a ZV[a'\x10\x81\x10\x15a\x19\xD3Wc\x01\xE13\x80a'\x10a\x19\xA5a\x1DL\x84a\x1C\x99V[a\x19\xB7\x90g\nh\x89\x06\xBD\x8B\0\0a\x1C\x82V[a\x19\xC1\x91\x90a ZV[a\x19}\x90g\x03x-\xAC\xE9\xD9\0\0a \x81V[a\x19)c\x01\xE13\x80g\r\xE0\xB6\xB3\xA7d\0\0a ZV[P_[\x93\x92PPPV[_a\x01@\x82\x84\x03\x12\x80\x15a\x1A\x05W__\xFD[P\x90\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1A!W__\xFD[PV[\x805a\x1A/\x81a\x1A\rV[\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a\x1AGW__\xFD[\x845\x93P` \x85\x015a\x1AY\x81a\x1A\rV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[__\x83`\x1F\x84\x01\x12a\x1A~W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\x95W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1A\xAFW__\xFD[\x92P\x92\x90PV[_____``\x86\x88\x03\x12\x15a\x1A\xCAW__\xFD[\x855a\x1A\xD5\x81a\x1A\rV[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xF0W__\xFD[a\x1A\xFC\x88\x82\x89\x01a\x1AnV[\x90\x95P\x93PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\x1BW__\xFD[a\x1B'\x88\x82\x89\x01a\x1AnV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_`\xE0\x82\x84\x03\x12\x80\x15a\x1A\x05W__\xFD[_a\x01 \x82\x84\x03\x12\x80\x15a\x1A\x05W__\xFD[_` \x82\x84\x03\x12\x15a\x1BkW__\xFD[\x815a\x19\xEC\x81a\x1A\rV[\x805\x80\x15\x15\x81\x14a\x1A/W__\xFD[\x845\x81Ra\x01\xA0\x81\x01a\x1B\x9A` \x87\x01a\x1A$V[`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R`@\x86\x81\x015\x90\x83\x01R``\x80\x87\x015\x90\x83\x01R`\x80\x80\x87\x015\x90\x83\x01R`\xA0\x80\x87\x015\x90\x83\x01R`\xC0\x80\x87\x015\x90\x83\x01R`\xE0\x80\x87\x015\x90\x83\x01Ra\x01\0\x80\x87\x015\x90\x83\x01Ra\x1B\xFCa\x01 \x87\x01a\x1BvV[\x15\x15a\x01 \x83\x01R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16a\x01@\x82\x01R`\x01`\x01`p\x1B\x03\x92\x83\x16a\x01`\x82\x01R\x91\x16a\x01\x80\x90\x91\x01R\x91\x90PV[_` \x82\x84\x03\x12\x15a\x1CEW__\xFD[PQ\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x1C]W__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x12\xB4Wa\x12\xB4a\x1CnV[\x81\x81\x03\x81\x81\x11\x15a\x12\xB4Wa\x12\xB4a\x1CnV[\x80Q`\x01`\x01`p\x1B\x03\x81\x16\x81\x14a\x1A/W__\xFD[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A/W__\xFD[_`\xC0\x82\x84\x03\x12\x80\x15a\x1C\xE6W__\xFD[P`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\x16WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x82Qa\x1D$\x81a\x1A\rV[\x81Ra\x1D2` \x84\x01a\x1C\xACV[` \x82\x01Ra\x1DC`@\x84\x01a\x1C\xC2V[`@\x82\x01Ra\x1DT``\x84\x01a\x1C\xC2V[``\x82\x01Ra\x1De`\x80\x84\x01a\x1C\xACV[`\x80\x82\x01Ra\x1Dv`\xA0\x84\x01a\x1C\xACV[`\xA0\x82\x01R\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x1D\x93W__\xFD[a\x1D\x9C\x83a\x1C\xACV[\x91Pa\x1D\xAA` \x84\x01a\x1C\xACV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x805a\xFF\xFF\x81\x16\x81\x14a\x1A/W__\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x815` \x80\x83\x01\x91\x90\x91Ra\x01\0\x82\x01\x90\x83\x015a\x1E\x01\x81a\x1A\rV[`\x01`\x01`\xA0\x1B\x03\x16`@\x83\x81\x01\x91\x90\x91Ra\xFF\xFF\x90a\x1E\"\x90\x85\x01a\x1D\xC7V[\x16``\x83\x81\x01\x91\x90\x91R\x83\x015`\x80\x80\x84\x01\x91\x90\x91R\x83\x015`\xA0\x80\x84\x01\x91\x90\x91R\x83\x015`\xC0\x80\x84\x01\x91\x90\x91R\x90\x92\x015`\xE0\x90\x91\x01R\x91\x90PV[a\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x12\xB4Wa\x12\xB4a\x1CnV[\x805`\xFF\x81\x16\x81\x14a\x1A/W__\xFD[\x815\x81Ra\x01 \x81\x01` \x83\x015a\x1E\xA0\x81a\x1A\rV[`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01Ra\x1E\xBA`@\x84\x01a\x1D\xC7V[a\xFF\xFF\x16`@\x83\x01R``\x83\x81\x015\x90\x83\x01R`\x80\x80\x84\x015\x90\x83\x01R`\xA0\x80\x84\x015\x90\x83\x01R`\xC0\x80\x84\x015\x90\x83\x01Ra\x1E\xF7`\xE0\x84\x01a\x1EyV[`\xFF\x16`\xE0\x83\x01Ra\x1F\x0Ca\x01\0\x84\x01a\x1BvV[\x80\x15\x15a\x01\0\x84\x01R[P\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a\x1F0W__\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[`\x01`\x01`p\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x1F\x16Wa\x1F\x16a\x1CnV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_`\x01`\x01`p\x1B\x03\x83\x16\x80a\x1F\x9EWa\x1F\x9Ea\x1FrV[\x80`\x01`\x01`p\x1B\x03\x84\x16\x04\x91PP\x92\x91PPV[`\x01`\x01`X\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x12\xB4Wa\x12\xB4a\x1CnV[`\x01`\x01`X\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x12\xB4Wa\x12\xB4a\x1CnV[_`@\x82\x84\x03\x12\x80\x15a \x02W__\xFD[P`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a 2WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Ra >\x83a\x1C\xC2V[\x81R` \x83\x01Qa N\x81a\x1A\rV[` \x82\x01R\x93\x92PPPV[_\x82a hWa ha\x1FrV[P\x04\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x12\xB4Wa\x12\xB4a\x1CnV\xFE\x98\xE7\xA3\xEEyht6\xFF\xE8\\\xF3\xA9\x99\xA4\xA8E\xB4\xA7\xC6\xDD-wy\xAAS|\xE4\x84\xAF-M\x98\xE7\xA3\xEEyht6\xFF\xE8\\\xF3\xA9\x99\xA4\xA8E\xB4\xA7\xC6\xDD-wy\xAAS|\xE4\x84\xAF-P\xA1dsolcC\0\x08\x1C\0\n";
    /// The bytecode of the contract.
    pub static POSITIONCOORDINATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0_W_5`\xE0\x1C\x80bi\x8B7\x14a\0cW\x80cS\x8D\xBB\x9A\x14a\0xW\x80ca4`q\x14a\0\x8BW\x80cu\xDF\x04\x9E\x14a\0\xA0W\x80c\xE5\xF2f\xE6\x14a\0\xB3W\x80c\xF3\xE2\x15\xA6\x14a\0\xC6W[__\xFD[a\0va\0q6`\x04a\x19\xF3V[a\0\xD9V[\0[a\0va\0\x866`\x04a\x1A4V[a\x05cV[a\x03 `@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0va\0\xAE6`\x04a\x1A\xB6V[a\x07\xCFV[a\0va\0\xC16`\x04a\x1B8V[a\x08\x92V[a\0va\0\xD46`\x04a\x1BIV[a\x0B\x8CV[`\x80\x81\x81\x015_\x90\x81R_Q` a \xB5_9_Q\x90_R` \x90\x81R`@\x80\x83 `\xA0\x80\x87\x015\x85R\x82\x85 \x83Q`\xC0\x80\x82\x01\x86R\x87\x82R\x95\x81\x01\x87\x90R\x93\x84\x01\x86\x90R``\x84\x01\x86\x90R\x95\x83\x01\x85\x90R\x82\x01\x93\x90\x93R_Q` a \x95_9_Q\x90_R\x93\x91\x85\x015\x15a\x02DWa\x01V\x85`\x80\x015a\x0F\xF8V[a\x01\x85`\x80\x86\x015`\xC0\x87\x015_a\x01t`@\x8A\x01` \x8B\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x10\xD4V[a\x01\xB2`\x80\x86\x015`\xC0\x87\x015a\x01\xA2`@\x89\x01` \x8A\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x11\xA4V[`\x01`\x01`p\x1B\x03\x16\x81R`\x02\x83\x01Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16c\xE4xy]a\x01\xE5`@\x88\x01` \x89\x01a\x1B[V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`\xC0\x88\x015`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02-W__\xFD[PZ\xF1\x15\x80\x15a\x02?W=__>=_\xFD[PPPP[`\xE0\x85\x015\x15a\x031Wa\x02[\x85`\xA0\x015a\x0F\xF8V[a\x02y`\xA0\x86\x015`\xE0\x87\x015_a\x01t`@\x8A\x01` \x8B\x01a\x1B[V[a\x02\x96`\xA0\x86\x015`\xE0\x87\x015a\x01\xA2`@\x89\x01` \x8A\x01a\x1B[V[`\x01`\x01`p\x1B\x03\x16` \x82\x81\x01\x91\x90\x91R`\x02\x83\x01Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE4xy]\x90a\x02\xD2\x90`@\x89\x01\x90\x89\x01a\x1B[V[`@Q`\xE0\x83\x81\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x82\x01R\x90\x88\x015`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03\x1AW__\xFD[PZ\xF1\x15\x80\x15a\x03,W=__>=_\xFD[PPPP[a\x03J3a\x03E`@\x88\x01` \x89\x01a\x1B[V[a\x12rV[`@\x80\x83\x01\x91\x90\x91Ra\x03m\x90_\x90a\x03h\x90\x88\x01` \x89\x01a\x1B[V[a\x12\xBAV[a\x03}`@\x86\x01` \x87\x01a\x1B[V[\x81Q` \x83\x01Q`@Qco\xE8d\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92co\xE8d\xC5\x92a\x03\xB6\x92\x8A\x923\x92\x91\x90`\x04\x01a\x1B\x85V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xF6\x91\x90a\x1C5V[``\x82\x01Ra\x04\x0B`@\x86\x01` \x87\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16c\x02\xA5\x03)\x82``\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04<\x91\x81R` \x01\x90V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04z\x91\x90a\x1CLV[`\xA0\x83\x01\x81\x90R`\x80\x83\x01\x82\x90Ra\x04\xD1\x91\x11\x80\x15a\x04\xCAWP`\x80\x82\x01Qa\x04\xA4\x90`da\x1C\x82V[\x82`@\x01Q\x83`\xA0\x01Q\x84`\x80\x01Qa\x04\xBD\x91\x90a\x1C\x99V[a\x04\xC7\x91\x90a\x1C\x82V[\x10\x15[`4a\x133V[a\x04\xE1`@\x86\x01` \x87\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x82``\x01Q\x7F\x03\xCF>e\x83#\xC4\x0CO\x8C\xCF\xCC\x9E\xE1G\xD2\x8D\x05\x13\xB6\xEB\xA2\xF7' Pk#7Ho9\x88`@\x015\x89``\x015\x8A`\xC0\x015\x8B`\xE0\x015`@Qa\x05T\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPPV[`@Qc\xEB\x02\xC3\x01`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R_\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xEB\x02\xC3\x01\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xCC\x91\x90a\x1C\xD5V[\x90Pa\x05\xE1\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16a\x0F\xF8V[a\x05\xF4\x81``\x01Qc\xFF\xFF\xFF\xFF\x16a\x0F\xF8V[`@Qc\x02\xA5\x03)`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R_\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x02\xA5\x03)\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x068W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\\\x91\x90a\x1CLV[`@Qc\x80Y@\t`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x87\x90R`D\x81\x01\x86\x90R\x90\x92P_\x91P\x81\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\x80Y@\t\x90`d\x01`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD9\x91\x90a\x1D\x82V[\x90\x92P\x90P`\x01`\x01`p\x1B\x03\x82\x16\x15a\x07\x15Wa\x07\x15\x84`@\x01Qc\xFF\xFF\xFF\xFF\x16\x83\x89`\x01`\x01`\xA0\x1B\x03\x16a\x13A\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01`\x01`p\x1B\x03\x81\x16\x15a\x07LWa\x07L\x84``\x01Qc\xFF\xFF\xFF\xFF\x16\x82\x89`\x01`\x01`\xA0\x1B\x03\x16a\x13A\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@Qc\x02\xA5\x03)`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R_\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x02\xA5\x03)\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x90W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xB4\x91\x90a\x1CLV[\x91PPa\x07\xC4\x84\x82\x10`;a\x133V[PPPPPPPPPV[_a\x07\xD8a\x14VV[\x90Pa\x07\xFD\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14`*a\x133V[_[\x84\x81\x10\x15a\x08\x89W_\x84\x84\x83\x81\x81\x10a\x08\x1AWa\x08\x1Aa\x1D\xB3V[\x90P` \x02\x015\x11\x15a\x08\x81Wa\x08\x81\x87\x83` \x01Q\x86\x86\x85\x81\x81\x10a\x08BWa\x08Ba\x1D\xB3V[\x90P` \x02\x015\x89\x89\x86\x81\x81\x10a\x08[Wa\x08[a\x1D\xB3V[\x90P` \x02\x01` \x81\x01\x90a\x08p\x91\x90a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x14\xACV[`\x01\x01a\x07\xFFV[PPPPPPPV[_Q` a \x95_9_Q\x90_R_a\x08\xB1`@\x84\x01` \x85\x01a\x1B[V[`@Qc\xEB\x02\xC3\x01`\xE0\x1B\x81R\x845`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xEB\x02\xC3\x01\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x1A\x91\x90a\x1C\xD5V[\x90Pa\t/\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16a\x0F\xF8V[a\tB\x81``\x01Qc\xFF\xFF\xFF\xFF\x16a\x0F\xF8V[_\x80a\tT`@\x86\x01` \x87\x01a\x1B[V[`@Qc\x02\xA5\x03)`\xE0\x1B\x81R\x865`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x02\xA5\x03)\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x98W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xBC\x91\x90a\x1CLV[\x90\x92P\x90Pa\n\x1Ca\t\xD0\x82a'\x10a\x1C\x82V[`\x05\x86\x01_a\t\xE5`@\x8A\x01` \x8B\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ Ta\n\x14\x90d\x01\0\0\0\0\x90\x04a\xFF\xFF\x16\x85a\x1C\x82V[\x10`7a\x133V[a\n0_a\x03h`@\x88\x01` \x89\x01a\x1B[V[a\n@`@\x86\x01` \x87\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16c\xE9\rLw3\x87`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\nm\x92\x91\x90a\x1D\xD8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\x84W__\xFD[PZ\xF1\x15\x80\x15a\n\x96W=__>=_\xFD[Pa\n\xAB\x92PPP`@\x86\x01` \x87\x01a\x1B[V[`@Qc\x02\xA5\x03)`\xE0\x1B\x81R\x865`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x02\xA5\x03)\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xEFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x13\x91\x90a\x1CLV[\x90\x92P\x90Pa\x0B\x85a\x0B'\x82a'\x10a\x1C\x82V[a\x03\xE8`\x05\x87\x01_a\x0B?`@\x8B\x01` \x8C\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ Ta\x0Bn\x91\x90d\x01\0\0\0\0\x90\x04a\xFF\xFF\x16a\x1E_V[a\x0B|\x90a\xFF\xFF\x16\x85a\x1C\x82V[\x10\x15`<a\x133V[PPPPPV[_Q` a \x95_9_Q\x90_R_a\x0B\xAB`@\x84\x01` \x85\x01a\x1B[V[`@Qc\xEB\x02\xC3\x01`\xE0\x1B\x81R\x845`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xEB\x02\xC3\x01\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x14\x91\x90a\x1C\xD5V[\x90Pa\x0C\\`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a\x0Co\x82`@\x01Qc\xFF\xFF\xFF\xFF\x16a\x0F\xF8V[a\x0C\x82\x82``\x01Qc\xFF\xFF\xFF\xFF\x16a\x0F\xF8V[a\x0C\x963a\x03E`@\x87\x01` \x88\x01a\x1B[V[\x81Ra\x0C\xA8`@\x85\x01` \x86\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16c\xA1'\x1CK\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xD3\x91\x90a\x1E\x89V[`\x80`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xEFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x13\x91\x90a\x1F\x1DV[`\x80\x85\x81\x01\x82\x90R``\x86\x01\x83\x90R`@\x86\x01\x93\x90\x93R` \x85\x01\x93\x90\x93R\x90\x84\x01Q`\x01`\x01`p\x1B\x03\x90\x81\x16\x80\x83\x03\x92\x10_\x81\x81\x03\x93\x90\x93\x18\x01`\xA0\x80\x86\x01\x82\x90R\x86\x01Q\x90\x91\x16\x80\x84\x03\x93\x10\x91\x82\x90\x03\x92\x90\x92\x18\x01`\xC0\x83\x01R\x15a\r\xB0Wa\r\xB0\x82`@\x01Qc\xFF\xFF\xFF\xFF\x16a\r\x90\x83`\xA0\x01Qa\x15\x05V[a\r\xA0`@\x88\x01` \x89\x01a\x1B[V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x13AV[`\xC0\x81\x01Q\x15a\r\xD5Wa\r\xD5\x82``\x01Qc\xFF\xFF\xFF\xFF\x16a\r\x90\x83`\xC0\x01Qa\x15\x05V[_\x81``\x01Q\x11\x80a\r\xEAWP_\x81`\x80\x01Q\x11[\x15a\x0E\xA9Wa\r\xFF`@\x85\x01` \x86\x01a\x1B[V[`@Qc\x02\xA5\x03)`\xE0\x1B\x81R\x855`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x02\xA5\x03)\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0ECW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Eg\x91\x90a\x1CLV[a\x01\0\x83\x01\x81\x90R`\xE0\x83\x01\x82\x90Ra\x0E\xA9\x91\x11\x80\x15a\x04\xCAWP`\xE0\x82\x01Qa\x0E\x92\x90`da\x1C\x82V[\x82Qa\x01\0\x84\x01Q`\xE0\x85\x01Qa\x04\xBD\x91\x90a\x1C\x99V[`@\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`\x03\x80\x87\x01` R\x83\x82 ``\x87\x01Q\x84\x16\x83R\x93\x90\x91 `\x02\x84\x01T\x91\x84\x01T\x90\x92a\x0F'\x92`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16\x92a\x0F\x0B\x92`\x01``\x1B\x90\x91\x04\x90\x91\x16\x900\x90a\x15\x19\x16V[`\x03\x85\x01T`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x15CV[`\x02\x81\x01T`\x03\x82\x01Ta\x0Fu\x91`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16\x91a\x0FY\x91`\x01``\x1B\x90\x91\x04\x160a\x15\x19V[`\x03\x84\x01T`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x15CV[a\x0F\x85`@\x87\x01` \x88\x01a\x1B[V[` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q_\x81R\x93\x84\x01\x92\x90\x92R\x82\x01R`\xA0\x80\x89\x015``\x83\x01R`\xC0\x89\x015`\x80\x83\x01R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x885\x91\x7F\x01i\xAF\x99m\xCD\"N\xDB\x94\n\xFB\x8E\x8B\xBD\xB0\x19\xB4\xA5\xAB\xAFd\x9C\xCAn7\xFF[\x13{x\xEE\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPV[c\xFF\xFF\xFF\xFE\x19\x81\x01a\x10\x07WPV[_\x81\x81R_Q` a \xB5_9_Q\x90_R` R`@\x90 \x80T`\x01`p\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16B\x11\x15a\x10\xD0W_a\x10@\x83a\x15\x8DV[`\x01\x83\x01T\x90\x91P_\x90a'\x10\x90a\x10c\x90`\x01`p\x1B\x90\x04a\xFF\xFF\x16\x84a\x1FPV[a\x10m\x91\x90a\x1F\x86V[`\x01\x84\x01\x80T`\x01`\x01`\x90\x1B\x03\x81\x16`\x01`\x90\x1B\x91\x82\x90\x04`\x01`\x01`p\x1B\x03\x90\x81\x16\x94\x90\x94\x01\x84\x16\x82\x02\x17\x90\x91U\x84T\x80\x83\x16\x90\x82\x90\x04\x83\x16\x94\x90\x94\x01\x90\x91\x16\x02c\xFF\xFF\xFF\xFF`p\x1B\x19\x16\x91\x90\x91\x17`\x01`p\x1BBc\xFF\xFF\xFF\xFF\x16\x02\x17\x82UP[PPV[_\x83\x81R_Q` a \xB5_9_Q\x90_R` R`@\x90 _Q` a \x95_9_Q\x90_R\x90\x82\x15a\x11QW\x83\x81`\x02\x01`\x15\x82\x82\x82\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`X\x1B\x03\x16a\x11(\x91\x90a\x1F\xB3V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`X\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`X\x1B\x03\x16\x02\x17\x90UPa\x11\x9CV[\x83\x81`\x02\x01`\x15\x82\x82\x82\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`X\x1B\x03\x16a\x11w\x91\x90a\x1F\xD2V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`X\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`X\x1B\x03\x16\x02\x17\x90UP[PPPPPPV[_\x82\x81R_Q` a \xB5_9_Q\x90_R` R`@\x81 _Q` a \x95_9_Q\x90_R\x90a\x11\xDE\x85a\x11\xD9\x86a\x15\x05V[a\x16\xE0V[\x92Pa\x11\xE9\x84a\x15\x05V[\x81T`\x01`\x01`p\x1B\x03`\x01`\x90\x1B\x80\x83\x04\x82\x16\x90\x93\x01\x81\x16\x90\x92\x02`\x01`\x01`\x90\x1B\x03\x90\x91\x16\x17\x82U`\x01\x90\x91\x01\x80T\x80\x83\x16\x85\x01\x83\x16`\x01`\x01`p\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x90\x96\x16_\x90\x81R`\t\x90\x92\x01` \x90\x81R`@\x80\x84 \x96\x84R\x95\x90R\x93\x90 \x80T\x80\x85\x16\x83\x01\x90\x94\x16\x93\x90\x94\x16\x92\x90\x92\x17\x90\x92U\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R\x7F\x98\xE7\xA3\xEEyht6\xFF\xE8\\\xF3\xA9\x99\xA4\xA8E\xB4\xA7\xC6\xDD-wy\xAAS|\xE4\x84\xAF-R` R`@\x90 Tb\x01\0\0\x90\x04a\xFF\xFF\x16[\x92\x91PPV[`@\x80Q\x80\x82\x01\x82Rc\xFF\xFF\xFF\xFF\x84\x16\x80\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16` \x93\x84\x01\x90\x81R\x84Q\x93\x84\x01\x92\x90\x92R\x90Q\x16\x81\x83\x01R\x81Q\x80\x82\x03\x83\x01\x81R``\x90\x91\x01\x90\x91R\x7F%\xACH\xEB.\x9D\xA4h\x18\xEF\xCE\xB7\xF5\x16\xCC\xED}\xAE\x8D.(\xDE\\\xD6Jy\xCDA\xF1\xE4\x8F>\x90a\x13.\x90\x82\x90a\x17mV[PPPV[\x81a\x10\xD0Wa\x10\xD0\x81a\x17\xB0V[_\x82\x81R_Q` a \xB5_9_Q\x90_R` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x7F\x98\xE7\xA3\xEEyht6\xFF\xE8\\\xF3\xA9\x99\xA4\xA8E\xB4\xA7\xC6\xDD-wy\xAAS|\xE4\x84\xAF-V\x83R\x81\x84 \x86\x85R\x90\x92R\x90\x91 T_Q` a \x95_9_Q\x90_R\x91\x90`\x01`\x01`p\x1B\x03\x90\x81\x16\x90\x84\x16\x81\x10\x15a\x13\xC2W\x80\x93P[_a\x13\xCD\x86\x86a\x18\x08V[\x83T`\x01`\x01`p\x1B\x03`\x01`\x90\x1B\x80\x83\x04\x82\x16\x93\x90\x93\x03\x81\x16\x90\x92\x02`\x01`\x01`\x90\x1B\x03\x90\x91\x16\x17\x84U`\x01\x90\x93\x01\x80T\x80\x85\x16\x87\x90\x03\x85\x16`\x01`\x01`p\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x90\x97\x16_\x90\x81R`\t\x90\x94\x01` \x90\x81R`@\x80\x86 \x97\x86R\x96\x90RPP\x92\x90 \x80T\x80\x84\x16\x92\x90\x92\x03\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x7F%\xACH\xEB.\x9D\xA4h\x18\xEF\xCE\xB7\xF5\x16\xCC\xED}\xAE\x8D.(\xDE\\\xD6Jy\xCDA\xF1\xE4\x8F>a\x14\x93\x81a\x18\x83V[\x80` \x01\x90Q\x81\x01\x90a\x14\xA6\x91\x90a\x1F\xF1V[\x91PP\x90V[`@Q\x81``R\x82`@R\x83``\x1B`,Rc#\xB8r\xDD``\x1B`\x0CR` _`d`\x1C_\x89Z\xF1\x80`\x01_Q\x14\x16a\x14\xF7W\x80=\x87;\x15\x17\x10a\x14\xF7Wcy9\xF4$_R`\x04`\x1C\xFD[P_``R`@RPPPPV[_`\x01`p\x1B\x82\x10a\x15\x15W__\xFD[P\x90V[_\x81`\x14Rcp\xA0\x821``\x1B_R` \x80`$`\x10\x86Z\xFA`\x1F=\x11\x16` Q\x02\x90P\x92\x91PPV[\x81`\x14R\x80`4Rc\xA9\x05\x9C\xBB``\x1B_R` _`D`\x10_\x87Z\xF1\x80`\x01_Q\x14\x16a\x15\x83W\x80=\x85;\x15\x17\x10a\x15\x83Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[P_`4RPPPV[_\x81\x81R_Q` a \xB5_9_Q\x90_R` R`@\x81 \x80T`\x01`p\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16B\x11\x15a\x16\xD8W_\x81_\x01`\x0E\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16B\x03\x90P_a\x16[\x83`\x02\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1B\xF8\xE7\xBE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x162W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16V\x91\x90a\x1C5V[a\x15\x05V[`\x02\x84\x01T\x84T\x91\x92P_\x91a\x16\x8F\x91a\x16V\x91`\xFF\x90\x91\x16\x90`\x01`\x01`p\x1B\x03`\x01`\x90\x1B\x90\x91\x04\x81\x16\x90\x86\x16a\x18\xD0V[\x84T\x90\x91Pg\r\xE0\xB6\xB3\xA7d\0\0\x90\x84\x90a\x16\xBA\x90`\x01`\x90\x1B\x90\x04`\x01`\x01`p\x1B\x03\x16\x84a\x1FPV[a\x16\xC4\x91\x90a\x1FPV[a\x16\xCE\x91\x90a\x1F\x86V[\x96\x95PPPPPPV[P_\x92\x91PPV[_c\xFF\xFF\xFF\xFE\x19\x83\x01a\x16\xF4WP_a\x12\xB4V[_\x83\x81R_Q` a \xB5_9_Q\x90_R` R`@\x81 `\x01\x81\x01T\x90\x91`\x01`\x01`p\x1B\x03\x90\x91\x16\x90\x03a\x17.W\x82\x91PPa\x12\xB4V[\x80T`\x01\x82\x01Ta\x17e\x91`\x01`\x01`p\x1B\x03`\x01`\x90\x1B\x90\x91\x04\x81\x16\x91a\x17[\x91\x90\x81\x16\x90\x87\x16a\x1C\x82V[a\x16V\x91\x90a ZV[\x94\x93PPPPV[`\x1C\x81\x01Q\x82]`\x1D\x81Q\x10a\x10\xD0W\x81_R\x80Q` \x82\x01\x01\x81\x82Q` \x1C_\x03` \x17_ \x03`<\x83\x01[\x80Q\x82\x82\x01]` \x01\x82\x81\x10\x15a\x0B\x85Wa\x17\x9AV[`0`\n\x82\x06\x01`\n\x82\x04\x91P`0`\n\x83\x06\x01`\n\x83\x04\x92P`0`\n\x84\x06\x01\x80`\x10\x1B\x82`\x08\x1B\x84\x01\x01fIM\0\0\0\0\0\x01`\xC8\x1B\x92PPPbF\x1B\xCD`\xE5\x1B_R` `\x04R`\x07`$R\x80`DR`d_\xFD[_c\xFF\xFF\xFF\xFE\x19\x83\x01a\x18\x1CWP_a\x12\xB4V[_\x83\x81R_Q` a \xB5_9_Q\x90_R` R`@\x81 `\x01\x81\x01T\x90\x91`\x01`\x01`p\x1B\x03\x90\x91\x16\x90\x03a\x18VW\x82\x91PPa\x12\xB4V[`\x01\x81\x01T\x81Ta\x17e\x91`\x01`\x01`p\x1B\x03\x90\x81\x16\x91a\x17[\x91`\x01`\x90\x1B\x90\x91\x04\x81\x16\x90\x87\x16a\x1C\x82V[`@Q_\x81R\x81\\`\x1C\x82\x01R\x80Q\x80\x82\x01` \x01`\x1D\x82\x10a\x18\xC1W\x83_R\x82` _ \x03`<\x84\x01[\x80\x82\x01\\\x81R` \x01\x82\x81\x10a\x18\xAEWPP[_\x81R` \x01`@RP\x91\x90PV[_\x80\x84\x80\x15a\x18\xE1Wa\x18\xE1a mV[\x03a\x19\xE9W_a\x18\xF1\x83\x85a \x81V[a\x18\xFD\x85a'\x10a\x1C\x82V[a\x19\x07\x91\x90a ZV[\x90Pa\x13\x88\x81\x10\x15a\x191Wa\x19)c\x01\xE13\x80g\x01cEx]\x8A\0\0a ZV[\x91PPa\x19\xECV[a%\x1C\x81\x10\x15a\x19\x87Wc\x01\xE13\x80a'\x10a\x19Oa\x13\x88\x84a\x1C\x99V[a\x19a\x90g\x02\x14\xE84\x8CO\0\0a\x1C\x82V[a\x19k\x91\x90a ZV[a\x19}\x90g\x01cEx]\x8A\0\0a \x81V[a\x19)\x91\x90a ZV[a'\x10\x81\x10\x15a\x19\xD3Wc\x01\xE13\x80a'\x10a\x19\xA5a\x1DL\x84a\x1C\x99V[a\x19\xB7\x90g\nh\x89\x06\xBD\x8B\0\0a\x1C\x82V[a\x19\xC1\x91\x90a ZV[a\x19}\x90g\x03x-\xAC\xE9\xD9\0\0a \x81V[a\x19)c\x01\xE13\x80g\r\xE0\xB6\xB3\xA7d\0\0a ZV[P_[\x93\x92PPPV[_a\x01@\x82\x84\x03\x12\x80\x15a\x1A\x05W__\xFD[P\x90\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1A!W__\xFD[PV[\x805a\x1A/\x81a\x1A\rV[\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a\x1AGW__\xFD[\x845\x93P` \x85\x015a\x1AY\x81a\x1A\rV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[__\x83`\x1F\x84\x01\x12a\x1A~W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\x95W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1A\xAFW__\xFD[\x92P\x92\x90PV[_____``\x86\x88\x03\x12\x15a\x1A\xCAW__\xFD[\x855a\x1A\xD5\x81a\x1A\rV[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xF0W__\xFD[a\x1A\xFC\x88\x82\x89\x01a\x1AnV[\x90\x95P\x93PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\x1BW__\xFD[a\x1B'\x88\x82\x89\x01a\x1AnV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_`\xE0\x82\x84\x03\x12\x80\x15a\x1A\x05W__\xFD[_a\x01 \x82\x84\x03\x12\x80\x15a\x1A\x05W__\xFD[_` \x82\x84\x03\x12\x15a\x1BkW__\xFD[\x815a\x19\xEC\x81a\x1A\rV[\x805\x80\x15\x15\x81\x14a\x1A/W__\xFD[\x845\x81Ra\x01\xA0\x81\x01a\x1B\x9A` \x87\x01a\x1A$V[`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R`@\x86\x81\x015\x90\x83\x01R``\x80\x87\x015\x90\x83\x01R`\x80\x80\x87\x015\x90\x83\x01R`\xA0\x80\x87\x015\x90\x83\x01R`\xC0\x80\x87\x015\x90\x83\x01R`\xE0\x80\x87\x015\x90\x83\x01Ra\x01\0\x80\x87\x015\x90\x83\x01Ra\x1B\xFCa\x01 \x87\x01a\x1BvV[\x15\x15a\x01 \x83\x01R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16a\x01@\x82\x01R`\x01`\x01`p\x1B\x03\x92\x83\x16a\x01`\x82\x01R\x91\x16a\x01\x80\x90\x91\x01R\x91\x90PV[_` \x82\x84\x03\x12\x15a\x1CEW__\xFD[PQ\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x1C]W__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x12\xB4Wa\x12\xB4a\x1CnV[\x81\x81\x03\x81\x81\x11\x15a\x12\xB4Wa\x12\xB4a\x1CnV[\x80Q`\x01`\x01`p\x1B\x03\x81\x16\x81\x14a\x1A/W__\xFD[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A/W__\xFD[_`\xC0\x82\x84\x03\x12\x80\x15a\x1C\xE6W__\xFD[P`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\x16WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x82Qa\x1D$\x81a\x1A\rV[\x81Ra\x1D2` \x84\x01a\x1C\xACV[` \x82\x01Ra\x1DC`@\x84\x01a\x1C\xC2V[`@\x82\x01Ra\x1DT``\x84\x01a\x1C\xC2V[``\x82\x01Ra\x1De`\x80\x84\x01a\x1C\xACV[`\x80\x82\x01Ra\x1Dv`\xA0\x84\x01a\x1C\xACV[`\xA0\x82\x01R\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x1D\x93W__\xFD[a\x1D\x9C\x83a\x1C\xACV[\x91Pa\x1D\xAA` \x84\x01a\x1C\xACV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x805a\xFF\xFF\x81\x16\x81\x14a\x1A/W__\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x815` \x80\x83\x01\x91\x90\x91Ra\x01\0\x82\x01\x90\x83\x015a\x1E\x01\x81a\x1A\rV[`\x01`\x01`\xA0\x1B\x03\x16`@\x83\x81\x01\x91\x90\x91Ra\xFF\xFF\x90a\x1E\"\x90\x85\x01a\x1D\xC7V[\x16``\x83\x81\x01\x91\x90\x91R\x83\x015`\x80\x80\x84\x01\x91\x90\x91R\x83\x015`\xA0\x80\x84\x01\x91\x90\x91R\x83\x015`\xC0\x80\x84\x01\x91\x90\x91R\x90\x92\x015`\xE0\x90\x91\x01R\x91\x90PV[a\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x12\xB4Wa\x12\xB4a\x1CnV[\x805`\xFF\x81\x16\x81\x14a\x1A/W__\xFD[\x815\x81Ra\x01 \x81\x01` \x83\x015a\x1E\xA0\x81a\x1A\rV[`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01Ra\x1E\xBA`@\x84\x01a\x1D\xC7V[a\xFF\xFF\x16`@\x83\x01R``\x83\x81\x015\x90\x83\x01R`\x80\x80\x84\x015\x90\x83\x01R`\xA0\x80\x84\x015\x90\x83\x01R`\xC0\x80\x84\x015\x90\x83\x01Ra\x1E\xF7`\xE0\x84\x01a\x1EyV[`\xFF\x16`\xE0\x83\x01Ra\x1F\x0Ca\x01\0\x84\x01a\x1BvV[\x80\x15\x15a\x01\0\x84\x01R[P\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a\x1F0W__\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[`\x01`\x01`p\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x1F\x16Wa\x1F\x16a\x1CnV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_`\x01`\x01`p\x1B\x03\x83\x16\x80a\x1F\x9EWa\x1F\x9Ea\x1FrV[\x80`\x01`\x01`p\x1B\x03\x84\x16\x04\x91PP\x92\x91PPV[`\x01`\x01`X\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x12\xB4Wa\x12\xB4a\x1CnV[`\x01`\x01`X\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x12\xB4Wa\x12\xB4a\x1CnV[_`@\x82\x84\x03\x12\x80\x15a \x02W__\xFD[P`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a 2WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Ra >\x83a\x1C\xC2V[\x81R` \x83\x01Qa N\x81a\x1A\rV[` \x82\x01R\x93\x92PPPV[_\x82a hWa ha\x1FrV[P\x04\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x12\xB4Wa\x12\xB4a\x1CnV\xFE\x98\xE7\xA3\xEEyht6\xFF\xE8\\\xF3\xA9\x99\xA4\xA8E\xB4\xA7\xC6\xDD-wy\xAAS|\xE4\x84\xAF-M\x98\xE7\xA3\xEEyht6\xFF\xE8\\\xF3\xA9\x99\xA4\xA8E\xB4\xA7\xC6\xDD-wy\xAAS|\xE4\x84\xAF-P\xA1dsolcC\0\x08\x1C\0\n";
    /// The deployed bytecode of the contract.
    pub static POSITIONCOORDINATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PositionCoordinator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PositionCoordinator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PositionCoordinator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PositionCoordinator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PositionCoordinator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PositionCoordinator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PositionCoordinator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    POSITIONCOORDINATOR_ABI.clone(),
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
                POSITIONCOORDINATOR_ABI.clone(),
                POSITIONCOORDINATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `accessAssets` (0x75df049e) function
        pub fn access_assets(
            &self,
            user: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 223, 4, 158], (user, tokens, amounts))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `divestFromV2LikePosition` (0xf3e215a6) function
        pub fn divest_from_v2_like_position(
            &self,
            ctx: V2LikePositionDivestmentContext,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 226, 21, 166], (ctx,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `investInV2LikePosition` (0x00698b37) function
        pub fn invest_in_v2_like_position(
            &self,
            ctx: V2LikePositionInvestmentContext,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 105, 139, 55], (ctx,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidateV2LikePosition` (0xe5f266e6) function
        pub fn liquidate_v2_like_position(
            &self,
            ctx: V2LikePositionLiquidationContext,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 242, 102, 230], (ctx,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reinvestmentFeeNumerator` (0x61346071) function
        pub fn reinvestment_fee_numerator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([97, 52, 96, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayV2LikeLiquidityPositionDebt` (0x538dbb9a) function
        pub fn repay_v2_like_liquidity_position_debt(
            &self,
            position_id: ::ethers::core::types::U256,
            worker: ::ethers::core::types::Address,
            repay_token_0: ::ethers::core::types::U256,
            repay_token_1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [83, 141, 187, 154],
                    (position_id, worker, repay_token_0, repay_token_1),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PositionDivested` event
        pub fn position_divested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionDivestedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PositionInvested` event
        pub fn position_invested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionInvestedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionCoordinatorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PositionCoordinator<M> {
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
    #[ethevent(
        name = "PositionDivested",
        abi = "PositionDivested(uint256,address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct PositionDivestedFilter {
        #[ethevent(indexed)]
        pub position_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub worker: ::ethers::core::types::Address,
        pub liquidity_burnt: ::ethers::core::types::U256,
        pub token_0_out: ::ethers::core::types::U256,
        pub token_1_out: ::ethers::core::types::U256,
        pub token_0_repaid: ::ethers::core::types::U256,
        pub token_1_repaid: ::ethers::core::types::U256,
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
        name = "PositionInvested",
        abi = "PositionInvested(uint256,address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct PositionInvestedFilter {
        #[ethevent(indexed)]
        pub position_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub worker: ::ethers::core::types::Address,
        pub token_0_in: ::ethers::core::types::U256,
        pub token_1_in: ::ethers::core::types::U256,
        pub token_0_borrowed: ::ethers::core::types::U256,
        pub token_1_borrowed: ::ethers::core::types::U256,
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
    pub enum PositionCoordinatorEvents {
        PositionDivestedFilter(PositionDivestedFilter),
        PositionInvestedFilter(PositionInvestedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PositionCoordinatorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PositionDivestedFilter::decode_log(log) {
                return Ok(PositionCoordinatorEvents::PositionDivestedFilter(decoded));
            }
            if let Ok(decoded) = PositionInvestedFilter::decode_log(log) {
                return Ok(PositionCoordinatorEvents::PositionInvestedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PositionCoordinatorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PositionDivestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionInvestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<PositionDivestedFilter> for PositionCoordinatorEvents {
        fn from(value: PositionDivestedFilter) -> Self {
            Self::PositionDivestedFilter(value)
        }
    }
    impl ::core::convert::From<PositionInvestedFilter> for PositionCoordinatorEvents {
        fn from(value: PositionInvestedFilter) -> Self {
            Self::PositionInvestedFilter(value)
        }
    }
    ///Container type for all input parameters for the `accessAssets` function with signature `accessAssets(address,address[],uint256[])` and selector `0x75df049e`
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
    #[ethcall(name = "accessAssets", abi = "accessAssets(address,address[],uint256[])")]
    pub struct AccessAssetsCall {
        pub user: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `divestFromV2LikePosition` function with signature `divestFromV2LikePosition((uint256,address,uint16,uint256,uint256,uint256,uint256,uint8,bool))` and selector `0xf3e215a6`
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
        name = "divestFromV2LikePosition",
        abi = "divestFromV2LikePosition((uint256,address,uint16,uint256,uint256,uint256,uint256,uint8,bool))"
    )]
    pub struct DivestFromV2LikePositionCall {
        pub ctx: V2LikePositionDivestmentContext,
    }
    ///Container type for all input parameters for the `investInV2LikePosition` function with signature `investInV2LikePosition((uint256,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,bool))` and selector `0x00698b37`
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
        name = "investInV2LikePosition",
        abi = "investInV2LikePosition((uint256,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,bool))"
    )]
    pub struct InvestInV2LikePositionCall {
        pub ctx: V2LikePositionInvestmentContext,
    }
    ///Container type for all input parameters for the `liquidateV2LikePosition` function with signature `liquidateV2LikePosition((uint256,address,uint16,uint256,uint256,uint256,uint256))` and selector `0xe5f266e6`
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
        name = "liquidateV2LikePosition",
        abi = "liquidateV2LikePosition((uint256,address,uint16,uint256,uint256,uint256,uint256))"
    )]
    pub struct LiquidateV2LikePositionCall {
        pub ctx: V2LikePositionLiquidationContext,
    }
    ///Container type for all input parameters for the `reinvestmentFeeNumerator` function with signature `reinvestmentFeeNumerator()` and selector `0x61346071`
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
    #[ethcall(name = "reinvestmentFeeNumerator", abi = "reinvestmentFeeNumerator()")]
    pub struct ReinvestmentFeeNumeratorCall;
    ///Container type for all input parameters for the `repayV2LikeLiquidityPositionDebt` function with signature `repayV2LikeLiquidityPositionDebt(uint256,address,uint256,uint256)` and selector `0x538dbb9a`
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
        name = "repayV2LikeLiquidityPositionDebt",
        abi = "repayV2LikeLiquidityPositionDebt(uint256,address,uint256,uint256)"
    )]
    pub struct RepayV2LikeLiquidityPositionDebtCall {
        pub position_id: ::ethers::core::types::U256,
        pub worker: ::ethers::core::types::Address,
        pub repay_token_0: ::ethers::core::types::U256,
        pub repay_token_1: ::ethers::core::types::U256,
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
    pub enum PositionCoordinatorCalls {
        AccessAssets(AccessAssetsCall),
        DivestFromV2LikePosition(DivestFromV2LikePositionCall),
        InvestInV2LikePosition(InvestInV2LikePositionCall),
        LiquidateV2LikePosition(LiquidateV2LikePositionCall),
        ReinvestmentFeeNumerator(ReinvestmentFeeNumeratorCall),
        RepayV2LikeLiquidityPositionDebt(RepayV2LikeLiquidityPositionDebtCall),
    }
    impl ::ethers::core::abi::AbiDecode for PositionCoordinatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccessAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessAssets(decoded));
            }
            if let Ok(decoded) = <DivestFromV2LikePositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivestFromV2LikePosition(decoded));
            }
            if let Ok(decoded) = <InvestInV2LikePositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvestInV2LikePosition(decoded));
            }
            if let Ok(decoded) = <LiquidateV2LikePositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidateV2LikePosition(decoded));
            }
            if let Ok(decoded) = <ReinvestmentFeeNumeratorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReinvestmentFeeNumerator(decoded));
            }
            if let Ok(decoded) = <RepayV2LikeLiquidityPositionDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RepayV2LikeLiquidityPositionDebt(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PositionCoordinatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccessAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DivestFromV2LikePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvestInV2LikePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidateV2LikePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReinvestmentFeeNumerator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayV2LikeLiquidityPositionDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PositionCoordinatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivestFromV2LikePosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvestInV2LikePosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidateV2LikePosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReinvestmentFeeNumerator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RepayV2LikeLiquidityPositionDebt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AccessAssetsCall> for PositionCoordinatorCalls {
        fn from(value: AccessAssetsCall) -> Self {
            Self::AccessAssets(value)
        }
    }
    impl ::core::convert::From<DivestFromV2LikePositionCall>
    for PositionCoordinatorCalls {
        fn from(value: DivestFromV2LikePositionCall) -> Self {
            Self::DivestFromV2LikePosition(value)
        }
    }
    impl ::core::convert::From<InvestInV2LikePositionCall> for PositionCoordinatorCalls {
        fn from(value: InvestInV2LikePositionCall) -> Self {
            Self::InvestInV2LikePosition(value)
        }
    }
    impl ::core::convert::From<LiquidateV2LikePositionCall>
    for PositionCoordinatorCalls {
        fn from(value: LiquidateV2LikePositionCall) -> Self {
            Self::LiquidateV2LikePosition(value)
        }
    }
    impl ::core::convert::From<ReinvestmentFeeNumeratorCall>
    for PositionCoordinatorCalls {
        fn from(value: ReinvestmentFeeNumeratorCall) -> Self {
            Self::ReinvestmentFeeNumerator(value)
        }
    }
    impl ::core::convert::From<RepayV2LikeLiquidityPositionDebtCall>
    for PositionCoordinatorCalls {
        fn from(value: RepayV2LikeLiquidityPositionDebtCall) -> Self {
            Self::RepayV2LikeLiquidityPositionDebt(value)
        }
    }
    ///Container type for all return fields from the `reinvestmentFeeNumerator` function with signature `reinvestmentFeeNumerator()` and selector `0x61346071`
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
    pub struct ReinvestmentFeeNumeratorReturn(pub ::ethers::core::types::U256);
}
