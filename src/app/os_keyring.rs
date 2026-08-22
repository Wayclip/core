use crate::models::error::WayclipError;
use keyring::Entry;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;

pub struct OsKeyring;

const SERVICE: &str = "com.wayclip.cli";
const USERNAME: &str = "session";
const USE_NATIVE_STORE: bool = true;

impl OsKeyring {
    fn get_entry() -> Result<Entry, WayclipError> {
        keyring::cli::use_native_store(USE_NATIVE_STORE)?;
        let entry = Entry::new(SERVICE, USERNAME)?;
        Ok(entry)
    }

    pub async fn store<T>(&self, data: T) -> Result<(), WayclipError>
    where
        T: Serialize + Send + 'static,
    {
        tokio::task::spawn_blocking(move || {
            let entry = Self::get_entry()?;
            let json_data = json!(data).to_string();
            // set_password is used because it handles UTF-8 strings, whilst set_secret handles &u8
            entry.set_password(&json_data)?;
            Ok::<(), WayclipError>(())
        })
        .await?
    }

    pub async fn get<T>(&self) -> Result<Option<T>, WayclipError>
    where
        T: DeserializeOwned + Send + 'static,
    {
        tokio::task::spawn_blocking(move || {
            let entry = Self::get_entry()?;
            let password = entry.get_password()?;
            if password.is_empty() {
                Ok::<Option<T>, WayclipError>(None)
            } else {
                Ok::<Option<T>, WayclipError>(Some(serde_json::from_str(&password)?))
            }
        })
        .await?
    }

    pub async fn clear(&self) -> Result<(), WayclipError> {
        tokio::task::spawn_blocking(move || {
            let entry = Self::get_entry()?;
            // we ensure that error recieved is not a NoEntry error, so that we can clear as many
            // times as we want, even if its already non-existant
            if let Err(err) = entry.delete_credential()
                && !matches!(err, keyring::Error::NoEntry)
            {
                Err(err.into())
            } else {
                Ok::<(), WayclipError>(())
            }
        })
        .await?
    }
}
