pub mod metrics;
pub use metrics::*;

use std::collections::HashMap;

/// Stores the scores of each document.
#[derive(Default, Debug)]
pub struct DocumentsScores {
    inner: HashMap<usize, f64>,
}


impl DocumentsScores {
    /// Creates a new empty structure to store documents scorers.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds to the score of a document.
    ///
    /// If the document is present in the struct, it adds to its current score.
    /// If the document is not present in the struct, it adds the document with
    /// the specified ID and score.
    pub fn add_score(&mut self, doc_id: usize, score: f64) {
        *self.inner.entry(doc_id).or_insert(0.0) += score;
    }

    /// Returns the score of the specified document.
    ///
    /// If the document is present in the struct it returns its score.
    /// If the document is not present, it returns 0.0.
    pub fn get_score(&self, doc_id: usize) -> f64 {
        *self.inner.get(&doc_id).unwrap_or(&0.0)
    }

    /// Returns a vector of the documents with the top n scores. 
    pub fn get_top_n(&self, top_n: usize) -> Vec<(&usize, &f64)> {
        let mut elements = self.inner.iter().collect::<Vec<_>>();
        elements.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
        elements.into_iter().take(top_n).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_get_score() {
        let mut ds = DocumentsScores::new();
        ds.add_score(0, 0.5);
        ds.add_score(1, 0.3);
        ds.add_score(0, 0.2);
        assert_eq!(ds.get_score(0), 0.7);
        assert_eq!(ds.get_score(1), 0.3);
        assert_eq!(ds.get_score(2), 0.0);
    }
}
