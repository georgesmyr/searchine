/// A pre-tokenizer.
///
/// This struct is used specifically to separate text into lowercase
/// alphanumeric terms.
pub(crate) struct PreTokenizer;

impl PreTokenizer {
    /// Creates a new pre-tokenizer
    pub(crate) fn new() -> Self {
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
    pub(crate) fn separate_text(&self, text: impl AsRef<str>) -> Vec<String> {
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
    fn test_pre_tokenizer() {
        let splitter = PreTokenizer::new();
        let text = "I want it, i got it.";
        let words = splitter.separate_text(text);
        assert_eq!(words, vec!["i", "want", "it", "i", "got", "it"]);
    }
}
