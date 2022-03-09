use std::process::Command;

#[test]
fn bats_tests_all_pass() {
    let exit_code = Command::new("bats")
        .arg("./tests/test.bats")
        .spawn()
        .expect("Was unable to create child process for BATS tests.")
        .wait()
        .expect("BATS tests never started.")
        .code()
        .expect("BATS tests terminated by signal.");

    assert_eq!(exit_code, 0, "BATS tests did not exit successfully.");
}
