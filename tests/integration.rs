use std::{process::{Command, Stdio, Output}, io::Write};

type TestResult = Result<(), Box<dyn std::error::Error>>;
const BIN_PATH: &str = "./target/debug/teratemplar";

#[test]
fn no_args() -> TestResult {
    let result = run_templar("");

    assert_eq!(result.status.code(), Some(2));
    
    Ok(())
}

#[test]
fn basic_toml() -> TestResult {
    let result = run_templar("-i tests/basic.toml -t tests/basic_template.tera");

    assert!(result.status.success());
    assert_eq!(String::from_utf8(result.stdout)?, "hello\n1\n2\n");

    Ok(())
}

fn run_templar(args: &str) -> Output {
    let child = Command::new(BIN_PATH)
        .args(args.split_whitespace())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child.wait_with_output().unwrap()
}