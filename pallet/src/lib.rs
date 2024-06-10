//! # PEAQ Storage Pallet
//!
//! The Storage pallet allows storing and managing IPFS CID ( content identifiers ) on the blockchain.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod enums;
pub mod traits;

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

pub mod weightinfo;
pub mod weights;
pub use weightinfo::WeightInfo;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use super::WeightInfo;
    use crate::enums::StorageError;
    use crate::traits::*;
    use frame_support::{
        pallet_prelude::{ValueQuery, *},
        traits::{Currency, NamedReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_256;
    use sp_runtime::traits::Saturating;
    use sp_runtime::BoundedVec;
    use sp_std::vec::Vec;

    pub(super) const MAX_ITEM_SIZE: usize = 256;
    pub(super) const MAX_ITEM_TYPE_SIZE: usize = 64;
    pub(super) const MAX_STORAGE_ITEM_SIZE: usize = MAX_ITEM_SIZE + MAX_ITEM_TYPE_SIZE;

    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
    pub type ReserveIdentifierOf<T> = <<T as Config>::Currency as NamedReservableCurrency<
        <T as frame_system::Config>::AccountId,
    >>::ReserveIdentifier;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
        #[pallet::constant]
        type BoundedDataLen: Get<u32>;
        /// Deposit amount for utilising storage
        #[pallet::constant]
        type StorageDepositBase: Get<BalanceOf<Self>>;
        /// Deposit amount per byte
        #[pallet::constant]
        type StorageDepositPerByte: Get<BalanceOf<Self>>;
        /// Currency Type
        type Currency: NamedReservableCurrency<Self::AccountId>;
        /// Reserve identifier
        #[pallet::constant]
        type ReserveIdentifier: Get<ReserveIdentifierOf<Self>>;
    }

    // Pallets use events to inform users when important changes are made.
    // Event documentation should end with an array that provides descriptive names for parameters.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a storage item has been added. [who, item_type, item]
        ItemAdded(
            T::AccountId,
            BoundedVec<u8, T::BoundedDataLen>,
            BoundedVec<u8, T::BoundedDataLen>,
        ),
        /// Event emitted when an item is read successfully
        ItemRead(BoundedVec<u8, T::BoundedDataLen>),
        /// Event emitted when an item has been updated. [who, item_type, item]
        ItemUpdated(
            T::AccountId,
            BoundedVec<u8, T::BoundedDataLen>,
            BoundedVec<u8, T::BoundedDataLen>,
        ),
        ItemRemoved(T::AccountId, BoundedVec<u8, T::BoundedDataLen>),
    }

    #[pallet::error]
    pub enum Error<T> {
        // Item not found with the given account and item_type
        ItemNotFound,

        // Item already exists with the given account and item_type
        ItemTypeAlreadyExists,

        // Item type is greater than 64
        ItemTypeExceedMax64,

        // Item is greater than 128
        ItemExceedMax128,

        // Item is greater than 256
        ItemExceedMax256,

        // Cannot convert
        ItemIternalError,
    }

    impl<T: Config> Error<T> {
        fn dispatch_error(err: StorageError) -> DispatchResult {
            match err {
                StorageError::NotFound => Err(Error::<T>::ItemNotFound.into()),
                StorageError::AlreadyExists => Err(Error::<T>::ItemTypeAlreadyExists.into()),
                StorageError::InternalError => Err(Error::<T>::ItemIternalError.into()),
            }
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn item_of)]
    pub(super) type ItemStore<T: Config> =
        StorageMap<_, Blake2_128Concat, [u8; 32], BoundedVec<u8, T::BoundedDataLen>, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a new item to the storage
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::add_item())]
        pub fn add_item(
            origin: OriginFor<T>,
            item_type: BoundedVec<u8, T::BoundedDataLen>,
            item: BoundedVec<u8, T::BoundedDataLen>,
        ) -> DispatchResult {
            // Check that an extrinsic was signed and get the signer
            // This fn returns an error if the extrinsic is not signed
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;

            ensure!(
                item_type.len() <= MAX_ITEM_TYPE_SIZE,
                Error::<T>::ItemTypeExceedMax64
            );
            ensure!(item.len() <= MAX_ITEM_SIZE, Error::<T>::ItemExceedMax256);

            T::Currency::reserve_named(
                &T::ReserveIdentifier::get(),
                &sender,
                Self::deposit_amount(),
            )?;

            match Self::create(&sender, &item_type, item.as_slice()) {
                Ok(()) => {
                    Self::deposit_event(Event::ItemAdded(sender.clone(), item_type, item));
                }
                Err(e) => return Error::<T>::dispatch_error(e),
            };

            Ok(())
        }

        /// Update an existing item in the storage
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::update_item())]
        pub fn update_item(
            origin: OriginFor<T>,
            item_type: BoundedVec<u8, T::BoundedDataLen>,
            item: BoundedVec<u8, T::BoundedDataLen>,
        ) -> DispatchResult {
            // Check that an extrinsic was signed and get the signer
            // This fn returns an error if the extrinsic is not signed
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;

            // Verify that the item len is 256 max
            ensure!(item.len() <= MAX_ITEM_SIZE, Error::<T>::ItemExceedMax256);

            match Self::update(&sender.clone(), &item_type, &item) {
                Ok(()) => {
                    Self::deposit_event(Event::ItemUpdated(sender.clone(), item_type, item));
                }
                Err(e) => return Error::<T>::dispatch_error(e),
            };
            Ok(())
        }

        /// Read storage item
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::get_item())]
        pub fn get_item(
            origin: OriginFor<T>,
            item_type: BoundedVec<u8, T::BoundedDataLen>,
        ) -> DispatchResult {
            // Check that an extrinsic was signed and get the signer
            // This fn returns an error if the extrinsic is not signed
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;

            let item = Self::read(&sender, &item_type);
            match item {
                Some(value) => {
                    Self::deposit_event(Event::ItemRead(BoundedVec::try_from(value).unwrap()));
                }
                None => return Err(Error::<T>::ItemNotFound.into()),
            }
            Ok(())
        }

        /// Read storage item
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::remove_item())]
        pub fn remove_item(
            origin: OriginFor<T>,
            item_type: BoundedVec<u8, T::BoundedDataLen>,
        ) -> DispatchResult {
            // Check that an extrinsic was signed and get the signer
            // This fn returns an error if the extrinsic is not signed
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;

            T::Currency::unreserve_named(
                &T::ReserveIdentifier::get(),
                &sender,
                Self::deposit_amount(),
            );

            match Self::remove(&sender, &item_type) {
                Ok(()) => {
                    Self::deposit_event(Event::ItemRemoved(sender.clone(), item_type));
                }
                Err(e) => return Error::<T>::dispatch_error(e),
            }
            Ok(())
        }
    }

    // implements the Storage trait to satisfied the required methods
    impl<T: Config> Storage<T::AccountId> for Pallet<T> {
        // Add new item of specific type
        fn create(owner: &T::AccountId, item_type: &[u8], item: &[u8]) -> Result<(), StorageError> {
            // Generate id for integrity check
            let id = Self::get_hashed_key(owner, item_type);

            // Check if item already exists with the given account and item_type
            if <ItemStore<T>>::contains_key(id) {
                return Err(StorageError::AlreadyExists);
            }

            let item =
                BoundedVec::try_from(item.to_vec()).map_err(|_| StorageError::InternalError)?;
            <ItemStore<T>>::insert(id, item);

            Ok(())
        }

        // Update existing item of specific type
        fn update(owner: &T::AccountId, item_type: &[u8], item: &[u8]) -> Result<(), StorageError> {
            let id = Self::get_hashed_key(owner, item_type);

            // Check if item exists with the given account and item_type
            if !<ItemStore<T>>::contains_key(id) {
                return Err(StorageError::NotFound);
            }

            let item =
                BoundedVec::try_from(item.to_vec()).map_err(|_| StorageError::InternalError)?;
            <ItemStore<T>>::mutate(id, |a| *a = item);
            Ok(())
        }

        // Fetch an item of specific type
        fn read(owner: &T::AccountId, item_type: &[u8]) -> Option<Vec<u8>> {
            let id = Self::get_hashed_key(owner, item_type);

            if <ItemStore<T>>::contains_key(id) {
                return Some(Self::item_of(id).to_vec());
            }
            None
        }

        fn remove(owner: &T::AccountId, item_type: &[u8]) -> Result<(), StorageError> {
            let id = Self::get_hashed_key(owner, item_type);

            // Check if item exists with the given account and item_type
            if !<ItemStore<T>>::contains_key(id) {
                return Err(StorageError::NotFound);
            }

            <ItemStore<T>>::remove(id);
            Ok(())
        }

        fn get_hashed_key(account: &T::AccountId, value: &[u8]) -> [u8; 32] {
            let mut bytes_in_value: Vec<u8> = value.to_vec();
            let mut bytes_to_hash: Vec<u8> = account.encode().as_slice().to_vec();
            bytes_to_hash.append(&mut bytes_in_value);
            blake2_256(&bytes_to_hash[..])
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn deposit_amount() -> BalanceOf<T> {
            let mut deposit = T::StorageDepositPerByte::get()
                .saturating_mul(BalanceOf::<T>::from(MAX_STORAGE_ITEM_SIZE as u32));
            deposit.saturating_accrue(T::StorageDepositBase::get());
            deposit
        }
    }
}
