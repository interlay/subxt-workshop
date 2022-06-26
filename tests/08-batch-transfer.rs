use sp_keyring::AccountKeyring;
use std::error::Error;
use subxt::{
    sp_core::sr25519::Pair,
    sp_runtime::{AccountId32, MultiAddress},
    DefaultConfig, PairSigner,
};
use subxt_workshop::{polkadot, with_default_client, EncodedCall, PolkadotRuntimeApi};

type BalancesCall = polkadot::runtime_types::pallet_balances::pallet::Call;

/// # Exercise 08 (A)
///
/// Implement a function to batch multiple `transfer` calls using the `utility` pallet.
///
/// ## Hint
///
/// ```
/// let calls = vec![EncodedCall::Pallet(PalletCall::extrinsic { params }];
/// ```
pub async fn batch_transfer(
    _api: PolkadotRuntimeApi,
    _signer: PairSigner<DefaultConfig, Pair>,
    _recipients: Vec<(MultiAddress<AccountId32, ()>, u128)>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

/// # Exercise 08 (B)
///
/// See: 02-get-balance.rs
pub async fn get_balance(_api: PolkadotRuntimeApi, _account: AccountId32) -> Result<u128, Box<dyn Error>> {
    Ok(Default::default())
}

#[tokio::test]
async fn should_batch_transfer() -> Result<(), Box<dyn Error>> {
    use futures::future::join_all;

    with_default_client(|api| async move {
        let recipients = vec![
            (AccountKeyring::Bob.to_account_id(), 10_000_000_000),
            (AccountKeyring::Charlie.to_account_id(), 1_000_000_000),
        ];
        let balances_before = join_all(recipients.iter().map(|(account_id, amount)| async {
            Ok((
                account_id.clone(),
                amount.clone(),
                get_balance(api.clone(), account_id.clone()).await?,
            ))
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

        batch_transfer(
            api.clone(),
            PairSigner::new(AccountKeyring::Alice.pair()),
            recipients
                .into_iter()
                .map(|(account_id, value)| (account_id.into(), value))
                .collect(),
        )
        .await?;

        // there is probably a nicer way to write this check - bonus points for refactoring
        let balances_after = join_all(
            balances_before
                .iter()
                .map(|(account_id, amount, balance_before)| async {
                    Ok((
                        account_id.clone(),
                        amount.clone(),
                        balance_before.clone(),
                        get_balance(api.clone(), account_id.clone()).await?,
                    ))
                }),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

        for (account_id, amount, before, after) in balances_after {
            assert_eq!(after, before + amount, "Balance was not sent to {account_id:?}!");
        }

        Ok(())
    })
    .await
}
