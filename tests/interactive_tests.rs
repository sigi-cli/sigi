mod run_sigi;

use run_sigi::{piping, sigi};

#[test]
fn sigi_interactive_preamble() {
    let res = sigi("_integ::interactive", &["interactive"]);
    res.assert_success();
    res.assert_stdout_line_starts_with("sigi 3.3");
    res.assert_stdout_line_starts_with(r#"Type "quit", "q", or "exit" to quit"#);
    res.assert_stdout_line_starts_with(
        r#"Type "?" for quick help, or "help" for a more verbose help message"#,
    );
    res.assert_stderr_empty();
}

#[test]
fn sigi_interactive_basic() {
    let res = piping(&["push hello world"]).into_sigi("_integ::interactive", &["interactive"]);
    res.assert_stdout_line_starts_with("sigi 3.3");
    res.assert_stdout_line_starts_with(r#"Type "quit", "q", or "exit" to quit"#);
    res.assert_stdout_line_starts_with(
        r#"Type "?" for quick help, or "help" for a more verbose help message"#,
    );
    res.assert_stdout_line_starts_with("Created: hello world");
    res.assert_stdout_line_starts_with("Ctrl+d: Buen biåhe!");
    res.assert_stderr_empty();
}
