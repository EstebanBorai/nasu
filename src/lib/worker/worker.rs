use anyhow::Result;

use crate::providers::http;
use crate::report::Report;
use crate::tasks::{Task, TaskType};

use super::perform::Perform;

pub struct Worker {
    perform: Box<dyn Perform + Sync + Send>,
}

impl Worker {
    pub async fn perform_task(&self) -> Result<Report> {
        self.perform.perform().await
    }
}

impl From<Task> for Worker {
    fn from(t: Task) -> Self {
        match t.task_type {
            TaskType::Http => {
                let http_provider = http::Provider::new(t).unwrap();

                Self {
                    perform: Box::new(http_provider),
                }
            }
        }
    }
}
