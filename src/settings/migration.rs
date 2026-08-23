use crate::{
    models::error::WayclipError,
    settings::schema::{MigrationChange, SCHEMA},
};
use serde_json::Value;

/// A wrapper struct responsible for migrating the config to newer versions
pub struct SettingsMigrate;

impl SettingsMigrate {
    /// Method to update from one version to the other
    /// Limited to only upgrading
    pub fn migrate(
        config: &mut Value,
        from: semver::Version,
        to: semver::Version,
    ) -> Result<(), WayclipError> {
        let mut versions: Vec<_> = SCHEMA
            .versions
            .iter()
            .filter(|v| v.version > from && v.version <= to)
            .collect();

        versions.sort_by(|a, b| a.version.cmp(&b.version));

        for entry in versions {
            for change in &entry.changes {
                change.implement(config)?;
            }
        }

        config["v"] = Value::String(to.to_string());

        Ok(())
    }
}

impl MigrationChange {
    /// Method to implement a migration change
    pub fn implement(&self, json: &mut Value) -> Result<(), WayclipError> {
        match self {
            MigrationChange::Insert { setting_id } => {
                let def =
                    SCHEMA.settings.get(setting_id).ok_or_else(|| {
                        WayclipError::Validation(format!(
                        "Migration error: setting_id '{setting_id}' not found in TOML schema"
                    ).into())
                    })?;

                let node = Self::find_or_create_node(json, &def.location)?;
                if let Some(obj) = node.as_object_mut() {
                    obj.entry(&def.field_name)
                        .or_insert_with(|| def.default.clone());
                } else {
                    return Err(WayclipError::Validation(
                        "Target location is not a JSON object".into(),
                    ));
                }
            }

            MigrationChange::Move {
                from_path,
                to_path,
                field,
            } => {
                let from_node = Self::find_node(json, from_path)?;

                let removed_value = if let Some(obj) = from_node.and_then(|n| n.as_object_mut()) {
                    obj.remove(field)
                } else {
                    None
                };

                if let Some(value) = removed_value {
                    let to_node = Self::find_or_create_node(json, to_path)?;
                    if let Some(obj) = to_node.as_object_mut() {
                        obj.insert(field.clone(), value);
                    } else {
                        return Err(WayclipError::Validation(
                            "Destination location is not a JSON object".into(),
                        ));
                    }
                }
            }

            MigrationChange::Remove {
                location,
                field_name,
            } => {
                let node = Self::find_node(json, location)?;
                if let Some(obj) = node.and_then(|n| n.as_object_mut()) {
                    obj.remove(field_name);
                }
            }
        }

        Ok(())
    }

    fn find_or_create_node<'a>(
        root: &'a mut Value,
        path: &str,
    ) -> Result<&'a mut Value, WayclipError> {
        let mut current = root;
        if !path.is_empty() {
            for part in path.split('.') {
                current = current
                    .as_object_mut()
                    .ok_or_else(|| {
                        WayclipError::Validation(
                            format!("Path segment '{part}' is not a JSON object").into(),
                        )
                    })?
                    .entry(part)
                    .or_insert_with(|| serde_json::json!({}));
            }
        }
        Ok(current)
    }

    fn find_node<'a>(
        root: &'a mut Value,
        path: &str,
    ) -> Result<Option<&'a mut Value>, WayclipError> {
        let mut current = root;

        if path.is_empty() {
            return Ok(Some(current));
        }

        for part in path.split('.') {
            match current.as_object_mut() {
                Some(obj) => match obj.get_mut(part) {
                    Some(next) => current = next,
                    None => return Ok(None),
                },
                None => return Ok(None),
            }
        }

        Ok(Some(current))
    }
}
