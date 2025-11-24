# rs-mahito

A CLI tool to clear metadata from files on Windows NTFS filesystems.

<img width="1200" height="540" alt="image" src="https://github.com/user-attachments/assets/b0369fbd-7a2a-4330-a09b-8109e959b700" />


## Features

- **NTFS Alternate Data Streams** - Removes Zone.Identifier and other ADS that track file origin
- **File Timestamps** - Resets created, modified, and accessed times to a neutral date
- **Office Document Properties** - Clears Author, Company, Last Modified By from .docx, .xlsx, .pptx files
- **File Owner** - Clears NTFS file ownership (requires Administrator)

## Installation

### From Source

```bash
git clone https://github.com/victormicco/rs-mahito.git
cd rs-mahito
cargo build --release
```

The binary will be at `target/release/rs-mahito.exe`

### From Cargo

```bash
cargo install rs-mahito
```

## Usage

```bash
# Clean a single file
rs-mahito file -p path/to/file.txt

# Clean all files in a directory (non-recursive)
rs-mahito dir -p path/to/folder

# Clean all files recursively
rs-mahito recursive -p path/to/folder

# View file metadata info
rs-mahito info -p path/to/file.txt

# Interactive mode - select file from current directory
rs-mahito file
```

### Options

| Flag | Description |
|------|-------------|
| `-n, --dry-run` | Preview changes without modifying files |
| `-v, --verbose` | Show detailed output |
| `-y, --yes` | Skip confirmation prompts |
| `-a, --admin` | Run with admin privileges (clears file owner) |

### Examples

```bash
# Preview what would be cleaned
rs-mahito file -p document.docx --dry-run

# Clean entire project folder recursively without prompts
rs-mahito recursive -p ./my-project -y

# Clean with verbose output
rs-mahito dir -p ./downloads -v

# Full clean including owner (run as Administrator)
rs-mahito file -p sensitive.xlsx --admin
```

## What Gets Cleaned

| Metadata Type | Description |
|---------------|-------------|
| Zone.Identifier | "Downloaded from internet" warning |
| SummaryInformation | OLE document properties |
| File timestamps | Created, modified, accessed dates |
| Office XML properties | Author, Company, Last Modified By |
| File owner (admin) | NTFS ownership information |

## Requirements

- Windows 10/11
- Rust 1.70+ (for building from source)

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
