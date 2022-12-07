
//! Autogenerated weights for `peaq_pallet_storage`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `jaypan-bravo-15-b5dd`, CPU: `AMD Ryzen 5 5600H with Radeon Graphics`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/peaq-node
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// peaq-pallet-storage
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for peaq_pallet_storage
pub trait WeightInfo {
	fn add_item () -> Weight;
	fn update_item() -> Weight;
	fn  get_item() -> Weight;	
}



/// Weight functions for `peaq_pallet_storage`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: PeaqStorage ItemStore (r:1 w:1)
	fn add_item() -> Weight {
		(23_014_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: PeaqStorage ItemStore (r:1 w:1)
	fn update_item() -> Weight {
		(25_127_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: PeaqStorage ItemStore (r:1 w:0)
	fn get_item() -> Weight {
		(23_725_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}
