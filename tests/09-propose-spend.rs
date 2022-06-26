use sp_keyring::{sr25519::sr25519::Pair, AccountKeyring};
use std::error::Error;
use subxt::{sp_runtime::AccountId32, DefaultConfig, PairSigner};
use subxt_workshop::{polkadot, with_default_client, PolkadotRuntimeApi};

type Proposal = polkadot::runtime_types::pallet_treasury::Proposal<AccountId32, u128>;
type ProposedEvent = polkadot::treasury::events::Proposed;

/// # Exercise 09 (A)
///
/// Implement a function to make a `treasury` proposal for the `signer`.
/// The function should return the `proposal_index` from the `ProposedEvent`.
///
/// ## Hint
///
/// ```
/// let event = events.find_first::<Event>()?.unwrap();
/// ```
pub async fn propose_spend(
    api: PolkadotRuntimeApi,
    signer: PairSigner<DefaultConfig, Pair>,
    value: u128,
) -> Result<u32, Box<dyn Error>> {
    let events = api
        .tx()
        .treasury()
        .propose_spend(value, signer.account_id().clone().into())?
        .sign_and_submit_then_watch_default(&signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    let proposed = events.find_first::<ProposedEvent>()?.unwrap();
    Ok(proposed.proposal_index)
}

/// # Exercise 09 (B)
///
/// Implement a function to return the `Proposal` in storage.
pub async fn get_proposal(api: PolkadotRuntimeApi, proposal_index: u32) -> Result<Proposal, Box<dyn Error>> {
    Ok(api
        .storage()
        .treasury()
        .proposals(&proposal_index, None)
        .await?
        .unwrap())
}

/// # Exercise 09 (C)
///
/// Implement a function to calculate the maximum proposal bond.
///
/// Source: https://github.com/paritytech/substrate/blob/polkadot-v0.9.18/frame/treasury/src/lib.rs#L410-L417
pub fn calculate_proposal_bond(api: PolkadotRuntimeApi, value: u128) -> Result<u128, Box<dyn Error>> {
    let proposal_bond = api.constants().treasury().proposal_bond()?;
    let proposal_bond_minimum = api.constants().treasury().proposal_bond_minimum()?;
    Ok(std::cmp::max(proposal_bond_minimum, proposal_bond * value))
}

#[tokio::test]
async fn should_propose_spend() -> Result<(), Box<dyn Error>> {
    with_default_client(|api| async move {
        let signer_account = AccountKeyring::Alice;
        let signer_account_id = signer_account.clone().to_account_id();
        let value = 10_000_000_000;
        // make the proposal, the first `Proposed` event will have the index
        let proposal_index = propose_spend(api.clone(), PairSigner::new(signer_account.pair()), value).await?;
        // read the treasury pallet constants and calculate the expected deposit
        let bond = calculate_proposal_bond(api.clone(), value)?;

        // check the proposal in storage matches our expectations
        assert_eq!(
            get_proposal(api, proposal_index).await?,
            Proposal {
                proposer: signer_account_id.clone(),
                value,
                beneficiary: signer_account_id,
                bond,
            },
            "Proposal does not match!"
        );
        Ok(())
    })
    .await
}
