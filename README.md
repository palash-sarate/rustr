# rust-utils

Fast, memory-safe CLI utilities written in Rust. This repository contains custom implementations of standard command-line tools tailored for speed and efficiency.

## Utilities

### 1. `rcat`
A lightweight replacement for the standard `cat` command, optimized for reading and printing file contents.

- Usage: `rcat <file_path>`
- Features: Validates command line arguments; safe error handling.

### 2. `rls`
A clean utility for listing directory contents.
- Usage: `rls [directory_path]` (defaults to `.`)

### 3. `rgrep`
A recursive pattern matcher for finding text within files.
- Usage: `rgrep <pattern> [path]` (defaults to `.`)

### 4. `rfind`
Searches a directory recursively for files or folders matching a query string.
- Usage: `rfind <query> [path]` (defaults to `.`)

### 5. `rfzf`
A fast CLI line-stream filter with fuzzy-matching logic.
- Usage: `<command> | rfzf <query>`  *or*  `rfzf <query> [file]`

### 6. `rdu`
Calculates recursive disk usage of a directory showing human-readable sizes (KB, MB, GB).
- Usage: `rdu [path]` (defaults to `.`)

### 7. `rsed`
Stream editor for replacing occurrences of a search pattern.
- Usage: `<command> | rsed <find> <replace>`  *or*  `rsed <find> <replace> [file]`

### 8. `rawk`
Extracts and prints specific columns from input lines.
- Usage: `<command> | rawk <cols> [delimiter] [file]` (where `cols` is comma-separated list of 1-based index numbers)
- Example: `rawk 1,3 "," data.csv`

---

## Installation

### Method 1: Cargo Install (Recommended for Rust users)
You can build and install all utilities directly from source using cargo:
```sh
cargo install --git https://github.com/palash-sarate/rustr.git
```
*Note: Make sure `~/.cargo/bin` is in your system `PATH` env variable.*

### Method 2: Direct Binary Download
1. Head over to the [Releases](https://github.com/palash-sarate/rustr/releases) page.
2. Download the binary corresponding to your target platform.
3. Make the binary executable (on Unix-like systems: `chmod +x <binary-name>`).
4. Move the binary into your system `PATH` directory (e.g. `/usr/local/bin`) to run it from anywhere.

---

## Build & Release

### Local Compilation
To compile the utilities locally:

1. Ensure you have the Rust toolchain installed.
2. Build the project in release mode:
   ```sh
   cargo build --release
   ```
3. The compiled binaries (`rcat.exe`, `rls.exe`, `rgrep.exe`, `rfind.exe`, `rfzf.exe`, `rdu.exe`, `rsed.exe`, `rawk.exe`) will be located under `target/release/` (on Windows).

### CI/CD Releases
This repository uses a GitHub Actions workflow to automatically build and release binaries for multiple platforms (Linux, macOS, Windows) when a new version tag is pushed:

1. Commit your changes.
2. Tag the commit (e.g., `v0.1.0`):
   ```sh
   git tag -a v0.1.0 -m "Release v0.1.0"
   ```
3. Push the tag to trigger the builder pipeline.
