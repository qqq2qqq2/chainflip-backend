//! Autogenerated weights for pallet_cf_relayer
//!
//! THIS FILE WAS AUTO-GENERATED USING CHAINFLIP NODE BENCHMARK CMD VERSION 4.0.0-dev
//! DATE: 2022-09-01, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("three-node-test"), DB CACHE: 1024

// Executed Command:
// ./target/release/chainflip-node
// benchmark
// pallet
// --chain
// three-node-test
// --extrinsic
// *
// --pallet
// pallet_cf_relayer
// --output
// state-chain/pallets/cf-relayer/src/weights.rs
// --execution=wasm
// --steps=20
// --repeat=10
// --template=state-chain/chainflip-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_cf_relayer.
pub trait WeightInfo {
	fn request_swap_intent() -> Weight;
}

/// Weights for pallet_cf_relayer using the Substrate node and recommended hardware.
pub struct PalletWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PalletWeight<T> {
	// Storage: Relayer IndexCounter (r:1 w:1)
	// Storage: Environment VaultContractAddress (r:1 w:0)
	// Storage: Relayer SwapIntents (r:0 w:1)
	fn request_swap_intent() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(31_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Relayer IndexCounter (r:1 w:1)
	// Storage: Environment VaultContractAddress (r:1 w:0)
	// Storage: Relayer SwapIntents (r:0 w:1)
	fn request_swap_intent() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(31_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}