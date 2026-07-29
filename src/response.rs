// This module represents a processed HTTP response: it takes the raw
// data from `request.rs` and prepares it for display, e.g. by pretty
// printing JSON bodies and computing byte size.

use crate::request::RawResponse;

pub struct Response {
    pub status: reqwest::StatusCode,
    pub elapsed_ms: u128,
    pub size_bytes: usize,
    pub body: String,
}

impl Response {
    pub fn from_raw(raw: RawResponse) -> Self {
        Self {
            status: raw.status,
            elapsed_ms: raw.elapsed.as_millis(),
            size_bytes: raw.body.len(),
            body: raw.body,
        }
    }

    /// Returns the body pretty-printed if it's valid JSON,
    /// otherwise returns the body as-is.
    pub fn pretty_body(&self) -> String {
        match serde_json::from_str::<serde_json::Value>(&self.body) {
            Ok(value) => serde_json::to_string_pretty(&value).unwrap_or_else(|_| self.body.clone()),
            Err(_) => self.body.clone(),
        }
    }
}
