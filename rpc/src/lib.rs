use std::convert::From;
use std::sync::Arc;

use codec::Codec;
use jsonrpsee::{
    core::{async_trait, Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
    types::error::{CallError, ErrorObject},
};
pub use peaq_pallet_storage_runtime_api::PeaqStorageApi as PeaqStorageRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

#[rpc(client, server)]
pub trait PeaqStorageApi<BlockHash, AccountId, BlockNumber, Moment> {
    #[method(name = "peaqstorage_readAttribute")]
    fn read_attribute(
        &self,
        did_account: AccountId,
        item_type: Vec<u8>,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<Vec<u8>>>;
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
impl<C, Block, AccountId, BlockNumber, Moment>
    PeaqStorageApiServer<<Block as BlockT>::Hash, AccountId, BlockNumber, Moment>
    for PeaqStorage<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: PeaqStorageRuntimeApi<Block, AccountId>,
    AccountId: Codec,
    BlockNumber: Codec,
    Moment: Codec,
{
    fn read_attribute(
        &self,
        did_account: AccountId,
        item_type: Vec<u8>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Option<Vec<u8>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or(
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash,
        ));
        api.read(&at, did_account, item_type)
            .map(|o| o.map(|item| item))
            .map_err(|e| {
                JsonRpseeError::Call(CallError::Custom(ErrorObject::owned(
                    Error::RuntimeError.into(),
                    "Unable to get value.",
                    Some(format!("{:?}", e)),
                )))
            })
    }
}
