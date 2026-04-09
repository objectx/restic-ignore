use std::process::Command;

fn restic_ignore() -> Command {
    Command::new(env!("CARGO_BIN_EXE_restic-ignore"))
}

#[test]
fn unknown_flag_exits_with_error() {
    let output = restic_ignore().arg("--unknown").output().unwrap();
    assert!(!output.status.success());
}
