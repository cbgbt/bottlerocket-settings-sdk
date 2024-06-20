/*!
This crate defines the FFI specification for Bottlerocket settings plugins.

The goal of a settings plugin is to enable a host program to construct and serialize instances of a
Rust struct without compile-time access to its definition. Instead, the struct is defined by a
cdylib crate, which can be loaded at runtime into the host program as a plugin. The host program
cannot access fields or methods on this type directly, only through functions exposed via FFI.

The crate also provides helper functionality that can be used by either the host program or by
plugins, to make the shared settings structure easier to implement and to interact with from
idiomatic Rust.

All of the heavy lifting is handled by the abi_stable crate, which provides FFI-safe wrapper types
and an interface for loading and verifying cdylibs at runtime.
*/

mod settings;
pub use settings::*;

// Export the derive macro via this crate, since it depends on the implementation details here.
pub use bottlerocket_settings_derive::SettingsPlugin;

// Re-export abi_stable to make it easier to synchronize versions across plugin & host programs.
pub use abi_stable;
