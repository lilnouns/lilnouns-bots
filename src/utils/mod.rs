use cfg_if::cfg_if;

pub(crate) mod ens;
pub(crate) mod link;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        pub use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

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
