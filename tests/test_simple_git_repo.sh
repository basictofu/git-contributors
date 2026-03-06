#!/usr/bin/env bash

# create git repo

which jq >/dev/null # ensure we have jq
if [[ ! $? == "0" ]]; then
    echo "Tests require dependency: jq"
    exit 1;
fi

green() {
	local text=$1
	printf "\x1b[1;92m$text\x1b[0m"
}

red() {
	local text=$1
	printf "\x1b[1;91m$text\x1b[0m"
}

run_test() {
	local cmd="$@"
	eval $cmd
	if [ $? -eq 0 ]; then
		echo "$(green [pass])" $cmd
	else
		echo "$(red [fail])" $cmd
	fi
}

make_simple_repo_if_needed() {
    local repo_path="tests/data/simple_repo"

    if [[ -d "$repo_path" ]]; then
        return
    fi

    mkdir -p "$repo_path"

    cd "$repo_path"

    git init
    touch a
    git add a
    git commit -m "first" --date="2023-10-27 14:30:00Z" --author="Bob <none>"
    touch b
    git add b
    git commit -m "second" --date="2023-10-27 15:30:00Z" --author="Bob <none>"

    cd - >/dev/null
}


test_bins_are_correct() {
    make_simple_repo_if_needed

    # run command
    cd tests/data/simple_repo
    output="$(../../../target/debug/git-contributors --start="2023-10-27 14:00:00Z" --end="2023-10-27 16:00:00Z" --numbins=2 --json)"
    cd - >/dev/null

    # check output json
    bins="$(echo "$output" | jq -c '.authors.[0].bins')"
    if [[ "$bins" = "[1,1]" ]]; then
        return 0
    else
        return 1
    fi
}

test_getting_repo_name() {
    # build dummy repo
    make_simple_repo_if_needed

    # run command
    cd tests/data/simple_repo
    output="$(../../../target/debug/git-contributors --numbins=2 --json)"
    cd - >/dev/null
 
    # check output json
    # -r => raw string output (no quotes)
    name="$(echo "$output" | jq -c -r '.repo_name')"
    if [[ "$name" = "simple_repo" ]]; then
        return 0
    else
        echo "Expected name 'simple_repo' but received:" $name
        return 1
    fi
}

main () {
    cargo build

    run_test test_bins_are_correct
    run_test test_getting_repo_name
}

main "$@"
