mod run_sigi;

use run_sigi::sigi;

#[test]
fn sigi_single_item_ops() {
    let stack = "_integ::single_item";

    let res = sigi(stack, &["delete-all"]);
    res.assert_success();

    let res = sigi(stack, &["push", "hello"]);
    res.assert_success();
    res.assert_stdout_eq("Created: hello\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &[]);
    res.assert_success();
    res.assert_stdout_eq("Now: hello\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["peek"]);
    res.assert_success();
    res.assert_stdout_eq("Now: hello\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["list"]);
    res.assert_success();
    res.assert_stdout_eq("Now: hello\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["head"]);
    res.assert_success();
    res.assert_stdout_eq("Now: hello\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["tail"]);
    res.assert_success();
    res.assert_stdout_eq("Now: hello\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["count"]);
    res.assert_success();
    res.assert_stdout_eq("1\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["is-empty"]);
    res.assert_failure();
    res.assert_stdout_eq("false\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["complete"]);
    res.assert_success();
    res.assert_stdout_eq("Completed: hello\nNow: NOTHING\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &[]);
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
}
