/*!
# API models

Bottlerocket has different variants supporting different features and use cases.
Each variant has its own set of software, and therefore needs its own configuration.
We support having an API model for each variant to support these different configurations.

The model here defines a top-level `Settings` structure, and delegates the actual implementation to a ["settings plugin"](https://github.com/bottlerocket/bottlerocket-settings-sdk/tree/settings-plugins).
Settings plugin are written in Rust as a "cdylib" crate, and loaded at runtime.

Each settings plugin must define its own private `Settings` structure.
It can use pre-defined structures inside, or custom ones as needed.

`apiserver::datastore` offers serialization and deserialization modules that make it easy to map between Rust types and the data store, and thus, all inputs and outputs are type-checked.

At the field level, standard Rust types can be used, or ["modeled types"](src/modeled_types) that add input validation.

The `#[model]` attribute on Settings and its sub-structs reduces duplication and adds some required metadata; see [its docs](model-derive/) for details.
*/

// Clippy has a false positive in the presence of the Scalar macro.
#![allow(clippy::derived_hash_with_manual_eq)]

// The "de" module contains custom deserialization trait implementation for models.
mod boot;
mod kubernetes;

pub use bottlerocket_modeled_types;
