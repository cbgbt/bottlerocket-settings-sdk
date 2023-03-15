use super::*;

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
    // TODO(seankell) replace these types with more constrained types
    /// The name of the setting to set
    #[arg(long)]
    pub setting: String,

    /// The version of the setting which should be used
    #[arg(long)]
    pub setting_version: Option<String>,

    /// The requested value to be set for the incoming setting
    #[arg(long)]
    pub value: serde_json::Value,

    /// The current value of this settings tree
    #[arg(long)]
    pub current_value: serde_json::Value,
}

#[derive(Args, Debug)]
pub struct GenerateCommand {
    /// A directory containing the set of rendered templates needed to generate this setting
    #[arg(long)]
    pub rendered_templates: PathBuf,
}

#[derive(Args, Debug)]
pub struct ValidateCommand {
    /// Templates rendered using incoming settings
    #[arg(long)]
    pub rendered_templates: PathBuf,
}

#[derive(Args, Debug)]
pub struct MigrateCommand {
    #[arg(long)]
    pub value: serde_json::Value,
    #[arg(long)]
    pub from_version: String,
    #[arg(long)]
    pub target_version: String,
}
