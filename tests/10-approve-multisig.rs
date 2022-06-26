use futures::StreamExt;
use sp_keyring::AccountKeyring;
use std::{error::Error, future::Future, time::Duration};
use subxt::{
    codec::{Decode, Encode},
    sp_core::{blake2_256, sr25519::Pair},
    sp_runtime::{traits::TrailingZeroInput, AccountId32},
    DefaultConfig, Event, PairSigner, WrapperKeepOpaque,
};
use subxt_workshop::{polkadot, with_default_client, EncodedCall, PolkadotRuntimeApi};

type NewMultisigEvent = polkadot::multisig::events::NewMultisig;
type MultisigExecutedEvent = polkadot::multisig::events::MultisigExecuted;
type BalancesCall = polkadot::runtime_types::pallet_balances::pallet::Call;
type Timepoint = polkadot::runtime_types::pallet_multisig::Timepoint<u32>;

const MAX_WEIGHT: u64 = 1_000_000_000_000;

/// # Exercise 10 (A)
///
/// Implement a function to subscribe to all events and filter `Ev`.
/// This should return immediately after the callback.
///
/// Tip: `subscribe_finalized` will make sure the events are included.
///
/// ## Hint
///
/// ```
/// let filter_events = api
///     .events()
///     .subscribe()
///     .await?
///     .filter_events::<(runtime::pallet::events::Event,)>();
///
/// while let Some(Ok(event)) = filter_events.next().await {
///     // do something
/// }
/// ```
pub async fn wait_for_event<Ev, Cb, Fut>(api: PolkadotRuntimeApi, callback: Cb) -> Result<(), Box<dyn Error>>
where
    Ev: Event,
    Cb: Fn(Ev) -> Fut,
    Fut: Future<Output = Result<(), Box<dyn Error>>>,
{
    let event_sub = api.events().subscribe_finalized().await?;
    let mut filter_events = event_sub.filter_events::<(Ev,)>();
    while let Some(Ok(filtered_event)) = filter_events.next().await {
        return callback(filtered_event.event).await;
    }

    Ok(())
}

/// # Exercise 10 (B)
///
/// Implement a function to create a 2-of-2 multisig operation.
///
/// Tip: use `as_multi` with a threshold of `2`.
pub async fn create_multisig(
    api: PolkadotRuntimeApi,
    signer: PairSigner<DefaultConfig, Pair>,
    other_signatories: Vec<AccountId32>,
    encoded_call: EncodedCall,
) -> Result<(), Box<dyn Error>> {
    api.tx()
        .multisig()
        .as_multi(
            2,
            other_signatories,
            None,
            WrapperKeepOpaque::from_encoded(encoded_call.encode()),
            true,
            MAX_WEIGHT,
        )?
        .sign_and_submit_then_watch_default(&signer)
        .await?
        .wait_for_finalized_success()
        .await?;
    Ok(())
}

/// # Exercise 10 (C)
///
/// Implement a getter to return the `Timepoint` stored in the `multisig` pallet
/// under the `Multisigs` double storage map.
///
/// This is required for `approve_multisig`.
pub async fn get_timepoint(
    api: PolkadotRuntimeApi,
    multisig_account_id: &AccountId32,
    call_hash: &[u8; 32],
) -> Result<Timepoint, Box<dyn Error>> {
    Ok(api
        .storage()
        .multisig()
        .multisigs(multisig_account_id, call_hash, None)
        .await?
        .unwrap()
        .when)
}

/// # Exercise 10 (D)
///
/// Implement a function to approve a 2-of-2 `multisig` call.
///
/// Tip: use `approve_as_multi` with a threshold of `2`.
pub async fn approve_multisig(
    api: PolkadotRuntimeApi,
    signer: PairSigner<DefaultConfig, Pair>,
    other_signatories: Vec<AccountId32>,
    timepoint: Timepoint,
    call_hash: [u8; 32],
) -> Result<(), Box<dyn Error>> {
    api.tx()
        .multisig()
        .approve_as_multi(
            2,                 // threshold
            other_signatories, // other_signatories
            Some(timepoint),   // maybe_timepoint
            call_hash,         // call_hash
            MAX_WEIGHT,        // max_weight
        )?
        .sign_and_submit_then_watch_default(&signer)
        .await?
        .wait_for_finalized_success()
        .await?;
    Ok(())
}

/// Source: https://github.com/paritytech/substrate/blob/polkadot-v0.9.18/frame/multisig/src/lib.rs#L510-L514
pub fn multi_account_id(who: &[AccountId32], threshold: u16) -> AccountId32 {
    let entropy = (b"modlpy/utilisuba", who, threshold).using_encoded(blake2_256);
    Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
        .expect("infinite length input; no invalid inputs for type; qed")
}

#[tokio::test]
async fn should_approve_multisig() -> Result<(), Box<dyn Error>> {
    with_default_client(|api| async move {
        {
            // start event listener first in its own task
            let api = api.clone();
            tokio::spawn(async move {
                let api = &api.clone();
                wait_for_event::<NewMultisigEvent, _, _>(api.clone(), |event| async move {
                    let timepoint = get_timepoint(api.clone(), &event.multisig, &event.call_hash.clone()).await?;
                    // bob should sign incoming multisig
                    approve_multisig(
                        api.clone(),
                        PairSigner::new(AccountKeyring::Bob.pair()),
                        vec![event.approving],
                        timepoint,
                        event.call_hash,
                    )
                    .await?;
                    Ok(())
                })
                .await
                .unwrap();
            });
        }

        // compute the multisig account id
        let account_id = multi_account_id(
            &[
                AccountKeyring::Bob.to_account_id(),
                AccountKeyring::Alice.to_account_id(),
            ],
            2,
        );

        // fund the multisig so it can afford the transfer
        api.tx()
            .balances()
            .transfer(account_id.into(), 100_000_000_000)?
            .sign_and_submit_then_watch_default(&PairSigner::new(AccountKeyring::Alice.pair()))
            .await?
            .wait_for_finalized_success()
            .await?;

        // alice should create the multisig operation
        create_multisig(
            api.clone(),
            PairSigner::new(AccountKeyring::Alice.pair()),
            vec![AccountKeyring::Bob.to_account_id()],
            EncodedCall::Balances(BalancesCall::transfer {
                dest: AccountKeyring::Charlie.to_account_id().into(),
                value: 10_000_000_000,
            }),
        )
        .await?;

        // we need to timeout if the event listener isn't implemented correctly
        tokio::time::timeout(Duration::from_secs(60), async {
            wait_for_event::<MultisigExecutedEvent, _, _>(api.clone(), |event| async move {
                event.result.map_err(|err| format!("Call failed: {err:?}").into())
            })
            .await
        })
        .await??;

        Ok(())
    })
    .await
}
