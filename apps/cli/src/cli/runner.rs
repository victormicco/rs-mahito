//! Command runner that executes CLI commands.
//!
//! This module bridges the CLI arguments with the core cleaning logic,
//! providing user-friendly output and progress indication.

use std::path::{Path, PathBuf};

use colored::Colorize;
use console::Term;
use dialoguer::{Confirm, FuzzySelect};
use indicatif::{ProgressBar, ProgressStyle};

use crate::core::{CleanMode, CleanOptions, CleanReport, MetadataCleaner};

use super::args::{Cli, Commands};

/// The command runner that executes CLI commands.
pub struct Runner {
    cli: Cli,
    #[allow(dead_code)]
    term: Term,
}

impl Runner {
    /// Creates a new runner with the parsed CLI arguments.
    pub fn new(cli: Cli) -> Self {
        Self {
            cli,
            term: Term::stderr(),
        }
    }

    /// Runs the appropriate command based on CLI arguments.
    pub fn run(&self) -> anyhow::Result<()> {
        let cwd = std::env::current_dir()?;

        match &self.cli.command {
            Commands::File { path } => {
                let target = self.resolve_file_path(path.clone(), &cwd)?;
                self.run_file(&target)
            }
            Commands::Dir { path } => {
                let target = path.clone().unwrap_or_else(|| cwd.clone());
                self.run_dir(&target)
            }
            Commands::Recursive { path } => {
                let target = path.clone().unwrap_or_else(|| cwd.clone());
                self.run_recursive(&target)
            }
            Commands::Info { path } => {
                let target = self.resolve_file_path(path.clone(), &cwd)?;
                self.run_info(&target)
            }
        }
    }

    /// Resolves a file path, prompting user to select if path is a directory or not provided.
    fn resolve_file_path(&self, path: Option<PathBuf>, cwd: &Path) -> anyhow::Result<PathBuf> {
        let target = path.unwrap_or_else(|| cwd.to_path_buf());

        if target.is_file() {
            return Ok(target);
        }

        if !target.is_dir() {
            anyhow::bail!("Path does not exist: {}", target.display());
        }

        // Collect files in the directory
        let files: Vec<PathBuf> = std::fs::read_dir(&target)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|p| p.is_file())
            .collect();

        if files.is_empty() {
            anyhow::bail!("No files found in directory: {}", target.display());
        }

