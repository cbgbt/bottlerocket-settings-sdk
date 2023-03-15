use bottlerocket_settings_sdk::api::{
    model, GenerateResult, SetResult, SettingsExtension, SettingsModel,
};
use serde::{Deserialize, Serialize};

use anyhow::{Context, Result};

const DEFAULT_VERSION: &str = "v1";

struct MotdExtension;

#[model]
struct MotdV1 {
    motd: String,
}

#[model]
struct MotdV2 {
    motd: String,
    motw: String,
}

impl SettingsExtension for MotdExtension {
    type E = anyhow::Error;

    fn set(
        &self,
        setting_version: Option<&str>,
        proposed_value: serde_json::Value,
        _current_value: serde_json::Value,
    ) -> Result<SetResult> {
        let setting_version = setting_version.unwrap_or(DEFAULT_VERSION);

        match setting_version.as_ref() {
            "v1" => {
                serde_json::from_value::<MotdV1>(proposed_value.clone())
                    .context("Failed to validate requested setting.")?;
            }
            "v2" => {
                serde_json::from_value::<MotdV2>(proposed_value.clone())
                    .context("Failed to validate requested setting.")?;
            }
            v => {
                anyhow::bail!("Setting version '{}' unrecognized.", v);
            }
        }
        Ok(SetResult {
            version: setting_version.to_string(),
            value: proposed_value,
        })
    }

    fn generate(
        &self,
        _existing_partial: Option<serde_json::Value>,
        _required_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult> {
        Ok(GenerateResult::Complete(
            serde_json::to_value(MotdV1 {
                motd: Some("Hello, world".to_string()),
            })
            .context("Failed to serialize generated motd.")?,
        ))
    }
}

fn main() -> Result<()> {
    bottlerocket_settings_sdk::run_extension(MotdExtension)?;
    Ok(())
}
