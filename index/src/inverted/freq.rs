use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json;

use documents::DocumentId;
use tokenize::Token;

use crate::doc::freq::DocumentFrequencyIndex;
use crate::doc::term::DocumentTermsCounter;
use crate::inverted::Index;
use crate::postings::*;

const SIGNATURE: &[u8] = b"SEARHINE";
const VERSION: u8 = 1;

/// An in-memory inverted index. The inverted index is a HashMap with
/// the token as the key and a postings list as the value.
#[derive(Default, Debug, Serialize, Deserialize)]
struct FrequencyInvertedIndex {
    inner: HashMap<Token, FrequencyPostingsList>,
}

impl FrequencyInvertedIndex {
    /// Inserts a document index into the in-memory frequency inverted
    /// index.
    ///
    /// For each token in the document index, the method inserts the
    /// token into the inverted index. If the token is already in the
    /// index, the posting is inserted into the postings list.
    fn index(&mut self, doc_index: DocumentFrequencyIndex) {
        let doc_id = doc_index.doc_id();

        for (token, token_freq) in doc_index {
            let posting = FrequencyPosting::new(doc_id, token_freq);
            if let Some(postings_list) = self.inner.get_mut(&token) {
                postings_list.add(posting);
            } else {
                let mut postings_list = FrequencyPostingsList::new();
                postings_list.add(posting);
                self.inner.insert(token, postings_list);
            }
        }
    }

    /// Initiates the process for writing the inverted index to a file.
    /// It creates a new file with the specified path and writes the header in it.
    fn pre_write_file(&self, path: impl AsRef<Path>) -> io::Result<File> {
        let mut file = File::create(path)?;
        file.write_all(SIGNATURE)?; // Write signature
        file.write_all(&[VERSION])?; // Write index version
        file.write_all(&(self.inner.len() as u32).to_be_bytes())?; // Write entry count
        Ok(file)
    }

    /// Writes the frequency inverted index to a file.
    fn into_file(self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let file = File::create(path)?;
        let file = BufWriter::new(file);
        serde_json::to_writer(file, &self)?;

        // let mut file = self.pre_write_file(path)?;
        // let entries = self.inner
        //     .into_iter()
        //     .collect::<BTreeMap<_, _>>();
        //
        // let mut buffer = Cursor::new(Vec::<u8>::new());
        // for (term_id, postings_list) in entries {
        //     // Clear buffer
        //     buffer.get_mut().clear();
        //     buffer.set_position(0);
        //
        //     let gamma_encoder = GammaEncoder::new(Cursor::new(vec![]));
        // entry_buffer.extend(&entry.ctime.to_be_bytes());
        // entry_buffer.extend(&entry.ctime_ns.to_be_bytes());
        // entry_buffer.extend(entry.path.to_string_lossy().as_bytes());
        // let padding = 8 - (entry_buffer.len() % 8);
        // entry_buffer.extend(vec![0; padding]);
        // file.write_all(&entry_buffer)
        //     .context("Failed to write entry in index")?;
        // }

        Ok(())
    }
}

/// Frequency indexing model.
///
/// It stores the inverted frequency index, and a structure
/// that stores the number of terms in each document in the
/// index.
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct FrequencyIndex {
    inverted_index: FrequencyInvertedIndex,
    doc_terms_counter: DocumentTermsCounter,
}

impl FrequencyIndex {
    /// Creates a new, empty frequency index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Indexes a document index with frequency postings.
    pub fn index(&mut self, doc_index: DocumentFrequencyIndex) {
        self.doc_terms_counter
            .insert_doc_terms(doc_index.doc_id(), doc_index.n_terms());
        self.inverted_index.index(doc_index);
    }

    /// Writes inverted index with frequency postings to file.
    pub fn into_file(self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        let file = File::create(path).context(format!(
            "Failed to create index file at: {}",
            path.display()
        ))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self).context("Failed to write index to writer.")
    }

    /// Loads inverted index with frequency postings from file.
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let file =
            File::open(path).context(format!("Failed to open file at: {}", path.display()))?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).context(format!(
            "Failed to read index from file: {}",
            path.display()
        ))
    }
}

impl Index for FrequencyIndex {
    fn n_docs(&self) -> u32 {
        self.doc_terms_counter.n_docs()
    }

    fn n_terms_total(&self) -> u32 {
        self.doc_terms_counter.n_terms_total()
    }

    fn doc_ids_containing(&self, term: &Token) -> Vec<DocumentId> {
        let res = self.inverted_index.inner.get(term);
        match res {
            Some(postings_list) => postings_list.doc_ids(),
            _ => Vec::new(),
        }
    }

    fn n_docs_containing(&self, term: &Token) -> u32 {
        self.inverted_index
            .inner
            .get(term)
            .map_or(0, |p_lst| p_lst.len() as u32)
    }

    fn n_terms(&self, doc_id: DocumentId) -> u32 {
        self.doc_terms_counter.n_terms(doc_id)
    }

    fn term_frequency(&self, doc_id: DocumentId, term: &Token) -> u32 {
        let x = self.inverted_index.inner.get(term).unwrap();
        let y = x.get(doc_id).unwrap();
        y.frequency()
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test_frequency_indexing() {
    //     let tokens_1 = vec![1, 2, 3, 1, 4];
    //     let mut doc_index_1 = DocumentFrequencyIndex::new(0);
    //     doc_index_1.index_tokens(tokens_1);
    //
    //     let tokens_2 = vec![1, 2, 5];
    //     let mut doc_index_2 = DocumentFrequencyIndex::new(1);
    //     doc_index_2.index_tokens(tokens_2);
    //
    //     let mut index = FrequencyIndex::new();
    //     index.index(doc_index_1);
    //     index.index(doc_index_2);
    //
    //     assert_eq!(index.n_docs(), 2);
    //     assert_eq!(index.n_docs_containing(1), 2);
    //     assert_eq!(index.n_terms(0), 5);
    //     assert_eq!(index.n_terms(1), 3);
    //     assert_eq!(index.term_frequency(0, 1), 2);
    //     assert_eq!(index.term_frequency(1, 1), 1);
    // }
}
