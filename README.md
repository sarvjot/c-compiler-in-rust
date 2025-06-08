# Introduction

This project is aimed at creating a personal compiler, implementing a subset of features of C programming language. It follows Nora Sandler's book, [Writing a C Compiler](https://nostarch.com/writing-c-compiler).

# Setup

Rust is used as the implementation language, as it has good support for sum types and pattern-matching ([resource](https://chadaustin.me/2015/07/sum-types/)).

The book uses Intel architecture, however M4 uses arm based architecture by default. Make your desired shell run in intel architecture: `arch -x86_64 zsh`.