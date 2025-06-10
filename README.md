# my-wc

A Rust implementation of the Unix `wc` (word count) command line utility.

## Description

This program counts the number of lines, words, bytes, and characters in text files. It provides similar functionality to the standard Unix `wc` command with a modern Rust implementation.

## Features

- Count lines (`-l`)
- Count bytes (`-c`)
- Count characters (`-m`)
- Count words (`-w`)
- Find the length of the longest line (`-L`)
- Process multiple files
- Simple and efficient implementation

## Installation

1. Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs)
2. Clone this repository
3. Build the project:
```bash
cargo build --release
```

## Usage

Basic usage:
```bash
cargo run -- [options] [file]
```

### Options

- `-l`: Count lines
- `-c`: Count bytes
- `-m`: Count characters
- `-w`: Count words
- `-L`: Count characters in the longest line
- `-h, --help`: Show help message
- `-v, --version`: Show version information

### Examples

Count lines in a file:
```bash
cargo run -- -l file.txt
```

Count words in a file:
```bash
cargo run -- -w file.txt
```

Get all statistics for a file:
```bash
cargo run -- file.txt
```

## Building

To build the project:
```bash
cargo build
```

For release build:
```bash
cargo build --release
```

## License

[Add your license information here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
