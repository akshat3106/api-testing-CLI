// unc - a lightweight command-line HTTP client.
//
// This file wires the app together: parse CLI args, send the request,
// process the response, then print it.

mod cli;
mod formatter;
mod request;
mod response;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use response::Response;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let raw = request::send(&args).await?;
    let response = Response::from_raw(raw);

    formatter::print(&response);

    Ok(())
}
