use anyhow::Context;
use clap::{Args, Subcommand};

use crate::error::Result;

#[derive(Args, Debug)]
pub struct Protocol1 {
    #[command(subcommand)]
    pub command: Proto1Command,
}

#[derive(Subcommand, Debug)]
pub enum Proto1Command {
    /// Modify values owned by this setting
    Set(SetCommand),
    /// Generate default values for this setting
    Generate(GenerateCommand),
    /// Validate values created by external settings
    Validate(ValidateCommand),
    /// Migrate this setting from one given version to another
    Migrate(MigrateCommand),
}

impl Proto1Command {}

#[derive(Args, Debug)]
pub struct SetCommand {
    /// The version of the setting which should be used
    #[arg(long)]
    pub setting_version: String,

    /// The requested value to be set for the incoming setting
    #[arg(long, value_parser = parse_json)]
    pub value: serde_json::Value,

    /// The current value of this settings tree
    #[arg(long, value_parser = parse_json)]
    pub current_value: Option<serde_json::Value>,
}

#[derive(Args, Debug)]
pub struct GenerateCommand {
    /// The version of the setting which should be used
    #[arg(long)]
    pub setting_version: String,

    /// A json value containing any partially generated data for this setting
    #[arg(long, value_parser = parse_json)]
    pub existing_partial: Option<serde_json::Value>,

    /// A json value containing any requested settings partials needed to generate this one
    #[arg(long, value_parser = parse_json)]
    pub required_settings: Option<serde_json::Value>,
}

#[derive(Args, Debug)]
pub struct ValidateCommand {
    /// The version of the setting which should be used
    #[arg(long)]
    pub setting_version: String,

    /// A json value containing any partially generated data for this setting
    #[arg(long, value_parser = parse_json)]
    pub value: serde_json::Value,

    /// A json value containing any requested settings partials needed to generate this one
    #[arg(long, value_parser = parse_json)]
    pub required_settings: Option<serde_json::Value>,
}

#[derive(Args, Debug)]
pub struct MigrateCommand {
    #[arg(long, value_parser = parse_json)]
    pub value: serde_json::Value,
    #[arg(long)]
    pub from_version: String,
    #[arg(long)]
    pub target_version: String,
}

fn parse_json(arg: &str) -> Result<serde_json::Value> {
    serde_json::from_str(&arg).context("Failed to parse CLI input as JSON.")
}
