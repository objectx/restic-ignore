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
