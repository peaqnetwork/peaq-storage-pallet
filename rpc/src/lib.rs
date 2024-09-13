use std::convert::From;
use std::sync::Arc;

use jsonrpsee::{
    core::{async_trait, RpcResult},
    proc_macros::rpc,
    types::{
        error::{INTERNAL_ERROR_CODE, INTERNAL_ERROR_MSG},
        ErrorObjectOwned,
    },
};

use parity_scale_codec::Codec;
use parity_scale_codec::{Decode, Encode};
pub use peaq_pallet_storage_runtime_api::PeaqStorageApi as PeaqStorageRuntimeApi;
use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_runtime::traits::Block as BlockT;

#[derive(Clone, Encode, Decode, Serialize, Deserialize)]
pub struct StorageRpcResult {
    pub item: Bytes,
}

impl From<Vec<u8>> for StorageRpcResult {
    fn from(item: Vec<u8>) -> Self {
        StorageRpcResult { item: item.into() }
    }
}
#[rpc(client, server)]
pub trait PeaqStorageApi<BlockHash, AccountId> {
    #[method(name = "peaqstorage_readAttribute")]
    fn read_attribute(
        &self,
        did_account: AccountId,
        item_type: Bytes,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<StorageRpcResult>>;
}

/// A struct that implements the [`PeaqStorageApi`].
pub struct PeaqStorage<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> PeaqStorage<C, B> {
    /// Create new `PeaqStorage` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        PeaqStorage {
            client,
            _marker: Default::default(),
        }
    }
}

pub enum Error {
    RuntimeError,
}

impl From<Error> for i32 {
    fn from(e: Error) -> i32 {
        match e {
            Error::RuntimeError => 1,
        }
    }
}

#[async_trait]
impl<C, Block, AccountId> PeaqStorageApiServer<<Block as BlockT>::Hash, AccountId>
    for PeaqStorage<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: PeaqStorageRuntimeApi<Block, AccountId>,
    AccountId: Codec,
{
    fn read_attribute(
        &self,
        did_account: AccountId,
        item_type: Bytes,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Option<StorageRpcResult>> {
        let api = self.client.runtime_api();
        let block_hash = at.unwrap_or_else(|| self.client.info().best_hash);
        api.read(block_hash, did_account, item_type.to_vec())
            .map(|o| o.map(StorageRpcResult::from))
            .map_err(|err| internal_err(err.to_string()))
    }
}

// This bit cribbed from frontier.
pub fn internal_err<T: ToString>(message: T) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(
        INTERNAL_ERROR_CODE,
        INTERNAL_ERROR_MSG,
        Some(message.to_string()),
    )
}
