//! Benchmarking setup for storage pallet

use super::*;

#[allow(unused)]
use crate::Pallet as STORAGE;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, account};
use frame_system::{Pallet as System, RawOrigin};
use crate::structs::Attribute;

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    System::<T>::assert_last_event(generic_event.into());
}

const CALLER_ACCOUNT_STR: &str = "Iredia1";
const ITEM_TYPE_BYTES: &[u8; 2] = b"test";
const ITEM_BYTES: &[u8; 17] = b"123456789";

benchmarks! {

    add_item {
        let caller: T::AccountId =  account(CALLER_ACCOUNT_STR,0, 0);
        
    }: _(RawOrigin::Signed)(caller.clone(), ITEM_TYPE_BYTES.to_vec(), ITEM_BYTES.to_vec())
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

        <STORAGE><T>>::add_item(
            RawOrigin::Signed(caller.clone()).into(),
            ITEM_TYPE_BYTES.to_vec(),
            ITEM_BYTES.to_vec() )?;             
        
    }: _(RawOrigin::Signed(caller.clone()), ITEM_TYPE_BYTES.to_vec(), new_attribute.to_vec())
    verify {
        assert_last_event::<T>(Event::<T>::ItemUpdated(
            caller.into(),
            ITEM_TYPE_BYTES.to_vec(),
            new_item.to_vec(), 
        ).into());
    }

    get_item {
        let caller : T::AccountId = account(CALLER_ACCOUNT_STR, 0, 0);        
        
        <STORAGE><T>>::add_item(
            RawOrigin::Signed(caller.clone()).into(),
            ITEM_TYPE_BYTES.to_vec(),
            ITEM_BYTES.to_vec() )?;             
        
    }: _(RawOrigin::Signed(caller.clone()), ITEM_TYPE_BYTES.to_vec())
    verify {
        assert_last_event::<T>(Event::<T>::ItemRead (
            caller.into(),
            ITEM_TYPE_BYTES.to_vec(),            
        ).into());
    }   
    

    
}          

#[cfg(test)]
mod tests {
    use crate::mock;
    use frame_support::sp_io::TestExternalities;

    pub fn new_test_ext() -> TestExternalities {
        mock::ExternalityBuilder::build()
    }
}

impl_benchmark_test_suite!(
    STORAGE,
    crate::benchmarking::tests::new_test_ext(),
    crate::mock::TestRuntime,
);
