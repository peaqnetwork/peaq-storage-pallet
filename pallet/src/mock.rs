use crate as peaq_storage;
use frame_support::{derive_impl, parameter_types};
use sp_io::TestExternalities;
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;
pub(crate) type Balance = u128;
pub(crate) const EXISTENTIAL_DEPOSIT: Balance = 2;

pub(crate) const DEPOSIT_BASE: Balance = 100;
pub(crate) const DEPOSIT_PER_BYTE: Balance = 2;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        PeaqStorage: peaq_storage,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
    // I made it bigger to avoid the error for data lenght test
    pub const BoundedDataLen: u32 = 2560;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountData = pallet_balances::AccountData<Balance>;
}

parameter_types! {
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
    type Balance = Balance;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const StorageDepositBase: Balance = DEPOSIT_BASE;
    pub const StorageDepositPerByte: Balance = DEPOSIT_PER_BYTE;
    pub const StorageReserveIdentifier: [u8; 8] = [b'p', b'e', b'a', b'q', b'_', b's', b't', b'o'];
}

impl peaq_storage::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = peaq_storage::weights::WeightInfo<Test>;
    type StorageDepositBase = StorageDepositBase;
    type StorageDepositPerByte = StorageDepositPerByte;
    type Currency = Balances;
    type BoundedDataLen = BoundedDataLen;
    type ReserveIdentifier = StorageReserveIdentifier;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    let balances: Vec<_> = (0..10)
        .map(|i| (i as u64, 1_400_000_000_000_000_000_000_000_000))
        .collect();

    pallet_balances::GenesisConfig::<Test> {
        balances,
        ..Default::default()
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    let mut ext = TestExternalities::from(storage);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
