#!/usr/bin/env bash

#!/bin/bash
args=''
for arg in "$@"; do 
    arg="${arg//\\/\\\\}"
    arg=$(echo "$arg" | sed 's/^-e/--e/')
    args="$args \"${arg//\"/\\\"}\""
done
echo "$args"
exec ./tp1/target/debug/tp1 "$args" # TODO: Change this path to be next to this shell script (./tp1)

args=$(printf " %q" "$@" | sed -r 's/(^|[^A-Za-z0-9_.-])-e/\1--e/g' | sed 's/\\/\\\\/g') # Replace -e1 and -e2 arguments with their long form --e1 and --e2
echo "$args"
echo ${args@Q}
# printf " %q" "$args"
exec ./tp1/target/debug/tp1 "$args" # TODO: Change this path to be next to this shell script (./tp1)
