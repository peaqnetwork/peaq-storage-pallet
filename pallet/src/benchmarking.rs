//! Benchmarking setup for peaq-pallet-storage

use super::*;
use crate::Pallet as STORAGE;
use frame_benchmarking::v1::{account, benchmarks};
use frame_system::{Pallet as System, RawOrigin};
use sp_runtime::BoundedVec;
use sp_std::vec;
use frame_support::traits::Currency;
use sp_runtime::traits::Bounded;

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    System::<T>::assert_last_event(generic_event.into());
}

const CALLER_ACCOUNT_STR: &str = "Iredia1";

benchmarks! {
    add_item {
        let caller: T::AccountId =  account(CALLER_ACCOUNT_STR,0, 0);
        let _ = <T as Config>::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
        let item_type = BoundedVec::try_from(vec![0; 64]).unwrap();
        let item = BoundedVec::try_from(vec![0; 256]).unwrap();

    }: _(RawOrigin::Signed(caller.clone()), item_type.clone(), item.clone())
    verify {
        assert_last_event::<T>(Event::<T>::ItemAdded(
            caller.into(),
            item_type,
            item,
        ).into());
    }
    update_item {
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);
        let item_type = BoundedVec::try_from(vec![0; 64]).unwrap();
        let new_item = BoundedVec::try_from(vec![1; 256]).unwrap();
        let _ = <T as Config>::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

        <STORAGE<T>>::add_item(
            RawOrigin::Signed(caller.clone()).into(),
            item_type.clone(),
            BoundedVec::try_from(vec![0;256]).unwrap())?;

    }: _(RawOrigin::Signed(caller.clone()), item_type.clone(), new_item.clone())
    verify {
        assert_last_event::<T>(Event::<T>::ItemUpdated(
            caller.into(),
            item_type,
            new_item,
        ).into());
    }

    get_item {
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);
        let item_type = BoundedVec::try_from(vec![0; 64]).unwrap();
        let item = BoundedVec::try_from(vec![1; 256]).unwrap();
        let _ = <T as Config>::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

        <STORAGE<T>>::add_item(
            RawOrigin::Signed(caller.clone()).into(),
            item_type.clone(),
            item.clone())?;

    }: _(RawOrigin::Signed(caller.clone()), item_type)
    verify {
        assert_last_event::<T>(Event::<T>::ItemRead (
            item,
        ).into());
    }

    remove_item {
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);
        let item_type = BoundedVec::try_from(vec![0; 64]).unwrap();
        let _ = <T as Config>::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

        <STORAGE<T>>::add_item(
            RawOrigin::Signed(caller.clone()).into(),
            item_type.clone(),
            BoundedVec::try_from(vec![0;256]).unwrap())?;

    }: _(RawOrigin::Signed(caller.clone()), item_type.clone())
    verify {
        assert_last_event::<T>(Event::<T>::ItemRemoved(
            caller.into(),
            item_type,
        ).into());
    }

    impl_benchmark_test_suite!(
        STORAGE,
        crate::mock::new_test_ext(),
        crate::mock::Test);
}
