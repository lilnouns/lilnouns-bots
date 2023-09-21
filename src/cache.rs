use log::error;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use worker::kv::{KvError, KvStore};
use worker::Env;

pub struct Cache {
    store: KvStore,
}

impl Cache {
    pub fn new(store: KvStore) -> Self {
        Self { store }
    }

    pub fn from(env: &Env) -> Self {
        let store_name = env.var("KV_STORE_NAME").unwrap().to_string();
        let store = env.kv(&store_name).unwrap();

        Self::new(store)
    }

    pub async fn put<T: Serialize>(&self, key: &str, value: &T) {
        if let Ok(put) = self.store.put(key, value) {
            if let Err(pe) = put.execute().await {
                error!("Failed updating KV: {}", pe)
            }
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, KvError> {
        self.store.get(key).json::<T>().await
    }
}
