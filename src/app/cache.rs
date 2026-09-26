use crate::models::error::WayclipError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{collections::HashMap, path::PathBuf};

/// A global key used for accessing the user data stored in 'cache'
pub const USERS_ME_DATA_KEY: &'static str = "users_me_data";
/// A global TTL used for defining user data. 1 Hour
pub const USERS_ME_DATA_TTL: i64 = 3600;

/// A struct to store a hashmap of all the entries for cache. Will store it in local cache dir
pub struct Cache<T> {
    cache_path: PathBuf,
    /// All the entries, indexed using a key
    entries: HashMap<String, CachedData<T>>,
}

impl<T: Serialize + DeserializeOwned + Clone> Cache<T> {
    /// Initialise & pull all the exisitng entries from file
    pub async fn init() -> Result<Self, WayclipError> {
        let cache_path = dirs::cache_dir()
            .ok_or_else(|| WayclipError::NotFound("No cache dir found".into()))?
            .join("wayclip")
            .join("data.json");

        let bytes = tokio::fs::read(&cache_path).await?;
        let entries = serde_json::from_slice::<HashMap<String, CachedData<T>>>(&bytes)?;

        Ok(Self {
            cache_path,
            entries,
        })
    }

    /// Insert a new item into the cache & immediately write to disk
    pub async fn insert_item(
        &mut self,
        key: String,
        data: T,
        ttl: i64,
    ) -> Result<CachedData<T>, WayclipError> {
        let cached_data = CachedData::new(data, ttl);
        self.entries.insert(key, cached_data.clone());

        let bytes = serde_json::to_vec(&self.entries)?;
        tokio::fs::write(&self.cache_path, &bytes).await?;

        Ok(cached_data)
    }

    /// Try to retrieve an item from cache
    /// If item is not found -> None
    /// If item is found, but expired -> Removed from disk & None
    /// If item is found, and not expired -> Some(T)
    pub async fn get_item(&mut self, key: String) -> Result<Option<T>, WayclipError> {
        match self.entries.get(&key) {
            Some(cached_data) => match cached_data.get() {
                Some(data) => Ok(Some(data)),
                None => {
                    self.entries.remove(&key);
                    let bytes = serde_json::to_vec(&self.entries)?;
                    tokio::fs::write(&self.cache_path, &bytes).await?;

                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }
}

/// An item that was cached in ~/.cache/ of type T, time to live (s) ttl and the time of creation
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct CachedData<T> {
    /// The actual serialized data that is stored on disk
    pub data: T,
    /// How long, in seconds, data can live for.
    /// Used for calculations when retrieving data
    pub ttl: i64,
    /// The timestamp/date when data was fetched/created
    pub created_at: DateTime<Utc>,
}

impl<T: Serialize + DeserializeOwned + Clone> CachedData<T> {
    /// Create a new cached object
    pub fn new(data: T, ttl: i64) -> Self {
        Self {
            data,
            ttl,
            created_at: chrono::Utc::now(),
        }
    }

    /// Get method for cached data.
    /// Return Some(T) if data has not yet exceeded its time to live (ttl), otherwise None is
    /// returned
    pub fn get(&self) -> Option<T> {
        if chrono::Utc::now()
            .signed_duration_since(self.created_at)
            .num_seconds()
            < self.ttl
        {
            Some(self.data.clone())
        } else {
            None
        }
    }
}
