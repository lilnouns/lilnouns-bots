use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
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
    pub fn new(path: &str) -> Cache {
        let storage = sled::open(path).expect("open");
        Cache {
            storage: Arc::new(Mutex::new(storage)),
        }
    }

    pub fn get(&self, key: Vec<u8>) -> Result<Option<Vec<u8>>, sled::Error> {
        let storage = self
            .storage
            .lock()
            .map_err(|_| sled::Error::Unsupported("Failed to acquire lock".to_string()))?;
        storage
            .get(key)
            .map(|opt_ivec| opt_ivec.map(|ivec| ivec.to_vec()))
    }

    pub fn set(&self, key: &[u8], value: &[u8]) -> Result<(), sled::Error> {
        let storage = self
            .storage
            .lock()
            .map_err(|_| sled::Error::Unsupported("Failed to acquire lock".to_string()))?;
        storage.insert(key, value).map(|_| ())
    }

    pub fn has(&self, key: &[u8]) -> Result<bool, sled::Error> {
        let storage = self
            .storage
            .lock()
            .map_err(|_| sled::Error::Unsupported("Failed to acquire lock".to_string()))?;
        storage.contains_key(key)
    }

    pub fn remove(&self, key: &[u8]) -> Result<Option<Vec<u8>>, sled::Error> {
        let storage = self
            .storage
            .lock()
            .map_err(|_| sled::Error::Unsupported("Failed to acquire lock".to_string()))?;
        storage
            .remove(key)
            .map(|opt_ivec| opt_ivec.map(|ivec| ivec.to_vec()))
    }
}
