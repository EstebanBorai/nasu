use anyhow::Result;
use async_trait::async_trait;

use crate::report::Report;

#[async_trait]
pub trait Perform {
    async fn perform(&self) -> Result<Report>;
}
