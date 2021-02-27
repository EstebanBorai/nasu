use anyhow::{Context, Error, Result};
use async_trait::async_trait;
use hyper::client::HttpConnector;
use hyper::{Body, Client as HyperClient, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;

use crate::report::Report;
use crate::tasks::{Params as TaskParams, Task};
use crate::utils::timestamp::current_timestamp;
use crate::worker::perform::Perform;

use super::{Params, Report as HttpReport};

pub struct Provider {
    task_title: String,
    http_client: HyperClient<HttpsConnector<HttpConnector>>,
    http_method: Method,
    params: Params,
    url: String,
}

impl Provider {
    pub fn new(task: Task) -> Result<Self> {
        let params = match task.params {
            TaskParams::Http(p) => p,
            _ => {
                return Err(Error::msg(format!(
                    "Inavlid params provided for task {}",
                    task.title
                )));
            }
        };

        let url = Url::from_str(params.url.as_str())
            .context(format!("Invalid URL provided for task {}", task.title))?;

        let http_client = HyperClient::builder().build(HttpsConnector::new());
        let http_method = Method::from_str(params.method.as_str()).context(format!(
            "Invalid HTTP Method provided for task {}",
            task.title
        ))?;

        Ok(Self {
            task_title: task.title,
            http_client,
            http_method,
            params,
            url: url.to_string(),
        })
    }
}

#[async_trait]
impl Perform for Provider {
    async fn perform(&self) -> Result<Report> {
        let req_start = current_timestamp();
        let request = Request::builder()
            .uri(Uri::from_str(self.url.as_str()).unwrap())
            .method(self.http_method.clone())
            .body(Body::empty())
            .context(format!(
                "Failed to build Request struct on {}",
                self.task_title
            ))?;

        let response = self.http_client.request(request).await.unwrap();
        let mut headers: HashMap<String, String> = HashMap::new();

        for (k, v) in response.headers() {
            headers.insert(k.to_string(), v.to_str().unwrap().to_string());
        }

        let http_report = HttpReport {
            id: self.task_title.clone(),
            status_code: response.status().as_u16(),
            headers,
            req_start,
            req_end: current_timestamp(),
        };

        Ok(Report::Http(http_report))
    }
}
