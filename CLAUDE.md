# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`restic-ignore` is a CLI tool that creates `.RESTIC-IGNORE` tag files in specified directories, used to exclude those directories from [restic](https://restic.net/) backups. The tag file contains a fixed UUID marker: `restic-ignore: 58B12CA6-717F-4DA1-894A-C3126D8DFB2E`.

## Status

The Rust implementation is complete. The previous Go implementation lives in `Attic/` for reference. The Go code used cobra (CLI), viper (config), and zerolog (logging), and supported `--dry-run` and `--verbose` flags.

## Key Behaviors (from previous implementation)

- Accepts one or more directory paths as arguments
- Creates the target directory (including parents) if it doesn't exist
- Places a `.RESTIC-IGNORE` file in each directory
- `--dry-run` / `-n`: skip all filesystem modifications
- `--verbose` / `-v`: increase log verbosity (stackable: `-vv`, `-vvv`)

## Build & Test

- Build: `cargo build`
- Release build: `cargo build --release`
- Run tests: `cargo test`
- Run a single test: `cargo test --test integration <test_name>`
- Run with args: `cargo run -- -v /path/to/dir`
