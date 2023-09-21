use log::error;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use worker::kv::KvError;
use worker::Env;

pub struct Cache<'a> {
    store_name: String,
    env: &'a Env,
}

impl<'a> Cache<'a> {
    pub fn new(store_name: String, env: &'a Env) -> Self {
        Self { store_name, env }
    }

    pub fn from(env: &'a Env) -> Self {
        let store_name = env.var("KV_STORE_NAME").unwrap().to_string();

        Self::new(store_name, env)
    }

    pub async fn put<T: Serialize>(&self, key: &str, value: &T) {
        let kv = self.env.kv(&self.store_name).unwrap();
        if let Ok(put) = kv.put(key, value) {
            if let Err(pe) = put.execute().await {
                error!("Failed updating KV: {}", pe)
            }
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, KvError> {
        let kv = self.env.kv(&self.store_name).unwrap();

        kv.get(key).json::<T>().await
    }
}
