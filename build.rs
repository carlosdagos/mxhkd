use std::process::Command;

// Build the documentation first
fn main() {
    let date = Command::new("date")
        .arg("-I")
        .arg("-r")
        .arg("Cargo.toml")
        .output()
        .unwrap();

    assert!(date.status.success());

    let date = String::from_utf8(date.stdout).unwrap();

    let ronn = Command::new("ronn")
        .arg("--date")
        .arg(date)
        .arg("doc/mxhkd.1.md")
        .status()
        .unwrap();

    assert!(ronn.success());
}
