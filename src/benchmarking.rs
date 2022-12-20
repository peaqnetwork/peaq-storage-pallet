//! Benchmarking setup for peaq-pallet-storage

use super::*;
use crate::Pallet as STORAGE;
use frame_benchmarking::{benchmarks, account};
use frame_system::{Pallet as System, RawOrigin};


/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    System::<T>::assert_last_event(generic_event.into());
}

const CALLER_ACCOUNT_STR: &str = "Iredia1";
const ITEM_TYPE_BYTES: &[u8; 4] = b"test";
const ITEM_BYTES: &[u8; 9] = b"123456789";

// SBP M3 Review: For benchmarking, we should try to cover worst case scenario
// For example: Here we can use maximum length of string for item type and item

benchmarks! {
    add_item {
        // SBP M3 Review: Let's use Substrate way of benchmarking account id
        // let caller: T::AccountId = whitelisted_caller();
        // please see https://github.com/paritytech/substrate/blob/ea3ca3f757ff9d9559665719a77da81f4cf0f0ce/bin/node-template/pallets/template/src/benchmarking.rs#L13
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