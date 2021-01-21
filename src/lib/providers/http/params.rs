use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    pub(crate) url: String,
    pub(crate) method: String,
    pub(crate) headers: HashMap<String, String>,
}
