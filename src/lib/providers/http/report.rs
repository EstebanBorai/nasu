use std::collections::HashMap;

#[derive(Debug)]
pub struct Report {
    pub id: String,
    pub headers: HashMap<String, String>,
    pub status_code: u16,
    pub req_start: u128,
    pub req_end: u128,
}
