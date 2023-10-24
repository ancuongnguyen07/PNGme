# PNGme

PNGme is a Rust application for encoding and decoding secret messages within PNG image files.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Have you ever wanted to hide secret messages within PNG image files? PNGme allows you to encode and decode messages using the least significant bit (LSB) technique.
This lightweight command-line tool provides a fun and creative way to share hidden messages with friends and colleagues.

## Features

- Encode a message into a PNG image.
- Decode a hidden message from a PNG image.
- Command-line interface for easy usage.
- Rust-based application for efficiency and speed.

## Installation

1. Clone the repository:

```bash
git clone https://github.com/ancuongnguyen07/PNGme
```

2. Change to the project directory:
```bash
cd PNGme
```

3. Build the project using Cargo
```bash
cargo build --release
```

4. You can find the compiled binary in the `target/release` directory

## Usage
- To encode a message into a PNG image and save the result:
```shell
pngme encode -i <input.png> -o <output.png> -m "Your secret message" -c <chunk_type>
```
- To decode a hidden message from a PNG image and print the message if one is found:
```bash
pngme decode -i <input.png> -c <chunk_type>
```

- To remove a chunk from a PNG file and save the result:
```bash
pngme remove -i <input.png> -c <chunk_type>
```

- To print all of the chunks in a PNG file:
```bash
pngme print -i <input.png>
```

**TIPS**: you can show the help text in each subcommand or in the app generally by switching on
the flag `-h`.

## Contributing
Contributions to PNGme are welcome! If you'd like to contribute the project, please follow
these steps:
1. Fork th repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and test thoroughly.
4. Submit a pull request (PR).

## License
This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.