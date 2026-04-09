use std::fs;
use std::path::Path;
use std::process::ExitCode;

const TAG_FILENAME: &str = ".RESTIC-IGNORE";
const TAG_CONTENT: &str = "restic-ignore: 58B12CA6-717F-4DA1-894A-C3126D8DFB2E";

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

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
