// This module is responsible for printing a `Response` to the terminal
// in a readable, colored format. It has no knowledge of networking.

use crate::response::Response;
use colored::Colorize;

/// Prints the full response: status/time/size summary, then the body.
pub fn print(response: &Response) {
    print_summary(response);
    println!();
    println!("{}", response.pretty_body());
}

fn print_summary(response: &Response) {
    let status_text = format!(
        "{} {}",
        response.status.as_str(),
        response.status.canonical_reason().unwrap_or("")
    );
    let status_colored = if response.status.is_success() {
        status_text.green()
    } else if response.status.is_client_error() || response.status.is_server_error() {
        status_text.red()
    } else {
        status_text.yellow()
    };

    println!("{} : {}", "Status".bold(), status_colored);
    println!("{} : {} ms", "Time".bold(), response.elapsed_ms);
    println!("{} : {} bytes", "Size".bold(), response.size_bytes);
}
