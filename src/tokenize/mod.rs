pub use simple::SimpleTokenizer;
pub use vocab::Vocabulary;

pub type Token = String;
pub type Tokens = Vec<String>;

pub mod simple;
pub mod vocab;


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


/// Trait defining a stemmer that converts a token into its stemmed form.
///
/// # Methods
///
/// * `stem` - Stems the input token and returns the stemmed version.
pub trait Stem {
    /// Stems the input token and returns the stemmed version.
    ///
    /// # Arguments
    ///
    /// * `token` - The input token to be stemmed. It is a string slice.
    ///
    /// # Returns
    ///
    /// A `String` representing the stemmed version of the input token.
    fn stem(&self, token: &str) -> String;
}