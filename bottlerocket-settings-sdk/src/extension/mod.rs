pub mod proto1;

use std::collections::HashMap;

use anyhow::Context;

use crate::cli;
use crate::error::Result;
use crate::model::Model;

#[derive(Debug)]
pub struct SettingsExtension {
    pub(crate) models: HashMap<String, Box<dyn Model>>,
}

impl SettingsExtension {
    pub fn with_models(models: Vec<Box<dyn Model>>) -> Self {
        let models = models
            .into_iter()
            .map(|model| (model.get_version().to_string(), model))
            .collect();
        Self { models }
    }

    pub fn run_extension(self) -> Result<()> {
        let args = cli::Cli::parse_args();
        match args.protocol {
            cli::Protocol::Proto1(p) => crate::extension::proto1::run_extension(self, p.command),
        }
        Ok(())
    }

    fn model(&self, version: &str) -> Option<&dyn Model> {
        self.models.get(version).map(|i| i.as_ref())
    }

    // TODO abstract migration out into a pluggable "migrator"
    /// Performs a migration of data from a starting model version to a requested one.
    pub(crate) fn perform_migration(
        &self,
        starting_value: serde_json::Value,
        starting_version: &str,
        target_version: &str,
    ) -> Result<serde_json::Value> {
        let starting_model = self.models.get(starting_version).context(format!(
            "Could not find model for starting version '{}'",
            starting_version
        ))?;
        self
            .find_migration_route(starting_version, target_version)
            .context(format!(
                "Could not find a defined migration for '{}' to '{}'",
                starting_version, target_version
            ))?
            .try_fold(
                (starting_value, starting_model),
                |(curr_value, curr_model), next| match next {
                    // TODO moving to `snafu` will clean up a lot of these error types.
                    MigrationDirection::Forward => {
                        let next_model = self
                            .models
                            .get(curr_model.migrates_forward_to().context(
                                "Failed to find forward migration which was previously found.",
                            )?)
                            .context(
                                "Failed to find forward migration which was previously found.",
                            )?;
                        let next_value = curr_model.migrate_forward(curr_value)
                            .context(format!("Failed to perform sub-migration from '{}' to '{}' as part of migration from '{}' to '{}'",
                                curr_model.get_version(), next_model.get_version(), starting_version, target_version))?;
                        Ok((next_value, next_model))
                    }
                    MigrationDirection::Backward => {
                        let next_model = self
                            .models
                            .get(curr_model.migrates_backward_to().context(
                                "Failed to find backward migration which was previously found.",
                            )?)
                            .context(
                                "Failed to find backward migration which was previously found.",
                            )?;
                        let next_value = curr_model.migrate_backward(curr_value)
                            .context(format!("Failed to perform sub-migration from '{}' to '{}' as part of migration from '{}' to '{}'",
                                curr_model.get_version(), next_model.get_version(), starting_version, target_version))?;
                        Ok((next_value, next_model))
                    }
                },
            )
            .map(|(final_value, _)| final_value)
    }

    /// Returns an iterator of migrations to be performed to transform data from a starting version to a target version.
    fn find_migration_route(
        &self,
        starting_version: &str,
        target_version: &str,
    ) -> Option<impl Iterator<Item = MigrationDirection>> {
        // Searches through the migrations in the given direction. If we find the target version,
        // we return the number of migrations to perform in the given direction to reach that version.
        let search_in_direction = |direction: MigrationDirection| {
            self.migration_iter(starting_version, direction)
                .enumerate()
                .find(|(_ndx, model)| model.get_version() == target_version)
                .map(|(ndx, _)| (ndx, direction))
        };
        let search_forward = || search_in_direction(MigrationDirection::Forward);
        let search_backward = || search_in_direction(MigrationDirection::Backward);

        search_forward()
            .or_else(search_backward)
            .map(|(ndx, direction)| std::iter::repeat(direction).take(ndx))
    }

    /// Iterate through the extensions chain of model migrations, starting at a given version.
    fn migration_iter(
        &self,
        starting_version: &str,
        direction: MigrationDirection,
    ) -> MigrationIter {
        MigrationIter {
            models: &self.models,
            next: self.models.get(starting_version).map(|i| i.as_ref()),
            direction,
        }
    }
}

/// Helper type for `SettingsExtension` designed to iterate through linear model migration chains.
struct MigrationIter<'a> {
    models: &'a HashMap<String, Box<dyn Model>>,
    next: Option<&'a dyn Model>,
    direction: MigrationDirection,
}

impl<'a> Iterator for MigrationIter<'a> {
    type Item = &'a dyn Model;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next?;

        self.next = match &self.direction {
            MigrationDirection::Forward => next.migrates_forward_to(),
            MigrationDirection::Backward => next.migrates_backward_to(),
        }
        .and_then(|next_version| self.models.get(next_version).map(|i| i.as_ref()));

        Some(next)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum MigrationDirection {
    Forward,
    Backward,
}
