use serde::Deserialize;

use crate::utils::ens::get_wallet_handle;

#[derive(Deserialize, Debug)]
struct ApiResponse {
  result: ApiResult,
}

#[derive(Deserialize, Debug)]
struct ApiResult {
  user: User,
}

#[derive(Deserialize, Debug)]
struct User {
  username: String,
}

pub async fn get_username_by_address(api_key: &str, address: &str) -> String {
  let client = reqwest::Client::new();
  let url = format!(
    "https://build.far.quest/farcaster/v2/user-by-connected-address?address={}",
    address
  );

  let response = match client
    .get(&url)
    .header("API-KEY", api_key)
    .header("accept", "application/json")
    .send()
    .await
  {
    Ok(res) => res,
    Err(_) => return get_wallet_handle(address, "xyz.farcaster").await,
  };

  if response.status().is_success() {
    match response.json::<ApiResponse>().await {
      Ok(api_response) => format!("@{}", api_response.result.user.username),
      Err(_) => get_wallet_handle(address, "xyz.farcaster").await,
    }
  } else {
    get_wallet_handle(address, "xyz.farcaster").await
  }
}
