use crate::providers::http;

#[derive(Debug)]
pub enum Report {
    Http(http::Report),
}
