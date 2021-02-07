use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::from_str as json_parse;
use std::fs;

use super::dto::HttpConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile(pub(crate) Vec<ConfigFileTarget>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ConfigFileTarget {
    #[serde(rename = "http")]
    Http(HttpConfig),
}

impl ConfigFile {
    pub fn read_config_file() -> Result<Self> {
        let file = fs::read_to_string("nasu.json").map_err(|e| {
            Error::msg(format!(
                "An error ocurred reading the config file. {}",
                e.to_string()
            ))
        })?;
        let config: ConfigFile = json_parse(file.as_str())
            .map_err(|e| Error::msg(format!("Unable to parse config file. {}", e.to_string())))?;

        Ok(config)
    }
}
