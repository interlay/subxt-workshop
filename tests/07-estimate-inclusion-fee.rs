use serde::Deserialize;
use sp_keyring::AccountKeyring;
use sp_rpc::number::NumberOrHex;
use std::error::Error;
use subxt::{
    rpc::{rpc_params, ClientT},
    PairSigner,
};
use subxt_workshop::{with_default_client, PolkadotRuntimeApi};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeDetails<Balance> {
    pub inclusion_fee: Option<InclusionFee<Balance>>,
    #[serde(skip)]
    pub tip: Balance,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InclusionFee<Balance> {
    pub base_fee: Balance,
    pub len_fee: Balance,
    pub adjusted_weight_fee: Balance,
}

impl InclusionFee<NumberOrHex> {
    pub fn inclusion_fee(&self) -> u128 {
        let base_fee = self.base_fee.into_u256();
        let len_fee = self.len_fee.into_u256();
        let adjusted_weight_fee = self.adjusted_weight_fee.into_u256();
        (base_fee + len_fee + adjusted_weight_fee).as_u128()
    }
}

/// # Exercise 07
///
/// Implement a function to estimate the inclusion fee for an encoded transaction.
///
/// ## Hint
///
/// ```
/// let thing: ThingToDecode = api
///     .client
///     .rpc()
///     .client
///     .request(
///         "pallet_methodName",
///         rpc_params![format!("0x{}", hex::encode(encoded_signed))],
///     )
///     .await?;
/// ```
pub async fn estimate_inclusion_fee(api: PolkadotRuntimeApi, encoded_signed: &[u8]) -> Result<u128, Box<dyn Error>> {
    let fee_details: FeeDetails<NumberOrHex> = api
        .client
        .rpc()
        .client
        .request(
            "payment_queryFeeDetails",
            rpc_params![format!("0x{}", hex::encode(encoded_signed))],
        )
        .await?;

    let inclusion_fee = fee_details.inclusion_fee.unwrap();
    Ok(inclusion_fee.inclusion_fee())
}

#[tokio::test]
async fn should_estimate_inclusion_fee() -> Result<(), Box<dyn Error>> {
    with_default_client(|api| async move {
        // don't look at this if you have not finished exercise 06
        let signed = api
            .tx()
            .balances()
            .transfer(AccountKeyring::Bob.to_account_id().into(), 10_000_000_000)?
            .create_signed(&PairSigner::new(AccountKeyring::Alice.pair()), Default::default())
            .await?;
        let encoded_signed = signed.encoded();

        // we cannot hardcode the expected fee here
        assert!(
            estimate_inclusion_fee(api.clone(), encoded_signed).await? > 10000000,
            "Extrinsic should cost more than `base_fee`!"
        );

        Ok(())
    })
    .await
}
