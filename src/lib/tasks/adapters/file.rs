use anyhow::{Context, Result};
use serde_json::from_str;
use std::fs::read_to_string;
use std::path::Path;

use crate::tasks::Task;

pub fn from_file<P>(path: P) -> Result<Vec<Task>>
where
    P: AsRef<Path> + std::fmt::Debug + std::marker::Copy,
{
    let contents = read_to_string(path).context(format!("Failed to read file at {:?}", path,))?;

    let config = from_str(&contents).context("Invalid config file provided")?;

    Ok(config)
}
