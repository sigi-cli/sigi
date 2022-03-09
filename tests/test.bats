#!/usr/bin/env bats

# Tested with BATS
# https://github.com/bats-core/bats-core/

@test "SETUP: 'cargo build' passes" {
  cargo build
}

sigi='target/debug/sigi'

@test "SETUP: '$sigi' is an executable" {
    [[ -x $sigi ]]
}

@test "SETUP: '$sigi --version' is 'sigi 3.*'" {
    result="$($sigi --version)"
    [[ $result == 'sigi 3.'* ]]
}

sigi_integ="$sigi --stack _integ"

@test "SETUP: '$sigi_integ delete-all' clears the integ stack" {
    result="$($sigi_integ delete-all)"
    [[ $result == Deleted:*items ]]
}

@test "[] 'sigi' says NOTHING" {
    result="$($sigi_integ)"
    [[ $result == "Now: NOTHING" ]]
}

@test "[] 'sigi peek' says NOTHING" {
    result="$($sigi_integ peek)"
    [[ $result == "Now: NOTHING" ]]
}

@test "[] 'sigi list' says NOTHINg" {
    result="$($sigi_integ list)"
    [[ $result == "Now: NOTHING" ]]
}

@test "[] 'sigi head' says NOTHING" {
    result="$($sigi_integ head)"
    [[ $result == "Now: NOTHING" ]]
}

@test "[] 'sigi tail' says NOTHING" {
    result="$($sigi_integ tail)"
    [[ $result == "Now: NOTHING" ]]
}

@test "[] 'sigi count' says 0" {
    result="$($sigi_integ count)"
    [[ $result == "0" ]]
}

@test "[] 'sigi is-empty' is true" {
    $sigi_integ is-empty
}

@test "[] 'sigi complete' is silent" {
    result="$($sigi_integ complete)"
    [[ $result == 'Now: NOTHING' ]]
}

@test "[] 'sigi delete' is silent" {
    result="$($sigi_integ delete)"
    [[ $result == 'Now: NOTHING' ]]
}

@test "[] 'sigi delete-all' says 0 items deleted" {
    result="$($sigi_integ delete-all)"
    [[ $result == 'Deleted: 0 items' ]]
}

@test "[hello] 'sigi push hello' creates an item" {
    result="$($sigi_integ push hello)"
    [[ $result == "Created: hello" ]]
}

@test "[hello] 'sigi' says hello" {
    result="$($sigi_integ)"
    [[ $result == "Now: hello" ]]
}

@test "[hello] 'sigi peek' says hello" {
    result="$($sigi_integ peek)"
    [[ $result == "Now: hello" ]]
}

@test "[hello] 'sigi list' says hello" {
    result="$($sigi_integ list)"
    [[ $result == "Now: hello" ]]
}

@test "[hello] 'sigi head' says hello" {
    result="$($sigi_integ head)"
    [[ $result == "Now: hello" ]]
}

@test "[hello] 'sigi tail' says hello" {
    result="$($sigi_integ tail)"
    [[ $result == "Now: hello" ]]
}

@test "[hello] 'sigi count' says 1" {
    result="$($sigi_integ count)"
    [[ $result == "1" ]]
}

@test "[hello] 'sigi is-empty' is false" {
    ! $sigi_integ is-empty
}

@test "[] 'sigi complete' completes hello" {
    result="$($sigi_integ complete)"
    [[ $result == 'Completed: hello'* ]]
    [[ $result == *'Now: NOTHING' ]]
}

@test "['a b c'] 'sigi push a b c' pushes one item" {
    $sigi_integ delete-all
    $sigi_integ push a b c
    result="$($sigi_integ peek)"
    [[ $result == 'Now: a b c' ]]
}

@test "[a] 'sigi push a'" {
    $sigi_integ delete-all
    $sigi_integ push a
    result="$($sigi_integ peek)"
    [[ $result == 'Now: a' ]]
    result="$($sigi_integ list)"
    [[ $result == 'Now: a' ]]
}

@test "[a, b] 'sigi push b'" {
    $sigi_integ push b
    result="$($sigi_integ peek)"
    [[ $result == 'Now: b' ]]
    result="$($sigi_integ list)"
    [[ $result == 'Now: b'*'1: a' ]]
}

@test "[a, b, c] 'sigi push c'" {
    $sigi_integ push c
    result="$($sigi_integ peek)"
    [[ $result == 'Now: c' ]]
    result="$($sigi_integ list)"
    [[ $result == 'Now: c'*'1: b'*'2: a' ]]
}

@test "[a, b, c] 'sigi push d'" {
    $sigi_integ push d
    result="$($sigi_integ peek)"
    [[ $result == 'Now: d' ]]
    result="$($sigi_integ list)"
    [[ $result == 'Now: d'*'1: c'*'2: b'*'3: a' ]]
}

@test "[a, b, d, c] 'sigi swap'" {
    $sigi_integ swap
    result="$($sigi_integ list)"
    [[ $result == 'Now: c'*'1: d'*'2: b'*'3: a' ]]
}

@test "[a, b, c, d] 'sigi swap'" {
    $sigi_integ swap
    result="$($sigi_integ list)"
    [[ $result == 'Now: d'*'1: c'*'2: b'*'3: a' ]]
}

@test "[a, d, b, c] 'sigi rot'" {
    $sigi_integ rot
    result="$($sigi_integ list)"
    [[ $result == 'Now: c'*'1: b'*'2: d'*'3: a' ]]
}

@test "[a, c, d, b] 'sigi rot'" {
    $sigi_integ rot
    result="$($sigi_integ list)"
    [[ $result == 'Now: b'*'1: d'*'2: c'*'3: a' ]]
}

@test "[a, b, c, d] 'sigi rot'" {
    $sigi_integ rot
    result="$($sigi_integ list)"
    [[ $result == 'Now: d'*'1: c'*'2: b'*'3: a' ]]
}

@test "[d, a, b, c] 'sigi next'" {
    $sigi_integ next
    result="$($sigi_integ list)"
    [[ $result == 'Now: c'*'1: b'*'2: a'*'3: d' ]]
}

@test "[c, d, a, b] 'sigi next'" {
    $sigi_integ next
    result="$($sigi_integ list)"
    [[ $result == 'Now: b'*'1: a'*'2: d'*'3: c' ]]
}

@test "[b, c, d, a] 'sigi next'" {
    $sigi_integ next
    result="$($sigi_integ list)"
    [[ $result == 'Now: a'*'1: d'*'2: c'*'3: b' ]]
}

@test "[a, b, c, d] 'sigi next'" {
    $sigi_integ next
    result="$($sigi_integ list)"
    [[ $result == 'Now: d'*'1: c'*'2: b'*'3: a' ]]
}

@test "[a, b, c] 'sigi delete'" {
    result="$($sigi_integ delete)"
    [[ $result == 'Deleted: d'*'Now: c' ]]
}

@test "[a, b] 'sigi complete'" {
    result="$($sigi_integ complete)"
    [[ $result == 'Completed: c'*'Now: b' ]]
}

@test "[] 'sigi delete-all'" {
    result="$($sigi_integ delete-all)"
    [[ $result == 'Deleted: 2 items' ]]
}

# TODO: More with multi-element stacks (lifecycle/views/shuffle)
# TODO: JSON tests
# TODO: CSV/TSV tests
