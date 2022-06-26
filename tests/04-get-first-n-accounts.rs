use std::error::Error;
use subxt::{codec::Decode, storage::StorageKeyPrefix};
use subxt_workshop::{polkadot, with_default_client, PolkadotRuntimeApi};

pub type AccountData = polkadot::runtime_types::pallet_balances::AccountData<u128>;
pub type AccountInfo = polkadot::runtime_types::frame_system::AccountInfo<u32, AccountData>;

/// # Exercise 04
///
/// Implement a function to page `n` `account`s in the `system` pallet.
///
/// ## Hint
///
/// ```
/// let prefix = StorageKeyPrefix::new::<polkadot::pallet_name::storage::StorageName>();
/// let keys = api.client.rpc().storage_keys_paged(Some(prefix.to_storage_key()), n, None, None).await?;
/// for k in keys {
///      let storage_data = api.client.storage().fetch_raw(k, None).await?;
///      let value = Value::decode(&mut &storage_data.0[..])?;
///      ...
/// }
/// ```
pub async fn get_first_n_accounts(_api: PolkadotRuntimeApi, _n: u32) -> Result<Vec<AccountInfo>, Box<dyn Error>> {
    Ok(Default::default())
}

#[tokio::test]
async fn should_get_first_n_accounts() -> Result<(), Box<dyn Error>> {
    with_default_client(|api| async move {
        assert_eq!(
            get_first_n_accounts(api, 2).await?,
            vec![
                AccountInfo {
                    nonce: 0,
                    consumers: 0,
                    providers: 1,
                    sufficients: 0,
                    data: AccountData {
                        free: 10000000000000000,
                        reserved: 0,
                        misc_frozen: 0,
                        fee_frozen: 0
                    }
                },
                AccountInfo {
                    nonce: 0,
                    consumers: 0,
                    providers: 1,
                    sufficients: 0,
                    data: AccountData {
                        free: 10000000000000000,
                        reserved: 0,
                        misc_frozen: 0,
                        fee_frozen: 0
                    }
                },
            ],
            "Accounts do not match!"
        );
        Ok(())
    })
    .await
}
