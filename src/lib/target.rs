use crate::http::Config as HttpConfig;

#[derive(Debug)]
pub enum Target {
    Http(HttpConfig),
}
