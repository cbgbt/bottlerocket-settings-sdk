use anyhow::{Context, Result};
use bottlerocket_settings_sdk::BottlerocketSetting;

pub(crate) mod v1;
pub(crate) mod v2;

fn main() -> Result<()> {
    bottlerocket_settings_sdk::SettingsExtension::with_models(vec![
        BottlerocketSetting::<v1::MotdV1>::model(),
        BottlerocketSetting::<v2::MotdV2>::model(),
    ])
    .run_extension()
    .context("Settings extension encountered an error.")
}
