pub use simple::SimpleTokenizer;

pub type Tokens = Vec<String>;

pub mod simple;
/// Trait defining a tokenizer that converts text into a vector of tokens.
///
/// # Methods
///
/// * `tokenize` - Tokenizes the input text and returns a vector of tokens.
pub trait Tokenize {
    /// Tokenizes the input text and returns a vector of tokens.
    ///
    /// # Arguments
    ///
    /// * `text` - The input text to be tokenized. It can be any type that implements `AsRef<str>`.
    ///
    /// # Returns
    ///
    /// A vector of tokens of type `String`.
    fn tokenize(&self, text: impl AsRef<str>) -> Vec<String>;
}
