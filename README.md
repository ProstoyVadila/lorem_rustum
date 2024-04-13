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
    let lorem = LoremRustum::new(length);
    let full_lorem = LoremRustum::default();

    println!("{}", lorem.to_string());
    println!("{}", full_lorem.to_string());
}
```

```rust

let mut lorem = LoremRustum::default();
let text = lorem.to_string();

lorem.shuffle();
let new_text = lorem.to_string();

assert_ne!(text, new_text);
```
