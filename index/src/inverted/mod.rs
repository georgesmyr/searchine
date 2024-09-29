use documents::DocumentId;
use tokenize::Token;

pub mod freq;

pub trait Index {
    /// Returns the number of documents in the index.
    fn n_docs(&self) -> u32;

    /// Returns the total number of terms in all the documents in
    /// the index.
    fn n_terms_total(&self) -> u32;

    /// Returns the document IDs that contain the specified term.
    fn doc_ids_containing(&self, term: &Token) -> Vec<DocumentId>;

    /// Returns the number of documents containing the specified token.
    fn n_docs_containing(&self, term: &Token) -> u32;

    /// Returns the number of terms in the document, counting the occurrence
    /// of the same term separately.
    fn n_terms(&self, doc_id: DocumentId) -> u32;

    /// Returns the frequency of a specified term in a specified document.
    fn term_frequency(&self, doc_id: DocumentId, term: &Token) -> u32;

    /// Calculates the inverse document frequency score (IDF)
    fn calc_idf(&self, term: &Token) -> f64 {
        let n_docs_containing = self.n_docs_containing(term);
        let n_docs = self.n_docs();
        crate::score::calc_idf(n_docs_containing, n_docs)
    }

    /// Calculates the term frequency score (TF) of a specified term.
    fn calc_tf(&self, doc_id: DocumentId, term: &Token) -> f64 {
        let term_count = self.term_frequency(doc_id, &term);
        let total_count = self.n_terms(doc_id);
        crate::score::calc_tf(term_count, total_count)
    }

    /// Calculates the TF-IDF score of a term for a specified document.
    fn calc_tf_idf(&self, doc_id: DocumentId, term: &Token) -> f64 {
        let tf = self.calc_tf(doc_id, term);
        let idf = self.calc_idf(term);
        crate::score::calc_tf_idf(tf, idf)
    }

    /// Calculates the BM25 score of a term for a specified document.
    fn calc_bm25(&self, doc_id: DocumentId, term: &Token, k: f64, b: f64) -> f64 {
        let term_freq = self.term_frequency(doc_id, term);
        let n_docs = self.n_docs();
        let n_docs_containing = self.n_docs_containing(term);
        let doc_terms = self.n_terms(doc_id);
        let avg = (self.n_terms_total() as f64) / (self.n_docs() as f64);
        crate::score::calc_bm25(term_freq, n_docs, n_docs_containing, doc_terms, avg, k, b)
    }
}