        // Build display names for the selector
        let display_names: Vec<String> = files
            .iter()
            .map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string())
            .collect();

        println!("{} {}", "Directory:".cyan(), target.display());

        let selection = FuzzySelect::new()
            .with_prompt("Select a file")
            .items(&display_names)
            .default(0)
            .interact()?;

        Ok(files[selection].clone())
    }

    /// Cleans a single file.
    fn run_file(&self, path: &Path) -> anyhow::Result<()> {
        self.print_header("Single File Mode");

        let cleaner = self.create_cleaner();

        // Show what we're about to do
        println!("{} {}", "Target:".cyan(), path.display());

        if self.cli.global.dry_run {
            println!("{}", "[DRY RUN] No changes will be made".yellow());
        }

        // Confirm unless --yes is passed
        if !self.confirm_action(&format!("Clean metadata from '{}'?", path.display()))? {
            println!("{}", "Operation cancelled.".yellow());
            return Ok(());
        }

        let spinner = self.create_spinner("Cleaning file...");

        match cleaner.clean_file(path) {
            Ok(result) => {
                spinner.finish_and_clear();
                if result.success {
                    self.print_success(&format!(
                        "Cleaned: {} (streams removed: {}, timestamps reset: {})",
                        path.display(),
                        result.streams_removed,
                        if result.timestamps_reset { "yes" } else { "no" }
                    ));
                } else {
                    self.print_error(&format!(
                        "Failed: {} - {}",
                        path.display(),
                        result.error.unwrap_or_default()
                    ));
                }
            }
            Err(e) => {
                spinner.finish_and_clear();
                self.print_error(&format!("Error: {}", e));
                return Err(e.into());
            }
        }

        Ok(())
    }

    /// Cleans a directory (non-recursive).
    fn run_dir(&self, path: &Path) -> anyhow::Result<()> {
        self.print_header("Directory Mode (Non-Recursive)");
        self.run_directory_clean(path, CleanMode::Shallow)
    }

    /// Cleans a directory recursively.
    fn run_recursive(&self, path: &Path) -> anyhow::Result<()> {
        self.print_header("Recursive Mode");
        self.run_directory_clean(path, CleanMode::Deep)
    }

    /// Common logic for directory cleaning.
    fn run_directory_clean(&self, path: &Path, mode: CleanMode) -> anyhow::Result<()> {
        let cleaner = self.create_cleaner();

        // First, collect files to show the user what will be processed
        println!("{} {}", "Target:".cyan(), path.display());
        println!("{} {}", "Mode:".cyan(), mode);

        if self.cli.global.dry_run {
            println!("{}", "[DRY RUN] No changes will be made".yellow());
        }

        let spinner = self.create_spinner("Scanning files...");
        let files = cleaner.collect_files(path, mode)?;
        spinner.finish_and_clear();

        if files.is_empty() {
            println!("{}", "No files found to process.".yellow());
            return Ok(());
        }

        println!("{} {} files", "Found:".cyan(), files.len());

        if self.cli.global.verbose {
            println!("\n{}", "Files to process:".cyan().bold());
            for file in &files {
                println!("  {}", file.display());
            }
            println!();
        }

        // Confirm unless --yes is passed
        if !self.confirm_action(&format!("Clean metadata from {} files?", files.len()))? {
            println!("{}", "Operation cancelled.".yellow());
            return Ok(());
        }

        // Process with progress bar
        let progress = self.create_progress_bar(files.len() as u64);
        let mut report = CleanReport::new();

        for file in &files {
            progress.set_message(format!("{}", file.file_name().unwrap_or_default().to_string_lossy()));

            match cleaner.clean_file(file) {
                Ok(result) => {
                    if self.cli.global.verbose {
                        if result.success {
                            progress.println(format!(
                                "  {} {}",
                                "✓".green(),
                                file.display()
                            ));
                        } else {
                            progress.println(format!(
                                "  {} {} - {}",
                                "✗".red(),
                                file.display(),
                                result.error.as_deref().unwrap_or("unknown error")
                            ));
                        }
                    }
                    report.add_result(result);
                }
                Err(e) => {
                    if self.cli.global.verbose {
                        progress.println(format!(
                            "  {} {} - {}",
                            "✗".red(),
                            file.display(),
                            e
                        ));
                    }
                    report.add_result(crate::core::FileResult::failure(file.clone(), e.to_string()));
                }
            }

            progress.inc(1);
        }

        progress.finish_and_clear();

        // Print summary
        self.print_report(&report);

        Ok(())
    }

    /// Displays metadata information about a file.
    fn run_info(&self, path: &Path) -> anyhow::Result<()> {
        self.print_header("File Information");

        if !path.exists() {
            self.print_error(&format!("File not found: {}", path.display()));
            return Ok(());
        }

        println!("{} {}\n", "File:".cyan(), path.display());

        // Get file metadata
        let metadata = std::fs::metadata(path)?;

        println!("{}", "Timestamps:".cyan().bold());
        if let Ok(created) = metadata.created() {
            println!("  Created:  {}", format_system_time(created));
        }
        if let Ok(modified) = metadata.modified() {
            println!("  Modified: {}", format_system_time(modified));
        }
        if let Ok(accessed) = metadata.accessed() {
            println!("  Accessed: {}", format_system_time(accessed));
        }

        println!("\n{}", "Attributes:".cyan().bold());
        println!("  Size:     {} bytes", metadata.len());
        println!("  Readonly: {}", metadata.permissions().readonly());

        // List alternate data streams
        #[cfg(windows)]
        {
            let _cleaner = MetadataCleaner::new();
            println!("\n{}", "Alternate Data Streams:".cyan().bold());

            // Use a simple approach to list streams
            let streams = list_streams_for_display(path);
            if streams.is_empty() {
                println!("  {}", "(none found)".dimmed());
            } else {
                for stream in streams {
                    println!("  {}", stream);
                }
            }
        }

        Ok(())
    }

    /// Creates a cleaner with the appropriate options.
    fn create_cleaner(&self) -> MetadataCleaner {
        let options = CleanOptions::all()
            .with_dry_run(self.cli.global.dry_run)
            .with_verbose(self.cli.global.verbose)
            .with_admin(self.cli.global.admin);

        MetadataCleaner::with_options(options)
    }

    /// Confirms an action with the user.
    fn confirm_action(&self, message: &str) -> anyhow::Result<bool> {
        if self.cli.global.yes {
            return Ok(true);
        }

        Ok(Confirm::new()
            .with_prompt(message)
            .default(false)
            .interact()?)
    }

    /// Prints a header for a command.
    fn print_header(&self, title: &str) {
        println!("\n{}", "━".repeat(50).dimmed());
        println!("{} {}", "▶".cyan(), title.bold());
        println!("{}\n", "━".repeat(50).dimmed());
    }

    /// Prints a success message.
    fn print_success(&self, message: &str) {
        println!("{} {}", "✓".green().bold(), message);
    }

    /// Prints an error message.
    fn print_error(&self, message: &str) {
        eprintln!("{} {}", "✗".red().bold(), message);
    }

    /// Prints a summary report.
    fn print_report(&self, report: &CleanReport) {
        println!("\n{}", "━".repeat(50).dimmed());
        println!("{}", "Summary".bold());
        println!("{}", "━".repeat(50).dimmed());

        println!("  Total files:     {}", report.total_files);
        println!("  {} {}", "Successful:".green(), report.successful);

        if report.failed > 0 {
            println!("  {} {}", "Failed:".red(), report.failed);
        } else {
            println!("  Failed:          {}", report.failed);
        }

        println!("  Streams removed: {}", report.total_streams_removed);

        if report.is_complete_success() {
            println!("\n{}", "All files cleaned successfully!".green().bold());
        } else {
            println!("\n{}", "Some files could not be cleaned.".yellow());
        }
    }

    /// Creates a spinner for indeterminate progress.
    fn create_spinner(&self, message: &str) -> ProgressBar {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        spinner.set_message(message.to_string());
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));
        spinner
    }

    /// Creates a progress bar for determinate progress.
    fn create_progress_bar(&self, total: u64) -> ProgressBar {
        let progress = ProgressBar::new(total);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.cyan} [{elapsed_precise}] [{bar:40.cyan/dim}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("█▓░"),
        );
        progress.enable_steady_tick(std::time::Duration::from_millis(100));
        progress
    }
}

