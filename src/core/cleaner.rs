//! Core metadata cleaning implementation.
//!
//! This module provides the main `MetadataCleaner` struct that handles
//! all metadata removal operations on Windows NTFS filesystems.

#![allow(dead_code)]

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[cfg(not(windows))]
use std::time::SystemTime;

use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

/// Escapes special regex characters in a string.
fn regex_escape(s: &str) -> String {
    let special_chars = ['\\', '.', '+', '*', '?', '(', ')', '[', ']', '{', '}', '|', '^', '$', ':'];
    let mut result = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if special_chars.contains(&c) {
            result.push('\\');
        }
        result.push(c);
    }
    result
}

use super::error::{CleanerError, CleanerResult};
use super::types::{CleanMode, CleanOptions, CleanReport, FileResult};

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

#[cfg(windows)]
use windows::core::PCWSTR;
#[cfg(windows)]
use windows::Win32::Foundation::{HANDLE, FILETIME, LocalFree};
#[cfg(windows)]
use windows::Win32::Storage::FileSystem::{
    DeleteFileW, FindClose, FindFirstStreamW, FindNextStreamW, SetFileTime,
    FILE_FLAG_BACKUP_SEMANTICS, WIN32_FIND_STREAM_DATA,
};
#[cfg(windows)]
use windows::Win32::Security::{OWNER_SECURITY_INFORMATION, PSID};
#[cfg(windows)]
use windows::Win32::Security::Authorization::{
    ConvertStringSidToSidW, SE_FILE_OBJECT, SetNamedSecurityInfoW,
};

/// The main metadata cleaner that orchestrates all cleaning operations.
#[derive(Debug, Default)]
pub struct MetadataCleaner {
    options: CleanOptions,
}

impl MetadataCleaner {
    /// Creates a new `MetadataCleaner` with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `MetadataCleaner` with the specified options.
    pub fn with_options(options: CleanOptions) -> Self {
        Self { options }
    }

    /// Cleans metadata from a single file.
    pub fn clean_file(&self, path: &Path) -> CleanerResult<FileResult> {
        let path = path.canonicalize().map_err(|_| CleanerError::PathNotFound(path.to_path_buf()))?;

        if !path.exists() {
            return Err(CleanerError::PathNotFound(path));
        }

        if path.is_dir() {
            return Err(CleanerError::NotAFile(path));
        }

        if self.options.dry_run {
            return Ok(FileResult::success(path, 0, false));
        }

        let mut streams_removed = 0;
        let mut timestamps_reset = false;

        // Remove alternate data streams
        if self.options.clear_streams {
            match self.remove_alternate_streams(&path) {
                Ok(count) => streams_removed = count,
                Err(e) => return Ok(FileResult::failure(path, e.to_string())),
            }
        }

        // Reset timestamps
        if self.options.clear_timestamps {
            match self.reset_timestamps(&path) {
                Ok(_) => timestamps_reset = true,
                Err(e) => return Ok(FileResult::failure(path, e.to_string())),
            }
        }

        // Clear file owner (requires Administrator privileges)
        // Only attempted when --admin flag is used
        if self.options.clear_owner {
            if let Err(e) = self.clear_owner(&path) {
                return Ok(FileResult::failure(path, e.to_string()));
            }
        }

        // Clear file properties (author, computer, etc.) from NTFS streams
        if self.options.clear_properties {
            if let Err(e) = self.clear_properties(&path) {
                return Ok(FileResult::failure(path, e.to_string()));
            }
        }

        // Clear embedded document properties from Office Open XML files
        // This removes Author, Company (Computer), Last Modified By, etc. from the Details tab
        if self.options.clear_properties {
            if let Err(e) = self.clear_office_xml_properties(&path) {
                return Ok(FileResult::failure(path, e.to_string()));
            }
        }

        Ok(FileResult::success(path, streams_removed, timestamps_reset))
    }

