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

	#[pallet::storage]
	#[pallet::getter(fn my_option)]
	pub type MyOption<T> = StorageValue<_, u32>;

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
