use crate::tokenize::Tokens;
use crate::tokenize::*;

/// A simple text tokenizer that splits text into tokens by non-alphanumeric characters.
pub struct SimpleTokenizer;

impl SimpleTokenizer {
    /// Creates a new `TextTokenizer`.
    pub fn new() -> Self {
        Self
    }
}

impl Tokenize for SimpleTokenizer {
    fn tokenize(&self, text: impl AsRef<str>) -> Tokens {
        let text = text.as_ref();
        text.split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect()
    }
}

