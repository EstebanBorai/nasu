use tokio::sync::mpsc::Sender;

use crate::report::Report;

use super::worker::Worker;

pub struct Squad {
    tx: Sender<Report>,
    workers: Vec<Worker>,
}

impl Squad {
    pub fn new(workers: Vec<Worker>, tx: Sender<Report>) -> Self {
        Self { tx, workers }
    }

    pub fn start(&self) {
        // Implement Loop for Request on Perform
    }
}
