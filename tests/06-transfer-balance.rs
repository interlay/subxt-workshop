use sp_keyring::AccountKeyring;
use std::error::Error;
use subxt::{
    sp_core::sr25519::Pair,
    sp_runtime::{AccountId32, MultiAddress},
    DefaultConfig, PairSigner,
};
use subxt_workshop::{with_default_client, PolkadotRuntimeApi};

/// # Exercise 06 (A)
///
/// Implement a function to `transfer` an `amount` from the `signer` to the `dest` using the `balances` pallet.
///
/// Tip: `wait_for_finalized_success` will make sure the transaction is included.
///
/// ## Hint
///
/// ```
/// let events = api
///      .tx()
///      .pallet_name()
///      .call_item_name(...)
///      .sign_and_submit_then_watch_default(&signer)
///      .await?
///      .wait_for_finalized_success()
///      .await?;
/// ```
pub async fn transfer_balance(
    api: PolkadotRuntimeApi,
    signer: PairSigner<DefaultConfig, Pair>,
    dest: MultiAddress<AccountId32, ()>,
    amount: u128,
) -> Result<(), Box<dyn Error>> {
    api.tx()
        .balances()
        .transfer(dest, amount)?
        .sign_and_submit_then_watch_default(&signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    Ok(())
}

/// # Exercise 06 (B)
///
/// See: 02-get-balance.rs
pub async fn get_balance(api: PolkadotRuntimeApi, account: AccountId32) -> Result<u128, Box<dyn Error>> {
    Ok(api.storage().system().account(&account, None).await?.data.free)
}

#[tokio::test]
async fn should_transfer_balance() -> Result<(), Box<dyn Error>> {
    with_default_client(|api| async move {
        let amount = 10_000_000_000;
        let dest_account_id = AccountKeyring::Bob.to_account_id();
        let balance_before = get_balance(api.clone(), dest_account_id.clone()).await?;

        // make the transfer from alice to bob
        transfer_balance(
            api.clone(),
            PairSigner::new(AccountKeyring::Alice.pair()),
            dest_account_id.clone().into(),
            amount,
        )
        .await?;

        assert_eq!(
            get_balance(api.clone(), dest_account_id.clone()).await?,
            balance_before + amount,
            "Balance was not sent!"
        );
        Ok(())
    })
    .await
}
