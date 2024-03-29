//! Benchmarking setup for peaq-pallet-storage

use super::*;
use crate::Pallet as STORAGE;
use frame_benchmarking::v1::{account, benchmarks};
use frame_system::{Pallet as System, RawOrigin};

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    System::<T>::assert_last_event(generic_event.into());
}

const CALLER_ACCOUNT_STR: &str = "Iredia1";
const ITEM_TYPE_BYTES: &[u8; 4] = b"test";
const ITEM_BYTES: &[u8; 9] = b"123456789";

benchmarks! {
    add_item {
        let caller: T::AccountId =  account(CALLER_ACCOUNT_STR,0, 0);

    }: _(RawOrigin::Signed(caller.clone()), ITEM_TYPE_BYTES.to_vec(), ITEM_BYTES.to_vec())
    verify {
        assert_last_event::<T>(Event::<T>::ItemAdded(
            caller.into(),
            ITEM_TYPE_BYTES.to_vec(),
            ITEM_BYTES.to_vec(),
        ).into());
    }
    update_item {
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);
        let new_item = b"987654321";

        <STORAGE<T>>::add_item(
            RawOrigin::Signed(caller.clone()).into(),
            ITEM_TYPE_BYTES.to_vec(),
            ITEM_BYTES.to_vec() )?;

    }: _(RawOrigin::Signed(caller.clone()), ITEM_TYPE_BYTES.to_vec(), new_item.to_vec())
    verify {
        assert_last_event::<T>(Event::<T>::ItemUpdated(
            caller.into(),
            ITEM_TYPE_BYTES.to_vec(),
            new_item.to_vec(),
        ).into());
    }

    get_item {
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);

        <STORAGE<T>>::add_item(
            RawOrigin::Signed(caller.clone()).into(),
            ITEM_TYPE_BYTES.to_vec(),
            ITEM_BYTES.to_vec() )?;

    }: _(RawOrigin::Signed(caller.clone()), ITEM_TYPE_BYTES.to_vec())
    verify {
        assert_last_event::<T>(Event::<T>::ItemRead (
            ITEM_BYTES.to_vec(),
        ).into());
    }

    impl_benchmark_test_suite!(
        STORAGE,
        crate::mock::new_test_ext(),
        crate::mock::Test);
}
