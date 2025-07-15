# uts

[![Crates.io](https://img.shields.io/crates/v/uts)](https://crates.io/crates/uts)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/rustcrate/uts/actions/workflows/ci.yml/badge.svg)](https://github.com/rustcrate/uts/actions)

A simple Rust utility for getting Unix timestamps.

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
uts = "0.1"
```

Then in your code:
```rust
use uts::timestamp;

fn main() {
    println!("Current timestamp: {}", timestamp());
}
```

## License

MIT - see [LICENSE](LICENSE) file
