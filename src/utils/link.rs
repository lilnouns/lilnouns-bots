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
  fn new(endpoint: String) -> Self {
    Self { endpoint }
  }

  pub fn new_from_env(env: &Env) -> Self {
    let endpoint = env.var("LINK_GENERATOR_ENDPOINT").unwrap().to_string();

    Self::new(endpoint)
  }

  pub async fn generate(&self, url: String) -> String {
    let client = Client::new();
    let body = RequestBody { url: url.clone() };

    let res = client
      .post(&self.endpoint)
      .json(&body)
      .send()
      .await
      .unwrap()
      .json::<ResponseBody>()
      .await
      .unwrap();

    if url == res.url {
      return format!("{}/{}", &self.endpoint, res.sqid);
    }

    url
  }
}
