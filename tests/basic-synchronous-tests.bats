# Tested with BATS
# https://github.com/bats-core/bats-core/

setup() {
    load 'test_helper/bats-support/load'
    load 'test_helper/bats-assert/load'
}

@test "SETUP: 'cargo build' passes" {
    run cargo build
    assert_success
}

sigi='target/debug/sigi'

@test "SETUP: '$sigi' is an executable" {
    [[ -x $sigi ]]
}

@test "SETUP: '$sigi --version' is 'sigi 3.*'" {
    run $sigi --version
    assert_success
    assert_output --regexp 'sigi 3\..+'
}

sigi_integ="$sigi --stack __basic_synchronous_tests"

@test "SETUP: '$sigi_integ delete-all' clears the integ stack" {
    run $sigi_integ delete-all
    assert_success
    assert_output --regexp 'Deleted: [0-9]+ items'
}

@test "[] 'sigi' says NOTHING" {
    run $sigi_integ
    assert_output 'Now: NOTHING'
}

@test "[] 'sigi peek' says NOTHING" {
    run $sigi_integ peek
    assert_success
    assert_output 'Now: NOTHING'
}

@test "[] 'sigi list' says NOTHINg" {
    run $sigi_integ list
    assert_success
    assert_output 'Now: NOTHING'
}

@test "[] 'sigi head' says NOTHING" {
    run $sigi_integ head
    assert_success
    assert_output 'Now: NOTHING'
}

@test "[] 'sigi tail' says NOTHING" {
    run $sigi_integ tail
    assert_success
    assert_output 'Now: NOTHING'
}

@test "[] 'sigi count' says 0" {
    run $sigi_integ count
    assert_success
    assert_output '0'
}

@test "[] 'sigi is-empty' is true" {
    run $sigi_integ is-empty
    assert_success
    assert_output 'true'
}

@test "[] 'sigi complete' is silent" {
    run $sigi_integ complete
    assert_success
    assert_output 'Now: NOTHING'
}

@test "[] 'sigi delete' is silent" {
    run $sigi_integ delete
    assert_success
    assert_output 'Now: NOTHING'
}

@test "[] 'sigi delete-all' says 0 items deleted" {
    run $sigi_integ delete-all
    assert_success
    assert_output 'Deleted: 0 items'
}

@test "[hello] 'sigi push hello' creates an item" {
    run $sigi_integ push hello
    assert_success
    assert_output 'Created: hello'
}

@test "[hello] 'sigi' says hello" {
    run $sigi_integ
    assert_success
    assert_output 'Now: hello'
}

@test "[hello] 'sigi peek' says hello" {
    run $sigi_integ peek
    assert_success
    assert_output 'Now: hello'
}

@test "[hello] 'sigi list' says hello" {
    run $sigi_integ list
    assert_success
    assert_output 'Now: hello'
}

@test "[hello] 'sigi head' says hello" {
    run $sigi_integ head
    assert_success
    assert_output 'Now: hello'
}

@test "[hello] 'sigi tail' says hello" {
    run $sigi_integ tail
    assert_success
    assert_output 'Now: hello'
}

@test "[hello] 'sigi count' says 1" {
    run $sigi_integ count
    assert_success
    assert_output '1'
}

@test "[hello] 'sigi is-empty' is false" {
    run $sigi_integ is-empty
    assert_failure
    assert_output 'false'
}

@test "[] 'sigi complete' completes hello" {
    run $sigi_integ complete
    assert_success
    assert_line 'Completed: hello'
    assert_line 'Now: NOTHING'
}

@test "['a b c'] 'sigi push a b c' pushes one item" {
    $sigi_integ push a b c

    run $sigi_integ peek
    assert_success
    assert_output 'Now: a b c'

    $sigi_integ delete
}

@test "[a] 'sigi push a'" {
    $sigi_integ push a

    run $sigi_integ peek
    assert_success
    assert_output 'Now: a'

    run $sigi_integ list
    assert_success
    assert_line 'Now: a'
}

@test "[a, b] 'sigi push b'" {
    $sigi_integ push b

    run $sigi_integ peek
    assert_success
    assert_output 'Now: b'

    run $sigi_integ list
    assert_success
    assert_line 'Now: b'
    assert_line '  1: a'
}

@test "[a, b, c] 'sigi push c'" {
    $sigi_integ push c

    run $sigi_integ peek
    assert_success
    assert_output 'Now: c'

    run $sigi_integ list
    assert_success
    assert_line 'Now: c'
    assert_line '  1: b'
    assert_line '  2: a'
}

@test "[a, b, c] 'sigi push d'" {
    $sigi_integ push d

    run $sigi_integ peek
    assert_success
    assert_output 'Now: d'

    run $sigi_integ list
    assert_success
    assert_line 'Now: d'
    assert_line '  1: c'
    assert_line '  2: b'
    assert_line '  3: a'
}

@test "[a, b, d, c] 'sigi swap'" {
    $sigi_integ swap

    run $sigi_integ list
    assert_success
    assert_line 'Now: c'
    assert_line '  1: d'
    assert_line '  2: b'
    assert_line '  3: a'
}

@test "[a, b, c, d] 'sigi swap'" {
    $sigi_integ swap

    run $sigi_integ list
    assert_success
    assert_line 'Now: d'
    assert_line '  1: c'
    assert_line '  2: b'
    assert_line '  3: a'
}

@test "[a, d, b, c] 'sigi rot'" {
    $sigi_integ rot

    run $sigi_integ list
    assert_success
    assert_line 'Now: c'
    assert_line '  1: b'
    assert_line '  2: d'
    assert_line '  3: a'
}

@test "[a, c, d, b] 'sigi rot'" {
    $sigi_integ rot

    run $sigi_integ list
    assert_success
    assert_line 'Now: b'
    assert_line '  1: d'
    assert_line '  2: c'
    assert_line '  3: a'
}

@test "[a, b, c, d] 'sigi rot'" {
    $sigi_integ rot

    run $sigi_integ list
    assert_success
    assert_line 'Now: d'
    assert_line '  1: c'
    assert_line '  2: b'
    assert_line '  3: a'
}

@test "[d, a, b, c] 'sigi next'" {
    $sigi_integ next

    run $sigi_integ list
    assert_success
    assert_line 'Now: c'
    assert_line '  1: b'
    assert_line '  2: a'
    assert_line '  3: d'
}

@test "[c, d, a, b] 'sigi next'" {
    $sigi_integ next

    run $sigi_integ list
    assert_success
    assert_line 'Now: b'
    assert_line '  1: a'
    assert_line '  2: d'
    assert_line '  3: c'
}

@test "[b, c, d, a] 'sigi next'" {
    $sigi_integ next

    run $sigi_integ list
    assert_success
    assert_line 'Now: a'
    assert_line '  1: d'
    assert_line '  2: c'
    assert_line '  3: b'
}

@test "[a, b, c, d] 'sigi next'" {
    $sigi_integ next

    run $sigi_integ list
    assert_success
    assert_line 'Now: d'
    assert_line '  1: c'
    assert_line '  2: b'
    assert_line '  3: a'
}

@test "[a, b, c] 'sigi delete'" {
    run $sigi_integ delete
    assert_success
    assert_line 'Deleted: d'
    assert_line 'Now: c'
}

@test "[a, b] 'sigi complete'" {
    run $sigi_integ complete
    assert_success
    assert_line 'Completed: c'
    assert_line 'Now: b'
}

@test "[] 'sigi delete-all'" {
    run $sigi_integ delete-all
    assert_success
    assert_output 'Deleted: 2 items'
}
