//! Type definitions for the metadata cleaner.
//!
//! Contains all the data structures used throughout the cleaning operations.

#![allow(dead_code)]

use std::path::PathBuf;

/// Specifies how deeply to clean files in a directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CleanMode {
    /// Clean a single file only.
    #[default]
    SingleFile,
    /// Clean all files in a directory (non-recursive).
    Shallow,
    /// Clean all files in a directory and subdirectories (recursive).
    Deep,
}

impl std::fmt::Display for CleanMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CleanMode::SingleFile => write!(f, "single file"),
            CleanMode::Shallow => write!(f, "shallow (non-recursive)"),
            CleanMode::Deep => write!(f, "deep (recursive)"),
        }
    }
}

/// Options for controlling the cleaning behavior.
#[derive(Debug, Clone, Default)]
pub struct CleanOptions {
    /// Whether to clear file timestamps (created, modified, accessed).
    pub clear_timestamps: bool,
    /// Whether to remove NTFS alternate data streams.
    pub clear_streams: bool,
    /// Whether to clear extended attributes.
    pub clear_attributes: bool,
    /// Whether to clear file owner information.
    pub clear_owner: bool,
    /// Whether to clear file properties (author, computer, etc.).
    pub clear_properties: bool,
    /// Whether to run in dry-run mode (no actual changes).
    pub dry_run: bool,
    /// Whether to show verbose output.
    pub verbose: bool,
}

impl CleanOptions {
    /// Creates options with all cleaning features enabled (non-admin mode by default).
    pub fn all() -> Self {
        Self {
            clear_timestamps: true,
            clear_streams: true,
            clear_attributes: true,
            clear_owner: false, // Requires admin, disabled by default
            clear_properties: true,
            dry_run: false,
            verbose: false,
        }
    }

    /// Sets dry-run mode.
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Sets verbose mode.
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Sets admin mode (enables owner clearing which requires elevated privileges).
    pub fn with_admin(mut self, admin: bool) -> Self {
        self.clear_owner = admin;
        self
    }
}

/// Result of cleaning a single file.
#[derive(Debug, Clone)]
pub struct FileResult {
    /// Path to the file that was processed.
    pub path: PathBuf,
    /// Whether the cleaning was successful.
    pub success: bool,
    /// Error message if cleaning failed.
    pub error: Option<String>,
    /// Number of alternate data streams removed.
    pub streams_removed: usize,
    /// Whether timestamps were reset.
    pub timestamps_reset: bool,
}

impl FileResult {
    /// Creates a successful file result.
    pub fn success(path: PathBuf, streams_removed: usize, timestamps_reset: bool) -> Self {
        Self {
            path,
            success: true,
            error: None,
            streams_removed,
            timestamps_reset,
        }
    }

    /// Creates a failed file result.
    pub fn failure(path: PathBuf, error: impl Into<String>) -> Self {
        Self {
            path,
            success: false,
            error: Some(error.into()),
            streams_removed: 0,
            timestamps_reset: false,
        }
    }
}

/// Summary report of a cleaning operation.
#[derive(Debug, Clone, Default)]
pub struct CleanReport {
    /// Total number of files processed.
    pub total_files: usize,
    /// Number of files successfully cleaned.
    pub successful: usize,
    /// Number of files that failed to clean.
    pub failed: usize,
    /// Number of files skipped.
    pub skipped: usize,
    /// Total alternate data streams removed.
    pub total_streams_removed: usize,
    /// Individual file results.
    pub file_results: Vec<FileResult>,
}

impl CleanReport {
    /// Creates a new empty report.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a file result to the report.
    pub fn add_result(&mut self, result: FileResult) {
        self.total_files += 1;
        if result.success {
            self.successful += 1;
            self.total_streams_removed += result.streams_removed;
        } else {
            self.failed += 1;
        }
        self.file_results.push(result);
    }

    /// Marks a file as skipped.
    pub fn add_skipped(&mut self) {
        self.skipped += 1;
    }

    /// Returns true if all files were successfully cleaned.
    pub fn is_complete_success(&self) -> bool {
        self.failed == 0
    }
}
