//! Command-line argument definitions.
//!
//! Uses clap's derive API for declarative argument parsing with
//! support for subcommands and global options.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A CLI tool to clear metadata from files on Windows NTFS filesystems.
///
/// Removes alternate data streams, resets timestamps, and clears
/// file attributes to protect your privacy.
#[derive(Debug, Parser)]
#[command(
    name = "rs-meta-cleaner",
    author,
    version,
    about = "Clear metadata from files to protect your privacy",
    long_about = "A powerful CLI tool to remove metadata from files on Windows.\n\n\
                  This includes:\n  \
                  - NTFS Alternate Data Streams (Zone.Identifier, etc.)\n  \
                  - File timestamps (created, modified, accessed)\n  \
                  - Extended file attributes\n\n\
                  Perfect for privacy-conscious users who want to share files\n\
                  without exposing personal information.",
    propagate_version = true,
    arg_required_else_help = true
)]
pub struct Cli {
    /// Global options that apply to all commands
    #[command(flatten)]
    pub global: GlobalOptions,

    /// The command to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Global options available for all commands.
#[derive(Debug, Parser)]
pub struct GlobalOptions {
    /// Run in dry-run mode (no actual changes will be made)
    #[arg(short = 'n', long, global = true)]
    pub dry_run: bool,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Skip confirmation prompts
    #[arg(short = 'y', long, global = true)]
    pub yes: bool,

    /// Run with admin privileges (attempts to clear file owner)
    ///
    /// Without this flag, only non-privileged operations are performed.
    /// Use this when running as Administrator to also clear the NTFS file owner.
    #[arg(short = 'a', long, global = true)]
    pub admin: bool,
}

/// Available commands for the CLI.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Clean metadata from a single file
    ///
    /// If no path is provided, prompts to select a file from the current directory.
    #[command(visible_alias = "f")]
    File {
        /// Path to the file to clean (defaults to current directory)
        #[arg(short, long, value_name = "FILE")]
        path: Option<PathBuf>,
    },

    /// Clean metadata from all files in a folder (non-recursive)
    ///
    /// Only processes files directly in the specified folder,
    /// not files in subfolders. Defaults to current directory.
    #[command(visible_alias = "d")]
    Dir {
        /// Path to the directory to clean (defaults to current directory)
        #[arg(short, long, value_name = "DIRECTORY")]
        path: Option<PathBuf>,
    },

    /// Clean metadata from all files in a folder and subfolders (recursive)
    ///
    /// Processes all files in the specified folder and all nested subfolders.
    /// Use with caution on large directory trees. Defaults to current directory.
    #[command(visible_alias = "r")]
    Recursive {
        /// Path to the directory to clean recursively (defaults to current directory)
        #[arg(short, long, value_name = "DIRECTORY")]
        path: Option<PathBuf>,
    },

    /// Display information about what metadata a file contains
    ///
    /// If no path is provided, prompts to select a file from the current directory.
    #[command(visible_alias = "i")]
    Info {
        /// Path to the file to inspect (defaults to current directory)
        #[arg(short, long, value_name = "FILE")]
        path: Option<PathBuf>,
    },
}
