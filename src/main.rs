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
