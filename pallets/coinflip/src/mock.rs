// Creating mock runtime here

pub use crate::{Module, Trait};
use sp_core::H256;
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use frame_support::traits::Randomness;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};

impl_outer_origin! {
	pub enum Origin for Test {}
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
impl frame_system::Trait for Test {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl balances::Trait for Test {
	type Balance = u64;
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
}

impl Randomness<<Test as frame_system::Trait>::Hash> for CoinFlipModule {
	fn random(_subject: &[u8]) -> <Test as frame_system::Trait>::Hash {
		match Self::nonce()  {
			0 => H256::from_low_u64_be(101),
			1 => H256::from_low_u64_be(150),
			_ => panic!("do not use other value except 1,2 for nonce in test cases"),
		}
	}
}

impl Trait for Test {
	type Randomness = CoinFlipModule;
	type Event = ();
}

pub type System = frame_system::Module<Test>;
pub type Balances = balances::Module<Test>;
pub type CoinFlipModule = Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	balances::GenesisConfig::<Test> {
		balances: vec![
			(1, 10),
			(2, 20),
		]
	}.assimilate_storage(&mut t).unwrap();
	
	t.into()
}
