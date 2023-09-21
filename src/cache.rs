use log::{debug, error};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use worker::kv::{KvError, KvStore};
use worker::Env;

pub struct Cache {
    store: KvStore,
}

impl Cache {
    pub fn new(store: KvStore) -> Self {
        debug!("Creating new Cache instance");
        Self { store }
    }

    pub fn from(env: &Env) -> Self {
        debug!("Creating Cache from Env");
        let store_name = env.var("KV_STORE_NAME").unwrap().to_string();
        let store = env.kv(&store_name).unwrap();

        Self::new(store)
    }

    pub async fn put<T: Serialize>(&self, key: &str, value: &T) {
        debug!("Putting value in KV Store");
        if let Ok(put) = self.store.put(key, value) {
            if let Err(pe) = put.execute().await {
                error!("Failed updating KV: {}", pe);
            } else {
                debug!("Successfully updated KV");
            }
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, KvError> {
        debug!("Getting value from KV Store");
        self.store.get(key).json::<T>().await
    }
}
