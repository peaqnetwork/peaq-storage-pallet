#![cfg_attr(not(feature = "std"), no_std)]
// The `too_many_arguments` warning originates from `decl_runtime_apis` macro.
#![allow(clippy::too_many_arguments)]
// The `unnecessary_mut_passed` warning originates from `decl_runtime_apis` macro.
#![allow(clippy::unnecessary_mut_passed)]

use parity_scale_codec::Codec;
use sp_std::vec::Vec;
// use peaq_pallet_storage::structs::UserAttribute;

sp_api::decl_runtime_apis! {
    pub trait PeaqStorageApi<AccountId> where
        AccountId: Codec,
        {
            fn read(did_account: AccountId, item_type: Vec<u8>) -> Option<Vec<u8>>;
        }
}
