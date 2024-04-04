
use rand::prelude::*;

mod data;


#[derive(PartialEq, Debug, Clone)]
pub struct LoremRustum {
    pub body: Vec<&'static str>,
    pub length: usize,
}

impl LoremRustum {
    pub fn new(length: usize) -> LoremRustum {
        let mut rng = rand::thread_rng();
        let body = LoremRustum::get_body(&mut rng, length);
        LoremRustum {
            body,
            length
        }
    }

    pub fn default() -> LoremRustum {
        let length = data::RUSTY_WORDS.len();
        LoremRustum::new(length)
    }

    fn get_body(rng: &mut ThreadRng, length: usize) -> Vec<&'static str> {
        if length > data::RUSTY_WORDS.len() {
            return LoremRustum::get_bigger_body(rng, length);
        }
        let mut rusty_words: Vec<&str> = data::RUSTY_WORDS.to_vec();
        rusty_words.shuffle(rng);
        rusty_words.drain(0..length).collect()
    }

    fn get_bigger_body(rng: &mut ThreadRng, length: usize) -> Vec<&'static str> {
        let mut body = vec![];
        for _ in 0..length {
            body.push(data::RUSTY_WORDS.choose(rng).unwrap().to_owned())
        }
        body
    }
}

impl ToString for LoremRustum {
    fn to_string(&self) -> String {
        self.body.join(" ")
    }
}


#[cfg(test)]
mod tests {
    use crate::{data, LoremRustum};

    #[test]
    fn test_new() {
        let length = data::RUSTY_WORDS.len();
        let full_text = String::from_iter(data::RUSTY_WORDS);
        let result = LoremRustum::new(length);
        for i in 0..length {
            assert!(full_text.contains(result.body[i]))
        }
    }

    #[test]
    fn test_default() {
        let length = data::RUSTY_WORDS.len();
        let lorem_rustum = LoremRustum::new(length);
        let result = LoremRustum::default();
        assert_eq!(result.body.len(), lorem_rustum.body.len());
    }

    #[test]
    fn test_length() {
        let rusty_words_len = data::RUSTY_WORDS.len();
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
        let body: Vec<&str> = data::RUSTY_WORDS
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i < length)
            .map(|(_, e)| e)
            .collect();
        let lorem_rustum = LoremRustum {
            body,
            length,
        };
        assert_ne!(result, lorem_rustum);
    }

    #[test]
    fn test_get_body() {
        let length = data::RUSTY_WORDS.len() / 2;
        let mut rng = rand::thread_rng();
        let result = LoremRustum::get_body(&mut rng, length);
        let body: Vec<&str> = data::RUSTY_WORDS
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i < length)
            .map(|(_, e)| e)
            .collect();
        assert_eq!(result.len(), body.len());
    }

    #[test]
    fn test_get_bigger_body() {
        use rand::prelude::*;
        let length = data::RUSTY_WORDS.len() * 2;
        let mut rng = rand::thread_rng();

        let result = LoremRustum::get_bigger_body(&mut rng, length);
        let mut body = vec![];
        for _ in 0..length {
            body.push(data::RUSTY_WORDS.choose(&mut rng).unwrap().to_owned())
        }
        assert_eq!(result.len(), body.len())
    }
}