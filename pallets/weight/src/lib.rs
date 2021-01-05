#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, dispatch,
	weights::{
		DispatchClass,
		ClassifyDispatch,
		WeighData,
		Weight,
		PaysFee,
		FunctionOf,
		Pays,
	}
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::SaturatedConversion;

struct WeightForCustomWeight(u32);

impl WeighData<(&u32,)> for WeightForCustomWeight {
	fn weigh_data(&self, target: (&u32,)) -> Weight {
		let multiplier = self.0;
		(*target.0 * multiplier).saturated_into::<Weight>()
	}
}

impl ClassifyDispatch<(&u32,)> for WeightForCustomWeight {
	fn classify_dispatch(&self, target: (&u32,)) -> DispatchClass {
		if *target.0 > 1000u32 {
			DispatchClass::Operational
		} else {
			DispatchClass::Normal
		}
	}
}

impl PaysFee<(&u32,)> for WeightForCustomWeight {
	fn pays_fee(&self, _target: (&u32,)) -> Pays {
		Pays::Yes
	}
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	trait Store for Module<T: Trait> as WeightModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Something get(fn something): Option<u32>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		SomethingStored(u32, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
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

		/// Just a dummy entry point.
		/// function that can be called by the external world as an extrinsics call
		/// takes a parameter of the type `AccountId`, stores it, and emits an event
		///
		/// # <weight>
		/// - `O(1)`
		/// - 1 DB change
		/// # </weight>
		#[weight = 100_000_000]
		pub fn fixed_weight_with_default(origin, something: u32) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			// Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// Here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		/// Just a dummy dispatchable call for operational purpose.
		#[weight = (100_000_000, DispatchClass::Operational)]
		pub fn fixed_weight_with_operational(origin, something: u32) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			// Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// Here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		/// Just a dummy dispatchable call for custom weight.
		#[weight = WeightForCustomWeight(100u32)]
		pub fn custom_weight(origin, something: u32) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			// Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// Here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		/// FunctionOf is deprecated and not suggest to use any more.
		/// Just a dummy dispatchable call for FunctionOf weight.
		#[weight = FunctionOf(|args: (&u32,)| (args.0 * 10) as Weight, DispatchClass::Normal, Pays::Yes)]
		pub fn function_of_weight(origin, something: u32) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			// Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// Here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

	}
}
