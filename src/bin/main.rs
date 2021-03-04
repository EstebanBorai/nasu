//! # ナース (Nāsu)
//! 🧑🏻‍⚕️ Command-line utility which poll on remote addresses in order to perform status checks periodically
//!
//! ## Motivation
//!
//! Nāsu (from Japanese ナース [Nāsu], which means nurse), is a command-line utility
//! to perform checks on remote addresses periodically.
//!
//! ## Usage
//!
//! Install nasu using `cargo install` command:
//!
//! ```shell
//! cargo install nasu
//! ```
//!
//! Create a `nasu.json` file and execute `nasu`.
//!
//! ## Terminology
//!
//! The following terminology is used to refer to nasu main components:
//!
//! ### Service
//!
//! Service to perform task against through the Worker, a service is created
//! from a task defined in the `nasu.json` file.
//!
//! ### Worker
//!
//! Worker in responsible of performing the task, holds the logic to interact
//! with the service in question.
//!
//! ### Task
//!
//! Defintion of steps to be performed by the worker. Is provided
//! in the `nasu.json` file.
//!
//! ### `nasu.json` Reference
//!
//! `nasu.json` is the default configuration file for Nasu. This file is
//! parsed at startup by Nasu to initialize `Workers`.
//!
//! The `nasu.json` file is composed by an array of `Task` objects as shown
//! below:
//!
//! ```json
//! // nasu.json
//! [
//!   {
//!     "id": "HTTPBIN POST Request",
//!     "type": "http",
//!     "task": {
//!       "interval": "* */10 * * * *"
//!     },
//!     "params": {
//!       "url": "http://httpbin.org/post",
//!       "method": "POST",
//!       "headers": {
//!         "authorization": "Bearer <Token>",
//!         "content-type": "application/json"
//!       }
//!     }
//!   }
//! ]
//! ```
//!
//! Each of these `Task` must contain the following properties:
//!
//! Property | Description | Required | Possible Values
//! --- | --- | --- | ---
//! `id` | The id of the service. Used as reference for the user | Yes | N/A
//! `type` | Type of service to perform check on | Yes | `http`
//! `task` | Task configuration | Yes | N/A
//! `task.interval` | Cron defintion to specify when to perform the test | Yes | N/A
//! `params` | Params for the `Worker` used on perform. [Refer to Worker Params](#worker-params) | Yes | N/A
//!
//! ### Task `interval` field
//!
//! sec  min   hour   day of month   month   day of week   year
//!
//! The `interval` field on a task uses a [cron](https://en.wikipedia.org/wiki/Cron) definition.
//!
//! ```
//!  ┌────────────── second (0 - 59)
//!  │ ┌───────────── minute (0 - 59)
//!  │ │ ┌───────────── hour (0 - 23)
//!  │ │ │ ┌───────────── day of the month (1, 15)
//!  │ │ │ │ ┌───────────── month (Mon, Wed, Fri)
//!  │ │ │ │ │ ┌───────────── day of the week (Mon,Wed,Fri)
//!  │ │ │ │ │ │ ┌──────────── year
//!  │ │ │ │ │ │ │
//!  │ │ │ │ │ │ │
//!  * * * * * * *
//! ```
//!
//! An example:
//!
//! ```
//! 0 30 9,12,15 1,15 May-Aug Mon,Wed,Fri 2018/2
//! ```
//!
//! This application uses [cron](https://docs.rs/cron) crate internally
//! to parse and calculate time intervals.
//!
//! ### Worker Params
//!
//! `Worker` params may vary based on the `type` of worker in question.
//! Properties defined below belong to an object specified in the `Task`
//! object, inside of the `params` property.
//!
//! #### HTTP Worker Params
//!
//! - Type: `http`
//! - Property: `params`
//!
//! Property | Description | Required | Possible Values
//! --- | --- | --- | ---
//! `url` | URL to perform the HTTP Request | Yes | N/A
//! `method` | HTTP Method to perform the request with | Yes | `GET`, `PATCH`, `POST`, `PUT`, `DELETE`
//! `headers` | HTTP Headers to provide to the request | No | N/A
//!
use libnasu::run;
use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("nasu-main-proc")
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        run().await.unwrap();
    });
}
