use std::fmt::Debug;
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
    pub fn get<K, V>(&self, key: &K) -> Result<Option<V>>
    where
        K: Serialize + Debug,
        V: DeserializeOwned,
    {
        let storage = self.storage.lock().unwrap();
        let key_bytes = serde_json::to_vec(&key)?;

        match storage.get(key_bytes) {
            Ok(Some(value_bytes)) => {
                let value: V = serde_json::from_slice(&value_bytes)?;
                Ok(Some(value))
            }
            Ok(None) => Ok(None),
            Err(err) => Err(anyhow!("Failed to get key {:?} from cache: {:?}", key, err)),
        }
    }

    pub fn set<K, V>(&self, key: K, value: V) -> Result<()>
    where
        K: Serialize + Debug,
        V: Serialize,
    {
        let storage = self.storage.lock().unwrap();
        let key_bytes = serde_json::to_vec(&key)?;
        let value_bytes = serde_json::to_vec(&value)?;

        storage.insert(key_bytes, value_bytes)?;

        Ok(())
    }
}
