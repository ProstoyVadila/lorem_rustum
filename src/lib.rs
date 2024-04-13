//! Build `LoremRustum` struct with specified length
//! by choosing random elements from `RUSTY_PHRASES` using `rand::thread_rng()`.
//!
//! # Usage
//! ```
//! use lorem_rustum::{LoremRustum, lorem};
//!
//! let lorem = LoremRustum::new(42);
//! println!("{}", lorem.to_string());
//!
//! let mut another_lorem = LoremRustum::default();
//! let text1 = another_lorem.to_string();
//!
//! another_lorem.shuffle();
//! let text2 = another_lorem.to_string();
//!
//! assert_ne!(text1, text2);
//!
//! let text3 = lorem!(156);
//! let text4 = lorem!();   // similar to LoremRustum::default().to_string()
//! ```
//! # Example Output
//! ```bash,ignore
//! checking heap rust-analyzer cargo package manager control
//! over memory allocation static borrowing wasm the most admired
//! language fast and reliable impl derive Ferris mascot actix multiple
//! threads can work on shared data without introducing memory-related
//! issues to_owned() no garbage collector safety sqlx scope mod rocket
//! concurrency pub code execution loved by developers rusty stack async
//! efficient and organized development
//! ```

use rand::prelude::*;

mod data;

#[derive(PartialEq, Debug, Clone)]
pub struct LoremRustum {
    pub body: Vec<&'static str>,
    pub length: usize,
}

impl LoremRustum {
    /// Build `LoremRustum` struct with specified length
    /// by choosing random elements from `RUSTY_PHRASES` using `rand::thread_rng()`.
    ///
    /// # Examples
    /// ```
    /// let lorem = lorem_rustum::LoremRustum::new(42);
    /// println!("{}", lorem.to_string());
    /// ```
    ///
    /// ### Rusty Phrases:
    /// ```
    /// pub static RUSTY_PHRASES: [&str; 132] = [
    ///     "rust", "borrowing", "tokio", "async", "unsafe", "thread", "trait", "&str",
    ///     "rust-analyzer", "scope", "await", "ryan", "ownership", "safety", "nightly",
    ///     "allocated", "rustlings", "stack", "heap", "no garbage collector", "runtime",
    ///     "macros", "code execution", "fmt", "clippy", "memory safe", "cargo", "built-in",
    ///     "performance", "golang sucks", "better than c++", "friendly community",
    ///     "loved by developers", "wasm", "webassembly", "actix", "axum", "rocket", "yew",
    ///     "diesel", "sqlx", "pub", "mod", "enum", "static", "missing lifetime", "rusty",
    ///     "the most admired language", "a safer alternative to C and C++", "cargo package manager",
    ///     "performance-critical services", "Ferris mascot", "fast and reliable", "control over memory allocation",
    ///     "deallocation", "deref", "derive", "impl", "implement trait for", "to_owned()",
    ///     "i'm not looking for a job", "<'static>", "mut", "&mut", "efficient and organized development workflows",
    ///     "concurrency", "multiple threads can work on shared data without introducing memory-related issues",
    ///     "low-level tools and kernels", "type checking", "unwrap", "please gouge out my eyes",
    ///     "Sync + Send", "thread safety", "spawn concurrent task", "non-blocking i/o",
    ///     "smart pointer", "<'a>", "cargo test", "async-std", "println!", "dbg!", "dyn",
    ///     "stderr", "mpsc", "async move", "Arc", "Rc", "RefCell", "Box", "||", "expect",
    ///     "map", "making", "building", "produce", "consume", "out of scope", "rustc",
    ///     "rustup", "panic", "generics", "<T>", "impl fmt::Display for", "macro",
    ///     "#[derive()]", "multiple bounds", "traits", "macro_rules!", "Some()", "Option<&T>",
    ///     "None", "RAII", "drop", "destructor", "mutability of data", "ref", "as",
    ///     "closures", "HOF", "Higher Order Functions", "lazy loading", "err", "error",
    ///     "Result<T, Error>", "()", "Err(_)", "std", "#[cfg(test)]", "assert!",
    ///     "cargo run", "publish crate", "code blocks below"
    /// ];
    /// ```
    pub fn new(length: usize) -> LoremRustum {
        let mut rng = rand::thread_rng();
        let body = LoremRustum::get_body(&mut rng, length);
        LoremRustum { body, length }
    }

    fn get_body(rng: &mut ThreadRng, length: usize) -> Vec<&'static str> {
        if length > data::RUSTY_PHRASES.len() {
            return LoremRustum::get_bigger_body(rng, length);
        }
        let mut rusty_words: Vec<&str> = data::RUSTY_PHRASES.to_vec();
        rusty_words.shuffle(rng);
        rusty_words.drain(0..length).collect()
    }

    fn get_bigger_body(rng: &mut ThreadRng, length: usize) -> Vec<&'static str> {
        let mut body = vec![];
        for _ in 0..length {
            body.push(data::RUSTY_PHRASES.choose(rng).unwrap().to_owned())
        }
        body
    }

    /// Shuffle body of `LoremRustum` in place using `shuffle()` from `rand` crate.
    ///
    /// # Examples
    /// ```
    /// let mut lorem = lorem_rustum::LoremRustum::default();
    /// let text = lorem.to_string();
    ///
    /// lorem.shuffle();
    /// let new_text = lorem.to_string();
    ///
    /// assert_ne!(text, new_text);
    /// ```
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.body.shuffle(&mut rng);
    }
}

