//! CLI tool for the Mjolnir Analyzer
//!
//! This binary provides a command-line interface to the analyzer,
//! allowing it to be called from other languages like JavaScript.

use mjolnir_analyzer::api::process_request;
use std::io::{self, Read};

fn main() -> Result<(), String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| format!("Failed to read from stdin: {}", e))?;

    let response = process_request(&input)?;

    println!("{}", response);

    Ok(())
}
