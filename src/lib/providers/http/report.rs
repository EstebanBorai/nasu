use std::collections::HashMap;

#[derive(Debug)]
pub struct Report {
    pub headers: HashMap<String, String>,
    pub status_code: u16,
}
