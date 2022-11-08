# Safe rm

[![Version](https://img.shields.io/crates/v/saferm?style=for-the-badge)](https://crates.io/crates/saferm)
[![License](https://img.shields.io/crates/l/saferm?style=for-the-badge)](https://crates.io/crates/saferm)

saferm is a simple wrapper around rm that prevents you from accidentally deleting files.

## Installation
```bash
cargo install saferm
```
## Usage
```bash
saferm [FLAGS] [OPTIONS] <FILE>...
```

## Alias in .bashrc or .zshrc etc.
```bash
alias rm="saferm"
```