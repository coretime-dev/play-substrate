//! Benchmark-demo pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{benchmarks, account};
use frame_system::RawOrigin;
use sp_std::prelude::*;

benchmarks!{
	_ {
		let b in 1 .. 1000 => ();
	}

	do_something {
		let b in ...;
		let caller = account("caller", 0, 0);
	}: _ (RawOrigin::Signed(caller), b.into())
	verify {
		let value = Something::get();
		assert_eq!(value, b.into());
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{new_test_ext, Test};
	use frame_support::assert_ok;

	#[test]
	fn test_benchmarks() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_do_something::<Test>());
		});
	}
}
