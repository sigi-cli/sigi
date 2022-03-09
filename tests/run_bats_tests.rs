use std::process::Command;

#[test]
fn bats_tests_pass() {
    let bats = "./tests/bats/bin/bats";
    let bats_test_file = "./tests/basic-synchronous-tests.bats";

    let exit_code = Command::new(bats)
        .arg(bats_test_file)
        .spawn()
        .expect("Was unable to create child process for BATS tests.")
        .wait()
        .expect("BATS tests never started.")
        .code()
        .expect("BATS tests terminated by signal.");

    assert_eq!(exit_code, 0, "BATS tests did not exit successfully.");
}
