use serde::{de::DeserializeOwned, Serialize};

pub mod cli;
pub(crate) mod extension;

pub mod api {
    pub use model_derive::model;
    pub mod scalar {
        pub use scalar::traits;
        pub use scalar_derive::Scalar;
    }
    pub use crate::{GenerateResult, SetResult, SettingsExtension, SettingsModel};
}

/// Convenience marker trait for settings models.
///
/// This trait is automatically implemented for types which implement the othe traits required to interop with the `settings-sdk`.
pub trait SettingsModel: Serialize + DeserializeOwned + std::fmt::Debug {
    // Empty
}

// Automatically implement `SettingsModel` trait for all structs with the requisite traits.
impl<T> SettingsModel for T
where
    T: Serialize + DeserializeOwned + std::fmt::Debug,
{
    // Empty
}

pub trait SettingsExtension {
    type E: std::error::Error + std::fmt::Debug;

    fn set(
        &self,
        setting_version: Option<&str>,
        proposed_value: serde_json::Value,
        current_value: serde_json::Value,
    ) -> Result<SetResult, Self::E>;

    fn generate(
        &self,
        existing_partial: Option<serde_json::Value>,
        required_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult, Self::E>;

    // fn migrate(&self, from_version: String, value: serde_json::Value, to_version: String);

    // fn validate(
    //     &self,
    //     current_value: serde_json::Value,
    //     validated_settings: serde_json::Value,
    // ) -> error::Result<bool>;
}

#[derive(Serialize, Debug)]
pub struct SetResult {
    pub version: String,
    pub value: serde_json::Value,
}

#[derive(Serialize, Debug)]
pub enum GenerateResult {
    NeedsData(serde_json::Value),
    Complete(serde_json::Value),
}

pub fn run_extension(extension: impl SettingsExtension) -> error::Result<()> {
    let args = cli::Cli::parse_args();
    match args.protocol {
        cli::Protocol::Proto1(p) => crate::extension::proto1::run_extension(p.command, extension),
    }
    Ok(())
}

mod error {
    use snafu::Snafu;

    #[derive(Debug, Snafu, Clone)]
    pub enum Error {}

    pub(crate) type Result<T> = std::result::Result<T, Error>;
}
