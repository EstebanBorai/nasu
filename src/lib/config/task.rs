use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::from_str as json_parse;
use std::{convert::TryFrom, fs};

use crate::http::Config as HttpConfig;
use crate::target::Target;
use crate::{
    config::{ConfigFile, ConfigFileTarget},
    target,
};

#[derive(Debug)]
pub struct TaskConfig(pub(crate) Vec<Target>);

impl TaskConfig {
    pub fn from_config_file(config_file: ConfigFile) -> Result<Self> {
        let targets = config_file
            .0
            .into_iter()
            .map(|target_config| match target_config {
                ConfigFileTarget::Http(conf) => Target::Http(HttpConfig::try_from(conf).unwrap()),
            })
            .collect();

        Ok(Self(targets))
    }
}
