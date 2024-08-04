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
