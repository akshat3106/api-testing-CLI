// This module defines the command-line interface for `unc`.
// It uses `clap` to parse arguments like the HTTP method, URL, headers, and body.

use clap::{Parser, ValueEnum};

/// A lightweight command-line tool for sending HTTP requests
/// and viewing formatted responses.
#[derive(Parser, Debug)]
#[command(name = "unc", version, about)]
pub struct Cli {
    /// HTTP method to use (GET, POST, PUT, DELETE)
    #[arg(value_enum, ignore_case = true)]
    pub method: Method,

    /// The URL to send the request to
    pub url: String,

    /// JSON request body, e.g. -d '{"title":"Hello"}'
    #[arg(short = 'd', long = "data")]
    pub body: Option<String>,

    /// Custom header, e.g. -H "Accept: application/json" (can be repeated)
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,
}

/// The HTTP methods supported by unc.
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}
