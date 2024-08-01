use std::collections::HashMap;

use crate::tokenize::Tokenize;

pub mod postings;
pub mod docs;

/// An in-memory index for a single file.
///
/// This index stores the indexed document in memory. For large document the index
/// might not fit in-memory, and you might need to use an on-disk index.
pub struct SimpleInMemoryIndex {
    /// The index of the words in the document. The key is the word, and the value is
    /// the frequency of the word in the document.
    pub index: Vec<(String, usize)>,
}

impl SimpleInMemoryIndex {
    /// Returns the frequency of a token in the document. If the token is not in the
    /// index it returns `None`.
    pub fn get(&self, token: &str) -> Option<usize> {
        self.index.iter().find_map(
            |(t, freq)| if t == token { Some(*freq) } else { None }
        )
    }

    /// Returns the number of tokens in the index.
    pub fn count(&self) -> usize {
        self.index.len()
    }
}

/// An indexer that indexes a single file.
pub struct FileIndexer<T> {
    tokenizer: T,
}

impl<T: Tokenize> FileIndexer<T> {
    /// Creates a new `FileIndexer` with the given tokenizer.
    pub fn new(tokenizer: T) -> Self {
        Self { tokenizer }
    }

    /// Indexes the content of a file.
    pub fn index(&self, content: String) -> SimpleInMemoryIndex {
        let tokens = self.tokenizer.tokenize(&content);
        let mut index = HashMap::<String, usize>::new();
        for token in tokens {
            let count = index.entry(token).or_insert(0);
            *count += 1;
        }

        let mut index = index.into_iter().collect::<Vec<_>>();
        index.sort_by_key(|&(_, freq)| freq);
        index.reverse();
        SimpleInMemoryIndex { index }
    }
}


