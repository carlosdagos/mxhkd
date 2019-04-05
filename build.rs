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
        .arg("docs/mxhkd.1.md")
        .status()
        .unwrap();

    assert!(ronn.success());

    let copy_github_pages = Command::new("mv")
        .arg("docs/mxhkd.1.html")
        .arg("docs/index.html")
        .status()
        .unwrap();

    assert!(copy_github_pages.success());
}
