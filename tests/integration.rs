use std::{process::{Command, Stdio, Output}, io::Write};

type TestResult = Result<(), Box<dyn std::error::Error>>;
const BIN_PATH: &str = "./target/debug/templar";

#[test]
fn no_args() -> TestResult {
    let result = run_templar("", "");

    assert_eq!(result.status.code(), Some(2));
    
    Ok(())
}

#[test]
fn basic_toml() -> TestResult {
    let input = "[some]
    body = 'hello'

    [[multi]]
    a = 1

    [[multi]]
    a = 2
    ";

    let result = run_templar("-f toml -t ./tests/basic_template.tera", input);

    assert!(result.status.success());
    assert_eq!(String::from_utf8(result.stdout)?, "hello\n1\n2\n");

    Ok(())
}

fn run_templar(args: &str, input: &str) -> Output {
    let mut child = Command::new(BIN_PATH)
        .args(args.split_whitespace())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(input.as_bytes()).unwrap();
    drop(stdin);

    child.wait_with_output().unwrap()
}