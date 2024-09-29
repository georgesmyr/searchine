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
pub fn calc_tf_idf(tf: f64, idf: f64) -> f64 {
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
pub fn calc_tf(t: u32, d: u32) -> f64 {
    if d == 0 {
        0.0
    } else {
        (t as f64) / (d as f64)
    }
}

/// Calculates the inverse term frequency of a term in a collection of documents.
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
pub fn calc_idf(d: u32, n: u32) -> f64 {
    let num = (n + 1) as f64;
    let den = (d + 1) as f64;
    (num / den).log(10.0)
}

/// Calculates the BM25 (Best Matching 25) score for a term in a collection of
/// documents.
///
/// # Arguments
///
/// * `f` - The number of times the term appears in the document.
/// * `n` - The total number of documents in the collection.
/// * `d` - The number of documents containing the term.
/// * `l` - The number of terms in the document.
/// * `a` - Average length of a document.
/// * `k` - Free parameter, usually in [1.2, 2.0]
/// * `b` - Free parameter, usually equal to 0.75.
pub fn calc_bm25(f: u32, n: u32, d: u32, l: u32, a: f64, k: f64, b: f64) -> f64 {
    let idf = calc_idf(d, n);
    let num = (f as f64) * (k + 1f64);
    let den = (f as f64) + k * (1f64 - b + b * (l as f64) / a);
    idf * num / den
}
