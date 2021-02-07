use std::convert::TryFrom;

use anyhow::Result;

mod config;
mod http;
mod macros;
mod resume;
mod target;
mod utils;

pub(crate) use macros::*;

pub async fn run() -> Result<()> {
    let config_file = config::ConfigFile::read_config_file()?;
    let targets = config::TaskConfig::from_config_file(config_file)?;

    targets.0.into_iter().for_each(|t| println!("{:#?}", t));

    Ok(())
}
