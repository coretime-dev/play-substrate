// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(1, 1);
	});
}

