# Lorem Rustum

A small tool for generating random lorem-ipsum with a rusty fleur

# Quict Start

## Installation

```bash
cargo add lorem-rustum
```

## Usage

```rust
use lorem_rustum::LoremRustum;

fn main() {
    let length = 25
    let text = LoremRustum::new(length);
    let full_text = LoremRustum::default();

    println!("{}", text.to_string());
    println!("{}", full_text.to_string());
}
```
