/// Calculates the TF-IDF score of a term in a document.
///
/// The TF-IDF score is the product of the term frequency (TF) and the inverse
/// document frequency (IDF). The term frequency is the number of times a term
/// appears in a document, divided by the total number of terms in the document.
/// The inverse document frequency is the logarithmically scaled inverse fraction
/// of the documents that contain the word.
///
/// # Arguments
///
/// * `tf` - The term frequency of the term in the document.
/// * `idf` - The inverse document frequency of the term in the collection.
///
/// # Returns
///
/// The TF-IDF score of the term in the document.
pub fn tf_idf(tf: f64, idf: f64) -> f64 {
    tf * idf
}

/// Calculates the term frequency of a term in a document.
///
/// # Arguments
///
/// * `t` - The number of times the term appears in the document.
/// * `d` - The total number of terms in the document.
///
/// # Returns
///
/// The term frequency of the term in the document.
pub fn tf(t: usize, d: usize) -> f64 {
    if d == 0 {
        0.0
    } else {
        (t as f64) / (d as f64)
    }
}

/// Calculates the inverse term frequency of aa tern in a collection of documents.
///
/// The inverse document frequency is a measure of how much information the word
/// provides, i.e., how common or rare it is across all documents. It is the
/// logarithmically scaled inverse fraction of the documents that contain the word
/// (obtained by dividing the total number of documents by the number of documents
/// containing the term, and then taking the logarithm of that quotient). If the
/// term is not in the corpus, this will lead to a division-by-zero. It is therefore
/// common to adjust the both numerator and denominator by adding 1 to the counts.
///
/// # Arguments
///
/// * `d` - The number of documents containing the term.
/// * `n` - The total number of documents in the collection.
///
pub fn idf(d: usize, n: usize) -> f64 {
    let num = (n + 1) as f64;
    let den = (d + 1) as f64;
    (num / den).log(10.0)
}
