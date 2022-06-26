use std::error::Error;
use subxt_workshop::{polkadot, with_default_client, PolkadotRuntimeApi};

type RuntimeVersion = polkadot::runtime_types::sp_version::RuntimeVersion;

/// # Exercise 05
///
/// Implement a function to return the `version` constant from the `system` pallet.
///
/// ## Hint
///
/// ```
/// let value = api.constants().pallet_name().constant_item_name()?;
/// ```
pub fn get_version(api: PolkadotRuntimeApi) -> Result<RuntimeVersion, Box<dyn Error>> {
    Ok(api.constants().system().version()?)
}

#[tokio::test]
async fn should_get_version() -> Result<(), Box<dyn Error>> {
    with_default_client(|api| async move {
        // copied this for the pinned polkadot node version, if that
        // changes this test will break
        assert_eq!(
            get_version(api)?,
            RuntimeVersion {
                spec_name: "polkadot".to_string(),
                impl_name: "parity-polkadot".to_string(),
                authoring_version: 0,
                spec_version: 9180,
                impl_version: 0,
                apis: vec![
                    ([223, 106, 203, 104, 153, 7, 96, 155], 4),
                    ([55, 227, 151, 252, 124, 145, 245, 228], 1),
                    ([64, 254, 58, 212, 1, 248, 149, 154], 5),
                    ([210, 188, 152, 151, 238, 208, 143, 21], 3),
                    ([247, 139, 39, 139, 229, 63, 69, 76], 2),
                    ([175, 44, 2, 151, 162, 62, 109, 61], 2),
                    ([73, 234, 175, 27, 84, 138, 12, 176], 1),
                    ([145, 213, 223, 24, 176, 210, 207, 88], 1),
                    ([237, 153, 197, 172, 178, 94, 237, 245], 3),
                    ([203, 202, 37, 227, 159, 20, 35, 135], 2),
                    ([104, 122, 212, 74, 211, 127, 3, 194], 1),
                    ([171, 60, 5, 114, 41, 31, 235, 139], 1),
                    ([188, 157, 137, 144, 79, 91, 146, 63], 1),
                    ([55, 200, 187, 19, 80, 169, 162, 168], 1)
                ],
                transaction_version: 12,
                state_version: 0,
            },
            "RuntimeVersion does not match!"
        );
        Ok(())
    })
    .await
}
