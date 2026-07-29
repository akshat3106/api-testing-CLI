// This module is responsible for building and sending HTTP requests.
// It takes the parsed CLI arguments and turns them into an actual
// network call using `reqwest`.

use crate::cli::{Cli, Method};
use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use std::time::{Duration, Instant};

/// How long to wait for a response before giving up.
const TIMEOUT: Duration = Duration::from_secs(30);

/// The result of sending a request: the raw response data plus timing info.
pub struct RawResponse {
    pub status: reqwest::StatusCode,
    pub body: String,
    pub elapsed: Duration,
}

/// Sends the HTTP request described by `args` and returns the response.
pub async fn send(args: &Cli) -> Result<RawResponse> {
    if let Some(body) = &args.body {
        serde_json::from_str::<serde_json::Value>(body)
            .with_context(|| format!("invalid JSON body: {body}"))?;
    }

    let client = Client::builder()
        .timeout(TIMEOUT)
        .build()
        .context("failed to set up the HTTP client")?;

    let method = match args.method {
        Method::GET => reqwest::Method::GET,
        Method::POST => reqwest::Method::POST,
        Method::PUT => reqwest::Method::PUT,
        Method::DELETE => reqwest::Method::DELETE,
    };

    let mut builder = client.request(method, &args.url);

    if let Some(body) = &args.body {
        builder = builder
            .header("Content-Type", "application/json")
            .body(body.clone());
    }

    for header in &args.headers {
        let (name, value) = parse_header(header)
            .with_context(|| format!("invalid header '{header}', expected 'Name: Value'"))?;
        builder = builder.header(name, value);
    }

    let start = Instant::now();
    let response = builder.send().await.map_err(|err| describe_send_error(&args.url, &err))?;
    let elapsed = start.elapsed();

    let status = response.status();
    let body = response
        .text()
        .await
        .context("failed to read response body")?;

    Ok(RawResponse {
        status,
        body,
        elapsed,
    })
}

/// Turns a low-level reqwest error into a beginner-friendly message.
fn describe_send_error(url: &str, err: &reqwest::Error) -> anyhow::Error {
    if err.is_builder() {
        anyhow!("invalid URL: {url}")
    } else if err.is_timeout() {
        anyhow!("request timed out after {} seconds", TIMEOUT.as_secs())
    } else if err.is_connect() {
        anyhow!("could not connect to {url} (check your internet connection or the URL)")
    } else {
        anyhow!("request to {url} failed: {err}")
    }
}

/// Splits a "Name: Value" header string into its two parts.
fn parse_header(raw: &str) -> Option<(&str, &str)> {
    let (name, value) = raw.split_once(':')?;
    Some((name.trim(), value.trim()))
}
