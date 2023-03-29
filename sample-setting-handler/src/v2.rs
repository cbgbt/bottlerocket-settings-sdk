use anyhow::Result;
use bottlerocket_settings_sdk::{GenerateResult, NoMigration, SettingsModel};
use serde::{Deserialize, Serialize};

use crate::v1;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct MotdV2 {
    pub(crate) motd: String,
    pub(crate) person: String,
}

impl SettingsModel for MotdV2 {
    type PartialType = MotdV2Partial;
    type ForwardMigrationTarget = NoMigration;
    type BackwardMigrationTarget = v1::MotdV1;

    fn get_version() -> &'static str {
        "v2"
    }

    fn set(_current_value: Option<Self>, target: Self) -> Result<Self> {
        Ok(target)
    }

    fn generate(
        _existing_partial: Option<Self::PartialType>,
        _dependent_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult<Self::PartialType, Self>> {
        Ok(GenerateResult::Complete(Some(MotdV2::default())))
    }

    fn validate(_value: Self, _validated_settings: Option<serde_json::Value>) -> Result<bool> {
        Ok(true)
    }

    fn migrate_forward(self) -> Result<Self::ForwardMigrationTarget> {
        NoMigration::no_defined_migration()
    }

    fn migrate_backward(self) -> Result<Self::BackwardMigrationTarget> {
        Ok(v1::MotdV1 { motd: self.motd })
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub(crate) struct MotdV2Partial {
    pub motd: Option<String>,
    pub person: Option<String>,
}
