#![cfg_attr(not(feature = "std"), no_std)]

/// A module for demo storage data types.
pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use frame_support::{pallet_prelude::*, Blake2_128Concat, Twox64Concat};
	use frame_system::pallet_prelude::*;

	use scale_info::TypeInfo;
	use sp_runtime::{Permill,
		traits::{
			Saturating
		}
	};
	use sp_std::prelude::*;
	use sp_core::{H256, U256};

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
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
	pub type MyString<T> = StorageValue<_, BoundedVec<u8, ConstU32<100>>, ValueQuery>;

	// float number, Percent, Permill, Perbill
	#[pallet::storage]
	#[pallet::getter(fn my_permill)]
	pub type MyPermill<T> = StorageValue<_, Permill, ValueQuery>;

	// time is usually the type alias of u64
	#[pallet::storage]
	#[pallet::getter(fn my_time)]
	pub type MyTime<T: Config> = StorageValue<_, T::Moment>;

	// AccountId is [u8,32]
	#[pallet::storage]
	#[pallet::getter(fn my_account_id)]
	pub type MyAccountId<T: Config> = StorageValue<_, T::AccountId>;

	// BlockNumber
	#[pallet::storage]
	#[pallet::getter(fn my_block_number)]
	pub type MyBlockNumber<T: Config> = StorageValue<_, BlockNumberFor<T>>;

	// tuple
	#[pallet::storage]
	#[pallet::getter(fn my_tuple)]
	pub type MyTuple<T> = StorageValue<_, (u8, bool)>;

	#[derive(Copy, Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
	pub enum Weekday {
		Monday,
		Tuesday,
		Wednesday,
		Other,
	}

	// enum
	#[pallet::storage]
	#[pallet::getter(fn my_enum)]
	pub type MyEnum<T> = StorageValue<_, Weekday>;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, Default, TypeInfo, MaxEncodedLen)]
	pub struct People {
		name: BoundedVec<u8, ConstU32<10>>,
		age: u8,
	}

	// struct
	#[pallet::storage]
	#[pallet::getter(fn my_struct)]
	pub type MyStruct<T> = StorageValue<_, People>;

	// Fixed Hash
	#[pallet::storage]
	#[pallet::getter(fn my_fixed_hashuct)]
	pub type MyFixedHash<T> = StorageValue<_, H256>;

	// Big integer
	#[pallet::storage]
	#[pallet::getter(fn my_big_integer)]
	pub type MyBigInteger<T> = StorageValue<_, U256>;

	// map
	#[pallet::storage]
	#[pallet::getter(fn my_map)]
	pub type MyMap<T: Config> = StorageMap<_, Twox64Concat, u8, T::Hash>;

	// double map
	#[pallet::storage]
	#[pallet::getter(fn my_double_map)]
	pub type MyDoubleMap<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Twox64Concat, u32, T::Hash>;

	// storage n map
	#[pallet::storage]
	#[pallet::getter(fn my_n_map)]
	pub type MyNMap<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>,
			NMapKey<Blake2_128Concat, BlockNumberFor<T>>, // owner
			NMapKey<Blake2_128Concat, u32>, // delegate
		),
		T::Hash,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		OptionSet(u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		NameTooLong,
		StringTooLong,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn set_option(origin: OriginFor<T>, value: u32) -> DispatchResult {
			ensure_signed(origin)?;

			MyOption::<T>::put(value);

			Self::deposit_event(Event::OptionSet(value));
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn modify_option(origin: OriginFor<T>) -> DispatchResult {
			ensure_signed(origin)?;

			let my_option = MyOption::<T>::get();
			match my_option {
				Some(v) => log::info!("old value when modify option: {:?}", v),
				None => log::info!("No value store"),
			}

			MyOption::<T>::mutate(|my_option| my_option.map(|v| v + 1));

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn delete_option(origin: OriginFor<T>) -> DispatchResult {
			ensure_signed(origin)?;

			MyOption::<T>::kill();

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		// this is for demonstration, you should never put all the operations in one call.
		pub fn play_number(origin: OriginFor<T>, number: u8) -> DispatchResult {
			ensure_signed(origin)?;

			MyUnsignedNumber::<T>::put(number);

			let _my_num = MyUnsignedNumber::<T>::get();

			MyUnsignedNumber::<T>::mutate(|value| value.saturating_add(1));

			MyUnsignedNumber::<T>::kill();

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		// this is for demonstration, you should never put all the operations in one call.
		pub fn play_integer(origin: OriginFor<T>, number: i8) -> DispatchResult {
			ensure_signed(origin)?;

			MySignedNumber::<T>::put(number);

			let _my_num = MySignedNumber::<T>::get();

			MySignedNumber::<T>::mutate(|value| value.saturating_sub(1));

			MySignedNumber::<T>::kill();

			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		// this is for demonstration, you should never put all the operations in one call.
		pub fn play_bool(origin: OriginFor<T>, value: bool) -> DispatchResult {
			ensure_signed(origin)?;

			MyBool::<T>::put(value);

			let my_bool = MyBool::<T>::get();

			if my_bool {
				log::info!("get true in bool demo");
			} else {
				log::info!("get false in bool demo");
			}

			MyBool::<T>::put(!my_bool);

			MyBool::<T>::kill();

			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(0)]
		// this is for demonstration, you should never put all the operations in one call.
		pub fn play_string(origin: OriginFor<T>, value: Vec<u8>) -> DispatchResult {
			ensure_signed(origin)?;

			let bounded_value = BoundedVec::<u8, ConstU32<100>>::try_from(value.clone()).map_err(|_| Error::<T>::StringTooLong)?;
			MyString::<T>::put(bounded_value);

			let my_string = MyString::<T>::get();

			let _new_string = my_string.iter().map(|v| v + 1);

			if my_string.len() > 10 {
				log::info!("string too long in string demo");
			}

			// MyString::kill();

			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(0)]
		// this is for demonstration, you should never put all the operations in one call.
		pub fn play_permill(origin: OriginFor<T>, value: u32) -> DispatchResult {
			ensure_signed(origin)?;

			// from_percent needs 1~100
			MyPermill::<T>::put(Permill::from_percent(value));

			let permill_one = Permill::from_parts(1000);
			let permill_two = Permill::from_rational(9 as u32,1001 as u32);
			let _mul_permil = permill_one.saturating_mul(permill_two);
			let _mul_result = permill_two * 20000 as u32;

			let my_permill = MyPermill::<T>::get();
			let calc_result = my_permill * 1_000_000 as u32;
			log::info!("get calc result from permill demo: {:?}", calc_result);

			Ok(())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(0)]
		pub fn play_time(origin: OriginFor<T>) -> DispatchResult {
			ensure_signed(origin)?;

			let _now = <pallet_timestamp::Pallet<T>>::get();
			MyTime::<T>::put(_now);

			Ok(())
		}

		#[pallet::call_index(9)]
		#[pallet::weight(0)]
		pub fn play_account_id(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			MyAccountId::<T>::put(sender);

			Ok(())
		}

		#[pallet::call_index(10)]
		#[pallet::weight(0)]
		pub fn play_block_number(origin: OriginFor<T>) -> DispatchResult {
			ensure_signed(origin)?;
			// MyBlockNumber::<T>::put();

			Ok(())
		}

		#[pallet::call_index(11)]
		#[pallet::weight(0)]
		pub fn play_tuple(origin: OriginFor<T>, first: u8, second: bool) -> DispatchResult {
			ensure_signed(origin)?;

			MyTuple::<T>::put((first, second));

			Ok(())
		}

		#[pallet::call_index(12)]
		#[pallet::weight(0)]
		pub fn play_enum(origin: OriginFor<T>, weekday: Weekday) -> DispatchResult {
			ensure_signed(origin)?;

			MyEnum::<T>::put(weekday);

			Ok(())
		}

		#[pallet::call_index(13)]
		#[pallet::weight(0)]
		pub fn play_struct(origin: OriginFor<T>, name: Vec<u8>, age: u8) -> DispatchResult {
			ensure_signed(origin)?;

			let people = People {
				name: BoundedVec::<u8, ConstU32<10>>::try_from(name.clone()).map_err(|_| Error::<T>::NameTooLong)?,
				age,
			};
			MyStruct::<T>::put(people);

			let _my_people = MyStruct::<T>::get();

			Ok(())
		}

		#[pallet::call_index(14)]
		#[pallet::weight(0)]
		pub fn insert_map(origin: OriginFor<T>, key: u8, value: T::Hash) -> DispatchResult {
			ensure_signed(origin)?;

			MyMap::<T>::insert(key, value);

			MyMap::<T>::contains_key(key);

			let my_value = MyMap::<T>::get(key);
			log::info!("get value from map demo: {:?}", my_value);

			MyMap::<T>::remove(key);

			Ok(())
		}

		#[pallet::call_index(15)]
		#[pallet::weight(0)]
		pub fn insert_double_map(origin: OriginFor<T>, key2: u32, value: T::Hash) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			MyDoubleMap::<T>::insert(&sender, key2, value);

			MyDoubleMap::<T>::get(&sender, key2);

			MyDoubleMap::<T>::remove(&sender, key2);

			let _cursor = MyDoubleMap::<T>::clear_prefix(&sender, 100, None);

			Ok(())
		}
	}
}
