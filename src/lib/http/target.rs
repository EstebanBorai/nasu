use core::panic;
use hyper::Method;
use std::str::FromStr;
use url::Url;

pub struct Target {
  pub(crate) method: Method,
  pub(crate) url: Url,
}

impl Target {
  pub fn new(url: &str, method: &str) -> Self {
    // Errors must be implemented, unwrap is not okay
    // at this point
    let url = Url::from_str(url).unwrap();
    let method = match Method::from_str(method) {
      Ok(m) => m,
      Err(e) => panic!(&e.to_string()),
    };

    Self { method, url }
  }
}

#[cfg(test)]
mod tests {
  use core::panic;

  use super::*;

  #[test]
  fn it_creates_a_target() {
    let target = Target::new("https://reddit.com", "GET");

    assert_eq!(target.method, Method::GET);
    assert_eq!(target.url, Url::from_str("https://reddit.com").unwrap());
  }
}
