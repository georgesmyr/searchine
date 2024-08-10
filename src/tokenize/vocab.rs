use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::path::Path;

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
/// vocab.add_tokens(["hello".to_string(), "world".to_string()]);
///
/// assert_eq!(vocab.get_token_id("hello"), Some(0));
/// assert_eq!(vocab.get_token_id("world"), Some(1));
/// ```
#[derive(Debug, Serialize, Deserialize)]
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

    /// Reads a vocabulary from disk.
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let token_to_id =
            serde_json::from_reader(reader).expect("Failed to read vocabulary from disk");
        Ok(Self { token_to_id })
    }

    /// Adds a token to the vocabulary.
    ///
    /// If the token already exists in the vocabulary, it will not be added again.
    /// Otherwise, the token will be added to the vocabulary and assigned an ID.
    fn add_token(&mut self, token: &str) {
        if !self.token_to_id.contains_key(token) {
            let id = self.token_to_id.len();
            self.token_to_id.insert(token.to_string(), id);
        }
    }

    /// Adds a list of tokens to the vocabulary.
    pub fn add_tokens<'a>(&mut self, tokens: impl IntoIterator<Item=&'a String>) {
        for token in tokens {
            self.add_token(token);
        }
    }

    /// Returns the ID of a token if it exists in the vocabulary.
    pub fn get_token_id(&self, token: &str) -> Option<usize> {
        self.token_to_id.get(token).copied()
    }

    /// Writes the vocabulary to disk.
    pub fn write_to_disk(self, path: impl AsRef<Path>) {
        let file = File::create(path).expect("Failed to create file");
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.token_to_id)
            .expect("Failed to write vocabulary to disk");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocabulary() {
        let mut vocab = Vocabulary::new();
        vocab.add_tokens(&["hello".to_string(), "world".to_string()]);

        assert_eq!(vocab.get_token_id("hello"), Some(0));
        assert_eq!(vocab.get_token_id("world"), Some(1));
    }
}
