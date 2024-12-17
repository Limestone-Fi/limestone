#![allow(clippy::all)]
//! This module contains abigen! generated bindings for solidity contracts.
//! This is autogenerated code.
//! Do not manually edit these files.
//! These files may be overwritten by the codegen system at any time.
pub mod address;
pub mod address_book;
pub mod address_utils;
pub mod cast;
pub mod clones;
pub mod cognitohazard;
pub mod constants;
pub mod context;
pub mod diamond_base;
pub mod diamond_base_storage;
pub mod diamond_fallback;
pub mod diamond_readable;
pub mod diamond_readable_internal;
pub mod diamond_writable;
pub mod diamond_writable_internal;
pub mod drome_multi_modal_worker;
pub mod ecdsa;
pub mod eip712;
pub mod enumerable_set;
pub mod erc165_base;
pub mod erc165_base_internal;
pub mod erc165_base_storage;
pub mod erc20;
pub mod erc20_base;
pub mod erc20_base_internal;
pub mod erc20_base_storage;
pub mod erc20_extended;
pub mod erc20_extended_internal;
pub mod erc20_metadata;
pub mod erc20_metadata_internal;
pub mod erc20_metadata_storage;
pub mod erc20_permit;
pub mod erc20_permit_internal;
pub mod erc20_permit_storage;
pub mod erc721;
pub mod errors;
pub mod fixed_point_math_lib;
pub mod fuzz;
pub mod fuzz_actor;
pub mod fuzz_base;
pub mod fuzz_config;
pub mod fuzz_global;
pub mod fuzz_lending_pool;
pub mod fuzz_lib_string;
pub mod fuzz_position_coordinator;
pub mod fuzz_setup;
pub mod fuzzlib;
pub mod helper_assert;
pub mod helper_base;
pub mod helper_clamp;
pub mod helper_log;
pub mod helper_math;
pub mod helper_random;
pub mod i_diamond_base;
pub mod i_diamond_fallback;
pub mod i_diamond_readable;
pub mod i_diamond_readable_internal;
pub mod i_diamond_writable;
pub mod i_diamond_writable_internal;
pub mod i_drome_callee;
pub mod i_drome_factory;
pub mod i_drome_gauge;
pub mod i_drome_pool;
pub mod i_drome_router;
pub mod i_emissions_controller;
pub mod i_factory_registry;
pub mod i_gauge;
pub mod i_initializable;
pub mod i_initializable_internal;
pub mod i_lending_pool;
pub mod i_lime_diamond;
pub mod i_multi_modal_worker;
pub mod i_multi_modal_worker_balancer_like;
pub mod i_ownable;
pub mod i_ownable_internal;
pub mod i_platform;
pub mod i_pool;
pub mod i_pool_callee;
pub mod i_pool_factory;
pub mod i_position_coordinator;
pub mod i_proxy;
pub mod i_router;
pub mod i_safe_ownable;
pub mod i_safe_ownable_internal;
pub mod i_solid_state_diamond;
pub mod i_solid_state_erc20;
pub mod i_uniswap_v2_pair;
pub mod i_warchest;
pub mod i_worker;
pub mod ierc1155_errors;
pub mod ierc1363;
pub mod ierc165;
pub mod ierc165_base;
pub mod ierc165_base_internal;
pub mod ierc165_internal;
pub mod ierc173;
pub mod ierc173_internal;
pub mod ierc20;
pub mod ierc20_base;
pub mod ierc20_base_internal;
pub mod ierc20_errors;
pub mod ierc20_extended;
pub mod ierc20_extended_internal;
pub mod ierc20_internal;
pub mod ierc20_metadata;
pub mod ierc20_metadata_internal;
pub mod ierc20_permit;
pub mod ierc20_permit_internal;
pub mod ierc2535_diamond_cut;
pub mod ierc2535_diamond_cut_internal;
pub mod ierc2535_diamond_loupe;
pub mod ierc2535_diamond_loupe_internal;
pub mod ierc2612;
pub mod ierc2612_internal;
pub mod ierc5267;
pub mod ierc721;
pub mod ierc721_enumerable;
pub mod ierc721_errors;
pub mod ierc721_metadata;
pub mod ierc721_token_receiver;
pub mod initializable;
pub mod initializable_internal;
pub mod initializable_storage;
pub mod iweth;
pub mod lending_pool;
pub mod lending_pool_handler;
pub mod lending_pool_lib;
pub mod lending_pool_storage;
pub mod lending_pool_wrapper;
pub mod lib_log;
pub mod lib_transient;
pub mod lib_zip;
pub mod math;
pub mod message_hash_utils;
pub mod mock_aero;
pub mod mock_erc20;
pub mod mock_erc721;
pub mod mock_factory_registry;
pub mod mock_gauge;
pub mod mock_pool;
pub mod mock_pool_factory;
pub mod mock_pool_fees;
pub mod mock_router;
pub mod mock_token;
pub mod mock_usdc;
pub mod mock_warchest;
pub mod mock_worker;
pub mod multi_modal_worker;
pub mod multi_modal_worker_storage;
pub mod nonces;
pub mod ownable;
pub mod ownable_internal;
pub mod ownable_storage;
pub mod panic;
pub mod platform_crytic;
pub mod position_coordinator;
pub mod proxy;
pub mod reentrancy_guard;
pub mod safe_cast;
pub mod safe_erc20;
pub mod safe_ownable;
pub mod safe_ownable_internal;
pub mod safe_ownable_storage;
pub mod safe_transfer_lib;
pub mod shared_types;
pub mod short_strings;
pub mod signed_math;
pub mod solid_state_diamond;
pub mod solid_state_erc20;
pub mod sstore2;
pub mod storage_slot;
pub mod strings;
pub mod swap_utils;
pub mod uint_utils;
pub mod velodrome_time_library;