// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use frame_support::traits::Currency;
use sp_runtime::traits::BadOrigin;
use super::*;
use pallet_balances::Error as BalancesError;

#[test]
fn set_payment_should_work() {
	new_test_ext().execute_with(|| {
		// asserting the function can be called successfully
		assert_ok!(CoinFlipModule::set_payment(Origin::signed(1), 100));
		// asserting that the stored value is equal to what we stored
        assert_eq!(CoinFlipModule::payment(), Some(100));
		assert_eq!(CoinFlipModule::pot(), 100);
		
		// do not update the stored value
		assert_ok!(CoinFlipModule::set_payment(Origin::signed(1), 200));
		// asserting that the stored value is equal to what we stored
        assert_eq!(CoinFlipModule::payment(), Some(100));
        assert_eq!(CoinFlipModule::pot(), 100);
	});
}

#[test]
fn play_security_check_should_work() {
	new_test_ext().execute_with(|| {
		// Test ensure_signed
		assert_noop!(CoinFlipModule::play(Origin::root()), BadOrigin);

		// Ensure the correct error if payment not set
		assert_noop!(CoinFlipModule::play(Origin::signed(2)), Error::<Test>::NonePaymentValue);

		// Check the balances in genesis config
		 assert_eq!(Balances::total_balance(&2), 20);

		 // set payment and pot, higher than the balances
		 <Payment<Test>>::put(30);
		 <Pot<Test>>::put(30);

		 assert_noop!(CoinFlipModule::play(Origin::signed(2)), BalancesError::<Test, _>::InsufficientBalance);

		 // set payment and pot, lower than the balances
		 <Payment<Test>>::put(10);
		 <Pot<Test>>::put(10);
		 assert_ok!(CoinFlipModule::play(Origin::signed(2)));
	})
}

#[test]
fn play_should_work_for_win() {
	new_test_ext().execute_with(|| {
		<Payment<Test>>::put(10);
		<Pot<Test>>::put(30);
		<Nonce<Test>>::put(0);

		assert_ok!(CoinFlipModule::play(Origin::signed(2)));
		assert_eq!(CoinFlipModule::payment(), Some(10));
		assert_eq!(CoinFlipModule::pot(), 10);
		assert_eq!(Balances::total_balance(&2), 40); // 20 - 10 (payment) + 30 (reward)
		assert_eq!(CoinFlipModule::nonce(), 1);
	})
}

#[test]
fn play_should_work_for_lose() {
	new_test_ext().execute_with(|| {
		<Payment<Test>>::put(10);
		<Pot<Test>>::put(30);
		<Nonce<Test>>::put(1);

		assert_ok!(CoinFlipModule::play(Origin::signed(2)));
		assert_eq!(CoinFlipModule::payment(), Some(10));
		assert_eq!(CoinFlipModule::pot(), 40);
		assert_eq!(Balances::total_balance(&2), 10); // 20 - 10 (payment)
		assert_eq!(CoinFlipModule::nonce(), 2);
	})
}
