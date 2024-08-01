use crate::tokenize::Tokenize;

/// A simple text tokenizer that splits text into tokens by non-alphanumeric characters.
pub struct SimpleTokenizer;

impl SimpleTokenizer {
    /// Creates a new `TextTokenizer`.
    pub fn new() -> Self {
        Self
    }
}

impl Tokenize for SimpleTokenizer {
    fn tokenize(&self, text: impl AsRef<str>) -> Vec<String> {
        let text = text.as_ref();
        text.split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_tokenizer() {
        let text = "Hello, world!";
        let tokenizer = SimpleTokenizer::new();
        let tokens = tokenizer.tokenize(text);
        assert_eq!(tokens, vec!["hello", "world"]);
    }
}
