use std::collections::HashMap;
use std::fs;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

use rust_stemmers::{Algorithm, Stemmer};
use serde::{Deserialize, Serialize};

/// A pre-tokenizer.
///
/// This struct is used specifically to separate text into lowercase
/// alphanumeric terms.
struct PreTokenizer;

impl PreTokenizer {
    /// Creates a new pre-tokenizer
    fn new() -> Self {
        Self
    }

    /// Separates the input text into a vector of lowercase alphanumeric terms.
    ///
    /// # Arguments
    ///
    /// * `text` - An input text that can be referenced as a string slice.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing the lowercase alphanumeric terms extracted
    /// from the input text.
    fn separate_text(&self, text: impl AsRef<str>) -> Vec<String> {
        let text = text.as_ref();
        text.split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect()
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Encoder {
    vocabulary: HashMap<String, u32>,
}

impl Encoder {
    /// Encodes a token.
    ///
    /// If the token is in the encoder's vocabulary, then the token ID
    /// is returned. Otherwise, it's added and the token ID is returned.
    fn encode(&mut self, token: &str) -> u32 {
        let term_id = self.vocabulary.len() as u32;
        *self.vocabulary.entry(token.to_string()).or_insert(term_id)
    }

    /// Decodes a token ID.
    ///
    /// If the token ID is in the encoder's vocabulary, a shared reference
    /// to the corresponding token is returned.
    #[allow(dead_code)]
    fn decode(&self, token_id: &u32) -> Option<&str> {
        self.vocabulary.iter().find_map(|(key, &value)| {
            if *token_id == value {
                Some(key.as_str())
            } else {
                None
            }
        })
    }

    /// Writes the encoder's vocabulary to file at specified path.
    fn to_file(self, path: impl AsRef<Path>) -> io::Result<()> {
        let file = fs::File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }

    /// Creates an encoder from
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let encoder = serde_json::from_reader(reader)?;
        Ok(encoder)
    }
}

/// A tokenizer that processes input text into tokens, and stems them.
pub struct Tokenizer {
    pre_tokenizer: PreTokenizer,
    stemmer: Stemmer,
    encoder: Encoder,
}

impl Default for Tokenizer {
    /// Creates a new builder with a default pre-tokenizer and stemmer,
    /// and no encoder.
    fn default() -> Self {
        Self {
            pre_tokenizer: PreTokenizer::new(),
            stemmer: Stemmer::create(Algorithm::English),
            encoder: Encoder::default(),
        }
    }
}

impl Tokenizer {
    /// Tokenizes the input text.
    ///
    /// The input text is separated into lowercase alphanumeric terms,
    /// stemmed, and returned as a vector of strings.
    pub fn tokenize(&mut self, text: impl AsRef<str>) -> Vec<u32> {
        let tokens = self.pre_tokenizer.separate_text(text);
        tokens
            .iter()
            .map(|token| {
                let stem = self.stemmer.stem(token);
                self.encoder.encode(stem.into_owned().as_str())
            })
            .collect()
    }

    /// Writes the tokenizer to files at specified path.
    ///
    /// What is written specifically is the encoder's vocabulary.
    pub fn to_file(self, path: impl AsRef<Path>) -> io::Result<()> {
        self.encoder.to_file(path)
    }

    /// Creates a tokenizer from file at specified path.
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let encoder = Encoder::from_file(path)?;
        Ok(Self {
            pre_tokenizer: PreTokenizer::new(),
            stemmer: Stemmer::create(Algorithm::English),
            encoder,
        })
    }
}

/// Structure that stores the token and its count.
///
/// For example, after we have tokenized a query, we can
/// create a token counter for more efficient relevance scoring.
#[derive(Debug)]
pub struct TokenCounts {
    inner: HashMap<u32, u32>,
}

impl TokenCounts {
    /// Returns the count of a specific token.
    pub fn get_count(&self, token: &u32) -> u32 {
        *self.inner.get(token).unwrap_or(&0)
    }
}

impl<I> From<I> for TokenCounts
where
    I: IntoIterator<Item=u32>,
{
    /// Creates token counter from a stream of tokens.
    fn from(tokens: I) -> Self {
        let mut tokens_counter = HashMap::new();
        for token in tokens {
            *tokens_counter.entry(token).or_insert(0) += 1;
        }
        TokenCounts {
            inner: tokens_counter,
        }
    }
}

impl IntoIterator for TokenCounts {
    type Item = (u32, u32);
    type IntoIter = std::collections::hash_map::IntoIter<u32, u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use rust_stemmers::{Algorithm, Stemmer};

    use super::*;

    #[test]
    fn test_pre_tokenizer() {
        let splitter = PreTokenizer::new();
        let text = "I want it, i got it.";
        let words = splitter.separate_text(text);
        assert_eq!(words, vec!["i", "want", "it", "i", "got", "it"]);
    }

    #[test]
    fn test_stemmer() {
        let stemmer = Stemmer::create(Algorithm::English);
        // Stemmer does not turn to lowercase
        assert_eq!(stemmer.stem("Intensely"), "Intens");
    }

    #[test]
    fn test_encoding_decoding() {
        let mut encoder = Encoder::default();

        // Encode
        assert_eq!(encoder.encode("i"), 0);
        assert_eq!(encoder.encode("want"), 1);
        assert_eq!(encoder.encode("it"), 2);
        assert_eq!(encoder.encode("i"), 0);
        assert_eq!(encoder.encode("got"), 3);
        assert_eq!(encoder.encode("it"), 2);

        // Decode
        assert_eq!(encoder.decode(&0), Some("i"));
        assert_eq!(encoder.decode(&1), Some("want"));
        assert_eq!(encoder.decode(&2), Some("it"));
        assert_eq!(encoder.decode(&3), Some("got"));
        assert_eq!(encoder.decode(&4), None);
    }

    #[test]
    fn test_tokenization() {
        let text = "I want it. I got it.".to_string();
        let mut tokenizer = Tokenizer::default();
        let tokens = tokenizer.tokenize(text);
        assert_eq!(tokens, vec![0, 1, 2, 0, 3, 2]);
    }

    #[test]
    fn test_construction() {
        let text = "I want it. I got it.".to_string();
        let mut tokenizer = Tokenizer::default();
        let tokens = tokenizer.tokenize(text);
        let token_counts = TokenCounts::from(tokens);
        assert_eq!(token_counts.get_count(&0), 2);
        assert_eq!(token_counts.get_count(&1), 1);
        assert_eq!(token_counts.get_count(&2), 2);
        assert_eq!(token_counts.get_count(&3), 1);
        assert_eq!(token_counts.get_count(&4), 0);
    }
}
