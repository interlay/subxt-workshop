use std::error::Error;
use subxt_workshop::{with_default_client, PolkadotRuntimeApi};

/// # Exercise 03
///
/// Implement a function to fetch all `account`s in the `system` pallet and sum the `frozen` amounts.
///
/// ## Hint
///
/// ```
/// let mut iter = api
///     .storage()
///     .pallet_name()
///     .storage_item_name_iter(None)
///     .await?;
/// while let Some(_) = iter.next().await? {
///     ...
/// }
/// ```
pub async fn get_total_frozen(_api: PolkadotRuntimeApi) -> Result<u128, Box<dyn Error>> {
    Ok(Default::default())
}

#[tokio::test]
async fn should_get_total_frozen() -> Result<(), Box<dyn Error>> {
    with_default_client(|api| async move {
        assert_eq!(get_total_frozen(api).await?, 1000000000000, "Incorrect frozen amount!");
        Ok(())
    })
    .await
}