/// Formats a SystemTime for display.
fn format_system_time(time: std::time::SystemTime) -> String {
    use std::time::UNIX_EPOCH;

    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            chrono_lite(secs)
        }
        Err(_) => "(invalid time)".to_string(),
    }
}

/// Simple timestamp formatting without external chrono dependency.
fn chrono_lite(unix_secs: u64) -> String {
    // Calculate date components from Unix timestamp
    let days = unix_secs / 86400;
    let remaining_secs = unix_secs % 86400;
    let hours = remaining_secs / 3600;
    let minutes = (remaining_secs % 3600) / 60;
    let seconds = remaining_secs % 60;

    // Approximate year/month/day (simplified, not accounting for all leap years)
    let mut year = 1970u64;
    let mut remaining_days = days;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let months = [31, 28 + if is_leap_year(year) { 1 } else { 0 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 1;
    for days_in_month in months {
        if remaining_days < days_in_month {
            break;
        }
        remaining_days -= days_in_month;
        month += 1;
    }
    let day = remaining_days + 1;

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Lists alternate data streams for display purposes.
#[cfg(windows)]
fn list_streams_for_display(path: &Path) -> Vec<String> {
    use windows::core::PCWSTR;
    use windows::Win32::Storage::FileSystem::{FindClose, FindFirstStreamW, FindNextStreamW, WIN32_FIND_STREAM_DATA};

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

        if let Ok(h) = handle {
            if !h.is_invalid() {
                loop {
                    let stream_name = String::from_utf16_lossy(
                        &find_data.cStreamName[..find_data.cStreamName.iter().position(|&c| c == 0).unwrap_or(find_data.cStreamName.len())]
                    );

                    if !stream_name.is_empty() && stream_name != "::$DATA" {
                        streams.push(format!("{} ({} bytes)", stream_name, find_data.StreamSize));
                    }

                    if FindNextStreamW(h, &mut find_data as *mut _ as *mut _).is_err() {
                        break;
                    }
                }
                let _ = FindClose(h);
            }
        }
    }

    streams
}

#[cfg(not(windows))]
fn list_streams_for_display(_path: &Path) -> Vec<String> {
    Vec::new()
}
