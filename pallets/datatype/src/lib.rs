#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet for data type

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, dispatch::DispatchResult,
	debug,
};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_runtime::{
	Permill,
	traits::{Saturating},
};
use codec::{Encode, Decode};
use sp_core::{U256, H256};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Encode, Decode, Eq, PartialEq, Debug)]
pub enum Weekday {
	Monday,
	Tuesday,
	Wednesday,
	Other,
}

impl Default for Weekday {
	fn default() -> Self {
		Weekday::Monday
	}
}

impl From<u8> for Weekday {
	fn from(value: u8) -> Self {
		match value {
			1 => Weekday::Monday,
			2 => Weekday::Tuesday,
			3 => Weekday::Wednesday,
			_ => Weekday::Other,
		}
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, Default)]
pub struct People {
	name: Vec<u8>,
	age: u8,
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait + pallet_timestamp::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as DataTypeModule {
		// init to be none, store optional value
		MyOption get(fn my_option): Option<u32>;

        // store unsigned integer, init to zero if not set, here we set it to 10
		MyUnsignedNumber get(fn unsigned_number): u8 = 10;

		// also init to zero, can store negative number
		MySignedNumber get(fn signed_number): i8;

		// init to false, store boolean value
		MyBool get(fn my_bool): bool;

		// runtime storage should not contain any human readable information
		// instead store the encoded limited vector.
		// default to 0x00
		MyString get(fn my_string): Vec<u8>;

		// float number, Percent, Permill, Perbill
		MyPermill get(fn my_permill): Permill;

		// time is type alias of u64
		MyTime get(fn my_time): T::Moment;

		// AccountId is [u8,32]
		MyAccountId get(fn my_account_id): T::AccountId;

		// BlockNumber
		MyBlockNumber get(fn my_block_number): T::BlockNumber;

		// tuple
		MyTuple get(fn my_tuple): (u8, bool);

		// enum
		MyEnum get(fn my_enum): Weekday;

		// struct
		MyStruct get(fn my_struct): People;

		MyFixedHash get(fn my_fixed_hash): H256;

		MyBigInteger: U256;

		// map
		MyMap get(fn my_map): map hasher(twox_64_concat) u8 => Vec<u8>;

		// double map
		MyDoubleMap get(fn my_double_map): double_map hasher(blake2_128_concat) T::AccountId, hasher(twox_64_concat) u32 => Vec<u8>; // syntax changed for master
	}
}

