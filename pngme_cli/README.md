# pngme_cli
pngme_cli is a command-line interface for the PNGme project, allowing users to encode, decode, remove, and print chunks of hidden data in PNG files.

## Features

- Encode a message into a PNG image.
- Decode a hidden message from a PNG image.
- Command-line interface for easy usage.
- Rust-based application for efficiency and speed.

## Installation
Build the CLI tool by running:
```bash
cargo install --path .
```

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