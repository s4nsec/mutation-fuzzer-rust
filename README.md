# Rust-based Mutation Fuzzer

## Introduction
This is a simple mutation-based fuzzer written in Rust. The purpose of this fuzzer is to test file parsing libraries, focusing on image files. It uses various techniques to mutate the content of files and then feed them to the targeted application, recording any bugs that are triggered.

## Features
- Byte Mutation: Mutates a single byte of the buffer randomly.
- Full Buffer Mutation: Completely randomizes the buffer.
- Controlled Byte Mutation: Mutates a single byte to either 0 or 255.
- Range Mutation: Mutates a contiguous range of bytes to either 0 or 255.
- Bug Tracking: Tracks the number and type of bugs encountered during fuzzing.

## Dependencies
- std::fs: File system operations
- std::io::prelude: I/O operations
- rand: Random number generation
- std::process::Command: To execute external commands
- regex::Regex: For regular expression matching
- std::collections::HashMap: For tracking bugs

# Quick Start
1. Clone the repository
2. Run the program

## How To Use
1. File Path: The file path for the image to be fuzzed is hardcoded for now. You'll need to update this path in the source code.
```Rust
let user_input = String::from("/home/m4st3rm1nd/Downloads/cross.jpg");
```
2. Command to Run: Similarly, the command that runs the target application is hardcoded. Replace this line to test a different target.
```Rust
let output = Command::new("/home/m4st3rm1nd/Downloads/jpeg2bmp")...
```
3. Running the Fuzzer: After making these changes, simply run the fuzzer.
```Bash
$ cargo run
```

## Example Output
Upon execution, the program will display bugs that have been triggered and their counts. For example,
```
[+] Bug: #1234, Count: 5
[+] Bug: #5678, Count: 2
```

## Future Work
- Parameterization of input file and command.
- Support for additional mutation techniques.
- Parallelization for speedup

## Contribution
Feel free to contribute to this project. Open issues, create pull requests or simply suggest new features.
