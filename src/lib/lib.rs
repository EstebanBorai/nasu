mod http;
mod macros;
mod resume;
mod utils;

pub(crate) use macros::*;

pub fn hello() {
  println!("Welcome to Nasu! 🧑🏻‍⚕️ Command-line utility which poll on remote addresses in order to perform status checks periodically");
}

/// HTTP Request parameters to provided to the
/// `perform_http` function.
#[derive(Debug)]
pub struct HttpTarget {
  pub url: String,
  pub method: String,
}

/// Details gathered from the `HttpTarget` performance
#[derive(Debug, PartialEq)]
pub struct HttpResult {
  pub(crate) status_code: u16,
  pub(crate) body: Option<Vec<u8>>,
}

pub async fn perform_http(_: &HttpTarget) -> HttpResult {
  // A HTTP Request must be performed in order to
  // built a `HttpResult` struct.
  //
  // The current value is provided as an example of
  // the expected response
  HttpResult {
    status_code: 200_u16,
    body: None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_retrieves_http_response() {
    let http_target = HttpTarget {
      url: String::from("https://httpbin.org/status/200"),
      method: String::from("GET"),
    };

    let http_result = HttpResult {
      status_code: 200 as u16,
      body: None,
    };

    assert_eq!(bo!(perform_http(&http_target)), http_result);
  }
}
