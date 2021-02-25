use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::interval;

use super::worker::Worker;
use crate::report::Report;

pub struct Squad {
    tx: Sender<Report>,
    workers: Vec<Worker>,
}

impl Squad {
    pub fn new(workers: Vec<Worker>, tx: Sender<Report>) -> Self {
        Self { tx, workers }
    }

    pub async fn start(&self) {
        let mut interval = interval(Duration::from_secs(3));

        loop {
            interval.tick().await;

            for worker in self.workers.iter() {
                match worker.perform_task().await {
                    Ok(report) => self.tx.send(report).await.unwrap(),
                    Err(err) => eprintln!("{:?}", err),
                }
            }
        }
    }
}
