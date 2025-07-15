# timestamp

[![Crates.io](https://img.shields.io/crates/v/timestamp)](https://crates.io/crates/timestamp)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/rustcrate/timestamp/actions/workflows/ci.yml/badge.svg)](https://github.com/rustcrate/timestamp/actions)

A simple Rust utility for getting Unix timestamps.

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
timestamp = "0.1"
```

Then in your code:
```rust
use timestamp::timestamp;

fn main() {
    println!("Current timestamp: {}", timestamp());
}
```

## License

MIT - see [LICENSE](LICENSE) file
