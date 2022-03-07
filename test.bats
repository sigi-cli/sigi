#!/usr/bin/env bats

# Tested with BATS
# https://github.com/bats-core/bats-core/

@test "sigi is compiling" {
  cargo build
}

sigi=target/debug/sigi

@test "$sigi is an executable" {
    [[ -x $sigi ]]
}

@test "$sigi --version" {
    result="$($sigi --version)"
    [[ $result =~ "sigi 3" ]]
}

sigi_integ="$sigi --stack '_integ'"

@test "$sigi_integ delete-all" {
    result="$($sigi_integ delete-all)"
    [[ $result == Deleted:*items ]]
}

@test "$sigi_integ delete-all # should now be zero" {
    result="$($sigi_integ delete-all)"
    [[ $result == 'Deleted: 0 items' ]]
}

@test "$sigi_integ" {
    result="$($sigi_integ)"
    [[ $result == "Now: NOTHING" ]]
}

@test "$sigi_integ peek" {
    result="$($sigi_integ peek)"
    [[ $result == "Now: NOTHING" ]]
}

@test "$sigi_integ list" {
    result="$($sigi_integ list)"
    [[ $result == "" ]]
}
