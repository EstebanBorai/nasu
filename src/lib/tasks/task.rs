use serde::{Deserialize, Serialize};

use crate::providers::http::Params as HttpParams;

#[derive(Debug, Deserialize, Serialize)]
pub enum TaskType {
    #[serde(rename = "http")]
    Http,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rules {
    interval: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Params {
    Http(HttpParams),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    #[serde(rename = "type")]
    pub task_type: TaskType,
    pub title: String,
    pub task: Rules,
    pub params: Params,
}
