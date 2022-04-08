mod run_sigi;

use run_sigi::sigi;

#[test]
fn sigi_version() {
    let res = sigi("_integ::version", &["--version"]);
    res.assert_success();
    res.assert_stdout_line_starts_with("sigi 3.1");
    res.assert_stderr_empty();
}

#[test]
fn sigi_empty_stack_stuff() {
    let stack = "_integ::empty_stack";

    let res = sigi(stack, &["delete-all"]);
    res.assert_success();

    let res = sigi(stack, &[]);
    res.assert_success();
    res.assert_stdout_eq("Now: NOTHING\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["peek"]);
    res.assert_success();
    res.assert_stdout_eq("Now: NOTHING\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["list"]);
    res.assert_success();
    res.assert_stdout_eq("Now: NOTHING\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["head"]);
    res.assert_success();
    res.assert_stdout_eq("Now: NOTHING\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["tail"]);
    res.assert_success();
    res.assert_stdout_eq("Now: NOTHING\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["count"]);
    res.assert_success();
    res.assert_stdout_eq("0\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["is-empty"]);
    res.assert_success();
    res.assert_stdout_eq("true\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["complete"]);
    res.assert_success();
    res.assert_stdout_eq("Now: NOTHING\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["delete"]);
    res.assert_success();
    res.assert_stdout_eq("Now: NOTHING\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["delete-all"]);
    res.assert_success();
    res.assert_stdout_eq("Deleted: 0 items\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["list-stacks"]);
    res.assert_success();
    res.assert_stdout_line_starts_with(stack);
    res.assert_stderr_empty();
}
