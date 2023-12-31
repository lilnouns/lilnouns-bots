use log::error;
use serde::{de::DeserializeOwned, ser::Serialize};
use worker::{
  kv::{KvError, KvStore},
  Env,
};

#[derive(Clone)]
pub struct Cache {
  store: KvStore,
}

impl Cache {
  pub fn new(store: KvStore) -> Self {
    Self { store }
  }

  pub fn new_from_env(env: &Env) -> Self {
    let store_name = env.var("KV_STORE_NAME").unwrap().to_string();
    let store = env.kv(&store_name).unwrap();

    Self::new(store)
  }

  pub async fn put<T: Serialize>(&self, key: &str, value: &T) {
    if let Ok(put) = self.store.put(key, value) {
      if let Err(pe) = put.execute().await {
        error!("Failed updating KV: {}", pe);
      }
    } else {
      error!("Failed to put key-value pair into the storage");
    }
  }

  pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, KvError> {
    self.store.get(key).json::<T>().await
  }

  pub async fn has(&self, key: &str) -> bool {
    match self.store.list().execute().await {
      Ok(key_list) => key_list.keys.iter().any(|k| k.name == key),
      Err(e) => {
        error!("Failed to retrieve list of keys: {}", e);
        false
      }
    }
  }
}
