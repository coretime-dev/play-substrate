#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet for coin flip game

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{
	decl_module,
	decl_storage,
	decl_event,
	decl_error,
	dispatch::DispatchResult,
};
use frame_system::{self as system, ensure_signed};
use frame_support::traits::{Currency, WithdrawReason, ExistenceRequirement, Randomness};
use sp_runtime::traits::{Zero, Hash, Saturating};
use codec::Encode;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: pallet_balances::Trait {
	// The dependency that generates random seed
	type Randomness: Randomness<Self::Hash>;

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as CoinFlipModule {
		// The fee that a player need to pay for the game
		pub Payment get(fn payment): Option<T::Balance>;
		
		// The jackpot holds available latest rewards
		pub Pot get(fn pot): T::Balance;

		Nonce get(fn nonce): u64;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where
		AccountId = <T as system::Trait>::AccountId,
		Balance = <T as pallet_balances::Trait>::Balance {
		/// Emit this event when payment was set
		PaymentSet(Balance),

		/// Emit this event when a user play the game
		PlayResult(AccountId, Balance),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NonePaymentValue,
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

		/// Start the game by initialize the storage items.
		#[weight = 0]
		fn set_payment(origin, value: T::Balance) -> DispatchResult {
			// Ensure the function call is a signed message (i.e. a transaction)
			ensure_signed(origin)?;

			// If `payment` is not initialized with some value, set the payment
			if Self::payment().is_none() {
				// set input value to the payment
				<Payment<T>>::put(value);
				
				// Initialize jackpot;
				<Pot<T>>::put(value);

				// Raise an event for the set payment
				Self::deposit_event(RawEvent::PaymentSet(value));
			}

			Ok(())
		}
		
		/// This function allow a user to play our coin flip game
		#[weight = 0]
		fn play(origin) -> DispatchResult {
			// Ensure that the function call is a signed message (i.e. a transaction)
			let sender = ensure_signed(origin)?;

			// Ensure the payment storage item has been set
			let payment = Self::payment().ok_or(Error::<T>::NonePaymentValue)?;

			// Read our storage values, and place them in memory variables
			let mut nonce = Self::nonce();
			let mut pot = Self::pot();

			// Try to withdraw the payment from the account, making sure that it will not kill the account
			let _ = <pallet_balances::Module<T> as Currency<_>>::withdraw(&sender, payment, WithdrawReason::Reserve.into(), ExistenceRequirement::KeepAlive)?;

			let mut winnings = Zero::zero();

			// Generate a random seed using randomness_collective_flip pallet
			let random_seed = T::Randomness::random_seed().using_encoded(T::Hashing::hash);
			let seed_arr = random_seed.as_ref();
		
			// as_ref returns an array of u8
			if seed_arr[seed_arr.len() - 1] < 128 {
				// If the user won the coin flip, deposit the pot winnings; cannot fail
				let _ = <pallet_balances::Module<T> as Currency<_>>::deposit_into_existing(&sender, pot)
					.expect("`sender` must exist since a transaction is being make and withdraw will keep alive; qed.");
				
				// Set the winnings
				winnings = pot;

				// Reduce the pot to zero
				pot = Zero::zero();
			}

			// No matter the outcome, increase the pot by the payment amount
			pot = pot.saturating_add(payment);

			// Increase the nonce
			nonce = nonce.wrapping_add(1);

			// Store the updated value for our storage items
			<Pot<T>>::put(pot);
			Nonce::put(nonce);

			// Raise event for the play result
			Self::deposit_event(RawEvent::PlayResult(sender, winnings));

			Ok(())
		}

	}
}
