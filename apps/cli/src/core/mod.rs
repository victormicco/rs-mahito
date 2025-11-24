//! Core module containing the metadata cleaning logic.
//!
//! This module provides the fundamental operations for clearing file metadata
//! on Windows systems, including NTFS alternate data streams and file timestamps.

mod cleaner;
mod error;
mod types;

pub use cleaner::MetadataCleaner;
#[allow(unused_imports)]
pub use error::{CleanerError, CleanerResult};
pub use types::{CleanMode, CleanOptions, CleanReport, FileResult};
