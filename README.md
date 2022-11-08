# Safe rm
safe-rm is a simple wrapper around rm that prevents you from accidentally deleting files.

## Installation
```bash
cargo install safe-rm
```
## Usage
```bash
safe-rm [FLAGS] [OPTIONS] <FILE>...
```

## Alias in .bashrc or .zshrc etc.
```bash
alias rm="safe-rm"
```