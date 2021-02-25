use anyhow::{Context, Error, Result};
use tokio::sync::mpsc::channel;

use crate::report::Report;
use crate::tasks::adapters::from_file;
use crate::worker::{Squad, Worker};

/// This is likely to change in the future in order
/// to be able to define where to gather the config
/// from. Default for now is "nasu.json"
const CONFIG_FILE_NAME: &str = "nasu.json";

pub async fn run() -> Result<()> {
    let tasks =
        from_file(CONFIG_FILE_NAME).context(format!("Failed to parse \"{}\"", CONFIG_FILE_NAME))?;
    let workers: Vec<Worker> = tasks.into_iter().map(|task| Worker::from(task)).collect();
    let (tx, mut rx) = channel::<Report>(1024);
    let squad = Squad::new(workers, tx);

    let print_proc = tokio::spawn(async move {
        while let Some(report) = rx.recv().await {
            println!("{:#?}", report);
        }
    });

    let run_proc = tokio::spawn(async move {
        squad.start().await;
    });

    tokio::select! {
      _ = print_proc => {
        Err(Error::msg("Output process stopped"))
      },
      _ = run_proc => {
        Err(Error::msg("Run process stopped"))
      },
    }
}
