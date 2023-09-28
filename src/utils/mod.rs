pub(crate) mod ens;

pub fn get_short_address(address: &str) -> String {
  let first_four: String = address.chars().take(4).collect();
  let last_four: String = address
    .chars()
    .rev()
    .take(4)
    .collect::<Vec<char>>()
    .iter()
    .rev()
    .collect();

  if first_four.len() == 4 && last_four.len() == 4 {
    format!("{}...{}", first_four, last_four)
  } else {
    address.to_string()
  }
}

pub fn get_explorer_address(address: &str) -> String {
  format!("https://etherscan.io/address/{}", address)
}
