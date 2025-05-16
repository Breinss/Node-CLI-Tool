# Node-CLI-Tool

A Rust-based command-line utility for finding and cleaning up node_modules directories that are located within cache folders on your system.

## Overview

This tool recursively scans your filesystem for `node_modules` directories located within `.cache` directories. These cached node_modules can consume significant disk space and are often safe to remove as they can be regenerated when needed.

## Features

- Recursively scans directories to locate node_modules within cache folders
- Lists all found directories with their paths
- Provides an interactive prompt to safely delete found directories
- Handles errors gracefully during scanning and deletion operations

## Installation

### Prerequisites

- Rust and Cargo (1.86.0 or later)

### Building from Source

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd Node-CLI-Tool
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be available at `target/release/Node-CLI-Tool`

## Usage

Run the tool with:

```bash
./target/release/Node-CLI-Tool
```

By default, the tool will:
1. Scan your system starting from the root directory (/)
2. List all node_modules directories found within .cache directories
3. Prompt you to confirm whether you want to delete these directories

## Customization

The tool is built using Clap for command-line argument parsing. Future versions will support command-line arguments for:
- Custom starting path
- Pattern matching
- Non-interactive mode

## How It Works

1. The tool recursively scans directories from the specified starting point
2. When a `node_modules` directory is found, it checks if any of its ancestors is a `.cache` directory
3. If a matching directory is found, it's added to a list of candidates for removal
4. After scanning is complete, the tool displays all found directories and prompts for confirmation before deletion

## License

[Add your license here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
