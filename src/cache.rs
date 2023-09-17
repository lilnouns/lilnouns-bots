use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sled::Db;

lazy_static! {
    pub static ref CACHE: Cache = {
        let storage =
            sled::open("./tmp/cache").unwrap_or_else(|_| panic!("Could not open storage"));
        Cache {
            storage: Arc::new(Mutex::new(storage)),
        }
    };
}

pub struct Cache {
    storage: Arc<Mutex<Db>>,
}

impl Cache {
    pub fn get<T: DeserializeOwned>(&self, key: &String) -> Result<Option<T>> {
        let storage = self.storage.lock().unwrap();
        match storage.get(key) {
            Ok(Some(ivec)) => {
                let data: T = serde_json::from_slice(&ivec)?;
                Ok(Some(data))
            }
            Ok(None) => Ok(None),
            Err(err) => Err(anyhow!("Failed to get key {} from cache: {}", key, err)),
        }
    }

    pub fn set<T: Serialize>(&self, key: &String, value: &T) -> Result<()> {
        let storage = self.storage.lock().unwrap();
        let serialized_value = serde_json::to_vec(value)?;
        storage.insert(key, serialized_value)?;
        Ok(())
    }
}
