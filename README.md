# rust-utils

Fast, memory-safe CLI utilities written in Rust. This repository contains custom implementations of standard command-line tools tailored for speed and efficiency.

## Utilities

### 1. `rcat`
A lightweight replacement for the standard `cat` command, optimized for reading and printing file contents.

- **Usage**:
  ```sh
  rcat <file_path>
  ```
- **Features**:
  - Validates command line arguments.
  - Safe error handling for missing or unreadable files.

### 2. `rls`
A clean utility for listing directory contents.

- **Usage**:
  ```sh
  rls [directory_path]
  ```
- **Features**:
  - Lists all files and directories in the target path.
  - Defaults to the current directory (`.`) if no path is provided.

---

## Build & Release

### Local Compilation
To compile the utilities locally:

1. Ensure you have the Rust toolchain installed.
2. Build the project in release mode:
   ```sh
   cargo build --release
   ```
3. The compiled binaries will be located under `target/release/rcat.exe` and `target/release/rls.exe` (on Windows).

### CI/CD Releases
This repository uses a GitHub Actions workflow to automatically build and release binaries for multiple platforms (Linux, macOS, Windows) when a new version tag is pushed:

1. Commit your changes.
2. Tag the commit (e.g., `v0.1.0`):
   ```sh
   git tag -a v0.1.0 -m "Release v0.1.0"
   ```
3. Push the tag to trigger the builder pipeline.
