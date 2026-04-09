# restic-ignore: Rust Rewrite Design

## Overview

Rewrite `restic-ignore` from Go to Rust for a smaller executable. The tool creates `.RESTIC-IGNORE` tag files in specified directories to exclude them from restic backups.

## Goals

- Feature parity with the Go implementation (in `Attic/`)
- Minimal binary size
- Minimal dependencies

## CLI Interface

```
restic-ignore [flags] [<directory>...]
```

### Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--dry-run` | `-n` | Print what would be done, don't touch the filesystem |
| `--verbose` | `-v` | Increase verbosity (stackable: `-vv`, `-vvv`) |

### Verbosity Levels

| Level | Output |
|-------|--------|
| 0 (default) | Errors only |
| 1 (`-v`) | Info — file creation messages |
| 2+ (`-vv`) | Debug — argument details |

## Architecture

Single file: `src/main.rs`. No modules, no internal library split.

### Argument Parsing

Use `lexopt` crate for argument parsing. Collect flags and positional directory arguments.

### Behavior

For each directory argument:

1. Create the directory and parents if they don't exist (`std::fs::create_dir_all`)
2. Write a `.RESTIC-IGNORE` file in the directory with content: `restic-ignore: 58B12CA6-717F-4DA1-894A-C3126D8DFB2E`

In dry-run mode, skip all filesystem writes but log what would happen at verbosity level 0 (always visible).

### Error Handling

- Print errors to stderr via `eprintln!`
- Continue processing remaining directories after a failure
- Exit 0 on success, exit 1 if any directory failed

### Logging

Use `eprintln!` directly — no logging crate. Verbosity level is checked inline before printing.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `lexopt` | CLI argument parsing |

No other dependencies.

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All directories processed successfully |
| 1 | One or more directories failed |

## Tag File

- Filename: `.RESTIC-IGNORE`
- Content: `restic-ignore: 58B12CA6-717F-4DA1-894A-C3126D8DFB2E`
- Permissions: default (no explicit chmod, unlike Go's 0644 — Rust's `fs::write` uses umask)
