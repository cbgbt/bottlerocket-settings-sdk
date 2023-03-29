//! Two separate traits are exposed to make a model which functions with the Bottlerocket SDK.
//!
//! Consumers will `#[derive(Model)]` and `impl SettingsModel` in order to interface with the SDK.
//!
//! `SettingsModel` is the developer interface for implementing a model. Because th e
//!
use std::fmt::Debug;
use std::{any::TypeId, marker::PhantomData};

use anyhow::{Context, Result};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait SettingsModel: Sized + Serialize + DeserializeOwned + Debug {
    type PartialType: Serialize + DeserializeOwned;
    type ForwardMigrationTarget: 'static + SettingsModel;
    type BackwardMigrationTarget: 'static + SettingsModel;

    fn get_version() -> &'static str;

    fn migrates_forward_to() -> Option<&'static str> {
        if TypeId::of::<Self::ForwardMigrationTarget>() == TypeId::of::<NoMigration>() {
            None
        } else {
            Some(Self::ForwardMigrationTarget::get_version())
        }
    }
    fn migrate_forward(self) -> Result<Self::ForwardMigrationTarget>;

    fn migrates_backward_to() -> Option<&'static str> {
        if TypeId::of::<Self::BackwardMigrationTarget>() == TypeId::of::<NoMigration>() {
            None
        } else {
            Some(Self::BackwardMigrationTarget::get_version())
        }
    }
    fn migrate_backward(self) -> Result<Self::BackwardMigrationTarget>;

    fn set(current_value: Option<Self>, target: Self) -> Result<Self>;

    fn generate(
        existing_partial: Option<Self::PartialType>,
        dependent_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult<Self::PartialType, Self>>;

    fn validate(_value: Self, _validated_settings: Option<serde_json::Value>) -> Result<bool>;
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct NoMigration;
impl NoMigration {
    pub fn no_defined_migration() -> Result<Self> {
        Ok(NoMigration)
    }
}

impl SettingsModel for NoMigration {
    type PartialType = NoMigration;

    type ForwardMigrationTarget = NoMigration;

    type BackwardMigrationTarget = NoMigration;

    fn get_version() -> &'static str {
        unimplemented!(
            "`NoMigration` used as a marker type. Its settings model should never be used."
        )
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<Self> {
        unimplemented!(
            "`NoMigration` used as a marker type. Its settings model should never be used."
        )
    }

    fn generate(
        _existing_partial: Option<Self::PartialType>,
        _dependent_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult<Self::PartialType, Self>> {
        unimplemented!(
            "`NoMigration` used as a marker type. Its settings model should never be used."
        )
    }

    fn validate(_value: Self, _validated_settings: Option<serde_json::Value>) -> Result<bool> {
        unimplemented!(
            "`NoMigration` used as a marker type. Its settings model should never be used."
        )
    }

    fn migrate_forward(self) -> Result<Self::ForwardMigrationTarget> {
        unimplemented!(
            "`NoMigration` used as a marker type. Its settings model should never be used."
        )
    }

    fn migrate_backward(self) -> Result<Self::BackwardMigrationTarget> {
        unimplemented!(
            "`NoMigration` used as a marker type. Its settings model should never be used."
        )
    }
}

// TODO we use `serde_json::Value` to do type erasure, but we should consider using `std::any::Any`
pub trait Model: Debug {
    fn get_version(&self) -> &'static str;

    fn migrates_forward_to(&self) -> Option<&'static str>;

    fn migrates_backward_to(&self) -> Option<&'static str>;

    fn set(
        &self,
        current: Option<serde_json::Value>,
        target: serde_json::Value,
    ) -> Result<serde_json::Value>;

    fn migrate_forward(&self, current: serde_json::Value) -> Result<serde_json::Value>;

    fn migrate_backward(&self, current: serde_json::Value) -> Result<serde_json::Value>;

    fn generate(
        &self,
        existing_partial: Option<serde_json::Value>,
        dependent_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult<serde_json::Value, serde_json::Value>>;

    fn validate(
        &self,
        value: serde_json::Value,
        validated_settings: Option<serde_json::Value>,
    ) -> Result<bool>;
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GenerateResult<Partial: Serialize, Complete: Serialize> {
    NeedsData(Option<Partial>),
    Complete(Option<Complete>),
}

impl<P: Serialize, C: Serialize> GenerateResult<P, C> {
    pub fn erase_type(self) -> Result<GenerateResult<serde_json::Value, serde_json::Value>> {
        Ok(match self {
            GenerateResult::NeedsData(optional_interior) => GenerateResult::NeedsData(
                optional_interior
                    .map(|i| serde_json::to_value(i))
                    .transpose()?,
            ),
            GenerateResult::Complete(optional_interior) => GenerateResult::Complete(
                optional_interior
                    .map(|i| serde_json::to_value(i))
                    .transpose()?,
            ),
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct BottlerocketSetting<T> {
    phantom: PhantomData<*const T>,
}
impl<T> BottlerocketSetting<T> {
    pub fn model() -> Box<Self> {
        Box::new(Self {
            phantom: PhantomData,
        })
    }
}

impl<T: SettingsModel> Model for BottlerocketSetting<T> {
    fn get_version(&self) -> &'static str {
        T::get_version()
    }

    fn migrates_forward_to(&self) -> Option<&'static str> {
        T::migrates_forward_to()
    }

    fn migrates_backward_to(&self) -> Option<&'static str> {
        T::migrates_backward_to()
    }

    fn set(
        &self,
        current: Option<serde_json::Value>,
        target: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let current = current.map(|v| serde_json::from_value(v)).transpose()?;
        let target = serde_json::from_value(target)?;
        T::set(current, target).and_then(|retval| {
            serde_json::to_value(retval).context("Failed to serialize result to JSON")
        })
    }

    fn migrate_forward(&self, current: serde_json::Value) -> Result<serde_json::Value> {
        let current: T =
            serde_json::from_value(current).context("Failed to parse incoming JSON value.")?;
        current.migrate_forward().and_then(|retval| {
            serde_json::to_value(retval).context("Failed to serialize result to JSON")
        })
    }

    fn migrate_backward(&self, current: serde_json::Value) -> Result<serde_json::Value> {
        let current: T = serde_json::from_value(current)?;
        current.migrate_backward().and_then(|retval| {
            serde_json::to_value(retval).context("Failed to serialize result to JSON")
        })
    }

    fn generate(
        &self,
        existing_partial: Option<serde_json::Value>,
        dependent_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult<serde_json::Value, serde_json::Value>> {
        let existing_partial = existing_partial.map(serde_json::from_value).transpose()?;

        T::generate(existing_partial, dependent_settings).and_then(|gr| gr.erase_type())
    }

    fn validate(
        &self,
        value: serde_json::Value,
        validated_settings: Option<serde_json::Value>,
    ) -> Result<bool> {
        let value = serde_json::from_value(value)?;
        T::validate(value, validated_settings)
    }
}
