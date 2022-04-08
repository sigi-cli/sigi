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
fn sigi_empty_stack_ops() {
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
    res.assert_stdout_line_eq(stack);
    res.assert_stderr_empty();
}

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

#[test]
fn sigi_abc_ops() {
    let stack = "_integ::abc";

    let res = sigi(stack, &["delete-all"]);
    res.assert_success();

    // ['a b c']
    let res = sigi(stack, &["push", "a", "b", "c"]);
    res.assert_success();
    res.assert_stdout_eq("Created: a b c\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["count"]);
    res.assert_success();
    res.assert_stdout_eq("1\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["delete"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Deleted: a b c",
        "Now: NOTHING"
    ]);
    res.assert_stderr_empty();

    // ['a']
    let res = sigi(stack, &["push", "a"]);
    res.assert_success();
    res.assert_stdout_eq("Created: a\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["peek"]);
    res.assert_success();
    res.assert_stdout_eq("Now: a\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["list"]);
    res.assert_success();
    res.assert_stdout_eq("Now: a\n");
    res.assert_stderr_empty();

    // ['a', 'b']
    let res = sigi(stack, &["push", "b"]);
    res.assert_success();
    res.assert_stdout_eq("Created: b\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["peek"]);
    res.assert_success();
    res.assert_stdout_eq("Now: b\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["list"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Now: b",
        "  1: a"
    ]);
    res.assert_stderr_empty();

    // ['a', 'b', 'c']
    let res = sigi(stack, &["push", "c"]);
    res.assert_success();
    res.assert_stdout_eq("Created: c\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["peek"]);
    res.assert_success();
    res.assert_stdout_eq("Now: c\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["list"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Now: c",
        "  1: b",
        "  2: a"
    ]);
    res.assert_stderr_empty();

    // ['a', 'b', 'c', 'd']
    let res = sigi(stack, &["push", "d"]);
    res.assert_success();
    res.assert_stdout_eq("Created: d\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["peek"]);
    res.assert_success();
    res.assert_stdout_eq("Now: d\n");
    res.assert_stderr_empty();

    let res = sigi(stack, &["list"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Now: d",
        "  1: c",
        "  2: b",
        "  3: a"
    ]);
    res.assert_stderr_empty();

    // swap
    let res = sigi(stack, &["swap"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Now: c",
        "  1: d",
        "  2: b",
        "  3: a"
    ]);
    res.assert_stderr_empty();
    
    let res = sigi(stack, &["swap"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Now: d",
        "  1: c",
        "  2: b",
        "  3: a"
    ]);
    res.assert_stderr_empty();

    // rot
    let res = sigi(stack, &["rot"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Now: c",
        "  1: b",
        "  2: d",
        "  3: a"
    ]);
    res.assert_stderr_empty();
    
    let res = sigi(stack, &["rot"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Now: b",
        "  1: d",
        "  2: c",
        "  3: a"
    ]);
    res.assert_stderr_empty();
    
    let res = sigi(stack, &["rot"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Now: d",
        "  1: c",
        "  2: b",
        "  3: a"
    ]);
    res.assert_stderr_empty();

    // next
    let res = sigi(stack, &["next"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Now: c",
        "  1: b",
        "  2: a",
        "  3: d"
    ]);
    res.assert_stderr_empty();
    
    let res = sigi(stack, &["next"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Now: b",
        "  1: a",
        "  2: d",
        "  3: c"
    ]);
    res.assert_stderr_empty();
    
    let res = sigi(stack, &["next"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Now: a",
        "  1: d",
        "  2: c",
        "  3: b"
    ]);
    res.assert_stderr_empty();
    
    let res = sigi(stack, &["next"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Now: d",
        "  1: c",
        "  2: b",
        "  3: a"
    ]);
    res.assert_stderr_empty();
    
    // removal tests
    let res = sigi(stack, &["delete"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Deleted: d",
        "Now: c"
    ]);
    res.assert_stderr_empty();

    let res = sigi(stack, &["complete"]);
    res.assert_success();
    res.assert_stdout_lines_eq(&[
        "Completed: c",
        "Now: b"
    ]);
    res.assert_stderr_empty();

    let res = sigi(stack, &["delete-all"]);
    res.assert_success();
    res.assert_stdout_eq("Deleted: 2 items\n");
    res.assert_stderr_empty();
}
