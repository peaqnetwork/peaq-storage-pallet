use crate as peaq_storage;
use frame_support::parameter_types;
use sp_core::{sr25519, Pair, H256};
use sp_io::TestExternalities;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub(crate) type Balance = u128;
pub(crate) type AccountId = sr25519::Public;
pub(crate) const EXISTENTIAL_DEPOSIT: Balance = 2;
pub(crate) type BlockNumber = u64;

pub(crate) const DEPOSIT_BASE: Balance = 100;
pub(crate) const DEPOSIT_PER_BYTE: Balance = 2;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        PeaqStorage: peaq_storage::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
    // I made it bigger to avoid the error for data lenght test
    pub const BoundedDataLen: u32 = 2560;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type RuntimeOrigin = RuntimeOrigin;
    type Index = u64;
    type RuntimeCall = RuntimeCall;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const MaxLocks: u32 = 4;
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxHolds = ();
    type HoldIdentifier = ();
    type MaxFreezes = ();
}
parameter_types! {
    pub const MinimumPeriod: u64 = 5;
    pub const StorageDepositBase: Balance = DEPOSIT_BASE;
    pub const StorageDepositPerByte: Balance = DEPOSIT_PER_BYTE;
}

impl peaq_storage::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = peaq_storage::weights::WeightInfo<Test>;
    type StorageDepositBase = StorageDepositBase;
    type StorageDepositPerByte = StorageDepositPerByte;
    type Currency = Balances;
    type BoundedDataLen = BoundedDataLen;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    // This will cause some initial issuance
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (account_key("Iredia1"), 1_400_000_000_000_000_000_000_000_000),
            (account_key("Iredia2"), 1_400_000_000_000_000_000_000_000_000),
            (account_key("Iredia3"), 1_400_000_000_000_000_000_000_000_000),
        ],
    }
    .assimilate_storage(&mut storage)
    .ok();

    let mut ext = TestExternalities::from(storage);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

pub fn account_key(s: &str) -> sr25519::Public {
    sr25519::Pair::from_string(&format!("//{}", s), None)
        .expect("static values are valid; qed")
        .public()
}
