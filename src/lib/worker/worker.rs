use crate::providers::http;
use crate::tasks::{Task, TaskType};

use super::perform::Perform;

pub struct Worker {
    perform: Box<dyn Perform>,
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
