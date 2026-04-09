# restic-ignore Rust Rewrite Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rewrite restic-ignore from Go to Rust as a single-file CLI tool with minimal dependencies.

**Architecture:** Single `src/main.rs` using `lexopt` for argument parsing and `eprintln!` for output. No modules, no logging crate.

**Tech Stack:** Rust, lexopt

---

## File Structure

- Create: `Cargo.toml` — project manifest with lexopt dependency
- Create: `src/main.rs` — entire application
- Create: `tests/integration.rs` — integration tests using the built binary

---

### Task 1: Initialize Rust project

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`

- [ ] **Step 1: Create Cargo.toml**

```toml
[package]
name = "restic-ignore"
version = "0.1.0"
edition = "2021"

[dependencies]
lexopt = "0.3"

[profile.release]
strip = true
lto = true
```

- [ ] **Step 2: Create minimal main.rs**

```rust
fn main() {
    println!("restic-ignore");
}
```

- [ ] **Step 3: Verify it compiles and runs**

Run: `cargo run`
Expected: prints `restic-ignore`

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml src/main.rs
git commit -m "feat: initialize Rust project with lexopt dependency"
```

---

### Task 2: Implement argument parsing

**Files:**
- Modify: `src/main.rs`
- Create: `tests/integration.rs`

- [ ] **Step 1: Write integration test for --help / unknown flag**

Create `tests/integration.rs`:

```rust
use std::process::Command;

fn restic_ignore() -> Command {
    Command::new(env!("CARGO_BIN_EXE_restic-ignore"))
}

#[test]
fn unknown_flag_exits_with_error() {
    let output = restic_ignore().arg("--unknown").output().unwrap();
    assert!(!output.status.success());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test integration`
Expected: FAIL — current main ignores arguments

- [ ] **Step 3: Implement argument parsing in main.rs**

Replace `src/main.rs` with:

```rust
use std::process::ExitCode;

struct Args {
    dry_run: bool,
    verbose: u32,
    directories: Vec<String>,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut dry_run = false;
    let mut verbose: u32 = 0;
    let mut directories = Vec::new();
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('n') | Long("dry-run") => dry_run = true,
            Short('v') | Long("verbose") => verbose = verbose.saturating_add(1),
            Value(val) => directories.push(val.into_string().map_err(|_| "invalid UTF-8")?),
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        dry_run,
        verbose,
        directories,
    })
}

fn run() -> Result<(), String> {
    let _args = parse_args().map_err(|e| e.to_string())?;
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test integration`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/main.rs tests/integration.rs
git commit -m "feat: implement CLI argument parsing with lexopt"
```

---

### Task 3: Implement tag file creation

**Files:**
- Modify: `src/main.rs`
- Modify: `tests/integration.rs`

- [ ] **Step 1: Write integration test for tag file creation**

Add to `tests/integration.rs`:

```rust
use std::fs;

#[test]
fn creates_tag_file_in_existing_directory() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("subdir");
    fs::create_dir(&target).unwrap();

    let output = restic_ignore()
        .arg(target.to_str().unwrap())
        .output()
        .unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));

    let tag = target.join(".RESTIC-IGNORE");
    assert!(tag.exists());
    assert_eq!(
        fs::read_to_string(&tag).unwrap(),
        "restic-ignore: 58B12CA6-717F-4DA1-894A-C3126D8DFB2E"
    );
}

#[test]
fn creates_directory_and_tag_file_if_directory_missing() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("a/b/c");

    let output = restic_ignore()
        .arg(target.to_str().unwrap())
        .output()
        .unwrap();
    assert!(output.status.success());

    let tag = target.join(".RESTIC-IGNORE");
    assert!(tag.exists());
    assert_eq!(
        fs::read_to_string(&tag).unwrap(),
        "restic-ignore: 58B12CA6-717F-4DA1-894A-C3126D8DFB2E"
    );
}
```

- [ ] **Step 2: Add tempfile as a dev dependency**

Add to `Cargo.toml`:

```toml
[dev-dependencies]
tempfile = "3"
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test --test integration`
Expected: FAIL — tag files are not created yet

- [ ] **Step 4: Implement tag file creation in main.rs**

Add this function to `src/main.rs`:

```rust
use std::fs;
use std::path::Path;

const TAG_FILENAME: &str = ".RESTIC-IGNORE";
const TAG_CONTENT: &str = "restic-ignore: 58B12CA6-717F-4DA1-894A-C3126D8DFB2E";

