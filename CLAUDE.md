# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

rs-mahito is a Windows CLI tool for removing metadata from files on NTFS filesystems. It clears privacy-sensitive information like Zone.Identifier streams, file timestamps, Office document properties, and file ownership.

This is a **monorepo** containing:
- **`apps/cli/`** - Rust CLI application
- **`apps/web/`** - Next.js documentation website

---

## Monorepo Structure

```
rs-mahito/
├── apps/
│   ├── cli/              # Rust CLI application
│   │   ├── src/
│   │   │   ├── cli/      # CLI argument parsing (clap)
│   │   │   ├── core/     # Metadata cleaning logic
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   └── web/              # Next.js documentation site
│       ├── src/
│       │   ├── app/      # Next.js app router
│       │   ├── components/
│       │   ├── content/docs/  # MDX documentation
│       │   ├── lib/
│       │   ├── styles/
│       │   └── utils/
│       └── package.json
├── config/
│   └── tsconfig/         # Shared TypeScript config
├── packages/             # Shared packages (future)
├── Cargo.toml            # Rust workspace config
├── package.json          # pnpm workspace config
├── turbo.json            # Turborepo tasks
├── biome.json            # Biome/Ultracite linting
└── pnpm-workspace.yaml
```

---

## CLI (apps/cli/)

### Build Commands

```bash
# Build release binary
cargo build --release

# Run clippy for linting
cargo clippy

# Run the CLI directly
cargo run -- <command>

# Run from workspace root
cargo run -p rs-mahito -- <command>
```

The release binary is output to `target/release/rs-mahito.exe`.

### Architecture

- **`cli/`** - Command-line interface using clap derive API
  - `args.rs` - Defines `Cli` struct with subcommands (`File`, `Dir`, `Recursive`, `Info`) and global options (`--dry-run`, `--verbose`, `--yes`, `--admin`)
  - `runner.rs` - Executes commands and handles user interaction (prompts, progress bars)

- **`core/`** - Metadata cleaning logic
  - `cleaner.rs` - `MetadataCleaner` struct with Windows API calls for:
    - Removing NTFS alternate data streams (`FindFirstStreamW`, `DeleteFileW`)
    - Resetting timestamps to Jan 1, 2000 (`SetFileTime`)
    - Clearing file owner via SID manipulation (`SetNamedSecurityInfoW`)
    - Cleaning Office XML properties by rewriting ZIP archives
  - `types.rs` - Data structures (`CleanOptions`, `CleanMode`, `FileResult`, `CleanReport`)
  - `error.rs` - Error types using `thiserror`

### Key Dependencies

- `windows` crate for Win32 API access (file system, security APIs)
- `clap` for CLI argument parsing
- `walkdir` for directory traversal
- `zip` for Office Open XML (docx/xlsx/pptx) manipulation
- `dialoguer` for interactive file selection

### Platform Notes

This tool is Windows-specific. The `#[cfg(windows)]` blocks contain the actual implementation; non-Windows builds compile with no-op stubs.

---

## Frontend (apps/web/)

The documentation website is built with Next.js 15 and Fumadocs, following the structure from [Kanpeki](https://github.com/fellipeutaka/kanpeki).

### Build Commands

```bash
# Install dependencies (from root)
pnpm install

# Run all workspaces in dev mode
pnpm dev

# Build the web app only
pnpm build:web

# Build the CLI only
pnpm build:cli

# Type check
pnpm type-check

# Lint and format
pnpm lint
pnpm format
pnpm check
```

### Key Dependencies

- **Next.js 15** - React framework with App Router
- **Fumadocs** - Documentation framework (fumadocs-core, fumadocs-ui, fumadocs-mdx)
- **Tailwind CSS v4** - Utility-first CSS
- **Zod** - Schema validation

### Code Style (Biome + Ultracite)

This project uses Biome with Ultracite presets for linting and formatting:

- **Double quotes** for strings
- **Kebab-case** for file names
- **Interfaces** over type aliases
- **No default exports** (except in Next.js app/ directory and config files)
- **Sorted Tailwind classes** via `cn`, `cva`, `cv`, `cx` functions
- **Organized imports** automatically

Run `pnpm check` to lint and format code.

### Path Aliases

In the web app, use these import aliases:
- `~/` → `./src/` (e.g., `import { cn } from "~/utils/cn"`)
- `~:content` → `./.source` (Fumadocs generated content)
