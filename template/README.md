# {{program_name_readable}}

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Pinocchio](https://img.shields.io/badge/Pinocchio-FF6B6B?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/anza-xyz/pinocchio)
[![Mollusk](https://img.shields.io/badge/Mollusk-2A7A7A?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/anza-xyz/mollusk)
[![Size](https://img.shields.io/badge/Size-13.85kb-AEFF23?style=for-the-badge&logoColor=white)](https://github.com/anza-xyz/pinocchio)

Out of the box, this Solana program is ~13.85kb, impressive size for what it does.

## Features

- `pinocchio` for program development.
- `mollusk` for testing and benchmarking.
- GH Action for building, testing and benchmarking.
- `cli` rust-based cli using `solana-client` and `solana-sdk`.

## How To

### Build contract

```sh
$ cargo build-sbf
```

### Deploy contract

```sh
$ solana program deploy -u d --program-id ./program-id.json \
  -k ./deployer.json \
  --upgrade-authority ./deployer.json \
  ./target/deploy/{{program_name_underscore}}.so
```

### Test

```sh
$ cargo test --features test-default
```

### Benchmark

```sh
$ cargo bench --features bench-default
```

## Attributions

Created with `gepetto`.
