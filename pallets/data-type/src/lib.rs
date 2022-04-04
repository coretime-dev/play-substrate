#![cfg_attr(not(feature = "std"), no_std)]

/// A module for demo storage data types.

pub use pallet::*;
use codec::{Encode, Decode};

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

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// init to be none, store optional value
	#[pallet::storage]
	#[pallet::getter(fn my_option)]
	pub type MyOption<T> = StorageValue<_, u32>;

	#[pallet::type_value]
	pub fn DefaultForMyUnsignedNumber() -> u8 {
		10
	}

	// store unsigned integer, init to zero if not set, here we set it to 10
	#[pallet::storage]
	#[pallet::getter(fn unsigned_number)]
	pub type MyUnsignedNumber<T> = StorageValue<_, u8, ValueQuery, DefaultForMyUnsignedNumber>;

	// also init to zero, can store negative number
	#[pallet::storage]
	#[pallet::getter(fn signed_number)]
	pub type MySignedNumber<T> = StorageValue<_, i8, ValueQuery>;

	// init to false, store boolean value
	#[pallet::storage]
	#[pallet::getter(fn my_bool)]
	pub type MyBool<T> = StorageValue<_, bool, ValueQuery>;

	// runtime storage should not contain any human readable information
	// instead store the encoded limited vector.
	// default to 0x00
	#[pallet::storage]
	#[pallet::getter(fn my_string)]
	pub type MyString<T> = StorageValue<_, Vec<u8>, ValueQuery>;

		// // float number, Percent, Permill, Perbill
		// MyPermill get(fn my_permill): Permill;

		// // time is type alias of u64
		// MyTime get(fn my_time): T::Moment;

		// // AccountId is [u8,32]
		// MyAccountId get(fn my_account_id): T::AccountId;

		// // BlockNumber
		// MyBlockNumber get(fn my_block_number): T::BlockNumber;

		// // tuple
		// MyTuple get(fn my_tuple): (u8, bool);

		// // enum
		// MyEnum get(fn my_enum): Weekday;

		// // struct
		// MyStruct get(fn my_struct): People;

		// MyFixedHash get(fn my_fixed_hash): H256;

		// MyBigInteger: U256;

		// // map
		// MyMap get(fn my_map): map hasher(twox_64_concat) u8 => Vec<u8>;

		// // double map
		// MyDoubleMap get(fn my_double_map): double_map hasher(blake2_128_concat) T::AccountId, hasher(twox_64_concat) u32 => Vec<u8>; // syntax changed for master

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	}

	#[pallet::error]
	pub enum Error<T> {
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_option(origin: OriginFor<T>, value: u32) -> DispatchResult {
			ensure_signed(origin)?;

			MyOption::<T>::put(value);

			Ok(())
		}
	}
}
