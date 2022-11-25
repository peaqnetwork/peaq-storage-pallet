//! # PEAQ Storage Pallet
//!
//! The Storage pallet allows storing and managing IPFS CID ( content identifiers ) on the blockchain.

#![cfg_attr(not(feature = "std"), no_std)]


pub mod structs;
pub mod enums;
pub mod traits;

#[cfg(test)]
 mod mock;

#[cfg(feature = "runtime-benchmarks")]
  mod benchmarking;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::enums::StorageError;
    use crate::traits::*;
    use frame_support::pallet_prelude::{*, ValueQuery};
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_256;
    use sp_std::vec::Vec;
    
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // Pallets use events to inform users when important changes are made.
    // Event documentation should end with an array that provides descriptive names for parameters.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when an storage item has been added. [who, item_type, item]
        ItemAdded(
            T::AccountId,
            Vec<u8>,
            Vec<u8>,
        ),
        /// Event emitted when an item is read successfully
        ItemRead(
            Vec<u8>,
        ),
        /// Event emitted when an item has been updated. [who, item_type, item]
        ItemUpdated(
            T::AccountId,
            Vec<u8>,
            Vec<u8>,
        ),
    }

    #[pallet::error]
    pub enum Error<T> {
        // Item not found with the given account and item_type 
        ItemNotFound,

        // Item already exists with the given account and item_type
        ItemTypeAlreadyExists,

        // Item type is greater that 64
        ItemTypeExceedMax64,

        // Item is greater that 128
        ItemExceedMax128,
    }

    impl<T: Config> Error<T> {
        fn dispatch_error(err: StorageError) -> DispatchResult {
            match err {
                StorageError::NotFound => return Err(Error::<T>::ItemNotFound.into()),
                StorageError::AlreadyExists => return Err(Error::<T>::ItemTypeAlreadyExists.into()),
            }
        }
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn item_of)]
    pub(super) type ItemStore<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        [u8; 32],
        Vec<u8>,
        ValueQuery,
    >;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a new item to the storage 
        #[pallet::weight(1_000)]
        pub fn add_item(
            origin: OriginFor<T>,
            item_type: Vec<u8>,
            item: Vec<u8>,
        ) -> DispatchResult {
            // Check that an extrinsic was signed and get the signer
            // This fn returns an error if the extrinsic is not signed
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;

            ensure!(item_type.len() <= 64, Error::<T>::ItemTypeExceedMax64);
            ensure!(item.len() <= 128, Error::<T>::ItemExceedMax128);

            match Self::create(&sender, &item_type, &item) {
                Ok(()) => {
                    Self::deposit_event(Event::ItemAdded(
                        sender,
                        item_type,
                        item
                    ));
                }
                Err(e) => return Error::<T>::dispatch_error(e),
            };

            Ok(())
        }

        /// Update an existing item in the storage
        #[pallet::weight(1_000)]
        pub fn update_item(
            origin: OriginFor<T>,
            item_type: Vec<u8>,
            item: Vec<u8>,
        ) -> DispatchResult {
            // Check that an extrinsic was signed and get the signer
            // This fn returns an error if the extrinsic is not signed
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;

            // Verify that the item len is 128 max
            ensure!(item.len() <= 128, Error::<T>::ItemExceedMax128);

            match Self::update(&sender, &item_type, &item) {
                Ok(()) => {
                    Self::deposit_event(Event::ItemUpdated(
                        sender,
                        item_type,
                        item
                    ));
                }
                Err(e) => return Error::<T>::dispatch_error(e),
            };
            Ok(())
        }

        /// Read storage item
        #[pallet::weight(1_000)]
        pub fn get_item(
            origin: OriginFor<T>,
            item_type: Vec<u8>
        ) -> DispatchResult {
            // Check that an extrinsic was signed and get the signer
            // This fn returns an error if the extrinsic is not signed
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;

            let item = Self::read(&sender, &item_type);
            match item {
                Some(value) => {
                    Self::deposit_event(Event::ItemRead(value));
                }
                None => return Err(Error::<T>::ItemNotFound.into()),
            }
            Ok(())
        }
    }

    // implements the Storage trait to satisfied the required methods
    impl<T: Config> Storage<T::AccountId> 
    for Pallet<T>
    {

        // Add new item of specific type
        fn create(
            owner: &T::AccountId,
            item_type: &[u8],
            item: &[u8],
        ) -> Result<(), StorageError> {
            // Generate id for integrity check
            let id = Self::get_hashed_key(&owner, &item_type);

            // Check if item already exists with the given account and item_type
            if <ItemStore<T>>::contains_key(&id) {
                return Err(StorageError::AlreadyExists);
            }

            <ItemStore<T>>::insert(&id, item);

            Ok(())
        }

        // Update existing item of specific type
        fn update(
            owner: &T::AccountId,
            item_type: &[u8],
            item: &[u8],
        ) -> Result<(), StorageError> {

            let id = Self::get_hashed_key(&owner, &item_type);

            // Check if item exists with the given account and item_type
            if !<ItemStore<T>>::contains_key(id) {
                return Err(StorageError::NotFound);
            }

            <ItemStore<T>>::mutate(&id, |a| *a = item.to_vec());
            Ok(())
        }

        // Fetch an item of specific type
        fn read(
            owner: &T::AccountId,
            item_type: &[u8],
        ) -> Option<Vec<u8>>
        {
            let id = Self::get_hashed_key(&owner, &item_type);

            if <ItemStore<T>>::contains_key(&id) {
                return Some(Self::item_of(&id));
            }
            None
        }

        fn get_hashed_key(account: &T::AccountId, value: &[u8]) -> [u8; 32] {
            let mut bytes_in_value: Vec<u8> = value.to_vec();
            let mut bytes_to_hash: Vec<u8> = account.encode().as_slice().to_vec();
            bytes_to_hash.append(&mut bytes_in_value);
            blake2_256(&bytes_to_hash[..])
        }
    }
}
