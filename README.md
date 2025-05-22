# PngMe

A command-line utility for hiding secret messages in PNG files using chunk
metadata. Supports encoding, decoding, removing messages, and inspecting PNG
chunks.

# Overview

PngMe is a Rust-based command-line tool that allows users to securely hide
messages within PNG image files by leveraging the chunk-based architecture of
the PNG format. It provides functionalities to encode new messages, decode
existing ones, remove previously hidden messages, and inspect the chunk
structure of PNG files. The tool is designed to maintain the integrity and
validity of the PNG file throughout these operations.

**Key Features**:

-   **Encode**: Embeds a secret message into a specified ancillary chunk of a
    PNG file. If the chunk type doesn't exist, it can be created.
-   **Decode**: Extracts and displays a hidden message from a specified chunk
    type within a PNG file.
-   **Remove**: Deletes a message (and optionally the chunk itself if it
    becomes empty or is no longer needed) from a PNG file.
-   **Print Chunks**: Lists all discoverable chunks within a PNG file, showing
    their type, size, and offset.
-   **Validation**: Performs CRC (Cyclic Redundancy Check) validation on chunks
    and validates chunk type naming conventions according to the PNG
    specification.
-   **PNG Structure Preservation**: Ensures that the critical chunk order and
    overall PNG file structure remain valid after any modification.

# Getting Started

## Prerequisites

-   Rust programming language
-   Cargo

## Installation & Setup

1.  **Clone the repository**

    ```bash
    git clone https://github.com/matee8/pngme.git
    cd pngme
    ```

2.  **Build and Install from Source**

    You can compile and install the `pngme` executable to your Cargo binary
    path (`~/.cargo/bin/` by default) using:

    ```bash
    cargo install --path .
    ```

    Once installed, you should be able to run `pngme` directly from your
    terminal.

    Alternatively, you can build the project and run the executable from the
    `target/release/` or `target/debug/` directory:

    ```bash
    cargo build --release
    ./target/release/pngme --help
    ```

## Usage

PngMe is operated via subcommands.

## Command-Line Interface Overview

```
pngme 0.1.0

USAGE:
    pngme <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decode    Show the secret message in a .png file
    encode    Hide a secret message in a .png file
    help      Prints this message or the help of the given subcommand(s)
    print     Print the whole .png file
    remove    Remove a secret message from a .png file
```

## Basic Command Examples

1.  **Encode a message**:

    Hide the string "Secret Message" in a chunk of type `TeSt` within
    `input.png`, saving the result to `output.png`.

    ```bash
    pngme encode ./path/to/input.png TeSt "Secret Message" ./path/to/output.png
    ```

2.  **Decode a message**:

    Extract and display the message hidden in the `TeSt` chunk of
    `image_with_secret.png`.

    ```bash
    pngme decode ./path/to/image_with_secret.png TeSt
    ```

3.  **Remove a message**:

    Delete the message (and typically the chunk itself if it's custom) from the
    `TeSt` chunk in `image_with_secret.png`. The output is usually written back
    to the input file.

    ```bash
    pngme remove ./path/to/image_with_secret.png TeSt
    ```

4.  **Print all chunks**:

    List all chunks found in `my_image.png`, showing their types, sizes, and
    potentially other metadata.

    ```bash
    pngme print ./path/to/my_image.png
    ```

## License

This project is licensed under the [MIT License](LICENSE).
