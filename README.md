# gitbulk

A command-line tool for performing bulk Git operations across multiple repositories in a directory.

## Overview

gitbulk helps you manage multiple Git repositories at once by executing common Git commands across all repositories found in subdirectories of your current working directory. This is particularly useful for developers who work with multiple related projects or microservices.

## Features

- **Automatic repository discovery**: Finds all Git repositories in subdirectories
- **Bulk operations**: Execute Git commands across multiple repos simultaneously
- **Safe pulls**: Only pulls when working tree is clean
- **Color-coded output**: Easy-to-read directory names with color highlighting
- **Simple CLI**: Intuitive command-line interface

## Installation

### Building from source

1. Clone this repository:
```bash
git clone <repository-url>
cd gitbulk
```

2. Build with Cargo:
```bash
cargo build --release
```

3. The binary will be available at `target/release/gitbulk`

### Adding to PATH (Optional)

Copy the binary to a directory in your PATH for global access:
```bash
# Windows
copy target\release\gitbulk.exe C:\your\path\bin\

# Unix-like systems
cp target/release/gitbulk /usr/local/bin/
```

## Usage

Navigate to a directory containing multiple Git repositories and run:

```bash
gitbulk <COMMAND>
```

### Available Commands

| Command | Description |
|---------|-------------|
| `list` | List all Git repositories in the current directory |
| `status` | Show Git status for all repositories |
| `fetch` | Fetch updates from remote for all repositories |
| `branch` | Show current branch for all repositories |
| `pull` | Pull changes (only if working tree is clean) |
| `help` | Show help message |

### Examples

```bash
# List all Git repositories
gitbulk list

# Check status of all repositories
gitbulk status

# Fetch from all remotes
gitbulk fetch

# Show current branch for each repo
gitbulk branch

# Pull changes (only for clean working trees)
gitbulk pull

# Show help
gitbulk help
```

## Directory Structure

gitbulk expects your projects to be organized like this:

```
your-workspace/
├── project1/
│   └── .git/
├── project2/
│   └── .git/
├── project3/
│   └── .git/
└── non-git-folder/
```

It will automatically discover and operate on `project1`, `project2`, and `project3`, while ignoring `non-git-folder`.

## Safety Features

- **Clean working tree check**: The `pull` command only executes if the repository has no uncommitted changes
- **Error handling**: Commands continue executing on other repositories even if one fails
- **Status verification**: Git status is checked before performing pull operations

## Output

gitbulk provides clear, color-coded output showing:
- **Directory names** in yellow for easy identification
- **Error messages** when repositories can't be pulled due to uncommitted changes
- **Git command output** for each repository

## Requirements

- Rust 1.0+ (for building)
- Git installed and available in PATH
- Windows, macOS, or Linux
