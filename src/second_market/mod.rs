use serde::{Deserialize, Serialize};

pub(crate) mod fetcher;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Floor {
  pub id: String,
  pub price: f64,
  pub source: String,
  pub created_at: String,
  pub previous_price: f64,
}
