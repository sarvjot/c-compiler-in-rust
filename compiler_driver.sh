#!/bin/bash

INPUT_FILE=""
RUN_LEXER=false

# Parse arguments
for arg in "$@"; do
    case $arg in
        --lex)
            RUN_LEXER=true
            ;;
        *)
            INPUT_FILE="$arg"
            ;;
    esac
done

if [ -z "$INPUT_FILE" ]; then
    echo "Usage: $0 [--lex] <input_file.c>"
    exit 1
fi

PREPROCESSED_FILE="${INPUT_FILE%.c}.i"
ASSEMBLY_FILE="${INPUT_FILE%.c}.s"
EXECUTABLE_FILE="${INPUT_FILE%.c}"
LEXER_DIR="lexer"

delete_if_exists() {
    local file="$1"
    if [ -f "$file" ]; then
        rm "$file"
    fi
}

build_lexer_if_not_exists() {
    if [ ! -d "lexer" ]; then
        echo "Lexer directory not found. Please build the lexer first."
        exit 1
    fi

    if [ ! -f "$LEXER_DIR/target/release/lexer" ]; then
        echo "Lexer binary not found. Building the lexer..."
        cd lexer
        cargo build --release
        if [ $? -ne 0 ]; then
            echo "Failed to build the lexer."
            exit 1
        fi
        cd ..
    fi
}   

# Preprocess the source code
gcc -E -P $INPUT_FILE -o $PREPROCESSED_FILE

if [ $RUN_LEXER = true ]; then
    build_lexer_if_not_exists
    $LEXER_DIR/target/release/lexer $PREPROCESSED_FILE
    lexing_status=$?
    delete_if_exists $PREPROCESSED_FILE
    if [ $lexing_status -ne 0 ]; then
        exit $lexing_status
    fi
else
    delete_if_exists $PREPROCESSED_FILE
fi