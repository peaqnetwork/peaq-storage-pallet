use crate::enums::StorageError;
use sp_std::vec::Vec;

pub trait Storage<AccountId> {
    fn create(owner: &AccountId, item_type: &[u8], item: &[u8]) -> Result<(), StorageError>;
    fn update(owner: &AccountId, item_type: &[u8], item: &[u8]) -> Result<(), StorageError>;
    fn read(owner: &AccountId, item_type: &[u8]) -> Option<Vec<u8>>;
    fn get_hashed_key(account: &AccountId, item_type: &[u8]) -> [u8; 32];
}
