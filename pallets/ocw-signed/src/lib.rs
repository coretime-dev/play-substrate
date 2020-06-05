#![cfg_attr(not(feature = "std"), no_std)]

/// A module for offchain worker send unsigned transaction

use frame_support::{
	debug,
	decl_module,
	decl_storage,
	decl_event,
	decl_error,
	dispatch::{DispatchResult},
};
use frame_system::{
	self as system,
	ensure_signed,
	offchain::{
		Signer,
		CreateSignedTransaction,
		SendSignedTransaction,
		AppCrypto,
	},
};
use sp_core::crypto::KeyTypeId;
use sp_std::vec::Vec;
use sp_runtime::{
		offchain::{
			http,
			Duration,
		},
};
use sp_std::prelude::*;
// We use `alt_serde`, and Xanewok-modified `serde_json` so that we can compile the program
//   with serde(features `std`) and alt_serde(features `no_std`).
use alt_serde::{Deserialize, Deserializer};
use codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// TODO
// type TokenPrice = u32;

const MAX_LEN: usize = 64; // TODO configurage

// Specifying serde path as `alt_serde`
// ref: https://serde.rs/container-attrs.html#crate
#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct PriceInfo {
	#[serde(rename(deserialize = "USD"), deserialize_with = "de_float_to_integer")]
    usd: u32,
}

pub fn de_float_to_integer<'de, D>(de: D) -> Result<u32, D::Error>
where D: Deserializer<'de> {
	let f: f32 = Deserialize::deserialize(de)?;
	Ok(f as u32)
}

/// Defines application identifier for crypto keys of this module.
///
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"btc!");
/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrappers.
/// We can use from supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// the types with this pallet-specific identifier.
pub mod sr25519 {
	use super::KEY_TYPE;
	use sp_application_crypto::{app_crypto, sr25519};

	app_crypto!(sr25519, KEY_TYPE);

	pub type AuthorityId = Public;
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
	/// The identifier type for an offchain worker.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	/// The overarching dispatch call type.
	type Call: From<Call<Self>>;
	
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as OcwSignedModule {
		/// A vector of recently submitted prices.
		/// 
		/// This is used to calculate average price, should have bounded size.
		Prices get(fn prices): Vec<u32>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		NewPrice(u32, AccountId),
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
		pub fn submit_price_signed(origin, price: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::add_price(who, price);
			
			Ok(())
		}
		
		fn offchain_worker() {
			debug::native::info!("Submit signed: Offchain working starts running");

			let res = Self::fetch_price_and_send_signed();

			if let Err(e) = res {
				debug::error!("Submit signed: Error happends: {}", e);
			}
		}
	}
}

impl<T: Trait> Module<T> {
	fn add_price(who: T::AccountId, price: u32) {
		debug::info!("Submit signed: Adding to the prices: {}", price);
		Prices::mutate(|prices| {
			if prices.len() < MAX_LEN {
				prices.push(price);
			} else {
				prices[price as usize % MAX_LEN] = price;
			}
		});

		Self::deposit_event(RawEvent::NewPrice(price, who));
	}

	fn fetch_price_and_send_signed() -> Result<(), &'static str> {
		let signer = Signer::<T, T::AuthorityId>::all_accounts();
		if !signer.can_sign() {
			return Err(
				"No local accounts available. Consider adding one via `author_insertKey` RPC."
			)?
		}

		// Make an external HTTP request to fetch the current price.
		// Note this call will block until response is received.
		let price = Self::fetch_price().map_err(|_| "Submit signed: Failed to fetch price")?;

		// Using `send_signed_transaction` associated type we create and submit a transaction
		// representing the call, we've just created.
		// Submit signed will return a vector of results for all accounts that were found in the
		// local keystore with expected `KEY_TYPE`.
		let results = signer.send_signed_transaction(
			|_account| {
				// Received price is wrapped into a call to `submit_price` public function of this pallet.
				// This means that the transaction, when executed, will simply call that function passing
				// `price` as an argument.
				Call::submit_price_signed(price)
			}
		);

		for (acc, res) in &results {
			match res {
				Ok(()) => debug::info!("Submit signed: [{:?}] Submitted price of {} cents", acc.id, price),
				Err(e) => debug::error!("Submit signed: [{:?}] Failed to submit transcation, {:?}", acc.id, e),
			}
		}

		Ok(())
	}

	fn fetch_price() -> Result<u32, http::Error> {
		let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(5000));
		// Initiate an external HTTP GET request.
		// This is using high-level wrappers from `sp_runtime`, for the low-level calls that
		// you can find in `sp_io`. The API is trying to be similar to `reqwest`, but
		// since we are running in a custom WASM execution environment we can't simply
		// import the library here.
		let request = http::Request::get(
			"https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD"
		);
		// We set the deadline for sending of the request, note that awaiting response can
		// have a separate deadline. Next we send the request, before that it's also possible
		// to alter request headers or stream body content in case of non-GET requests.
		let pending = request
			.deadline(deadline)
			.send()
			.map_err(|_| http::Error::IoError)?;
		// The request is already being processed by the host, we are free to do anything
		// else in the worker (we can send multiple concurrent requests too).
		// At some point however we probably want to check the response though,
		// so we can block current thread and wait for it to finish.
		// Note that since the request is being driven by the host, we don't have to wait
		// for the request to have it complete, we will just not read the response.
		let response = pending.try_wait(deadline)
			.map_err(|_| http::Error::DeadlineReached)??;

		if response.code != 200 {
			debug::warn!("Submit signed: Unexpected status code: {}", response.code);
			return Err(http::Error::Unknown);
		}

		let body = response.body().collect::<Vec<u8>>();

		let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
			debug::warn!("Not UTF8 body");
			http::Error::Unknown
		})?;

		let price_info: PriceInfo = serde_json::from_str(&body_str).unwrap();
		debug::warn!("Submit Signed: Got price: {} cents", price_info.usd);

		Ok(price_info.usd)
	}

}
