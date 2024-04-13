# Lorem Rustum

A small tool for generating random lorem-ipsum with a rusty fleur

# Quict Start

## Installation

```bash
cargo add lorem-rustum
```

## Usage

```rust
use lorem_rustum::{LoremRustum, lorem};

let lorem = LoremRustum::new(42);
println!("{}", lorem.to_string());

let mut another_lorem = LoremRustum::default();
let text1 = another_lorem.to_string();

another_lorem.shuffle();
let text2 = another_lorem.to_string();

assert_ne!(text1, text2);

let text3 = lorem!(156);
let text4 = lorem!();   // similar to LoremRustum::default().to_string()
```

### Example output

```
checking heap rust-analyzer cargo package manager control over memory allocation static borrowing wasm the most admired language fast and reliable impl derive Ferris mascot actix multiple threads can work on shared data without introducing memory-related issues to_owned() no garbage collector safety sqlx scope mod rocket concurrency pub code execution loved by developers rusty stack async efficient and organized development

```
