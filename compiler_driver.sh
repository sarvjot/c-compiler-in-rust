#!/bin/bash

delete_if_exists() {
    local file="$1"
    if [ -f "$file" ]; then
        rm "$file"
    fi
}

INPUT_FILE=$1
PREPROCESSED_FILE="${INPUT_FILE%.c}.i"
ASSEMBLY_FILE="${INPUT_FILE%.c}.s"
EXECUTABLE_FILE="${INPUT_FILE%.c}"

# E: only preprocess the source code
# P: do not include line markers in the output
gcc -E -P $INPUT_FILE -o $PREPROCESSED_FILE

# TODO: Replace real compiler with custom compiler
# S: only generate assembly code
gcc -S $PREPROCESSED_FILE -o $ASSEMBLY_FILE
compile_status=$?
delete_if_exists $PREPROCESSED_FILE
if [ $compile_status -ne 0 ]; then
    exit $compile_status
fi

gcc $ASSEMBLY_FILE -o $EXECUTABLE_FILE
delete_if_exists $ASSEMBLY_FILE