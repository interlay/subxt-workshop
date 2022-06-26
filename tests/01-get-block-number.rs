use std::error::Error;
use subxt::sp_core::H256;
use subxt_workshop::{with_default_client, PolkadotRuntimeApi};

/// # Exercise 01
///
/// Implement a function to fetch the block `number` from the `system` pallet.
///
/// ## Hint
///
/// ```
/// let value = api
///     .storage()
///     .pallet_name()
///     .storage_item_name(Some(block_hash))
///     .await?;
/// ```
pub async fn get_block_number(api: PolkadotRuntimeApi, block_hash: H256) -> Result<u32, Box<dyn Error>> {
    Ok(api.storage().system().number(Some(block_hash)).await?)
}

#[tokio::test]
async fn should_get_block_number() -> Result<(), Box<dyn Error>> {
    with_default_client(|api| async move {
        // we need to know the block number up-front, so read storage
        // at a specific block - in this case 2
        let block_hash = api.client.rpc().block_hash(Some(2u32.into())).await?.unwrap();
        assert_eq!(
            get_block_number(api, block_hash).await?,
            2,
            "Height must be equal to 2!"
        );
        Ok(())
    })
    .await
}
