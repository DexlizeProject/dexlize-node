// Copyright 2019-2020
//     by  Dexlize Foundation PTE Ltd.
// This file is part of Dexlize Project.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! # Generic Asset Gateway Module
//!
//! The Generic Asset module provides functionality for handling basic process of deposit and withdraw.
//!
//! ## Overview
//!
//! The Generic Asset Gateway module provides functions for:
//!
//! - Creating a new kind of asset.
//! - 

#![cfg_attr(not(feature = "std"), no_std)]

#[allow(unused_imports)]
use codec::{Decode, Encode, Error as codecErr, HasCompact, Input, Output};
use sp_std::prelude::*;
use sp_core::H256;
use frame_support::{Parameter, decl_module, decl_storage, decl_event, dispatch::DispatchResult};
use sp_runtime::traits::{Member, SimpleArithmetic, Zero, StaticLookup};
use system::ensure_signed;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum DepositStatus {
	Pending,
	Confirmed,
	Completed
}
impl Default for DepositStatus {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum WithdrawStatus {
	Requested,
	Approved,
	Rejected,
	Completed
}
impl Default for WithdrawStatus {
    fn default() -> Self {
        Self::Requested
    }
}

#[derive(Encode, Decode, Default, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct DepositRecord<AccountId, Balance> {
	pub assetId: u64,
	/// the address who send btc
	pub from: Vec<u8>,
	/// the account who will receive "amount" of SBTC
	pub to: AccountId,
    /// BTC Transaction Hash
    pub tx_hash: Option<Vec<u8>>,
	pub amount: Balance,
	pub status: DepositStatus,
}

#[derive(Encode, Decode, Default, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct WithdrawRecord<AccountId, Balance> {
	pub assetId: u64,
    /// the account who will receive "amount" of SBTC
	pub from: AccountId,
	/// the btc withdraw to
	pub to: Vec<u8>,
    /// tx_hash is the hash of the transaction that transfers BTC into TBD
	pub tx_hash: Option<Vec<u8>>,
	/// SBTC 1:1 BTC
	pub amount: Balance,
	pub status: WithdrawStatus,
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Balance: Member + Parameter + SimpleArithmetic + Default + Copy;
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as GenericAssetGateway {
		// Deposit Related
		DepositXpubKey get(deposit_xpub_key): linked_map Option<u32> => Option<H256>;
		DepositHistory get(deposit_history) : linked_map Vec<u8> => Vec<DepositRecord<T::AccountId, T::Balance>>;
		NextDepositIndex get(next_deposit_index): linked_map u64 => u64;
		// Withdraw related
		NextWithdrawId get(next_withdraw_id): u64;
		WithdrawHistory get(withdraw_history): linked_map Option<u32> => Vec<WithdrawRecord<T::AccountId, T::Balance>>;
	}
}

// The pallet's dispatchable functions.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		pub fn detecte_deposit(origin) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Ok(())
		}
		
		pub fn set_admin_key(origin) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Ok(())
		}

		pub fn request_withdraw(origin) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Ok(())
		}

		pub fn approve_withdraw(origin) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Ok(())
		}

		pub fn reject_withdraw(origin) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Ok(())
		}

		pub fn execute_withdraw(origin) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		AssetDeposited(u32, AccountId),
		AssetWithdrawRequested(u32, AccountId),
		AssetWithdrawApproved(u32, AccountId),
		AssetWithdrawRejected(u32, AccountId),
		AssetWithdrawSuccess(u32, AccountId),
	}
);

/// tests for this pallet
#[cfg(test)]
mod tests {
	use super::*;

	use sp_core::H256;
	use frame_support::{impl_outer_origin, assert_ok, parameter_types, weights::Weight};
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the pallet, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
		type ModuleToIndex = ();
	}
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_default_value() {
		new_test_ext().execute_with(|| {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(TemplateModule::something(), Some(42));
		});
	}
}
