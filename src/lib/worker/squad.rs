use std::sync::Arc;
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

    pub async fn start(self) {
        let workers = Arc::new(
            self.workers
                .into_iter()
                .map(|w| Arc::new(w))
                .collect::<Vec<Arc<Worker>>>(),
        );
        let tx = Arc::new(self.tx);
        let mut interval = interval(Duration::from_secs(3));

        loop {
            interval.tick().await;
            let workers = Arc::clone(&workers);
            let tx = Arc::clone(&tx);

            for worker in workers.iter() {
                let worker = Arc::clone(worker);
                let tx = Arc::clone(&tx);

                tokio::spawn(async move {
                    match worker.perform_task().await {
                        Ok(report) => tx.send(report).await.unwrap(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                });
            }
        }
    }
}
