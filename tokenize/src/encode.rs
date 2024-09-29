use std::collections::HashMap;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::{fs, io};

use serde::{Deserialize, Serialize};

type TokenId = u32;

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct Encoder {
    vocabulary: HashMap<String, TokenId>,
}

impl Encoder {
    /// Encodes a token.
    ///
    /// If the token is in the encoder's vocabulary, then the token ID
    /// is returned. Otherwise, it's added and the token ID is returned.
    pub(crate) fn encode(&mut self, token: &str) -> TokenId {
        let term_id = self.vocabulary.len() as u32;
        *self.vocabulary.entry(token.to_string()).or_insert(term_id)
    }

    /// Decodes a token ID.
    ///
    /// If the token ID is in the encoder's vocabulary, a shared reference
    /// to the corresponding token is returned.
    #[allow(dead_code)]
    fn decode(&self, token_id: &TokenId) -> Option<&str> {
        self.vocabulary.iter().find_map(|(key, &value)| {
            if *token_id == value {
                Some(key.as_str())
            } else {
                None
            }
        })
    }

    /// Writes the encoder's vocabulary to file at specified path.
    pub(crate) fn into_file(self, path: impl AsRef<Path>) -> io::Result<()> {
        let file = fs::File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }

    /// Creates an encoder from
    pub(crate) fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let encoder = serde_json::from_reader(reader)?;
        Ok(encoder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
