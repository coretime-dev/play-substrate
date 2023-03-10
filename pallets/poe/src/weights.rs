
//! Autogenerated weights for pallet_poe
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-10, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Kaichaos-MacBook-Pro-720.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_poe
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --json-file=raw.json
// --output
// ./pallets/poe/src/weights.rs
// --template
// .maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_poe.
pub trait WeightInfo {
	fn create_claim(d: u32, ) -> Weight;
	fn revoke_claim(d: u32, ) -> Weight;
	fn transfer_claim(d: u32, ) -> Weight;
}

/// Weights for pallet_poe using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: PoeModule Proofs (r:1 w:1)
	/// The range of component `d` is `[0, 512]`.
	fn create_claim(d: u32, ) -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(16_015_281)
			// Standard Error: 343
			.saturating_add(Weight::from_ref_time(4_244).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: PoeModule Proofs (r:1 w:1)
	/// The range of component `d` is `[0, 512]`.
	fn revoke_claim(d: u32, ) -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(16_237_031)
			// Standard Error: 313
			.saturating_add(Weight::from_ref_time(13_159).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: PoeModule Proofs (r:1 w:1)
	/// The range of component `d` is `[0, 512]`.
	fn transfer_claim(d: u32, ) -> Weight {
		// Minimum execution time: 11_000 nanoseconds.
		Weight::from_ref_time(11_203_230)
			// Standard Error: 212
			.saturating_add(Weight::from_ref_time(12_607).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: PoeModule Proofs (r:1 w:1)
	/// The range of component `d` is `[0, 512]`.
	fn create_claim(d: u32, ) -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(16_015_281)
			// Standard Error: 343
			.saturating_add(Weight::from_ref_time(4_244).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: PoeModule Proofs (r:1 w:1)
	/// The range of component `d` is `[0, 512]`.
	fn revoke_claim(d: u32, ) -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(16_237_031)
			// Standard Error: 313
			.saturating_add(Weight::from_ref_time(13_159).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: PoeModule Proofs (r:1 w:1)
	/// The range of component `d` is `[0, 512]`.
	fn transfer_claim(d: u32, ) -> Weight {
		// Minimum execution time: 11_000 nanoseconds.
		Weight::from_ref_time(11_203_230)
			// Standard Error: 212
			.saturating_add(Weight::from_ref_time(12_607).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}
