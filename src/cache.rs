use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
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
    pub fn get(&self, key: Vec<u8>) -> Result<Option<Vec<u8>>> {
        let storage = self
            .storage
            .lock()
            .map_err(|_| anyhow!("Failed to acquire lock"))?;
        storage
            .get(key)
            .map_err(|err| anyhow!(err))
            .map(|opt_ivec| opt_ivec.map(|ivec| ivec.to_vec()))
    }

    pub fn set(&self, key: &[u8], value: &[u8]) -> Result<()> {
        let storage = self
            .storage
            .lock()
            .map_err(|_| anyhow!("Failed to acquire lock"))?;
        storage
            .insert(key, value)
            .map_err(|err| anyhow!(err))
            .map(|_| ())
    }
}
