# HawkOps🪽

A StackHawk companion CLI

## Purpose

HawkOps is a CLI tool that provides a set of commands to interact with the StackHawk API. This tool is intended to be used by developers and DevOps engineers to automate tasks that are not available in the official StackHawk UI or CLI.

## Initial Goals

Here are a few itches I'd like to scratch with this tool.

1. Setup initial authentication with StackHawk API
2. Maintain authorization - check and if necessary refresh token on each command
3. List all applications
4. List latest scans for an application
5. List users and their teams
6. Query apps for scan configs with particular parameters
8. Create other reports as needed
9. Model API queries to support StackHawk prospects and customers
10. Create a query language to support more complex queries

Maybe outside of the StackHawk API...

1. Detect SSL MitM proxy and extract SH-ready PEM
2. Prune a HAR file for a specific host
3. Analyze HAR for unique hosts
4. Prune OAS files - include/exclude

## Usage

```zsh
hawkops [noun-command] [verb-command] [options]
hawkops --help
hawkops [noun-command] --help
hawkops [noun-command] [verb-command] --help
```


## Build and Run

```zsh
cargo build
./target/debug/hawkops --help
```

## Development Setup

### Install Rust
    
```zsh
brew install rustup-init
rustup-init
```

### Install cross-compile toolchain
    
```zsh
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
cargo install cross
```

### And that's not all!
    
```zsh
cargo install cargo-binstall    # To download compiled deps
cargo install cargo-edit        # To add dependencies
cargo install cargo-watch       # For live reload
```
