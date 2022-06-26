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
pub async fn get_total_frozen(api: PolkadotRuntimeApi) -> Result<u128, Box<dyn Error>> {
    let mut iter = api.storage().system().account_iter(None).await?;

    let mut frozen = 0;
    while let Some((_, account)) = iter.next().await? {
        frozen += account.data.misc_frozen;
    }

    Ok(frozen)
}

#[tokio::test]
async fn should_get_total_frozen() -> Result<(), Box<dyn Error>> {
    with_default_client(|api| async move {
        assert_eq!(get_total_frozen(api).await?, 1000000000000, "Incorrect frozen amount!");
        Ok(())
    })
    .await
}
