use anyhow::{anyhow, Result};
use log::error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use worker::Env;

#[derive(Serialize)]
struct RequestBody {
  url: String,
}

#[derive(Deserialize)]
struct ResponseBody {
  sqid: String,
  url: String,
}

pub struct Link {
  endpoint: String,
}

impl Link {
  pub fn new(endpoint: String) -> Self {
    Self { endpoint }
  }

  pub fn new_from_env(env: &Env) -> Self {
    match env.var("LINK_GENERATOR_ENDPOINT") {
      Ok(endpoint) => Self::new(endpoint.to_string()),
      Err(e) => {
        error!("Failed to get LINK_GENERATOR_ENDPOINT: {}", e);
        panic!();
      }
    }
  }

  pub async fn generate(&self, url: String) -> Result<String> {
    let client = Client::new();
    let body = RequestBody { url: url.clone() };

    let res = client.post(&self.endpoint).json(&body).send().await?;

    match res.json::<ResponseBody>().await {
      Ok(v) => {
        if url == v.url {
          return Ok(format!("{}/{}", &self.endpoint, v.sqid));
        }
        Ok(url)
      }
      Err(e) => {
        error!("Failed to deserialize response body: {}", e);
        Err(anyhow!("Failed to deserialize response body: {}", e))
      }
    }
  }
}
