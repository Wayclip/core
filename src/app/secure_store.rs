use crate::models::error::WayclipError;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;
use std::{fs, io::ErrorKind, path::PathBuf};

/// An empty struct acting as a wrapper around the actions assosciated with storing secure data.
/// Every method here has to be called in a `blocking` task.
pub struct SecureStore;

impl SecureStore {
    fn get_path() -> Result<PathBuf, WayclipError> {
        let dir = dirs::config_dir()
            .ok_or_else(|| WayclipError::NotFound("No config dir found".into()))?;
        Ok(dir.join("wayclip").join("session"))
    }

    /// A method that allows us to store any data that can be serialised
    pub async fn store<T>(&self, data: T) -> Result<(), WayclipError>
    where
        T: Serialize + Send + 'static,
    {
        tokio::task::spawn_blocking(move || {
            let path = Self::get_path()?;
            if let Some(p) = path.parent() {
                fs::create_dir_all(p)?;
            }

            let json_data = json!(data).to_string();

            match cfg!(unix) {
                true => {
                    use std::io::Write;
                    use std::os::unix::fs::OpenOptionsExt;

                    let mut file = fs::OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .mode(0o600)
                        .open(&path)?;

                    file.write_all(json_data.as_bytes())?;
                }
                false => {
                    fs::write(&path, json_data.as_bytes())?;
                }
            }

            Ok::<(), WayclipError>(())
        })
        .await?
    }

    /// A method that allows us to retrieve any data that can be deserialised into an owned object
    /// from inside the keyring
    pub async fn get<T>(&self) -> Result<Option<T>, WayclipError>
    where
        T: DeserializeOwned + Send + 'static,
    {
        tokio::task::spawn_blocking(move || {
            let path = Self::get_path()?;

            if !path.exists() {
                return Ok(None);
            }

            let content = fs::read_to_string(path)?;
            if content.trim().is_empty() {
                return Ok(None);
            }

            let data: T = serde_json::from_str(&content)?;
            Ok(Some(data))
        })
        .await?
    }

    /// A method that allows us to clear all the data stored
    pub async fn clear(&self) -> Result<(), WayclipError> {
        tokio::task::spawn_blocking(move || {
            let path = Self::get_path()?;

            if path.exists() {
                if let Err(err) = fs::remove_file(&path) {
                    if err.kind() != ErrorKind::NotFound {
                        return Err(err.into());
                    }
                }
            }

            Ok::<(), WayclipError>(())
        })
        .await?
    }
}
