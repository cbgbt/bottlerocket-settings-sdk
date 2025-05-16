mod linear;
mod null;

mod common {
    macro_rules! define_model {
        ($name:ident, $version:expr) => {
            #[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
            pub struct $name;

            impl SettingsModel for $name {
                type PartialKind = Self;
                type ErrorKind = anyhow::Error;

                fn get_version() -> &'static str {
                    $version
                }

                fn set(_: Option<Self>, _: Self) -> Result<()> {
                    unimplemented!()
                }
            }
        };
    }

    pub(crate) use define_model;
}
