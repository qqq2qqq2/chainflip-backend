
//! Autogenerated weights for pallet_cf_reputation
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
// pallet_cf_reputation
// --extrinsic
// *
// --output
// state-chain/pallets/cf-reputation/src/weights.rs
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

/// Weight functions needed for pallet_cf_reputation.
pub trait WeightInfo {
	fn update_accrual_ratio() -> Weight;
	fn set_penalty() -> Weight;
	fn update_missed_heartbeat_penalty() -> Weight;
	fn heartbeat() -> Weight;
	fn submit_network_state() -> Weight;
	fn on_initialize_no_action() -> Weight;
}

/// Weights for pallet_cf_reputation using the Substrate node and recommended hardware.
pub struct PalletWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PalletWeight<T> {
	/// Storage: `Reputation::AccrualRatio` (r:0 w:1)
	/// Proof: `Reputation::AccrualRatio` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_accrual_ratio() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_448_000 picoseconds.
		Weight::from_parts(10_048_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Reputation::Penalties` (r:1 w:1)
	/// Proof: `Reputation::Penalties` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_penalty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `286`
		//  Estimated: `3751`
		// Minimum execution time: 18_573_000 picoseconds.
		Weight::from_parts(18_659_000, 3751)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Reputation::Penalties` (r:0 w:1)
	/// Proof: `Reputation::Penalties` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_missed_heartbeat_penalty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_944_000 picoseconds.
		Weight::from_parts(11_033_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `AccountRoles::AccountRoles` (r:1 w:0)
	/// Proof: `AccountRoles::AccountRoles` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Reputation::Reputations` (r:1 w:1)
	/// Proof: `Reputation::Reputations` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Reputation::LastHeartbeat` (r:1 w:1)
	/// Proof: `Reputation::LastHeartbeat` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Reputation::AccrualRatio` (r:1 w:0)
	/// Proof: `Reputation::AccrualRatio` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn heartbeat() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `564`
		//  Estimated: `4029`
		// Minimum execution time: 21_946_000 picoseconds.
		Weight::from_parts(22_804_000, 4029)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Environment::RuntimeSafeMode` (r:1 w:0)
	/// Proof: `Environment::RuntimeSafeMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Validator::CurrentAuthorities` (r:1 w:0)
	/// Proof: `Validator::CurrentAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Reputation::LastHeartbeat` (r:1 w:0)
	/// Proof: `Reputation::LastHeartbeat` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Reputation::Penalties` (r:1 w:0)
	/// Proof: `Reputation::Penalties` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Emissions::CurrentAuthorityEmissionInflation` (r:1 w:0)
	/// Proof: `Emissions::CurrentAuthorityEmissionInflation` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Flip::TotalIssuance` (r:1 w:0)
	/// Proof: `Flip::TotalIssuance` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Emissions::BackupNodeEmissionInflation` (r:1 w:0)
	/// Proof: `Emissions::BackupNodeEmissionInflation` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Validator::Backups` (r:1 w:0)
	/// Proof: `Validator::Backups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Validator::BackupRewardNodePercentage` (r:1 w:0)
	/// Proof: `Validator::BackupRewardNodePercentage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Emissions::CurrentAuthorityEmissionPerBlock` (r:0 w:1)
	/// Proof: `Emissions::CurrentAuthorityEmissionPerBlock` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Emissions::BackupNodeEmissionPerBlock` (r:0 w:1)
	/// Proof: `Emissions::BackupNodeEmissionPerBlock` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn submit_network_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1291`
		//  Estimated: `4756`
		// Minimum execution time: 45_739_000 picoseconds.
		Weight::from_parts(46_207_000, 4756)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Environment::RuntimeSafeMode` (r:1 w:0)
	/// Proof: `Environment::RuntimeSafeMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_initialize_no_action() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `335`
		//  Estimated: `1820`
		// Minimum execution time: 3_868_000 picoseconds.
		Weight::from_parts(4_025_000, 1820)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Reputation::AccrualRatio` (r:0 w:1)
	/// Proof: `Reputation::AccrualRatio` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_accrual_ratio() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_448_000 picoseconds.
		Weight::from_parts(10_048_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Reputation::Penalties` (r:1 w:1)
	/// Proof: `Reputation::Penalties` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_penalty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `286`
		//  Estimated: `3751`
		// Minimum execution time: 18_573_000 picoseconds.
		Weight::from_parts(18_659_000, 3751)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Reputation::Penalties` (r:0 w:1)
	/// Proof: `Reputation::Penalties` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_missed_heartbeat_penalty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_944_000 picoseconds.
		Weight::from_parts(11_033_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `AccountRoles::AccountRoles` (r:1 w:0)
	/// Proof: `AccountRoles::AccountRoles` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Reputation::Reputations` (r:1 w:1)
	/// Proof: `Reputation::Reputations` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Reputation::LastHeartbeat` (r:1 w:1)
	/// Proof: `Reputation::LastHeartbeat` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Reputation::AccrualRatio` (r:1 w:0)
	/// Proof: `Reputation::AccrualRatio` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn heartbeat() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `564`
		//  Estimated: `4029`
		// Minimum execution time: 21_946_000 picoseconds.
		Weight::from_parts(22_804_000, 4029)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Environment::RuntimeSafeMode` (r:1 w:0)
	/// Proof: `Environment::RuntimeSafeMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Validator::CurrentAuthorities` (r:1 w:0)
	/// Proof: `Validator::CurrentAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Reputation::LastHeartbeat` (r:1 w:0)
	/// Proof: `Reputation::LastHeartbeat` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Reputation::Penalties` (r:1 w:0)
	/// Proof: `Reputation::Penalties` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Emissions::CurrentAuthorityEmissionInflation` (r:1 w:0)
	/// Proof: `Emissions::CurrentAuthorityEmissionInflation` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Flip::TotalIssuance` (r:1 w:0)
	/// Proof: `Flip::TotalIssuance` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Emissions::BackupNodeEmissionInflation` (r:1 w:0)
	/// Proof: `Emissions::BackupNodeEmissionInflation` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Validator::Backups` (r:1 w:0)
	/// Proof: `Validator::Backups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Validator::BackupRewardNodePercentage` (r:1 w:0)
	/// Proof: `Validator::BackupRewardNodePercentage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Emissions::CurrentAuthorityEmissionPerBlock` (r:0 w:1)
	/// Proof: `Emissions::CurrentAuthorityEmissionPerBlock` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Emissions::BackupNodeEmissionPerBlock` (r:0 w:1)
	/// Proof: `Emissions::BackupNodeEmissionPerBlock` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn submit_network_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1291`
		//  Estimated: `4756`
		// Minimum execution time: 45_739_000 picoseconds.
		Weight::from_parts(46_207_000, 4756)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Environment::RuntimeSafeMode` (r:1 w:0)
	/// Proof: `Environment::RuntimeSafeMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_initialize_no_action() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `335`
		//  Estimated: `1820`
		// Minimum execution time: 3_868_000 picoseconds.
		Weight::from_parts(4_025_000, 1820)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
}
