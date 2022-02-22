// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_identity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-17, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/identity/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_identity.
pub trait WeightInfo {
	fn request_approved_publisher() -> Weight;
	fn approve_publisher() -> Weight;
	fn revoke_publisher() -> Weight;
	fn publish_service(n: u32, ) -> Weight;
	fn subscribe_service() -> Weight;
	fn unsubscribe_service() -> Weight;
	fn renew_subscription() -> Weight;
	fn unpublished_service() -> Weight;
	// fn set_subs_old(p: u32, ) -> Weight;
	// fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight;
	// fn request_judgement(r: u32, x: u32, ) -> Weight;
	// fn cancel_request(r: u32, x: u32, ) -> Weight;
	// fn set_fee(r: u32, ) -> Weight;
	// fn set_account_id(r: u32, ) -> Weight;
	// fn set_fields(r: u32, ) -> Weight;
	// fn provide_judgement(r: u32, x: u32, ) -> Weight;
	// fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight;
	// fn add_sub(s: u32, ) -> Weight;
	// fn rename_sub(s: u32, ) -> Weight;
	// fn remove_sub(s: u32, ) -> Weight;
	// fn quit_sub(s: u32, ) -> Weight;
}

/// Weights for pallet_identity using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Identity Registrars (r:1 w:1)
	fn request_approved_publisher() -> Weight {
		(19_176_000 as Weight)
	}

	fn approve_publisher() -> Weight {
		(19_176_000 as Weight)
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	fn revoke_publisher() -> Weight {
		(44_668_000 as Weight)
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:1 w:1)
	fn publish_service(n: u32) -> Weight {
		(38_917_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((5_331_000 as Weight).saturating_mul(n as Weight))
	}

	fn subscribe_service() -> Weight {
		(19_176_000 as Weight)
	}

	fn unsubscribe_service() -> Weight {
		(19_176_000 as Weight)
	}

	fn renew_subscription() -> Weight {
		(19_176_000 as Weight)
	}

	fn unpublished_service() -> Weight {
		(19_176_000 as Weight)
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:1)
	// fn set_subs_old(p: u32, ) -> Weight {
	// 	(36_057_000 as Weight)
	// 		// Standard Error: 1_000
	// 		.saturating_add((1_756_000 as Weight).saturating_mul(p as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(2 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(1 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	// }
	// // Storage: Identity SubsOf (r:1 w:1)
	// // Storage: Identity IdentityOf (r:1 w:1)
	// // Storage: Identity SuperOf (r:0 w:100)
	// fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
	// 	(44_348_000 as Weight)
	// 		// Standard Error: 9_000
	// 		.saturating_add((183_000 as Weight).saturating_mul(r as Weight))
	// 		// Standard Error: 1_000
	// 		.saturating_add((1_724_000 as Weight).saturating_mul(s as Weight))
	// 		// Standard Error: 1_000
	// 		.saturating_add((439_000 as Weight).saturating_mul(x as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(2 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(2 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	// }
	// // Storage: Identity Registrars (r:1 w:0)
	// // Storage: Identity IdentityOf (r:1 w:1)
	// fn request_judgement(r: u32, x: u32, ) -> Weight {
	// 	(46_592_000 as Weight)
	// 		// Standard Error: 5_000
	// 		.saturating_add((321_000 as Weight).saturating_mul(r as Weight))
	// 		// Standard Error: 0
	// 		.saturating_add((858_000 as Weight).saturating_mul(x as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(2 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity IdentityOf (r:1 w:1)
	// fn cancel_request(r: u32, x: u32, ) -> Weight {
	// 	(43_556_000 as Weight)
	// 		// Standard Error: 6_000
	// 		.saturating_add((174_000 as Weight).saturating_mul(r as Weight))
	// 		// Standard Error: 0
	// 		.saturating_add((850_000 as Weight).saturating_mul(x as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(1 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity Registrars (r:1 w:1)
	// fn set_fee(r: u32, ) -> Weight {
	// 	(7_971_000 as Weight)
	// 		// Standard Error: 4_000
	// 		.saturating_add((283_000 as Weight).saturating_mul(r as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(1 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity Registrars (r:1 w:1)
	// fn set_account_id(r: u32, ) -> Weight {
	// 	(8_234_000 as Weight)
	// 		// Standard Error: 4_000
	// 		.saturating_add((280_000 as Weight).saturating_mul(r as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(1 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity Registrars (r:1 w:1)
	// fn set_fields(r: u32, ) -> Weight {
	// 	(8_126_000 as Weight)
	// 		// Standard Error: 4_000
	// 		.saturating_add((275_000 as Weight).saturating_mul(r as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(1 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity Registrars (r:1 w:0)
	// // Storage: Identity IdentityOf (r:1 w:1)
	// fn provide_judgement(r: u32, x: u32, ) -> Weight {
	// 	(30_949_000 as Weight)
	// 		// Standard Error: 5_000
	// 		.saturating_add((286_000 as Weight).saturating_mul(r as Weight))
	// 		// Standard Error: 0
	// 		.saturating_add((856_000 as Weight).saturating_mul(x as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(2 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity SubsOf (r:1 w:1)
	// // Storage: Identity IdentityOf (r:1 w:1)
	// // Storage: System Account (r:1 w:1)
	// // Storage: Identity SuperOf (r:0 w:100)
	// fn kill_identity(r: u32, s: u32, _x: u32, ) -> Weight {
	// 	(63_792_000 as Weight)
	// 		// Standard Error: 11_000
	// 		.saturating_add((242_000 as Weight).saturating_mul(r as Weight))
	// 		// Standard Error: 1_000
	// 		.saturating_add((1_738_000 as Weight).saturating_mul(s as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(3 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(3 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	// }
	// // Storage: Identity IdentityOf (r:1 w:0)
	// // Storage: Identity SuperOf (r:1 w:1)
	// // Storage: Identity SubsOf (r:1 w:1)
	// fn add_sub(s: u32, ) -> Weight {
	// 	(48_751_000 as Weight)
	// 		// Standard Error: 1_000
	// 		.saturating_add((193_000 as Weight).saturating_mul(s as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(3 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(2 as Weight))
	// }
	// // Storage: Identity IdentityOf (r:1 w:0)
	// // Storage: Identity SuperOf (r:1 w:1)
	// fn rename_sub(s: u32, ) -> Weight {
	// 	(15_892_000 as Weight)
	// 		// Standard Error: 0
	// 		.saturating_add((49_000 as Weight).saturating_mul(s as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(2 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity IdentityOf (r:1 w:0)
	// // Storage: Identity SuperOf (r:1 w:1)
	// // Storage: Identity SubsOf (r:1 w:1)
	// fn remove_sub(s: u32, ) -> Weight {
	// 	(49_746_000 as Weight)
	// 		// Standard Error: 1_000
	// 		.saturating_add((181_000 as Weight).saturating_mul(s as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(3 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(2 as Weight))
	// }
	// // Storage: Identity SuperOf (r:1 w:1)
	// // Storage: Identity SubsOf (r:1 w:1)
	// fn quit_sub(s: u32, ) -> Weight {
	// 	(32_286_000 as Weight)
	// 		// Standard Error: 1_000
	// 		.saturating_add((166_000 as Weight).saturating_mul(s as Weight))
	// 		.saturating_add(T::DbWeight::get().reads(2 as Weight))
	// 		.saturating_add(T::DbWeight::get().writes(2 as Weight))
	// }
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Identity Registrars (r:1 w:1)
	fn request_approved_publisher() -> Weight {
		(19_176_000 as Weight)
	}

	fn approve_publisher() -> Weight {
		(19_176_000 as Weight)
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	fn revoke_publisher() -> Weight {
		(44_668_000 as Weight)
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:1 w:1)
	fn publish_service(n: u32) -> Weight {
		(38_917_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((5_331_000 as Weight).saturating_mul(n as Weight))
	}

	fn subscribe_service() -> Weight {
		(19_176_000 as Weight)
	}

	fn unsubscribe_service() -> Weight {
		(19_176_000 as Weight)
	}

	fn renew_subscription() -> Weight {
		(19_176_000 as Weight)
	}

	fn unpublished_service() -> Weight {
		(19_176_000 as Weight)
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:1)
	// fn set_subs_old(p: u32, ) -> Weight {
	// 	(36_057_000 as Weight)
	// 		// Standard Error: 1_000
	// 		.saturating_add((1_756_000 as Weight).saturating_mul(p as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	// }
	// // Storage: Identity SubsOf (r:1 w:1)
	// // Storage: Identity IdentityOf (r:1 w:1)
	// // Storage: Identity SuperOf (r:0 w:100)
	// fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
	// 	(44_348_000 as Weight)
	// 		// Standard Error: 9_000
	// 		.saturating_add((183_000 as Weight).saturating_mul(r as Weight))
	// 		// Standard Error: 1_000
	// 		.saturating_add((1_724_000 as Weight).saturating_mul(s as Weight))
	// 		// Standard Error: 1_000
	// 		.saturating_add((439_000 as Weight).saturating_mul(x as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	// }
	// // Storage: Identity Registrars (r:1 w:0)
	// // Storage: Identity IdentityOf (r:1 w:1)
	// fn request_judgement(r: u32, x: u32, ) -> Weight {
	// 	(46_592_000 as Weight)
	// 		// Standard Error: 5_000
	// 		.saturating_add((321_000 as Weight).saturating_mul(r as Weight))
	// 		// Standard Error: 0
	// 		.saturating_add((858_000 as Weight).saturating_mul(x as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity IdentityOf (r:1 w:1)
	// fn cancel_request(r: u32, x: u32, ) -> Weight {
	// 	(43_556_000 as Weight)
	// 		// Standard Error: 6_000
	// 		.saturating_add((174_000 as Weight).saturating_mul(r as Weight))
	// 		// Standard Error: 0
	// 		.saturating_add((850_000 as Weight).saturating_mul(x as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity Registrars (r:1 w:1)
	// fn set_fee(r: u32, ) -> Weight {
	// 	(7_971_000 as Weight)
	// 		// Standard Error: 4_000
	// 		.saturating_add((283_000 as Weight).saturating_mul(r as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity Registrars (r:1 w:1)
	// fn set_account_id(r: u32, ) -> Weight {
	// 	(8_234_000 as Weight)
	// 		// Standard Error: 4_000
	// 		.saturating_add((280_000 as Weight).saturating_mul(r as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity Registrars (r:1 w:1)
	// fn set_fields(r: u32, ) -> Weight {
	// 	(8_126_000 as Weight)
	// 		// Standard Error: 4_000
	// 		.saturating_add((275_000 as Weight).saturating_mul(r as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity Registrars (r:1 w:0)
	// // Storage: Identity IdentityOf (r:1 w:1)
	// fn provide_judgement(r: u32, x: u32, ) -> Weight {
	// 	(30_949_000 as Weight)
	// 		// Standard Error: 5_000
	// 		.saturating_add((286_000 as Weight).saturating_mul(r as Weight))
	// 		// Standard Error: 0
	// 		.saturating_add((856_000 as Weight).saturating_mul(x as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity SubsOf (r:1 w:1)
	// // Storage: Identity IdentityOf (r:1 w:1)
	// // Storage: System Account (r:1 w:1)
	// // Storage: Identity SuperOf (r:0 w:100)
	// fn kill_identity(r: u32, s: u32, _x: u32, ) -> Weight {
	// 	(63_792_000 as Weight)
	// 		// Standard Error: 11_000
	// 		.saturating_add((242_000 as Weight).saturating_mul(r as Weight))
	// 		// Standard Error: 1_000
	// 		.saturating_add((1_738_000 as Weight).saturating_mul(s as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(3 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	// }
	// // Storage: Identity IdentityOf (r:1 w:0)
	// // Storage: Identity SuperOf (r:1 w:1)
	// // Storage: Identity SubsOf (r:1 w:1)
	// fn add_sub(s: u32, ) -> Weight {
	// 	(48_751_000 as Weight)
	// 		// Standard Error: 1_000
	// 		.saturating_add((193_000 as Weight).saturating_mul(s as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(3 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	// }
	// // Storage: Identity IdentityOf (r:1 w:0)
	// // Storage: Identity SuperOf (r:1 w:1)
	// fn rename_sub(s: u32, ) -> Weight {
	// 	(15_892_000 as Weight)
	// 		// Standard Error: 0
	// 		.saturating_add((49_000 as Weight).saturating_mul(s as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	// }
	// // Storage: Identity IdentityOf (r:1 w:0)
	// // Storage: Identity SuperOf (r:1 w:1)
	// // Storage: Identity SubsOf (r:1 w:1)
	// fn remove_sub(s: u32, ) -> Weight {
	// 	(49_746_000 as Weight)
	// 		// Standard Error: 1_000
	// 		.saturating_add((181_000 as Weight).saturating_mul(s as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(3 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	// }
	// // Storage: Identity SuperOf (r:1 w:1)
	// // Storage: Identity SubsOf (r:1 w:1)
	// fn quit_sub(s: u32, ) -> Weight {
	// 	(32_286_000 as Weight)
	// 		// Standard Error: 1_000
	// 		.saturating_add((166_000 as Weight).saturating_mul(s as Weight))
	// 		.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	// 		.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	// }
}