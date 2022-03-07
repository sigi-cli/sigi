#!/usr/bin/env bats

# Tested with BATS
# https://github.com/bats-core/bats-core/

@test "sigi is compiling" {
  cargo build
}

sigi='target/debug/sigi'

@test "$sigi is an executable" {
    [[ -x $sigi ]]
}

@test "$sigi --version" {
    result="$($sigi --version)"
    [[ $result =~ "sigi 3" ]]
}

sigi_integ="$sigi --stack _integ"

@test "$sigi_integ delete-all" {
    result="$($sigi_integ delete-all)"
    [[ $result == Deleted:*items ]]
}

@test "[] $sigi_integ delete-all # should now be zero" {
    result="$($sigi_integ delete-all)"
    [[ $result == 'Deleted: 0 items' ]]
}

@test "[] $sigi_integ" {
    result="$($sigi_integ)"
    [[ $result == "Now: NOTHING" ]]
}

@test "[] $sigi_integ peek" {
    result="$($sigi_integ peek)"
    [[ $result == "Now: NOTHING" ]]
}

@test "[] $sigi_integ list" {
    result="$($sigi_integ list)"
    [[ $result == "Now: NOTHING" ]]
}

@test "[] $sigi_integ head" {
    result="$($sigi_integ head)"
    [[ $result == "Now: NOTHING" ]]
}

@test "[] $sigi_integ tail" {
    result="$($sigi_integ tail)"
    [[ $result == "Now: NOTHING" ]]
}

@test "[] $sigi_integ count" {
    result="$($sigi_integ count)"
    [[ $result == "0" ]]
}

@test "[] $sigi_integ is-empty" {
    $sigi_integ is-empty
}

@test "[hello] $sigi_integ push hello" {
    result="$($sigi_integ push hello)"
    [[ $result == "Created: hello" ]]
}

@test "[hello] $sigi_integ" {
    result="$($sigi_integ)"
    [[ $result == "Now: hello" ]]
}

@test "[hello] $sigi_integ peek" {
    result="$($sigi_integ peek)"
    [[ $result == "Now: hello" ]]
}

@test "[hello] $sigi_integ list" {
    result="$($sigi_integ list)"
    [[ $result == "Now: hello" ]]
}

@test "[hello] $sigi_integ head" {
    result="$($sigi_integ head)"
    [[ $result == "Now: hello" ]]
}

@test "[hello] $sigi_integ tail" {
    result="$($sigi_integ tail)"
    [[ $result == "Now: hello" ]]
}

@test "[hello] $sigi_integ count" {
    result="$($sigi_integ count)"
    [[ $result == "1" ]]
}

@test "[hello] $sigi_integ is-empty" {
    ! $sigi_integ is-empty
}

@test "[] $sigi_integ complete" {
    result="$($sigi_integ complete)"
    [[ $result == 'Completed: hello'* ]]
    [[ $result == *'Now: NOTHING' ]]
}

# TODO: Multi-element stack (lifecycle/views/shuffle)
# TODO: JSON tests
# TODO: CSV/TSV tests
