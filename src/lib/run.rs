use anyhow::{Context, Error, Result};
use tokio::sync::mpsc::channel;

use crate::providers::http::Report as HttpReport;
use crate::report::Report;
use crate::tasks::adapters::from_file;
use crate::utils::timestamp::current_timestamp;
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
        println!(
            "{0: <15} | {1: <15} | {2: <20} | {3: <15} | {4: <15}",
            "Log Time", "Task", "HTTP. Status Code", "Req. Time", "Res. Time"
        );
        println!("==========================================================================================");
        while let Some(report) = rx.recv().await {
            match report {
                Report::Http(HttpReport {
                    id,
                    req_end,
                    req_start,
                    status_code,
                    ..
                }) => {
                    println!(
                        "{0: <15} | {1: <15} | {2: <20} | {3: <15} | {4: <15}",
                        current_timestamp(),
                        id,
                        status_code,
                        req_start,
                        req_end
                    );
                }
            }
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
