//! Settings related to Custom CA Certificates.
use bottlerocket_model_derive::model;
use bottlerocket_modeled_types::{Identifier, PemCertificateString};
use bottlerocket_settings_sdk::SettingsModel;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, convert::Infallible};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PkiSettingsV1 {
    pub pki: HashMap<Identifier, PemCertificate>,
}

impl Serialize for PkiSettingsV1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.pki.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PkiSettingsV1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pki = HashMap::deserialize(deserializer)?;
        Ok(Self { pki })
    }
}

#[model(impl_default = true)]
struct PemCertificate {
    data: PemCertificateString,
    trusted: bool,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for PkiSettingsV1 {
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // Set anything that parses as PkiSettingsV1.
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    static VALID_PEM: &str = include_str!("../tests/data/test-pem");

    #[test]
    fn test_serde_pki() {
        let test_json = json!({
            "foo": {
                "data": VALID_PEM,
                "trusted": true
            }
        });

        let test_json_str = test_json.to_string();

        let pki: PkiSettingsV1 = serde_json::from_str(&test_json_str).unwrap();

        let mut expected_pki: HashMap<Identifier, PemCertificate> = HashMap::new();
        expected_pki.insert(
            Identifier::try_from("foo").unwrap(),
            PemCertificate {
                data: Some(PemCertificateString::try_from(VALID_PEM).unwrap()),
                trusted: Some(true),
            },
        );

        assert_eq!(pki, PkiSettingsV1 { pki: expected_pki });

        let serialized_json: serde_json::Value = serde_json::to_string(&pki)
            .map(|s| serde_json::from_str(&s).unwrap())
            .unwrap();

        assert_eq!(serialized_json, test_json);
    }
}
