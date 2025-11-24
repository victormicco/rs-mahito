//! Error types for the metadata cleaner.
//!
//! Provides a comprehensive error handling system using `thiserror` for
//! ergonomic error definitions and propagation.

#![allow(dead_code)]

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for cleaner operations.
pub type CleanerResult<T> = Result<T, CleanerError>;

/// Errors that can occur during metadata cleaning operations.
#[derive(Debug, Error)]
pub enum CleanerError {
    /// The specified path does not exist.
    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),

    /// The path exists but is not a file when a file was expected.
    #[error("Expected a file but found a directory: {0}")]
    NotAFile(PathBuf),

    /// The path exists but is not a directory when a directory was expected.
    #[error("Expected a directory but found a file: {0}")]
    NotADirectory(PathBuf),

    /// Permission denied when accessing the file or directory.
    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    /// Failed to read directory contents.
    #[error("Failed to read directory '{path}': {source}")]
    DirectoryReadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to clean file metadata.
    #[error("Failed to clean metadata for '{path}': {reason}")]
    CleaningFailed { path: PathBuf, reason: String },

    /// Windows API error.
    #[error("Windows API error for '{path}': {message}")]
    WindowsApiError { path: PathBuf, message: String },

    /// Generic I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Failed to enumerate alternate data streams.
    #[error("Failed to enumerate data streams for '{0}'")]
    StreamEnumerationFailed(PathBuf),
}

impl CleanerError {
    /// Creates a new cleaning failed error.
    pub fn cleaning_failed(path: impl Into<PathBuf>, reason: impl Into<String>) -> Self {
        Self::CleaningFailed {
            path: path.into(),
            reason: reason.into(),
        }
    }

    /// Creates a new Windows API error.
    pub fn windows_api_error(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::WindowsApiError {
            path: path.into(),
            message: message.into(),
        }
    }
}
