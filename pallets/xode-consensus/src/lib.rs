// Copyright 2019-2022 Rockson Tech
// This file is part of Xode Blockchain.

// Xode Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Xode Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! # Xode Consensus
//! Minimal consensus and staking pallet that implements collator selection by total backed stake.
//!
//! Details: https://wiki.xode.net/app/page/1kWGdbgYQOk0LLZnEuDHZ6BQvMm73n6vqpXejLEZkkeQ?p=14_D9JXPSHF8yCRxMoqLaC6ne0PeKgfwX 
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;

use frame_support::traits::{ExecuteBlock, FindAuthor};
use sp_consensus_aura::{digests::CompatibleDigestItem, Slot};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};

type Aura<T> = pallet_aura::Pallet<T>;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// The configuration trait.
	#[pallet::config]
	pub trait Config: pallet_aura::Config + frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);
    
}

