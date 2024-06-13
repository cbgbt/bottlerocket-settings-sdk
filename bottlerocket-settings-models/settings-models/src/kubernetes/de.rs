use bottlerocket_modeled_types::{KubernetesLabelKey, KubernetesTaintValue};
use serde::de::{Error, MapAccess, Visitor};
use serde::Deserializer;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Formatter;
use toml::Value;

// Our standard representation of node-taints is a `HashMap` of label keys to a list of taint values and effects;
// for backward compatibility, we also allow a `HashMap` of label keys to a singular taint value/effect.
pub(crate) fn deserialize_node_taints<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<KubernetesLabelKey, Vec<KubernetesTaintValue>>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct NodeTaints;

    impl<'de> Visitor<'de> for NodeTaints {
        type Value = Option<HashMap<KubernetesLabelKey, Vec<KubernetesTaintValue>>>;
        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("TOML table")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut node_taints: HashMap<KubernetesLabelKey, Vec<KubernetesTaintValue>> =
                HashMap::new();
            while let Some((k, v)) = map.next_entry()? {
                match v {
                    // If we encounter a singular mapped value, convert it into a list of one
                    Value::String(taint_val) => {
                        node_taints.insert(
                            k,
                            vec![KubernetesTaintValue::try_from(taint_val)
                                .map_err(M::Error::custom)?],
                        );
                    }
                    // If we encounter a list of values, just insert it as is
                    Value::Array(taint_values) => {
                        let taint_values = taint_values
                            .iter()
                            .map(|v| v.to_owned().try_into().map_err(M::Error::custom))
                            .collect::<Result<Vec<KubernetesTaintValue>, _>>()?;
                        if taint_values.is_empty() {
                            return Err(M::Error::custom("empty taint value"));
                        }
                        node_taints.insert(k, taint_values);
                    }
                    _ => {
                        return Err(M::Error::custom("unsupported taint value type"));
                    }
                }
            }
            Ok(Some(node_taints))
        }
    }

    deserializer.deserialize_map(NodeTaints)
}

#[cfg(test)]
mod node_taint_tests {
    use super::super::KubernetesSettings;
    use bottlerocket_modeled_types::KubernetesTaintValue;
    use std::convert::TryFrom;
    static TEST_NODE_TAINT_LIST: &str = include_str!("../../../tests/data/node-taint-list-val");
    static TEST_NODE_TAINT_SINGLE: &str = include_str!("../../../tests/data/node-taint-single-val");
    static TEST_NODE_TAINT_EMPTY_LIST: &str =
        include_str!("../../../tests/data/node-taint-empty-list");

    #[test]
    fn node_taints_list_representation() {
        let k8s_settings = toml::from_str::<KubernetesSettings>(TEST_NODE_TAINT_LIST).unwrap();
        assert_eq!(
            k8s_settings
                .node_taints
                .as_ref()
                .unwrap()
                .get("key1")
                .unwrap()
                .to_owned(),
            vec![
                KubernetesTaintValue::try_from("value1:NoSchedule").unwrap(),
                KubernetesTaintValue::try_from("value1:NoExecute").unwrap()
            ]
        );
        assert_eq!(
            k8s_settings
                .node_taints
                .as_ref()
                .unwrap()
                .get("key2")
                .unwrap()
                .to_owned(),
            vec![KubernetesTaintValue::try_from("value2:NoSchedule").unwrap()]
        );
    }

    #[test]
    fn node_taint_single_representation() {
        let k8s_settings = toml::from_str::<KubernetesSettings>(TEST_NODE_TAINT_SINGLE).unwrap();
        assert_eq!(
            k8s_settings
                .node_taints
                .as_ref()
                .unwrap()
                .get("key1")
                .unwrap()
                .to_owned(),
            vec![KubernetesTaintValue::try_from("value1:NoSchedule").unwrap()]
        );
        assert_eq!(
            k8s_settings
                .node_taints
                .as_ref()
                .unwrap()
                .get("key2")
                .unwrap()
                .to_owned(),
            vec![KubernetesTaintValue::try_from("value2:NoExecute").unwrap()]
        );
    }

    #[test]
    fn node_taint_none_representation() {
        let k8s_settings = toml::from_str::<KubernetesSettings>("").unwrap();
        assert!(k8s_settings.node_taints.is_none());
    }

    #[test]
    fn node_taint_empty_list() {
        assert!(toml::from_str::<KubernetesSettings>(TEST_NODE_TAINT_EMPTY_LIST).is_err());
    }
}
