#!/usr/bin/env bash

# Replace -e1 and -e2 arguments with their long form --e1 and --e2
args=()
for arg in "$@"; do
    [[ "$arg" == '-e1' ]] && arg='--e1'
    [[ "$arg" == '-e2' ]] && arg='--e2'
    args+=("${arg}")
done

exec ./implementation/target/release/tp1 "${args[@]}" # TODO: Change this path to be next to this shell script (./tp1)
