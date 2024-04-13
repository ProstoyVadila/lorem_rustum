# Lorem Rustum

A simple lib for generating random lorem-ipsum with a rusty fleur

## Installation

```bash
cargo add lorem-rustum
```

## Quict Start

```rust
use lorem_rustum::LoremRustum;

let lorem = LoremRustum::new(42);
println!("{}", lorem.to_string());
```

## Other examples

```rust
use lorem_rustum::{LoremRustum, lorem};

let mut lorem = LoremRustum::default();
let text1 = lorem.to_string();

lorem.shuffle();
let text2 = lorem.to_string();

assert_ne!(text1, text2);

let text3 = lorem!(156);
let text4 = lorem!();   // similar to LoremRustum::default().to_string()
```

### Example output

```
checking heap rust-analyzer cargo package manager control over memory allocation static borrowing wasm the most admired language fast and reliable impl derive Ferris mascot actix multiple threads can work on shared data without introducing memory-related issues to_owned() no garbage collector safety sqlx scope mod rocket concurrency pub code execution loved by developers rusty stack async efficient and organized development

```
