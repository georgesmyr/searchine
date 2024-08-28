use std::collections::HashMap;

use rust_stemmers::{Algorithm, Stemmer};

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
/// A tokenizer that processes input text into tokens, and stems them.
pub struct Tokenizer {
    pre_tokenizer: PreTokenizer,
    stemmer: Stemmer,
}

impl Tokenizer {
    /// Tokenizes the input text.
    ///
    /// The input text is separated into lowercase alphanumeric terms,
    /// stemmed, and returned as a vector of strings.
    pub fn tokenize(&self, text: impl AsRef<str>) -> Vec<String> {
        let tokens = self.pre_tokenizer.separate_text(text);
        tokens
            .iter()
            .map(|token| self.stemmer.stem(token).to_string())
            .collect()
    }
}

impl Default for Tokenizer {
    /// Creates a new builder with a default pre-tokenizer and stemmer,
    /// and no encoder.
    fn default() -> Self {
        Self {
            pre_tokenizer: PreTokenizer::new(),
            stemmer: Stemmer::create(Algorithm::English),
        }
    }
}

/// Structure that stores the token and its count.
///
/// For example, after we have tokenized a query, we can
/// create a token counter for more efficient relevance scoring.
#[derive(Debug)]
pub struct TokenCounts {
    inner: HashMap<String, usize>,
}

impl TokenCounts {
    /// Returns the count of a specific token.
    pub fn get_count(&self, token: &str) -> usize {
        *self.inner.get(token).unwrap_or(&0)
    }
}

impl<I> From<I> for TokenCounts
where
    I: IntoIterator<Item=String>,
{
    /// Creates token counter from a stream of tokens.
    fn from(tokens: I) -> Self {
        let mut tokens_counter = HashMap::new();
        for token in tokens {
            *tokens_counter.entry(token).or_insert(0) += 1;
        }
        TokenCounts { inner: tokens_counter }
    }
}

impl IntoIterator for TokenCounts {
    type Item = (String, usize);
    type IntoIter = std::collections::hash_map::IntoIter<String, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenization() {
        let text = "I want it. I got it.".to_string();
        let tokenizer = Tokenizer::default();
        let tokens = tokenizer.tokenize(text);
        assert_eq!(tokens, vec![
            "i".to_string(), "want".to_string(), "it".to_string(),
            "i".to_string(), "got".to_string(), "it".to_string(),
        ])
    }

    #[test]
    fn test_construction() {
        let text = "I want it. I got it.".to_string();
        let tokenizer = Tokenizer::default();
        let tokens = tokenizer.tokenize(text);
        let token_counts = TokenCounts::from(tokens);
        assert_eq!(token_counts.get_count("i"), 2);
        assert_eq!(token_counts.get_count("want"), 1);
        assert_eq!(token_counts.get_count("it"), 2);
        assert_eq!(token_counts.get_count("got"), 1);
        assert_eq!(token_counts.get_count("you"), 0);
    }
}