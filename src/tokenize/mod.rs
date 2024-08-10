pub mod vocab;

use rust_stemmers::{Algorithm, Stemmer};

pub use vocab::Vocabulary;

/// A pre-tokenizer.
///
/// This struct is used specifically to separate text into lowercase
/// alphanumeric terms.
pub struct PreTokenizer;

impl PreTokenizer {
    /// Creates a new pre-tokenizer
    pub fn new() -> Self {
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
    pub fn separate_text(&self, text: impl AsRef<str>) -> Vec<String> {
        let text = text.as_ref();
        text.split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect()
    }
}

/// A marker struct that indicates that the encoder is not specified.
pub struct NoEncoder;

/// A builder for creating instances of `Tokenizer`.
///
/// # Type Parameters
///
/// * `E` - The type of the encoder used by the tokenizer.
pub struct Builder<E> {
    pre_tokenizer: PreTokenizer,
    stemmer: Stemmer,
    encoder: E,
}

impl Default for Builder<NoEncoder> {
    /// Creates a new builder with a default pre-tokenizer and stemmer,
    /// and no encoder.
    fn default() -> Self {
        Self {
            pre_tokenizer: PreTokenizer::new(),
            stemmer: Stemmer::create(Algorithm::English),
            encoder: NoEncoder,
        }
    }
}

impl Builder<NoEncoder> {
    /// Creates a new `Builder` instance with the specified encoder.
    ///
    /// # Arguments
    ///
    /// * `encoder` - The encoder to be used by the tokenizer.
    ///
    /// # Returns
    ///
    /// A `Builder<Encoder>` instance with the specified encoder, and default
    /// pre-tokenizer and stemmer.
    pub fn with_encoder(self, encoder: Encoder) -> Builder<Encoder> {
        Builder {
            pre_tokenizer: PreTokenizer::new(),
            stemmer: Stemmer::create(Algorithm::English),
            encoder,
        }
    }

    /// Builds a `Tokenizer` instance with no encoder.
    ///
    /// # Returns
    ///
    /// A `Tokenizer<NoEncoder>` instance with the pre-tokenizer and stemmer
    /// from the builder, and no encoder.
    pub fn build(self) -> Tokenizer<NoEncoder> {
        Tokenizer {
            pre_tokenizer: self.pre_tokenizer,
            stemmer: self.stemmer,
            encoder: NoEncoder,
        }
    }
}

impl Builder<Encoder> {
    /// Builds a `Tokenizer` instance with the specified encoder.
    ///
    /// # Returns
    ///
    /// A `Tokenizer<Encoder>` instance with the pre-tokenizer and stemmer
    /// from the builder, and the specified encoder.
    pub fn build(self) -> Tokenizer<Encoder> {
        Tokenizer {
            pre_tokenizer: self.pre_tokenizer,
            stemmer: self.stemmer,
            encoder: self.encoder,
        }
    }
}

/// A tokenizer that processes input text into tokens, stems them,
/// and optionally encodes them.
///
/// # Type Parameters
///
/// * `E` - The type of the encoder used by the tokenizer.
/// This can be `NoEncoder` if no encoding is required.
pub struct Tokenizer<E> {
    pre_tokenizer: PreTokenizer,
    stemmer: Stemmer,
    encoder: E,
}

impl Tokenizer<NoEncoder> {
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

impl Tokenizer<Encoder> {
    /// Tokenizes the input text.
    ///
    /// The input text is separated into lowercase alphanumeric terms,
    /// stemmed, encoded, and returned as a vector of token ids.
    pub fn tokenize(&self, text: impl AsRef<str>) -> Vec<usize> {
        let tokens = self.pre_tokenizer.separate_text(text);
        let tokens = tokens
            .iter()
            .map(|token| self.stemmer.stem(token).to_string())
            .collect();
        self.encoder.encode(tokens)
    }
}

/// Token encoder.
///
/// The encoder contains a vocabulary, mapping each token to a token id.
/// The encoder is used to encode a list of tokens into a list of token ids.
pub struct Encoder {
    vocab: Vocabulary,
}

impl Encoder {
    /// Encodes a list of tokens into a list of token ids.
    pub fn encode(&self, tokens: Vec<String>) -> Vec<usize> {
        tokens
            .iter()
            .filter_map(|token| self.vocab.get_token_id(token))
            .collect::<Vec<_>>()
    }
}

impl From<Vocabulary> for Encoder {
    /// Creates a new `Encoder` instance from a `Vocabulary`.
    fn from(vocab: Vocabulary) -> Self {
        Self { vocab }
    }
}
