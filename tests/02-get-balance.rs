use sp_keyring::AccountKeyring;
use std::error::Error;
use subxt::sp_runtime::AccountId32;
use subxt_workshop::{with_default_client, PolkadotRuntimeApi};

/// # Exercise 02
///
/// Implement a function to fetch the `.data.free` balance of an `account` from the `system` pallet.
///
/// ## Hint
///
/// ```
/// let value = api
///     .storage()
///     .pallet_name()
///     .storage_item_name(..., None)
///     .await?;
/// ```
pub async fn get_balance(_api: PolkadotRuntimeApi, _account: AccountId32) -> Result<u128, Box<dyn Error>> {
    Ok(Default::default())
}

#[tokio::test]
async fn should_get_balance() -> Result<(), Box<dyn Error>> {
    with_default_client(|api| async move {
        // uses the dave account so as not to interfere with later tests
        assert_eq!(
            get_balance(api, AccountKeyring::Dave.to_account_id()).await?,
            10000000000000000,
            "Dave does not have sufficient funds!"
        );
        Ok(())
    })
    .await
}
