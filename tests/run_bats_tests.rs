use std::process::Command;

const BATS: &str = "./tests/bats/bin/bats";
const SKIP_BATS_TESTS: Option<&str> = std::option_env!("SKIP_BATS_TESTS");

#[test]
fn basic_syncronous_tests() {
    run_bats_test_file("./tests/basic-synchronous-tests.bats");
}

// TODO: More tests on multi-element stacks (parallelizable lifecycle/views/shuffle tests)

// TODO: Format tests for JSON, CSV, TSV

fn run_bats_test_file(test_file_path: &str) {
    if let Some("1") = SKIP_BATS_TESTS {
        println!("SKIPPING TESTS.");
        return;
    }

    let exit_code = Command::new(BATS)
        .arg(test_file_path)
        .spawn()
        .expect(&format!(
            "\n{}\n{}\n",
            "Unable to create child process for BATS tests.",
            "Do you need to run 'git submodule update --init'?"
        ))
        .wait()
        .expect("BATS tests never started.")
        .code()
        .expect("BATS tests terminated by signal.");

    assert_eq!(exit_code, 0, "BATS tests did not exit successfully.")
}
