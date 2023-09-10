use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use sled::Db;

pub static CACHE: Lazy<Cache> = Lazy::new(|| Cache::new("my_cache.db").unwrap());

pub struct Cache {
    db: Db,
}

impl Cache {
    pub fn new(db_name: &str) -> Result<Self> {
        let db = sled::open(db_name)?;
        Ok(Cache { db })
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        match self.db.get(key) {
            Ok(Some(value)) => Some(value.to_vec()),
            _ => None,
        }
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.db.insert(key, value)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn has(&self, key: &[u8]) -> Result<bool> {
        match self.db.contains_key(key) {
            Ok(exists) => Ok(exists),
            Err(_) => Err(anyhow!("Failed to check for key")),
        }
    }
}
