# rep

A simple grep implementation in Rust.

## Overview

rep is a command-line tool that searches for patterns in text files, similar to the Unix grep utility.

## Features

- Pattern matching in text files
- Support for multiple file inputs
- Line number display
- Match counting
- Case-sensitive/insensitive search options

## Usage

```bash
rep [FLAGS] PATTERN [FILES...]
```

### Examples

Search for a pattern in a file:
```bash
rep "hello" file.txt
```

Search in multiple files:
```bash
rep "pattern" file1.txt file2.txt
```

## Building

Build the project using Cargo:

```bash
cargo build
```

Run in development mode:
```bash
cargo run -- "pattern" file.txt
```

## License

This project is open source. 