use rust_stemmers::{Algorithm, Stemmer};

use crate::pre::PreTokenizer;
use crate::Token;

/// A tokenizer that processes input text into tokens, and stems them.
pub struct Tokenizer {
    pre_tokenizer: PreTokenizer,
    stemmer: Stemmer,
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

impl Tokenizer {
    /// Tokenizes the input text.
    ///
    /// The input text is separated into lowercase alphanumeric terms,
    /// stemmed, and returned as a vector of strings.
    pub fn tokenize(&mut self, text: impl AsRef<str>) -> Vec<Token> {
        let tokens = self.pre_tokenizer.separate_text(text);
        tokens
            .iter()
            .map(|token| {
                let stem = self.stemmer.stem(token).to_string();
                stem
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use rust_stemmers::{Algorithm, Stemmer};

    use super::*;

    #[test]
    fn test_stemmer() {
        let stemmer = Stemmer::create(Algorithm::English);
        // Stemmer does not turn to lowercase
        assert_eq!(stemmer.stem("Intensely"), "Intens");
    }

    #[test]
    fn test_tokenization() {
        let text = "I want it. I got it.".to_string();
        let mut tokenizer = Tokenizer::default();
        let tokens = tokenizer.tokenize(text);
        assert_eq!(
            tokens,
            vec![
                "i".to_string(),
                "want".to_string(),
                "it".to_string(),
                "i".to_string(),
                "got".to_string(),
                "it".to_string(),
            ]
        );
    }
}
