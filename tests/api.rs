use std::process::Command;

#[test]
fn test_without_args_t_0() {
    let output = Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("failed to execute process");
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("the following required arguments were not provided:"))
}

#[test]
fn test_without_args_t_1() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--read-from", "test_CHANGELOG.md"])
        .output()
        .expect("failed to execute process");
    assert!(String::from_utf8_lossy(&output.stderr).contains(
        "\nerror: the following required arguments were not provided:\n  --start-str <START_STR>"
    ));
}

#[test]
fn test_without_args_t_2() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--start-str", "test_CHANGELOG.md"])
        .output()
        .expect("failed to execute process");
    assert!(String::from_utf8_lossy(&output.stderr).contains(
        "\nerror: the following required arguments were not provided:\n  --read-from <READ_FROM>"
    ));
}


