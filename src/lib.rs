use std::{error::Error, future::Future};
use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};

#[subxt::subxt(
    runtime_metadata_path = "polkadot_metadata.scale",
    derive_for_all_types = "Debug",
    derive_for_type(type = "sp_version::RuntimeVersion", derive = "Eq, PartialEq"),
    derive_for_type(type = "frame_system::AccountInfo", derive = "Eq, PartialEq"),
    derive_for_type(type = "pallet_balances::AccountData", derive = "Eq, PartialEq"),
    derive_for_type(type = "pallet_treasury::Proposal", derive = "Eq, PartialEq")
)]
pub mod polkadot {
    #[subxt(substitute_type = "sp_arithmetic::per_things::Permill")]
    use subxt::sp_runtime::Permill;

    #[subxt(substitute_type = "sp_runtime::DispatchError")]
    use subxt::sp_runtime::DispatchError;
}

pub type PolkadotRuntimeApi = polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>;

pub type EncodedCall = polkadot::runtime_types::polkadot_runtime::Call;

pub async fn with_default_client<F, R>(f: F) -> Result<(), Box<dyn Error>>
where
    F: Fn(PolkadotRuntimeApi) -> R,
    R: Future<Output = Result<(), Box<dyn Error>>>,
{
    f(ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<PolkadotRuntimeApi>())
    .await
}
