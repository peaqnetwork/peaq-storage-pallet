// SBP M3 Review: Add copyrights

// SBP M3 Review: Pallet documentation should be more descriptive. For example: You can add `Overview`, `Goals`, information about dispatchable functions etc
// You can refer any Substrate pallet

//! # PEAQ Storage Pallet
//!
//! The Storage pallet allows storing and managing IPFS CID ( content identifiers ) on the blockchain.

#![cfg_attr(not(feature = "std"), no_std)]

// SBP M3 Review: Please run `cargo fmt` to improve formatting, remove extra spaces/lines

// SBP M3 Review: File names are very generic. They should be very specific.
pub mod structs;
pub mod enums;
pub mod traits;

#[cfg(test)]
// SBP M3 Review: Please run `cargo fmt`
 mod mock;

#[cfg(feature = "runtime-benchmarks")]
// SBP M3 Review: Please run `cargo fmt`
  mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    // SBP M3 Review: Better to use `use super::*;`
    use super::WeightInfo;
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
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    // SBP M3 Review: Remove below description
    // Pallets use events to inform users when important changes are made.
    // Event documentation should end with an array that provides descriptive names for parameters.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // SBP M3 Review: It should be something like 'A storage item has been added. [who, item_type, item]'
        /// Event emitted when an storage item has been added. [who, item_type, item]
        ItemAdded(
            T::AccountId,
            Vec<u8>,
            Vec<u8>,
        ),
        // SBP M3 Review: This is not needed. Please read review comment of `get_item`
        /// Event emitted when an item is read successfully
        ItemRead(
            Vec<u8>,
        ),
        // SBP M3 Review: It should be something like 'An item has been updated. [who, item_type, item]'
        /// Event emitted when an item has been updated. [who, item_type, item]
        ItemUpdated(
            T::AccountId,
            Vec<u8>,
            Vec<u8>,
        ),
    }

    #[pallet::error]
    pub enum Error<T> {
        // SBP M3 Review: Outer line doc should start with 3 slashes
        // Item not found with the given account and item_type 
        ItemNotFound,

        // SBP M3 Review: Outer line doc should start with 3 slashes
        // Item already exists with the given account and item_type
        ItemTypeAlreadyExists,

        // SBP M3 Review: Outer line doc should start with 3 slashes
        // Item type is greater that 64
        ItemTypeExceedMax64,

        // SBP M3 Review: Outer line doc should start with 3 slashes
        // Item is greater that 128
        ItemExceedMax128,
    }

    // SBP M3 Review: This is not needed
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
    // SBP M3 Review: function name should be 'item_store'.
    // It should follow substrate storage naming convention. Please read https://docs.substrate.io/build/runtime-storage/#declaring-storage-items
    #[pallet::getter(fn item_of)]
    pub(super) type ItemStore<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        [u8; 32],
        // SBP M3 Review: Please use BoundedVec(https://crates.parity.io/frame_support/storage/bounded_vec/struct.BoundedVec.html) rather than Vec<_>.
        // For example: here you can use 'BoundedVec<u8, Configurable length>'
        // Take this https://github.com/paritytech/substrate/blob/6e73c85b7ddbc2ec2a9f6629ddc06aca2f83bcf3/frame/assets/src/lib.rs#L334 as an example
        Vec<u8>,
        ValueQuery,
    >;

    // SBP M3 Review: Why this is needed?
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // SBP M3 Review: Remove below description
    // Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a new item to the storage 
        #[pallet::weight(T::WeightInfo::add_item())]
        pub fn add_item(
            origin: OriginFor<T>,
            // SBP M3 Review: Its a good practice to put both variable in a struct
            item_type: Vec<u8>,
            item: Vec<u8>,
        ) -> DispatchResult {
            // SBP M3 Review: This description is not needed
            // Check that an extrinsic was signed and get the signer
            // This fn returns an error if the extrinsic is not signed
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;

            // SBP M3 Review: If you have bounded vec in storage,
            // you can use below pattern for validation and storage. This will be more clean and substarte way.
            // let item: BoundedVec<u8, ConstU32<128>> =
            //     item.try_into().map_err(|_| Error::<T>::ItemExceedMax128)?;
            //
            // let item_type: BoundedVec<u8, ConstU32<64>> =
            //     item_type.try_into().map_err(|_| Error::<T>::ItemTypeExceedMax64)?;
            //
            // let id = Self::get_hashed_key(&sender, item_type.to_vec());
            //
            // // Check if item already exists with the given account and item_type
            // ensure!(!<ItemStore<T>>::contains_key(&id), Error::<T>::ItemTypeAlreadyExists);
            //
            // <ItemStore<T>>::insert(&id, item.clone());
            //
            // Self::deposit_event(Event::ItemAdded(
            //     sender,
            //     item_type.to_vec(),
            //     item.to_vec()
            // ));

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

        // SBP M3 Review: `add_item` feedback also applies here. Please refactor accordingly
        /// Update an existing item in the storage
        #[pallet::weight(T::WeightInfo::update_item())]        
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

        // SBP M3 Review: It is strongly not recommended to have dispatchable function to
        // read item from storage. We can either use getter function or expose RPC end point
        // Read storage item.
        // For example: If we have a storage map: 
	       // ```
        // #[pallet::storage]
	       // #[pallet::getter(fn product_information)]
	       // pub type ProductInformation<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Product<T::AccountId, T::Hash>>;
	       // ```
	       // We can read the item by caling `Self::product_information(id)`. 
 	      //      
        // Please refer this https://docs.substrate.io/build/runtime-storage/#declaring-storage-items
        // to get better understanding of declaring storage items in Substarte pallet.
        #[pallet::weight(T::WeightInfo::get_item())]
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

    // SBP M3 Review: After addressing above review comments, we won't need these methods.
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

        // SBP M3 Review: This is more like a utility function and can be moved in primitives or common util class
        fn get_hashed_key(account: &T::AccountId, value: &[u8]) -> [u8; 32] {
            let mut bytes_in_value: Vec<u8> = value.to_vec();
            let mut bytes_to_hash: Vec<u8> = account.encode().as_slice().to_vec();
            bytes_to_hash.append(&mut bytes_in_value);
            blake2_256(&bytes_to_hash[..])
        }
    }
}
