
//! Autogenerated weights for pallet_cf_witnesser
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-03, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-9-222`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./chainflip-node
// benchmark
// pallet
// --pallet
// pallet_cf_witnesser
// --extrinsic
// *
// --output
// state-chain/pallets/cf-witnesser/src/weights.rs
// --execution=wasm
// --steps=20
// --repeat=10
// --template=state-chain/chainflip-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_cf_witnesser.
pub trait WeightInfo {
	fn witness_at_epoch() -> Weight;
	fn remove_storage_items(n: u32, ) -> Weight;
	fn on_idle_with_nothing_to_remove() -> Weight;
}

/// Weights for pallet_cf_witnesser using the Substrate node and recommended hardware.
pub struct PalletWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PalletWeight<T> {
	/// Storage: `AccountRoles::AccountRoles` (r:1 w:0)
	/// Proof: `AccountRoles::AccountRoles` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Validator::LastExpiredEpoch` (r:1 w:0)
	/// Proof: `Validator::LastExpiredEpoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Validator::CurrentEpoch` (r:1 w:0)
	/// Proof: `Validator::CurrentEpoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Validator::HistoricalAuthorities` (r:1 w:0)
	/// Proof: `Validator::HistoricalAuthorities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Validator::AuthorityIndex` (r:1 w:0)
	/// Proof: `Validator::AuthorityIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Witnesser::Votes` (r:1 w:1)
	/// Proof: `Witnesser::Votes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Witnesser::CallHashExecuted` (r:2 w:1)
	/// Proof: `Witnesser::CallHashExecuted` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Witnesser::ExtraCallData` (r:1 w:0)
	/// Proof: `Witnesser::ExtraCallData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Environment::RuntimeSafeMode` (r:1 w:0)
	/// Proof: `Environment::RuntimeSafeMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn witness_at_epoch() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1294`
		//  Estimated: `7234`
		// Minimum execution time: 50_504_000 picoseconds.
		Weight::from_parts(50_879_000, 7234)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Witnesser::Votes` (r:254 w:254)
	/// Proof: `Witnesser::Votes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[1, 255]`.
	fn remove_storage_items(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `159 + n * (38 ±0)`
		//  Estimated: `1149 + n * (2513 ±0)`
		// Minimum execution time: 4_933_000 picoseconds.
		Weight::from_parts(3_503_846, 1149)
			// Standard Error: 3_349
			.saturating_add(Weight::from_parts(1_052_999, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2513).saturating_mul(n.into()))
	}
	/// Storage: `Environment::RuntimeSafeMode` (r:1 w:0)
	/// Proof: `Environment::RuntimeSafeMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Witnesser::WitnessedCallsScheduledForDispatch` (r:1 w:1)
	/// Proof: `Witnesser::WitnessedCallsScheduledForDispatch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Witnesser::EpochsToCull` (r:1 w:0)
	/// Proof: `Witnesser::EpochsToCull` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_idle_with_nothing_to_remove() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `472`
		//  Estimated: `1957`
		// Minimum execution time: 9_483_000 picoseconds.
		Weight::from_parts(9_974_000, 1957)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `AccountRoles::AccountRoles` (r:1 w:0)
	/// Proof: `AccountRoles::AccountRoles` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Validator::LastExpiredEpoch` (r:1 w:0)
	/// Proof: `Validator::LastExpiredEpoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Validator::CurrentEpoch` (r:1 w:0)
	/// Proof: `Validator::CurrentEpoch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Validator::HistoricalAuthorities` (r:1 w:0)
	/// Proof: `Validator::HistoricalAuthorities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Validator::AuthorityIndex` (r:1 w:0)
	/// Proof: `Validator::AuthorityIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Witnesser::Votes` (r:1 w:1)
	/// Proof: `Witnesser::Votes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Witnesser::CallHashExecuted` (r:2 w:1)
	/// Proof: `Witnesser::CallHashExecuted` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Witnesser::ExtraCallData` (r:1 w:0)
	/// Proof: `Witnesser::ExtraCallData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Environment::RuntimeSafeMode` (r:1 w:0)
	/// Proof: `Environment::RuntimeSafeMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn witness_at_epoch() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1294`
		//  Estimated: `7234`
		// Minimum execution time: 50_504_000 picoseconds.
		Weight::from_parts(50_879_000, 7234)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Witnesser::Votes` (r:254 w:254)
	/// Proof: `Witnesser::Votes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[1, 255]`.
	fn remove_storage_items(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `159 + n * (38 ±0)`
		//  Estimated: `1149 + n * (2513 ±0)`
		// Minimum execution time: 4_933_000 picoseconds.
		Weight::from_parts(3_503_846, 1149)
			// Standard Error: 3_349
			.saturating_add(Weight::from_parts(1_052_999, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2513).saturating_mul(n.into()))
	}
	/// Storage: `Environment::RuntimeSafeMode` (r:1 w:0)
	/// Proof: `Environment::RuntimeSafeMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Witnesser::WitnessedCallsScheduledForDispatch` (r:1 w:1)
	/// Proof: `Witnesser::WitnessedCallsScheduledForDispatch` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Witnesser::EpochsToCull` (r:1 w:0)
	/// Proof: `Witnesser::EpochsToCull` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_idle_with_nothing_to_remove() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `472`
		//  Estimated: `1957`
		// Minimum execution time: 9_483_000 picoseconds.
		Weight::from_parts(9_974_000, 1957)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