fn process_directory(dir: &Path, dry_run: bool, verbose: u32) -> Result<(), String> {
    let tag_path = dir.join(TAG_FILENAME);

    if dry_run {
        eprintln!("would create: {}", tag_path.display());
        return Ok(());
    }

    if verbose >= 2 {
        eprintln!("debug: processing directory: {}", dir.display());
    }

    fs::create_dir_all(dir).map_err(|e| format!("failed to create directory {}: {e}", dir.display()))?;

    if verbose >= 1 {
        eprintln!("create: {}", tag_path.display());
    }

    fs::write(&tag_path, TAG_CONTENT)
        .map_err(|e| format!("failed to create {}: {e}", tag_path.display()))?;

    Ok(())
}
```

Update the `run()` function:

```rust
fn run() -> Result<(), String> {
    let args = parse_args().map_err(|e| e.to_string())?;
    let mut had_error = false;

    if args.verbose >= 2 {
        eprintln!("debug: args = {:?}", args.directories);
    }

    for dir in &args.directories {
        if let Err(e) = process_directory(Path::new(dir), args.dry_run, args.verbose) {
            eprintln!("error: {e}");
            had_error = true;
        }
    }

    if had_error {
        Err("one or more directories failed".to_string())
    } else {
        Ok(())
    }
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test --test integration`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml Cargo.lock src/main.rs tests/integration.rs
git commit -m "feat: implement tag file creation with directory auto-creation"
```

---

### Task 4: Implement dry-run and verbose behavior

**Files:**
- Modify: `tests/integration.rs`

- [ ] **Step 1: Write integration test for dry-run**

Add to `tests/integration.rs`:

```rust
#[test]
fn dry_run_does_not_create_files() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("nope");

    let output = restic_ignore()
        .args(["-n", target.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(!target.exists(), "directory should not be created in dry-run mode");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("would create"), "dry-run should print what it would do");
}

#[test]
fn verbose_prints_create_message() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("vdir");

    let output = restic_ignore()
        .args(["-v", target.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("create:"), "verbose mode should print creation messages");
}
```

- [ ] **Step 2: Run tests to verify they pass**

Run: `cargo test --test integration`
Expected: PASS — dry-run and verbose logic was already implemented in Task 3

- [ ] **Step 3: Commit (if any adjustments were needed)**

```bash
git add tests/integration.rs
git commit -m "test: add integration tests for dry-run and verbose flags"
```

---

### Task 5: Handle multiple directories and error continuation

**Files:**
- Modify: `tests/integration.rs`

- [ ] **Step 1: Write integration tests for multiple directories and error cases**

Add to `tests/integration.rs`:

```rust
#[test]
fn handles_multiple_directories() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a");
    let b = dir.path().join("b");

    let output = restic_ignore()
        .args([a.to_str().unwrap(), b.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(a.join(".RESTIC-IGNORE").exists());
    assert!(b.join(".RESTIC-IGNORE").exists());
}

#[test]
fn no_arguments_succeeds_with_no_output() {
    let output = restic_ignore().output().unwrap();
    assert!(output.status.success());
}
```

- [ ] **Step 2: Run tests to verify they pass**

Run: `cargo test --test integration`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add tests/integration.rs
git commit -m "test: add tests for multiple directories and no-args case"
```

---

### Task 6: Update project files

**Files:**
- Modify: `CLAUDE.md`
- Modify: `.gitignore`

- [ ] **Step 1: Update CLAUDE.md with build commands**

Add build/test commands and update status in `CLAUDE.md` to reflect the Rust implementation:

```markdown
## Build & Test

- Build: `cargo build`
- Release build: `cargo build --release`
- Run tests: `cargo test`
- Run a single test: `cargo test --test integration <test_name>`
- Run with args: `cargo run -- -v /path/to/dir`
```

Update the Status section to note the Rust implementation is complete.

- [ ] **Step 2: Add Rust build artifacts to .gitignore**

Append to `.gitignore`:

```
/target/
```

- [ ] **Step 3: Verify everything works end to end**

Run: `cargo test`
Expected: all tests pass

Run: `cargo build --release && ls -lh target/release/restic-ignore`
Expected: small binary exists

- [ ] **Step 4: Commit**

```bash
git add CLAUDE.md .gitignore
git commit -m "chore: update CLAUDE.md and .gitignore for Rust project"
```
