# Gepetto

Solana's `pinnochio` project scaffold creator.

## Features

- `new` This command helps scaffold a Solana pinocchio project featuring mollusk tests and benches.

## Installation

### Install from Source

To install Gepetto from source:

```bash
# Clone the repository
git clone <repository-url>
cd gepetto

# Install the CLI globally
cargo install --path .
```

After installation, you can use `gepetto` command from anywhere in your terminal.

### Install from Git Repository

You can also install directly from the git repository:

```bash
cargo install --git <repository-url>
```

### Build for Development

To build the project for development without installing:

```bash
cargo build --release
```

## Usage

```bash
# Run the CLI
gepetto new

# Show help
gepetto --help
```
