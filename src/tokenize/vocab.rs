use std::fs::File;
use std::path::Path;

use crate::tokenize::Token;

use serde::{Deserialize, Serialize};
use serde_json;

/// A vocabulary that maps tokens to IDs and vice versa.
///
/// The vocabulary is used to map tokens to IDs and vice versa.
///
/// # Examples
///
/// ```
/// use crate::tokenize::Vocabulary;
///
/// let mut vocab = Vocabulary::new();
/// vocab.add_token("hello");
/// vocab.add_token("world");
///
/// assert_eq!(vocab.get_id("hello"), Some(0));
/// assert_eq!(vocab.get_id("world"), Some(1));
///
/// assert_eq!(vocab.get_token(0), Some(&"hello".to_string()));
/// assert_eq!(vocab.get_token(1), Some(&"world".to_string()));
/// ```
#[derive(Serialize, Deserialize)]
pub struct Vocabulary {
    token_to_id: std::collections::HashMap<String, usize>,
}

impl Vocabulary {
    /// Creates a new vocabulary.
    pub fn new() -> Self {
        Self {
            token_to_id: std::collections::HashMap::new(),
        }
    }

    /// Adds a token to the vocabulary.
    ///
    /// If the token already exists in the vocabulary, it will not be added again.
    /// Otherwise, the token will be added to the vocabulary and assigned an ID.
    pub fn add_token(&mut self, token: &str) {
        if self.token_to_id.contains_key(token) {
            return;
        }

        let id = self.token_to_id.len();
        self.token_to_id.insert(token.to_string(), id);
    }

    /// Adds a list of tokens to the vocabulary.
    pub fn add_tokens<'a>(&mut self, tokens: impl IntoIterator<Item=&'a Token>) {
        for token in tokens {
            self.add_token(token);
        }
    }

    /// Returns the ID of a token if it exists in the vocabulary.
    pub fn get_id(&self, token: &str) -> Option<usize> {
        self.token_to_id.get(token).copied()
    }

    /// Writes the vocabulary to disk.
    pub fn write_to_disk(self, path: impl AsRef<Path>) {
        let file = File::create(path).expect("Failed to create file");
        serde_json::to_writer_pretty(file, &self.token_to_id)
            .expect("Failed to write vocabulary to disk");
    }

    /// Reads a vocabulary from disk.
    pub fn read_from_disk(path: impl AsRef<Path>) -> Self {
        let file = File::open(path).expect("Failed to open file");
        let token_to_id = serde_json::from_reader(file).
            expect("Failed to read vocabulary from disk");
        Self { token_to_id }
    }
}

impl IntoIterator for Vocabulary {
    type Item = (String, usize);
    type IntoIter = std::collections::hash_map::IntoIter<String, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.token_to_id.into_iter()
    }
}


impl<'a> IntoIterator for &'a Vocabulary {
    type Item = (&'a String, &'a usize);
    type IntoIter = std::collections::hash_map::Iter<'a, String, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.token_to_id.iter()
    }
}
