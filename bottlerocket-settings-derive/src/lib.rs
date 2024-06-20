/*!
This crate provides a derive macro for implementing the provider side of a Bottlerocket settings
plugin. It should be applied to a custom settings struct in the cdylib crate, and implements the
FFI protocol expected by the host process that will load the plugin.
*/

use darling::{FromDeriveInput, ToTokens};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// A macro to simplify implementing a settings plugin.
#[proc_macro_derive(SettingsPlugin)]
pub fn derive_settings(input: TokenStream) -> TokenStream {
    // Parse the AST and "deserialize" into SettingsPlugin
    let ast = parse_macro_input!(input as DeriveInput);
    let n = SettingsPlugin::from_derive_input(&ast).expect("Unable to parse macro arguments");
    quote!(#n).into()
}

#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_named))]
struct SettingsPlugin {
    ident: syn::Ident,
}

impl ToTokens for SettingsPlugin {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let SettingsPlugin { ident } = self;
        tokens.extend(quote! {
            use bottlerocket_settings_plugin::abi_stable;

            // Provide the "serialize" interface expected for this type.
            impl<'a> abi_stable::erased_types::SerializeType<'a> for #ident {
                type Interface = bottlerocket_settings_plugin::BottlerocketSettingsInterface;

                fn serialize_impl(&'a self) -> Result<abi_stable::external_types::RawValueBox, abi_stable::std_types::RBoxError> {
                    // Call the shared function to serialize to JSON.
                    bottlerocket_settings_plugin::serialize_json(self)
                }
            }

            // Provide the "deserialize" function that's required for FFI.
            // This function refers to the type, but isn't otherwise tied to it or namespaced in
            // any way, which means the derive macro can't be used for more than one type in the
            // module.
            #[abi_stable::sabi_extern_fn]
            fn deserialize_settings(s: abi_stable::std_types::RStr<'_>) -> abi_stable::std_types::RResult<bottlerocket_settings_plugin::BottlerocketSettingsProvider, abi_stable::std_types::RBoxError> {
                // Call the shared function to deserialize from JSON.
                bottlerocket_settings_plugin::deserialize_json::<#ident>(s).map(abi_stable::DynTrait::from_value)
            }

            // Provide the "defaults" function that's required for FFI.
            // This function also refers to the type, with the same caveats as above.
            #[abi_stable::sabi_extern_fn]
            fn default_settings() -> bottlerocket_settings_plugin::BottlerocketSettingsProvider {
                // Requires a Default impl on the type.
                abi_stable::DynTrait::from_value(#ident::default())
            }

            // Make the `deserialize_settings` and `default_settings` functions available via FFI
            // as the exported interface for this plugin.
            #[abi_stable::export_root_module]
            fn get_library() -> bottlerocket_settings_plugin::BottlerocketSettingsPluginRef {
                abi_stable::prefix_type::PrefixTypeTrait::leak_into_prefix(
                    bottlerocket_settings_plugin::BottlerocketSettingsPlugin {
                        default_settings,
                        deserialize_settings,
                    })
            }
        });
    }
}