    /// Cleans metadata from all files in a directory (non-recursive).
    pub fn clean_directory_shallow(&self, path: &Path) -> CleanerResult<CleanReport> {
        self.clean_directory_internal(path, CleanMode::Shallow)
    }

    /// Cleans metadata from all files in a directory and subdirectories (recursive).
    pub fn clean_directory_deep(&self, path: &Path) -> CleanerResult<CleanReport> {
        self.clean_directory_internal(path, CleanMode::Deep)
    }

    /// Internal method to clean a directory with the specified mode.
    fn clean_directory_internal(&self, path: &Path, mode: CleanMode) -> CleanerResult<CleanReport> {
        let path = path.canonicalize().map_err(|_| CleanerError::PathNotFound(path.to_path_buf()))?;

        if !path.exists() {
            return Err(CleanerError::PathNotFound(path));
        }

        if !path.is_dir() {
            return Err(CleanerError::NotADirectory(path));
        }

        let mut report = CleanReport::new();

        let walker = match mode {
            CleanMode::Shallow => WalkDir::new(&path).min_depth(1).max_depth(1),
            CleanMode::Deep => WalkDir::new(&path).min_depth(1),
            CleanMode::SingleFile => unreachable!(),
        };

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            let entry_path = entry.path();

            // Skip directories
            if entry_path.is_dir() {
                continue;
            }

            match self.clean_file(entry_path) {
                Ok(result) => report.add_result(result),
                Err(e) => {
                    report.add_result(FileResult::failure(entry_path.to_path_buf(), e.to_string()));
                }
            }
        }

