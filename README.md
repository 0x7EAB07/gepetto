# Gepetto

Solana's `pinnochio` project scaffold creator.

## Features

- TODO

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
cargo run

# Show help
cargo run -- --help
```

## Development

### Running Tests

```bash
cargo test
```

### Testing the Library

```bash
cargo test -p gepetto-core
```
