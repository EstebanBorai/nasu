use anyhow::Error;
use core::panic;
use hyper::Method;
use std::convert::TryFrom;
use std::str::FromStr;
use url::Url;

use crate::config::dto::HttpConfig;

#[derive(Debug)]
pub struct Config {
    pub(crate) title: String,
    pub(crate) method: Method,
    pub(crate) url: Url,
}

impl Config {
    pub fn new(title: &str, url: &str, method: &str) -> Self {
        // Errors must be implemented, unwrap is not okay
        // at this point
        let url = Url::from_str(url).unwrap();
        let method = match Method::from_str(method) {
            Ok(m) => m,
            Err(e) => panic!(&e.to_string()),
        };

        Self {
            title: title.to_string(),
            method,
            url,
        }
    }
}

impl TryFrom<HttpConfig> for Config {
    type Error = Error;

    fn try_from(value: HttpConfig) -> Result<Self, Self::Error> {
        let method = Method::from_str(value.method.as_str()).map_err(|e| {
            Error::msg(format!(
                "Invalid method value provided for target: {}. {}",
                value.title,
                e.to_string()
            ))
        })?;
        let url = Url::from_str(value.url.as_str()).map_err(|e| {
            Error::msg(format!(
                "Invalid URL provided for target: {}. {}",
                value.title,
                e.to_string()
            ))
        })?;

        Ok(Config {
            title: value.title,
            method,
            url,
        })
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn it_creates_a_target() {
        let target = Config::new("Reddit", "https://reddit.com", "GET");

        assert_eq!(target.method, Method::GET);
        assert_eq!(target.url, Url::from_str("https://reddit.com").unwrap());
    }
}
