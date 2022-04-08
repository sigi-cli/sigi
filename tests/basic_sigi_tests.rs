mod run_sigi;

use run_sigi::sigi;

#[test]
fn sigi_version() {
    let res = sigi(&["--version"]);

    res.assert_success();
    res.assert_stdout_line_starts_with("sigi 3.1");
    res.assert_stderr_empty();
}

#[test]
fn sigi_empty_stack_stuff() {
    let with_args = |etc| vec!["--stack", "_integ_empty_stack"].(etc);

    let res = sigi(&["--version"]);

    res.assert_success();
    res.assert_stdout_line_starts_with("sigi 3.1");
    res.assert_stderr_empty();
}
