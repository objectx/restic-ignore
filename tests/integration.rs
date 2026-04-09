use std::fs;
use std::process::Command;

fn restic_ignore() -> Command {
    Command::new(env!("CARGO_BIN_EXE_restic-ignore"))
}

#[test]
fn unknown_flag_exits_with_error() {
    let output = restic_ignore().arg("--unknown").output().unwrap();
    assert!(!output.status.success());
}

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