        Ok(report)
    }

    /// Removes alternate data streams from a file.
    #[cfg(windows)]
    fn remove_alternate_streams(&self, path: &Path) -> CleanerResult<usize> {
        let streams = self.enumerate_streams(path)?;
        let mut removed_count = 0;

        for stream_name in streams {
            // Skip the main data stream (::$DATA)
            if stream_name == "::$DATA" || stream_name.is_empty() {
                continue;
            }

            // Build the full stream path
            let stream_path = format!("{}:{}", path.display(), stream_name.trim_start_matches(':').trim_end_matches(":$DATA"));

            let wide_path: Vec<u16> = stream_path.encode_utf16().chain(std::iter::once(0)).collect();

            unsafe {
                let result = DeleteFileW(PCWSTR(wide_path.as_ptr()));
                if result.is_ok() {
                    removed_count += 1;
                }
            }
        }

        Ok(removed_count)
    }

    #[cfg(not(windows))]
    fn remove_alternate_streams(&self, _path: &Path) -> CleanerResult<usize> {
        // Non-Windows systems don't have NTFS alternate data streams
        Ok(0)
    }

    /// Enumerates all alternate data streams for a file.
    #[cfg(windows)]
    fn enumerate_streams(&self, path: &Path) -> CleanerResult<Vec<String>> {
        let wide_path: Vec<u16> = path.as_os_str()
            .to_string_lossy()
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        let mut streams = Vec::new();
        let mut find_data = WIN32_FIND_STREAM_DATA::default();

        unsafe {
            let handle = FindFirstStreamW(
                PCWSTR(wide_path.as_ptr()),
                windows::Win32::Storage::FileSystem::FindStreamInfoStandard,
                &mut find_data as *mut _ as *mut _,
                0,
            );

            match handle {
                Ok(h) if !h.is_invalid() => {
                    loop {
                        let stream_name = String::from_utf16_lossy(
                            &find_data.cStreamName[..find_data.cStreamName.iter().position(|&c| c == 0).unwrap_or(find_data.cStreamName.len())]
                        );

                        if !stream_name.is_empty() {
                            streams.push(stream_name);
                        }

                        if FindNextStreamW(h, &mut find_data as *mut _ as *mut _).is_err() {
                            break;
                        }
                    }
                    let _ = FindClose(h);
                }
                _ => {
                    // No streams found or error - this is not necessarily an error condition
                }
            }
        }

        Ok(streams)
    }

    #[cfg(not(windows))]
    fn enumerate_streams(&self, _path: &Path) -> CleanerResult<Vec<String>> {
        Ok(Vec::new())
    }

    /// Resets file timestamps to January 1, 2000 (a neutral, anonymous date).
    #[cfg(windows)]
    fn reset_timestamps(&self, path: &Path) -> CleanerResult<()> {
        use std::os::windows::io::AsRawHandle;

        // Open the file with write access for setting times
        let file = OpenOptions::new()
            .write(true)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS.0)
            .open(path)
            .map_err(|e| CleanerError::cleaning_failed(path, e.to_string()))?;

        // FILETIME is in 100-nanosecond intervals since January 1, 1601 (UTC)
        // January 1, 2000 00:00:00 UTC = 125911584000000000 (100-ns intervals since 1601)
        // This is calculated as: days from 1601 to 2000 * 24 * 60 * 60 * 10_000_000
        // Using a neutral date that doesn't reveal when the file was actually created
        const FILETIME_JAN_1_2000: u64 = 125911584000000000;

        let epoch_time = FILETIME {
            dwLowDateTime: (FILETIME_JAN_1_2000 & 0xFFFFFFFF) as u32,
            dwHighDateTime: (FILETIME_JAN_1_2000 >> 32) as u32,
        };

        unsafe {
            let handle = HANDLE(file.as_raw_handle() as _);
            SetFileTime(
                handle,
                Some(&epoch_time), // Creation time
                Some(&epoch_time), // Last access time
                Some(&epoch_time), // Last write time
            )
            .map_err(|e| CleanerError::windows_api_error(path, e.to_string()))?;
        }

        Ok(())
    }

    #[cfg(not(windows))]
    fn reset_timestamps(&self, path: &Path) -> CleanerResult<()> {
        // On non-Windows systems, use filetime crate or similar
        // For now, just touch the file
        let now = SystemTime::now();
        let file = OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|e| CleanerError::cleaning_failed(path, e.to_string()))?;

        file.set_modified(now)
            .map_err(|e| CleanerError::cleaning_failed(path, e.to_string()))?;

        Ok(())
    }

    /// Clears the file owner by setting it to the "Everyone" well-known SID.
    /// This effectively anonymizes the file ownership.
    #[cfg(windows)]
    fn clear_owner(&self, path: &Path) -> CleanerResult<()> {
        use windows::Win32::Foundation::HLOCAL;

        // Use "S-1-5-32-544" which is the "BUILTIN\Administrators" well-known SID
        // This is a generic admin SID that doesn't identify any specific user
        let sid_string: Vec<u16> = "S-1-5-32-544\0".encode_utf16().collect();
        let wide_path: Vec<u16> = path.as_os_str()
            .to_string_lossy()
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            let mut sid: PSID = PSID::default();

            // Convert the string SID to a binary SID
            let result = ConvertStringSidToSidW(
                PCWSTR(sid_string.as_ptr()),
                &mut sid,
            );

            if result.is_err() {
                return Err(CleanerError::windows_api_error(path, "Failed to convert SID string"));
            }

            // Set the owner on the file using SetNamedSecurityInfoW
            let result = SetNamedSecurityInfoW(
                PCWSTR(wide_path.as_ptr()),
                SE_FILE_OBJECT,
                OWNER_SECURITY_INFORMATION,
                sid,
                PSID::default(),
                None,
                None,
            );

            // Free the SID memory allocated by ConvertStringSidToSidW
            let _ = LocalFree(HLOCAL(sid.0));

            // SetNamedSecurityInfoW returns WIN32_ERROR, ERROR_SUCCESS (0) means success
            if result.0 != 0 {
                return Err(CleanerError::windows_api_error(path, format!("Failed to set owner (error {}). Run as Administrator.", result.0)));
            }
        }

        Ok(())
    }

    #[cfg(not(windows))]
    fn clear_owner(&self, _path: &Path) -> CleanerResult<()> {
        // Owner clearing is Windows-specific
        Ok(())
    }

    /// Clears file properties stored in NTFS extended attributes and various streams.
    /// This removes author, computer name, and other metadata from the Details tab.
    #[cfg(windows)]
    fn clear_properties(&self, path: &Path) -> CleanerResult<()> {
        // Windows stores various metadata in alternate data streams:
        // - Zone.Identifier: Downloaded file info (includes URL, computer info)
        // - SummaryInformation: OLE document properties
        // - DocumentSummaryInformation: Extended document properties
        // - Afp_AfpInfo, encryptable, OECustomProperty, etc.

        let streams_to_remove = [
            "Zone.Identifier",
            "\x05SummaryInformation",
            "\x05DocumentSummaryInformation",
            "Afp_AfpInfo",
            "encryptable",
            "OECustomProperty",
        ];

        for stream_name in streams_to_remove {
            let stream_path = format!("{}:{}", path.display(), stream_name);
            let wide_path: Vec<u16> = stream_path.encode_utf16().chain(std::iter::once(0)).collect();

            unsafe {
                // Attempt to delete the stream - ignore errors as the stream may not exist
                let _ = DeleteFileW(PCWSTR(wide_path.as_ptr()));
            }
        }

        Ok(())
    }

    #[cfg(not(windows))]
    fn clear_properties(&self, _path: &Path) -> CleanerResult<()> {
        // Properties clearing is Windows-specific
        Ok(())
    }

    /// Clears embedded document properties from Office Open XML files (.docx, .xlsx, .pptx, etc.).
    /// These files are ZIP archives containing XML metadata in docProps/core.xml and docProps/app.xml.
    /// This function removes the "Owner", "Computer", "Author", "Last Modified By", etc. fields
    /// that appear in Windows File Properties → Details tab.
    fn clear_office_xml_properties(&self, path: &Path) -> CleanerResult<bool> {
        // Check if this is an Office Open XML file by extension
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        let is_office_xml = matches!(
            extension.as_deref(),
            Some("docx") | Some("xlsx") | Some("pptx") |
            Some("docm") | Some("xlsm") | Some("pptm") |
            Some("dotx") | Some("xltx") | Some("potx")
        );

        if !is_office_xml {
            return Ok(false);
        }

        // Try to open as a ZIP archive
        let file = File::open(path)
            .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to open file: {}", e)))?;

        let mut archive = match ZipArchive::new(file) {
            Ok(a) => a,
            Err(_) => return Ok(false), // Not a valid ZIP/Office file
        };

        // Create a temporary file for the modified archive
        let temp_path = path.with_extension("tmp_meta_clean");
        let temp_file = File::create(&temp_path)
            .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to create temp file: {}", e)))?;

        let mut zip_writer = ZipWriter::new(temp_file);
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        // Process each file in the archive
        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)
                .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to read archive entry: {}", e)))?;

            let entry_name = entry.name().to_string();

            // Handle docProps/core.xml - contains Author, Last Modified By, etc.
            if entry_name == "docProps/core.xml" {
                let mut content = String::new();
                entry.read_to_string(&mut content)
                    .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to read core.xml: {}", e)))?;

                let cleaned_content = self.clean_core_xml(&content);

                zip_writer.start_file(&entry_name, options)
                    .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to write to archive: {}", e)))?;
                zip_writer.write_all(cleaned_content.as_bytes())
                    .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to write content: {}", e)))?;
            }
            // Handle docProps/app.xml - contains Application, Company (Computer), etc.
            else if entry_name == "docProps/app.xml" {
                let mut content = String::new();
                entry.read_to_string(&mut content)
                    .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to read app.xml: {}", e)))?;

                let cleaned_content = self.clean_app_xml(&content);

                zip_writer.start_file(&entry_name, options)
                    .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to write to archive: {}", e)))?;
                zip_writer.write_all(cleaned_content.as_bytes())
                    .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to write content: {}", e)))?;
            }
            // Copy all other files unchanged
            else {
                let mut buffer = Vec::new();
                entry.read_to_end(&mut buffer)
                    .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to read entry: {}", e)))?;

                zip_writer.start_file(&entry_name, options)
                    .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to write to archive: {}", e)))?;
                zip_writer.write_all(&buffer)
                    .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to write content: {}", e)))?;
            }
        }

        // Finalize the ZIP
        zip_writer.finish()
            .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to finalize archive: {}", e)))?;

        // Replace the original file with the cleaned version
        std::fs::remove_file(path)
            .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to remove original file: {}", e)))?;
        std::fs::rename(&temp_path, path)
            .map_err(|e| CleanerError::cleaning_failed(path, format!("Failed to rename temp file: {}", e)))?;

        Ok(true)
    }

    /// Cleans the docProps/core.xml file, removing author, last modified by, etc.
    fn clean_core_xml(&self, content: &str) -> String {
        let mut result = content.to_string();

        // Elements to clear in core.xml (Dublin Core and CP namespaces)
        let elements_to_clear = [
            ("dc:creator", ""),           // Author
            ("cp:lastModifiedBy", ""),    // Last Modified By
            ("dc:title", ""),             // Title
            ("dc:subject", ""),           // Subject
            ("dc:description", ""),       // Comments
            ("cp:keywords", ""),          // Keywords/Tags
            ("cp:category", ""),          // Category
            ("cp:contentStatus", ""),     // Content Status
        ];

        for (tag, replacement) in elements_to_clear {
            // Match <tag>content</tag> or <tag attr="...">content</tag>
            let pattern = format!(r"<{}[^>]*>.*?</{}>", regex_escape(tag), regex_escape(tag));
            if let Ok(re) = regex_lite::Regex::new(&pattern) {
                let new_tag = format!("<{}>{}</{}>", tag, replacement, tag);
                result = re.replace_all(&result, new_tag.as_str()).to_string();
            }
        }

        result
    }

    /// Cleans the docProps/app.xml file, removing company (computer), manager, etc.
    fn clean_app_xml(&self, content: &str) -> String {
        let mut result = content.to_string();

        // Elements to clear in app.xml
        let elements_to_clear = [
            ("Company", ""),              // Company/Computer name
            ("Manager", ""),              // Manager
            ("HyperlinkBase", ""),        // Hyperlink base
        ];

        for (tag, replacement) in elements_to_clear {
            // Match <tag>content</tag>
            let pattern = format!(r"<{}[^>]*>.*?</{}>", tag, tag);
            if let Ok(re) = regex_lite::Regex::new(&pattern) {
                let new_tag = format!("<{}>{}</{}>", tag, replacement, tag);
                result = re.replace_all(&result, new_tag.as_str()).to_string();
            }
        }

        result
    }

    /// Collects all files that would be processed.
    pub fn collect_files(&self, path: &Path, mode: CleanMode) -> CleanerResult<Vec<PathBuf>> {
        let path = path.canonicalize().map_err(|_| CleanerError::PathNotFound(path.to_path_buf()))?;

        match mode {
            CleanMode::SingleFile => {
                if path.is_file() {
                    Ok(vec![path])
                } else {
                    Err(CleanerError::NotAFile(path))
                }
            }
            CleanMode::Shallow => {
                if !path.is_dir() {
                    return Err(CleanerError::NotADirectory(path));
                }
                Ok(WalkDir::new(&path)
                    .min_depth(1)
                    .max_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_file())
                    .map(|e| e.path().to_path_buf())
                    .collect())
            }
            CleanMode::Deep => {
                if !path.is_dir() {
                    return Err(CleanerError::NotADirectory(path));
                }
                Ok(WalkDir::new(&path)
                    .min_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_file())
                    .map(|e| e.path().to_path_buf())
                    .collect())
            }
        }
    }
}
