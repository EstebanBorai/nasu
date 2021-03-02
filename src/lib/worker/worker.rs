use anyhow::Result;
use chrono::{DateTime, Utc};
use cron::Schedule;
use std::str::FromStr;

use crate::providers::http;
use crate::report::Report;
use crate::tasks::{Task, TaskType};

use super::perform::Perform;

pub struct Worker {
    perform: Box<dyn Perform + Sync + Send>,
    interval: Schedule,
    last_run_at: Option<DateTime<Utc>>,
}

impl Worker {
    /// Performs this Worker's task
    pub async fn perform_task(&self) -> Result<Report> {
        self.perform.perform().await
    }

    /// Checks if the Worker must be executed or not based
    /// on the defined interval
    pub fn must_run(&self) -> bool {
        match self.last_run_at {
            Some(last_run_at) => {
                let now = Utc::now();
                let next_run_at = self.interval.after(&last_run_at).next().unwrap();

                if now > next_run_at {
                    return true;
                }

                false
            }
            None => true,
        }
    }

    pub fn tick(&mut self) {
        self.last_run_at = Some(Utc::now());
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
                    last_run_at: None,
                }
            }
        }
    }
}
