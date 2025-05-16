//! The kernel settings can be used to configure settings related to the kernel, e.g.  
//! kernel modules
use bottlerocket_model_derive::model;
use bottlerocket_modeled_types::{KmodKey, Lockdown, SysctlKey};
use bottlerocket_settings_sdk::SettingsModel;
use std::collections::HashMap;
use std::convert::Infallible;

#[model(impl_default = true)]
struct KernelSettingsV1 {
    lockdown: Lockdown,
    modules: HashMap<KmodKey, KmodSetting>,
    // Values are almost always a single line and often just an integer... but not always.
    sysctl: HashMap<SysctlKey, String>,
}

#[model]
struct KmodSetting {
    allowed: bool,
    autoload: bool,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for KernelSettingsV1 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // allow anything that parses as KernelSettingsV1
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serde_kernel() {
        let test_json = r#"{
            "lockdown": "integrity",
            "modules": {"foo": {"allowed": true, "autoload": true}},
            "sysctl": {"key": "value"}
        }"#;

        let kernel: KernelSettingsV1 = serde_json::from_str(test_json).unwrap();

        let mut modules = HashMap::new();
        modules.insert(
            KmodKey::try_from("foo").unwrap(),
            KmodSetting {
                allowed: Some(true),
                autoload: Some(true),
            },
        );
        let modules = Some(modules);

        let mut sysctl = HashMap::new();
        sysctl.insert(SysctlKey::try_from("key").unwrap(), String::from("value"));
        let sysctl = Some(sysctl);

        assert_eq!(
            kernel,
            KernelSettingsV1 {
                lockdown: Some(Lockdown::try_from("integrity").unwrap()),
                modules,
                sysctl,
            }
        );
    }
}
