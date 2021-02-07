use hyper::client::HttpConnector;
use hyper::Client;
use hyper::{Method, Uri};
use hyper_tls::HttpsConnector;

use crate::resume::Resume;
use crate::utils::current_moment;

use super::Config;

/// A `Provider` is an agent which task is to perform
/// an action on an specific `Target`.
///
/// The `http::Provider` is used to perform HTTP requests
/// on the specified `Target`.
pub struct Provider {
    pub(crate) client: Client<HttpsConnector<HttpConnector>>,
    pub(crate) target: Config,
    uri: Uri,
}

impl Provider {
    /// Creates a new `Provider` for HTTP requests.
    pub fn new(target: Config) -> Self {
        let url = target.url.to_string();
        let uri: Uri = url.parse().unwrap();

        Self {
            client: Client::builder().build(HttpsConnector::new()),
            target,
            uri,
        }
    }

    /// Performs `Provider`'s taks on the `Target`
    pub async fn perform(&self) -> Resume {
        match self.target.method {
            Method::GET => {
                match self.client.get(self.uri.clone()).await {
                    Ok(_) => {
                        // The fragment must be parsed, currently
                        // we are returning an empty `Vec`
                        // Perhaps is a good idea to have `fragment` as
                        // an `Option<Vec<u8>>` instance instead
                        Resume {
                            host: self.uri.to_string(),
                            fragment: Vec::new(),
                            is_success: true,
                            issued_at: current_moment(),
                        }
                    }
                    Err(err) => Resume {
                        host: self.uri.to_string(),
                        fragment: err.to_string().as_bytes().to_vec(),
                        is_success: false,
                        issued_at: current_moment(),
                    },
                }
            }
            _ => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bo;

    use super::*;

    #[test]
    fn it_performs_a_target() {
        let target = Config::new("https://httpbin.org/status/200", "GET");
        let provider = Provider::new(target);

        let resume = bo!(provider.perform());

        assert!(resume.is_success);
        assert_eq!(resume.fragment, Vec::new());
        assert_eq!(resume.host, "https://httpbin.org/status/200");
    }
}
