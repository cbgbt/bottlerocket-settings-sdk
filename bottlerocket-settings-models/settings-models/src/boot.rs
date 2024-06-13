use bottlerocket_model_derive::model;
use bottlerocket_modeled_types::{BootConfigKey, BootConfigValue};
use std::collections::HashMap;

// Kernel boot settings
#[model(impl_default = true)]
struct BootSettings {
    reboot_to_reconcile: bool,
    #[serde(
        alias = "kernel",
        rename(serialize = "kernel"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    kernel_parameters: HashMap<BootConfigKey, Vec<BootConfigValue>>,
    #[serde(
        alias = "init",
        rename(serialize = "init"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    init_parameters: HashMap<BootConfigKey, Vec<BootConfigValue>>,
}
