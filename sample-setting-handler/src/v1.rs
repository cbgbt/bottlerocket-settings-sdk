use anyhow::Result;
use bottlerocket_settings_sdk::{GenerateResult, NoMigration, SettingsModel};
use serde::{Deserialize, Serialize};

use crate::v2;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct MotdV1 {
    pub(crate) motd: String,
}

impl SettingsModel for MotdV1 {
    type PartialType = MotdV1Partial;
    type ForwardMigrationTarget = v2::MotdV2;
    type BackwardMigrationTarget = NoMigration;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, target: Self) -> Result<Self> {
        Ok(target)
    }

    fn generate(
        _: Option<Self::PartialType>,
        _: Option<serde_json::Value>,
    ) -> Result<GenerateResult<Self::PartialType, Self>> {
        Ok(GenerateResult::Complete(Some(MotdV1::default())))
    }

    fn validate(_value: Self, _validated_settings: Option<serde_json::Value>) -> Result<bool> {
        Ok(true)
    }

    fn migrate_forward(self) -> Result<Self::ForwardMigrationTarget> {
        Ok(v2::MotdV2 {
            motd: self.motd,
            person: "Sean".to_string(),
        })
    }

    fn migrate_backward(self) -> Result<Self::BackwardMigrationTarget> {
        NoMigration::no_defined_migration()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub(crate) struct MotdV1Partial {
    pub motd: Option<String>,
}
