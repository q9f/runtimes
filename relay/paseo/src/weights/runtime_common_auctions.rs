// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `runtime_common::auctions`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `a3dce7bd4066`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("spec-polkadot.json")`, DB CACHE: 1024

// Executed Command:
// /builds/polkadot-sdk/target/production/polkadot
// benchmark
// pallet
// --chain=spec-polkadot.json
// --pallet=runtime_common::auctions
// --extrinsic=
// --output=/builds/runtimes/relay/polkadot/src/weights
// --header=/builds/bench/header.txt
// --no-median-slopes
// --no-min-squares

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `runtime_common::auctions`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_common::auctions::WeightInfo for WeightInfo<T> {
	/// Storage: `Auctions::AuctionInfo` (r:1 w:1)
	/// Proof: `Auctions::AuctionInfo` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Auctions::AuctionCounter` (r:1 w:1)
	/// Proof: `Auctions::AuctionCounter` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn new_auction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `1493`
		// Minimum execution time: 11_570_000 picoseconds.
		Weight::from_parts(11_811_000, 0)
			.saturating_add(Weight::from_parts(0, 1493))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Auctions::AuctionCounter` (r:1 w:0)
	/// Proof: `Auctions::AuctionCounter` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Auctions::AuctionInfo` (r:1 w:0)
	/// Proof: `Auctions::AuctionInfo` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Slots::Leases` (r:1 w:0)
	/// Proof: `Slots::Leases` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Auctions::Winning` (r:1 w:1)
	/// Proof: `Auctions::Winning` (`max_values`: None, `max_size`: Some(1920), added: 4395, mode: `MaxEncodedLen`)
	/// Storage: `Auctions::ReservedAmounts` (r:2 w:2)
	/// Proof: `Auctions::ReservedAmounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn bid() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `695`
		//  Estimated: `6060`
		// Minimum execution time: 94_382_000 picoseconds.
		Weight::from_parts(101_437_000, 0)
			.saturating_add(Weight::from_parts(0, 6060))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Auctions::AuctionInfo` (r:1 w:1)
	/// Proof: `Auctions::AuctionInfo` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Babe::NextRandomness` (r:1 w:0)
	/// Proof: `Babe::NextRandomness` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Babe::EpochStart` (r:1 w:0)
	/// Proof: `Babe::EpochStart` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Auctions::AuctionCounter` (r:1 w:0)
	/// Proof: `Auctions::AuctionCounter` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Auctions::Winning` (r:3600 w:3600)
	/// Proof: `Auctions::Winning` (`max_values`: None, `max_size`: Some(1920), added: 4395, mode: `MaxEncodedLen`)
	/// Storage: `Auctions::ReservedAmounts` (r:37 w:36)
	/// Proof: `Auctions::ReservedAmounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:36 w:36)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Slots::Leases` (r:7 w:7)
	/// Proof: `Slots::Leases` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:1)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ActionsQueue` (r:1 w:1)
	/// Proof: `Paras::ActionsQueue` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6946951`
		//  Estimated: `15822990`
		// Minimum execution time: 7_679_812_000 picoseconds.
		Weight::from_parts(7_852_648_000, 0)
			.saturating_add(Weight::from_parts(0, 15822990))
			.saturating_add(T::DbWeight::get().reads(3687))
			.saturating_add(T::DbWeight::get().writes(3682))
	}
	/// Storage: `Auctions::ReservedAmounts` (r:37 w:36)
	/// Proof: `Auctions::ReservedAmounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:36 w:36)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Auctions::Winning` (r:3600 w:3600)
	/// Proof: `Auctions::Winning` (`max_values`: None, `max_size`: Some(1920), added: 4395, mode: `MaxEncodedLen`)
	/// Storage: `Auctions::AuctionInfo` (r:0 w:1)
	/// Proof: `Auctions::AuctionInfo` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn cancel_auction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177732`
		//  Estimated: `15822990`
		// Minimum execution time: 5_899_491_000 picoseconds.
		Weight::from_parts(6_010_908_000, 0)
			.saturating_add(Weight::from_parts(0, 15822990))
			.saturating_add(T::DbWeight::get().reads(3673))
			.saturating_add(T::DbWeight::get().writes(3673))
	}
}