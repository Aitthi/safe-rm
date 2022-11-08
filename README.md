# Safe rm
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