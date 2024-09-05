# pngme_core
pngme_core is the core library of the PNGme project. It provides the main functionality for encoding and decoding secret messages within PNG files using Rust.

## Features
- Encode hidden messages using least significant bit (LSB) technique.
- Decode hidden messages from PNG files.
- Efficient Rust-based implementation.

## Installation
To add pngme_core to your project, add the following to your Cargo.toml:
```toml
[dependencies]
pngme_core = "0.1.0"
```

## Usage
```rust
use pngme_core::PngMe;

let message = "Secret message";
let input_file = "input.png";
let output_file = "output.png";

// Encode a message
PngMe::encode(input_file, output_file, message).unwrap();

// Decode a message
let decoded_message = PngMe::decode(output_file).unwrap();
println!("{}", decoded_message);
```