impl ToString for LoremRustum {
    fn to_string(&self) -> String {
        self.body.join(" ")
    }
}

impl Default for LoremRustum {
    /// Build `LoremRustum` with all `RUSTY_PHRASES` (contains 132 phrases).
    ///
    /// # Examples
    /// ```
    /// let lorem = lorem_rustum::LoremRustum::default();
    /// println!("{}", lorem.to_string());
    /// ```
    fn default() -> Self {
        let length = data::RUSTY_PHRASES.len();
        LoremRustum::new(length)
    }
}

/// Generates lorem rustum text with given length
/// or with default length if no argument was passed.
///
/// Examples
///
/// ```
/// use lorem_rustum::{LoremRustum, lorem};
///
/// let text: String = lorem!(17);
///
/// println!("{}", lorem!(5));
/// println!("{}", lorem!());
/// ```
///
#[macro_export]
macro_rules! lorem {
    ($length:expr) => {{
        LoremRustum::new($length).to_string()
    }};
    () => {{
        LoremRustum::default().to_string()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_text(text: String) -> String {
        let mut s: Vec<char> = text.chars().collect();
        s.sort();
        let text_sorted: String = s.into_iter().filter(|x| !x.is_whitespace()).collect();
        text_sorted
    }

    #[test]
    fn test_new() {
        let length = data::RUSTY_PHRASES.len();
        let full_text = String::from_iter(data::RUSTY_PHRASES);
        let result = LoremRustum::new(length);

        assert!(result.body.len() == length);
        for i in 0..length {
            assert!(full_text.contains(result.body[i]))
        }
    }

    #[test]
    fn test_default() {
        let length = data::RUSTY_PHRASES.len();
        let lorem_rustum = LoremRustum::new(length);
        let result = LoremRustum::default();

        let full_text = String::from_iter(data::RUSTY_PHRASES);
        let full_text_sorted = sort_text(full_text);
        let result_sorted = sort_text(result.to_string());

        assert_eq!(result.body.len(), lorem_rustum.body.len());
        assert_eq!(full_text_sorted.len(), result_sorted.len());
        assert_eq!(result_sorted, result_sorted);
    }

    #[test]
    fn test_length() {
        let rusty_words_len = data::RUSTY_PHRASES.len();
        let length = rusty_words_len / 2;
        let result = LoremRustum::new(length);

        assert_eq!(result.body.len(), length);

        let length = rusty_words_len * 4;
        let result = LoremRustum::new(length);

        assert_eq!(result.body.len(), length);
    }

    #[test]
    fn test_to_string() {
        let result = LoremRustum::new(5);
        let string = result.body.join(" ");

        assert_eq!(string, result.to_string());
    }

    #[test]
    fn test_random() {
        let length = 25;
        let result = LoremRustum::new(length);
        let body: Vec<&str> = data::RUSTY_PHRASES
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i < length)
            .map(|(_, e)| e)
            .collect();
        let lorem = LoremRustum { body, length };

        assert_ne!(result, lorem);
    }

    #[test]
    fn test_get_body() {
        let length = data::RUSTY_PHRASES.len() / 2;
        let mut rng = rand::thread_rng();
        let result = LoremRustum::get_body(&mut rng, length);
        let body: Vec<&str> = data::RUSTY_PHRASES
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i < length)
            .map(|(_, e)| e)
            .collect();

        assert!(result.len() == length);
        assert_eq!(result.len(), body.len());
    }

    #[test]
    fn test_get_bigger_body() {
        use rand::prelude::*;
        let length = data::RUSTY_PHRASES.len() * 2;
        let mut rng = rand::thread_rng();

        let result = LoremRustum::get_bigger_body(&mut rng, length);
        let mut body = vec![];
        for _ in 0..length {
            body.push(data::RUSTY_PHRASES.choose(&mut rng).unwrap().to_owned())
        }

        assert!(result.len() == length);
        assert_eq!(result.len(), body.len())
    }

    #[test]
    fn test_shuffle() {
        let mut lorem = LoremRustum::default();
        let text = lorem.to_string();
        lorem.shuffle();
        let new_text = lorem.to_string();

        assert_eq!(text.len(), new_text.len());
        assert_ne!(text, new_text);
    }

    #[test]
    fn test_lorem_macro() {
        let length = 16;
        let full_text = String::from_iter(data::RUSTY_PHRASES);
        let text = lorem!(length);
        let split_words: Vec<&str> = text.split(' ').collect();

        for word in split_words {
            assert!(full_text.contains(word));
        }
    }

    #[test]
    fn test_lorem_macro_without_args() {
        let full_text = String::from_iter(data::RUSTY_PHRASES);
        let full_text_sorted = sort_text(full_text);

        let result = lorem!();
        let result_sorted = sort_text(result);

        assert_eq!(full_text_sorted, result_sorted);
    }
}
