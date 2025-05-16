//! The updates settings can be used to configure settings related to updates, e.g. the
//! seed that determines in which wave the instance will update, etc.
pub mod generate;

use bottlerocket_model_derive::model;
use bottlerocket_modeled_types::{FriendlyVersion, Url};
use bottlerocket_settings_sdk::SettingsModel;
use std::convert::Infallible;

#[model(impl_default = true)]
pub struct UpdatesSettingsV1 {
    metadata_base_url: Url,
    targets_base_url: Url,
    seed: u32,
    // Version to update to when updating via the API.
    version_lock: FriendlyVersion,
    ignore_waves: bool,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for UpdatesSettingsV1 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // allow anything that parses as UpdatesSettingsV1
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serde_updates() {
        let test_json = r#"{
            "metadata-base-url": "https://example.net",
            "targets-base-url": "https://example.net",
            "seed": 1,
            "version-lock": "latest",
            "ignore-waves": false
        }"#;

        let updates: UpdatesSettingsV1 = serde_json::from_str(test_json).unwrap();

        assert_eq!(
            updates,
            UpdatesSettingsV1 {
                metadata_base_url: Some(Url::try_from("https://example.net").unwrap()),
                targets_base_url: Some(Url::try_from("https://example.net").unwrap()),
                seed: Some(1),
                version_lock: Some(FriendlyVersion::try_from("latest").unwrap()),
                ignore_waves: Some(false),
            }
        );
    }
}
