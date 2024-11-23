# Text Compressor (TC)
**Text Compressor (TC)** is a Rust library designed for efficient text compression using **Huffman encoding**, a widely used lossless data compression algorithm. This library aims to provide developers with a lightweight, easy-to-integrate solution for compressing and decompressing textual data.

## Features
- **Huffman Encoding**: Compress text data using the well-known Huffman algorithm.
- **Lossless Compression**: Ensures that the original data can be perfectly reconstructed after decompression.
- **Lightweight and Fast**: Designed to minimize overhead and maximize performance.
- **Flexible API**: Provides easy-to-use functions for compressing and decompressing strings.

## Installation
Add ```tc``` to your ```Cargo.toml```:

```toml
[dependencies]
tc = { git = "https://github.com/root-hunter/tc.git" }
```

Then, include it in your code:
```rust
use tc::core::{compress, decompress}
```
## Usage


## Contribution

Contributions, bug reports, and feature requests are welcome! Feel free to open an issue or a pull request on the [GitHub repository](https://github.com/root-hunter/tc).

### Development Setup

1) Clone the repository:
```sh
git clone https://github.com/root-hunter/tc.git
```
2) Build the project:
```sh
cargo build
```
3) Run tests:
```sh
cargo test
```

## License
This project is licensed under the [MIT License](https://github.com/root-hunter/tc/blob/master/LICENSE).

Feel free to use, modify, and distribute the library under the terms of the license.

Happy compressing! 🎉