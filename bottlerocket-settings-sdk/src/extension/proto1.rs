use crate::error::Result;
use anyhow::Context;

use crate::cli::proto1::Proto1Command;
use crate::cli::proto1::{GenerateCommand, MigrateCommand, SetCommand, ValidateCommand};
use crate::SettingsExtension;

// TODO(seankell) transform errors into appropriate CLI output
pub(crate) fn run_extension(extension: SettingsExtension, cmd: Proto1Command) {
    let command_output = match cmd {
        Proto1Command::Set(s) => extension.set(s),
        Proto1Command::Generate(g) => extension.generate(g),
        Proto1Command::Migrate(m) => extension.migrate(m),
        Proto1Command::Validate(v) => extension.validate(v),
    }
    .and_then(|value| {
        serde_json::to_string_pretty(&value).context("Failed to write settings result to JSON.")
    });

    match command_output {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        Ok(output) => {
            println!("{}", &output);
        }
    }
}

trait Proto1 {
    fn set(&self, args: SetCommand) -> Result<serde_json::Value>;
    fn generate(&self, args: GenerateCommand) -> Result<serde_json::Value>;
    fn migrate(&self, args: MigrateCommand) -> Result<serde_json::Value>;
    fn validate(&self, args: ValidateCommand) -> Result<serde_json::Value>;
}

impl Proto1 for SettingsExtension {
    fn set(&self, args: SetCommand) -> Result<serde_json::Value> {
        self.model(&args.setting_version)
            .context(format!(
                "Requested model version '{}' not found",
                args.setting_version
            ))?
            .set(args.current_value, args.value)
    }

    fn generate(&self, args: GenerateCommand) -> Result<serde_json::Value> {
        self.model(&args.setting_version)
            .context(format!(
                "Requested model version '{}' not found",
                args.setting_version
            ))?
            .generate(args.existing_partial, args.required_settings)
            .and_then(|generated_data| {
                serde_json::to_value(generated_data).context("Failed to JSONify generated data.")
            })
    }

    fn migrate(&self, args: MigrateCommand) -> Result<serde_json::Value> {
        self.perform_migration(args.value, &args.from_version, &args.target_version)
    }

    fn validate(&self, args: ValidateCommand) -> Result<serde_json::Value> {
        self.model(&args.setting_version)
            .context(format!(
                "Requested model version '{}' not found",
                args.setting_version
            ))?
            .validate(args.value, args.required_settings)
            .and_then(|validation| {
                serde_json::to_value(validation).context("Failed to JSONify validation result.")
            })
    }
}
