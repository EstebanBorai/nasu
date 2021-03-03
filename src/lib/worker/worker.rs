use anyhow::Result;
use chrono::{DateTime, Utc};
use cron::Schedule;
use std::str::FromStr;
use tokio::sync::RwLock;

use crate::providers::http;
use crate::report::Report;
use crate::tasks::{Task, TaskType};

use super::perform::Perform;

pub struct Worker {
    perform: Box<dyn Perform + Sync + Send>,
    interval: Schedule,
    last_run_at: RwLock<Option<DateTime<Utc>>>,
}

impl Worker {
    /// Performs this Worker's task
    pub async fn perform_task(&self) -> Result<Report> {
        let report = self.perform.perform().await?;
        *self.last_run_at.write().await = Some(Utc::now());

        Ok(report)
    }

    /// Checks if the Worker must be executed or not based
    /// on the defined interval
    pub async fn must_run(&self) -> bool {
        if let Some(last_run_at) = *self.last_run_at.read().await {
            let now = Utc::now();
            let next_run_at = self.interval.after(&last_run_at).next().unwrap();

            if now > next_run_at {
                return true;
            }

            return false;
        }

        true
    }
}

impl From<Task> for Worker {
    fn from(t: Task) -> Self {
        match t.task_type {
            TaskType::Http => {
                let interval = Schedule::from_str(&t.task.interval)
                    .expect(&format!("Invalid chron expression: {}", t.task.interval));
                let http_provider = http::Provider::new(t).unwrap();

                Self {
                    perform: Box::new(http_provider),
                    interval,
                    last_run_at: RwLock::new(None),
                }
            }
        }
    }
}
