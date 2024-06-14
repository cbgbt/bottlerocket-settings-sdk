/*!
# Settings models

Bottlerocket supports building different variants with their own features and use cases.
Each variant has its own set of software, and therefore needs its own configuration.
We support having an API model for each variant to support these different configurations via
Settings Plugins.

Settings Plugins are composed of Rust structs which, at minimum, provide `Default` and `Deserialize`
implementations.
This package provides settings structs for common settings used in Bottlerocket, which can be added
to Settings Plugins.
The package also exports a set of `modeled_types` which can be helpful in creating additional
settings structures that helpfully validate inputs on deserialize.
*/

mod boot;
mod kubernetes;

// Expose types for creating new settings structs
pub use bottlerocket_model_derive as model_derive;
pub use bottlerocket_modeled_types as modeled_types;
pub use bottlerocket_scalar as scalar;
pub use bottlerocket_scalar_derive as scalar_derive;
pub use bottlerocket_string_impls_for as string_impls_for;

// Expose common settings structs
pub use crate::boot::BootSettingsV1;
pub use crate::kubernetes::KubernetesSettingsV1;
pub use settings_extension_autoscaling::AutoScalingSettingsV1;
pub use settings_extension_aws::AwsSettingsV1;
pub use settings_extension_bootstrap_containers::BootstrapContainersSettingsV1;
pub use settings_extension_cloudformation::CloudFormationSettingsV1;
pub use settings_extension_container_registry::RegistrySettingsV1;
pub use settings_extension_container_runtime::ContainerRuntimeSettingsV1;
pub use settings_extension_dns::DnsSettingsV1;
pub use settings_extension_ecs::ECSSettingsV1;
pub use settings_extension_host_containers::HostContainersSettingsV1;
pub use settings_extension_kernel::KernelSettingsV1;
pub use settings_extension_metrics::MetricsSettingsV1;
pub use settings_extension_motd::MotdV1;
pub use settings_extension_network::NetworkSettingsV1;
pub use settings_extension_ntp::NtpSettingsV1;
pub use settings_extension_oci_defaults::OciDefaultsV1;
pub use settings_extension_oci_hooks::OciHooksSettingsV1;
pub use settings_extension_pki::PkiSettingsV1;
pub use settings_extension_updates::UpdatesSettingsV1;
