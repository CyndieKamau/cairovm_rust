# SIMPLE CAIRO VM WRITTEN IN RUST;

This repo is an implementation of a simple cairo vm for compiling Cairo language, written in rust.


# Introduction

Cairo is a unique language tailored for zk-STARKs and has applications in creating scalable and transparent computational integrity proofs. 

This project aims to provide a Rust-based environment for compiling and running Cairo programs.

# Features of the VM

The VM Consists of a ;

* ### Lexer : Tokenize Cairo Code

* ### Parser: Analyzes the tokenized output from the lexer and builds an Abstract Syntax Tree (AST).

* ### Type-Checker: Validates the types used in the Cairo programs to ensure correctness.

* ### Translator: Converts the AST into bytecode or machine code suitable for the VM.

* ### Runtime environment: Executes the translated code.


N.B extra features and complexity will be added to mimic starknet and cairo architcture properly.

The lexer is currently under construction. It's a manual lexer, for more efficiency the `logos` Rust crate will be used to build a more efficient and stable lexer.

# Clone the Repo

```sh
git clone https://github.com/CyndieKamau/cairovm_rust

cd cairovm_rust

cargo build

```

Here's an overview of how the current lexer is functioning;

```sh
hp@Cyndie:~/Desktop/cairovm/lexer/src$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `/home/hp/Desktop/cairovm/lexer/target/debug/lexer`
Please enter a simple cairo code here: 
let mut x = 5_u32
[Let, Mut, Identifier("x"), Assign, Number("5", None), Identifier("u32")]
```

# Deployment of Actix Web Server using DigitalOcean

## STEP 1: Create a Digital Ocean account

## STEP 2: 







