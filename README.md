# pngme

A command-line utility for hiding secret messages in PNG files using chunk
metadata. Supports encoding, decoding, removing messages, and inspecting PNG
chunks.

## Features

- **Encode**: Hide secret messages in PNG files
- **Decode**: Extract hidden messages from PNG files
- **Remove**: Delete hidden messages from PNG files
- **Print**: List all searchable chunks in a PNG file
- **Validation**: CRC checks and chunk type validation
- **PNG Compliance**: Maintains valid PNG structure during operations

## Installation

### Prerequisites
- Rust (1.50+)
- Cargo

### Build from Source
```bash
git clone https://github.com/matee8/pngme.git
cd pngme
cargo install --path .
```

## Usage

### Basic Commands
```bash
# Encode a message
pngme encode input.png TeSt "Secret Message" output.png

# Decode a message
pngme decode input.png TeSt

# Remove a message
pngme remove input.png TeSt

# Print all chunks
pngme print input.png
```

### Options
```
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

## Project Structure

```
/src
├── args.rs       # CLI argument parsing
├── chunk.rs      # PNG chunk implementation
├── chunk_type.rs # Chunk type validation
├── commands.rs   # Command implementations
├── main.rs       # Main entry point
├── png.rs        # PNG file handling
```

Key Components:
- Chunk validation with CRC-32 checks
- Proper handling of critical vs ancillary chunks
- Preservation of PNG structure during operations
- ASCII validation for chunk types

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- PNG file format specification
- `structopt` for CLI parsing
- `crc` crate for checksum validation
