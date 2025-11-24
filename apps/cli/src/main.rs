//! rs-mahito - A CLI tool to clear file metadata on Windows.
//!
//! This tool removes metadata from files including:
//! - NTFS Alternate Data Streams (Zone.Identifier, etc.)
//! - File timestamps (created, modified, accessed)
//! - Extended file attributes
//!
//! # Usage
//!
//! ```bash
//! # Clean a single file
//! rs-mahito file path/to/file.txt
//!
//! # Clean all files in a directory (non-recursive)
//! rs-mahito dir path/to/folder
//!
//! # Clean all files recursively
//! rs-mahito recursive path/to/folder
//!
//! # View file metadata info
//! rs-mahito info path/to/file.txt
//! ```

mod cli;
mod core;

use clap::Parser;
use colored::Colorize;

use cli::{Cli, Runner};

fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Create and run the command runner
    let runner = Runner::new(cli);

    if let Err(e) = runner.run() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
