//! Settings related to CloudFormation signaling
use bottlerocket_model_derive::model;
use bottlerocket_modeled_types::SingleLineString;
use bottlerocket_settings_sdk::SettingsModel;
use std::convert::Infallible;

#[model(impl_default = true)]
pub struct CloudFormationSettingsV1 {
    should_signal: bool,
    stack_name: SingleLineString,
    logical_resource_id: SingleLineString,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for CloudFormationSettingsV1 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // Set anything that can be parsed as CloudFormationSettingsV1.
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_serde_cloudformation() {
        let test_json = json!({
            "logical-resource-id": "MyEC2Instance",
            "should-signal":true,
            "stack-name":"MyStack"
        });

        let test_json_str = test_json.to_string();

        let cloudformation_settings: CloudFormationSettingsV1 =
            serde_json::from_str(&test_json_str).unwrap();

        assert_eq!(
            cloudformation_settings,
            CloudFormationSettingsV1 {
                logical_resource_id: Some(SingleLineString::try_from("MyEC2Instance").unwrap()),
                should_signal: Some(true),
                stack_name: Some(SingleLineString::try_from("MyStack").unwrap())
            }
        );

        let serialized_json: serde_json::Value = serde_json::to_string(&cloudformation_settings)
            .map(|s| serde_json::from_str(&s).unwrap())
            .unwrap();

        assert_eq!(serialized_json, test_json);
    }
}
