#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet for coin flip game

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::traits::{Currency, WithdrawReasons, ExistenceRequirement, Randomness};
	use sp_runtime::traits::{Zero, Hash, Saturating};

	// TODO: refactor to inject currency trait
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config {
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn payment)]
	pub type Payment<T: Config> = StorageValue<_, T::Balance>;

	#[pallet::storage]
	#[pallet::getter(fn pot)]
	pub type Pot<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	pub type Nonce<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PaymentSet(T::Balance),
		PlayResult(T::AccountId, T::Balance),
	}

	#[pallet::error]
	pub enum Error<T> {
		NonePaymentValue,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Start the game by initialize the storage items.
		#[pallet::weight(0)]
		pub fn set_payment(origin: OriginFor<T>, value: T::Balance) -> DispatchResultWithPostInfo {
			// Ensure the function call is a signed message (i.e. a transaction)
			ensure_signed(origin)?;

			// If `payment` is not initialized with some value, set the payment
			if Self::payment().is_none() {
				// set input value to the payment
				<Payment<T>>::put(value);
				
				// Initialize jackpot;
				<Pot<T>>::put(value);

				// Raise an event for the set payment
				Self::deposit_event(Event::PaymentSet(value));
			}

			Ok(().into())
		}
		
		/// This function allow a user to play our coin flip game
		#[pallet::weight(0)]
		pub fn play(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			// Ensure that the function call is a signed message (i.e. a transaction)
			let sender = ensure_signed(origin)?;

			// Ensure the payment storage item has been set
			let payment = Self::payment().ok_or(Error::<T>::NonePaymentValue)?;

			// Read our storage values, and place them in memory variables
			let mut nonce = Self::nonce();
			let mut pot = Self::pot();

			// Try to withdraw the payment from the account, making sure that it will not kill the account
			let _ = <pallet_balances::Pallet<T> as Currency<_>>::withdraw(&sender, payment, WithdrawReasons::RESERVE.into(), ExistenceRequirement::KeepAlive)?;

			let mut winnings = Zero::zero();

			// Generate a random seed using randomness_collective_flip pallet
			let random_seed = T::Randomness::random_seed().0.using_encoded(T::Hashing::hash);
			let seed_arr = random_seed.as_ref();
		
			// as_ref returns an array of u8
			if seed_arr[seed_arr.len() - 1] < 128 {
				// If the user won the coin flip, deposit the pot winnings; cannot fail
				let _ = <pallet_balances::Pallet<T> as Currency<_>>::deposit_into_existing(&sender, pot)
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
			Nonce::<T>::put(nonce);

			// Raise event for the play result
			Self::deposit_event(Event::PlayResult(sender, winnings));

			Ok(().into())
		}
	}

}