// The pallet's events
decl_event!(
	pub enum Event {
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		#[weight = 0]
		pub fn set_option(origin, value: u32) -> DispatchResult {
			ensure_signed(origin)?;

			MyOption::put(value);

			Ok(())
		}

		#[weight = 0]
		pub fn modify_option(origin) -> DispatchResult {
			ensure_signed(origin)?;

			let my_option = MyOption::get();
			match my_option {
				Some(v) => debug::info!("old value when modify option: {:?}", v),
				None => debug::info!("No value store"),
			}

			MyOption::mutate(|my_option| my_option.map(|v| v + 1));

			Ok(())
		}

		#[weight = 0]
		pub fn delete_option(origin) -> DispatchResult {
			ensure_signed(origin)?;

			MyOption::kill();

			Ok(())
		}

		#[weight = 0]
		// this is for demonstration, you should never put all the operations in one call.
		pub fn play_number(origin, number: u8) -> DispatchResult {
			ensure_signed(origin)?;

			MyUnsignedNumber::put(number);

			let _my_num = MyUnsignedNumber::get();

			MyUnsignedNumber::mutate(|value| value.saturating_add(1));

			MyUnsignedNumber::kill();

			Ok(())
		}

		#[weight = 0]
		// this is for demonstration, you should never put all the operations in one call.
		pub fn play_integer(origin, number: i8) -> DispatchResult {
			ensure_signed(origin)?;

			MySignedNumber::put(number);

			let _my_num = MySignedNumber::get();

			MySignedNumber::mutate(|value| value.saturating_sub(1));

			MySignedNumber::kill();

			Ok(())
		}

		#[weight = 0]
		// this is for demonstration, you should never put all the operations in one call.
		pub fn play_bool(origin, value: bool) -> DispatchResult {
			ensure_signed(origin)?;

			MyBool::put(value);

			let my_bool = MyBool::get();

			if my_bool {
				debug::info!("get true in bool demo");
			} else {
				debug::info!("get false in bool demo");
			}

			MyBool::put(!my_bool);

			MyBool::kill();

			Ok(())
		}

		#[weight = 0]
		// this is for demonstration, you should never put all the operations in one call.
		pub fn play_string(origin, value: Vec<u8>) -> DispatchResult {
			ensure_signed(origin)?;

			MyString::put(value);

			let mut my_string = MyString::get();

			my_string.push(10);

			let _new_string = my_string.iter().map(|v| v + 1);

			if my_string.len() > 10 {
				debug::info!("string too long in string demo");
			}

			// MyString::kill();

			Ok(())
		}

		#[weight = 0]
		// this is for demonstration, you should never put all the operations in one call.
		pub fn play_permill(origin, value: u32) -> DispatchResult {
			ensure_signed(origin)?;
			
			// from_percent needs 1~100
			MyPermill::put(Permill::from_percent(value));

			let permill_one = Permill::from_parts(1000);
			let permill_two = Permill::from_rational_approximation(9 as u32,1001 as u32);
			let _mul_permil = permill_one.saturating_mul(permill_two);
			let _mul_result = permill_two * 20000 as u32;

			let my_permill = MyPermill::get();
			let calc_result = my_permill * 1_000_000 as u32;
			debug::info!("get calc result from permill demo: {:?}", calc_result);

			Ok(())
		}

		#[weight = 0]
		pub fn play_time(origin) -> DispatchResult {
			ensure_signed(origin)?;
			
			let _now = <pallet_timestamp::Module<T>>::get();
			MyTime::<T>::put(_now);

			Ok(())
		}

		#[weight = 0]
		pub fn play_account_id(origin) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			MyAccountId::<T>::put(sender);

			Ok(())
		}

		#[weight = 0]
		pub fn play_block_number(origin) -> DispatchResult {
			ensure_signed(origin)?;
			// MyBlockNumber::<T>::put();

			Ok(())
		}

		#[weight = 0]
		pub fn play_tuple(origin, first: u8, second: bool) -> DispatchResult {
			ensure_signed(origin)?;
			
			MyTuple::put((first, second));

			Ok(())
		}

		#[weight = 0]
		pub fn play_enum(origin, workday: u8) -> DispatchResult {
			ensure_signed(origin)?;
			
			let weekday: Weekday = workday.into();
			MyEnum::put(weekday);

			Ok(())
		}

		#[weight = 0]
		pub fn play_struct(origin, name: Vec<u8>, age: u8) -> DispatchResult {
			ensure_signed(origin)?;

			let people = People {
				name,
				age,
			};
			MyStruct::put(people);

			let _my_people = MyStruct::get();

			Ok(())
		}

		#[weight = 0]
		pub fn insert_map(origin, key: u8, value: Vec<u8>) -> DispatchResult {
			ensure_signed(origin)?;

			MyMap::insert(key, value);

			MyMap::contains_key(key);

			let my_value = MyMap::get(key);
			debug::info!("get value from map demo: {:?}", my_value);

			MyMap::remove(key);

			Ok(())
		}

		#[weight = 0]
		pub fn insert_double_map(origin, key2: u32, value: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			MyDoubleMap::<T>::insert(&sender, key2, value);

			MyDoubleMap::<T>::get(&sender, key2);

			MyDoubleMap::<T>::remove(&sender, key2);

			MyDoubleMap::<T>::remove_prefix(&sender);

			Ok(())
		}

	}
}
