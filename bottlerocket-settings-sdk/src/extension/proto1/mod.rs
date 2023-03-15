use crate::api::SettingsExtension;
use crate::cli::proto1::Proto1Command;

pub(crate) fn run_extension(cmd: Proto1Command, extension: impl SettingsExtension) {
    match cmd {
        Proto1Command::Set(s) => {
            let setting_version = s.setting_version.as_deref();
            let proposed_value = s.value;
            let current_value = s.current_value;

            // TODO(seankell) transform errors into appropriate CLI output
            let returned_setting = extension
                .set(setting_version, proposed_value, current_value)
                .unwrap();

            println!("{}", serde_json::to_string(&returned_setting).unwrap())
        }
        Proto1Command::Generate(g) => {
            todo!()
        }
        Proto1Command::Migrate(m) => {
            todo!()
        }
        Proto1Command::Validate(v) => {
            todo!()
        }
    }
}
