//! CLI module for rs-meta-cleaner.
//!
//! This module provides the command-line interface using clap's derive API,
//! following best practices for subcommand organization and argument handling.

mod args;
mod runner;

pub use args::Cli;
pub use runner::Runner;
