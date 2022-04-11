use std::process::{Command, Output};

pub const SIGI_PATH: &str = std::env!("CARGO_BIN_EXE_sigi");

pub fn sigi(stack: &str, args: &[&str]) -> SigiOutput {
    return Command::new(SIGI_PATH)
        .arg("--stack")
        .arg(stack)
        .args(args)
        .output()
        .expect("Error running process")
        .into();
}

pub struct SigiOutput {
    success: bool,
    stdout: String,
    stderr: String,
}

impl SigiOutput {
    pub fn assert_success(&self) {
        assert!(self.success);
    }

    pub fn assert_failure(&self) {
        assert!(!self.success);
    }

    pub fn assert_stdout_eq(&self, expected_stdout: &str) {
        assert_eq!(
            &self.stdout, expected_stdout,
            "sigi stdout did not exactly match expectation."
        );
    }

    pub fn assert_stdout_line_eq(&self, expected_line: &str) {
        let some_line_eqs = self.stdout.lines().any(|line| line == expected_line);
        assert!(
            some_line_eqs,
            "sigi stdout had no line matching: `{:?}`.",
            expected_line
        );
    }

    pub fn assert_stdout_lines_eq(&self, expected_lines: &[&str]) {
        self.stdout
            .lines()
            .zip(expected_lines.iter())
            .enumerate()
            .for_each(|(i, (actual, expected))| {
                assert_eq!(
                    &actual, expected,
                    "sigi stdout did not match expected output on line {}\n",
                    i
                )
            });
    }

    pub fn assert_stdout_line_starts_with(&self, expected_prefix: &str) {
        let some_line_eqs = self
            .stdout
            .lines()
            .any(|line| line.starts_with(expected_prefix));
        assert!(
            some_line_eqs,
            "sigi stdout had no line starting with: {}",
            expected_prefix
        );
    }

    pub fn assert_stderr_empty(&self) {
        assert_eq!(&self.stderr, "", "sigi stderr was expected to be empty.");
    }
}

impl From<Output> for SigiOutput {
    fn from(output: Output) -> SigiOutput {
        SigiOutput {
            success: output.status.success(),
            stdout: String::from_utf8(output.stdout).expect("Couldn't read stdout"),
            stderr: String::from_utf8(output.stderr).expect("Couldn't read stderr"),
        }
    }
}

#[test]
fn assert_success() {
    let output = SigiOutput { success: true, stdout: String::new(), stderr: String::new()};

    output.assert_success();
}

#[test]
fn assert_failure() {
    let output = SigiOutput { success: false, stdout: String::new(), stderr: String::new()};

    output.assert_failure();
}

#[test]
fn assert_stdout_eq() {
    let output = SigiOutput { success: true, stdout: "hello".to_string(), stderr: String::new()};

    output.assert_stdout_eq("hello");
}

#[test]
fn assert_stdout_line_eq() {
    let output = SigiOutput { success: true, stdout: "hey\nhello".to_string(), stderr: String::new()};

    output.assert_stdout_line_eq("hello");
}

#[test]
fn assert_stdout_lines_eq() {
    let output = SigiOutput { success: true, stdout: "hey\nhello there".to_string(), stderr: String::new()};

    output.assert_stdout_lines_eq(&["hey", "hello there"]);
}

#[test]
fn assert_stdout_line_starts_with() {
    let output = SigiOutput { success: true, stdout: "hey\nhello there".to_string(), stderr: String::new()};

    output.assert_stdout_line_starts_with("hello");
}

#[test]
fn assert_stderr_empty() {
    let output = SigiOutput { success: true, stdout: "hey\nhello there".to_string(), stderr: String::new()};

    output.assert_stderr_empty();
}
