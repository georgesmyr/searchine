#[derive(Debug, PartialEq)]
pub struct Posting {
    doc_id: usize,
    term_freq: usize,
}

impl Posting {
    /// Creates a new `Posting` with the given document ID and term frequency.
    #[allow(dead_code)]
    fn new(doc_id: usize, term_freq: usize) -> Self {
        Self { doc_id, term_freq }
    }
}

pub struct Postings {
    postings: Vec<Posting>,
}

impl Postings {
    /// Creates a new `Postings` with an empty list of postings.
    pub fn new() -> Self {
        Self { postings: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { postings: Vec::with_capacity(capacity) }
    }

    /// Adds a posting to the postings, by specifying the document ID and term frequency.
    pub fn add(&mut self, doc_id: usize, term_freq: usize) {
        self.postings.push(Posting { doc_id, term_freq });
    }

    /// Adds a posting to the postings, by specifying the posting.
    pub fn add_posting(&mut self, posting: Posting) {
        self.postings.push(posting);
    }
}


/// Intersects two postings and returns a new `Postings` with the intersection of the two.
///
/// # Examples
///
/// ```
/// use crate::index::postings::*;
///
/// let mut postings1 = Postings::new();
/// postings1.add(1, 2);
/// postings1.add(2, 3);
///
/// let mut postings2 = Postings::new();
/// postings2.add(1, 3);
///
/// let merged = intersect_postings(postings1, postings2);
/// assert_eq!(merged.postings, vec![Posting::new(1, 5)]);
/// ```
pub fn merge_postings(postings1: Postings, postings2: Postings) -> Postings {
    // TODO: Postings::with_capacity(_) vs Postings::new()
    let mut merged = Postings::new();
    let mut postings1 = postings1.postings.iter();
    let mut postings2 = postings2.postings.iter();
    let mut p1 = postings1.next();
    let mut p2 = postings2.next();
    while let (Some(p1_), Some(p2_)) = (p1, p2) {
        if p1_.doc_id == p2_.doc_id {
            merged.add(p1_.doc_id, p1_.term_freq + p2_.term_freq);
            p1 = postings1.next();
            p2 = postings2.next()
        } else if p1_.doc_id < p2_.doc_id {
            p1 = postings1.next();
        } else {
            p2 = postings2.next();
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merging() {
        let mut postings1 = Postings::new();
        postings1.add(1, 2);
        postings1.add(2, 3);
        postings1.add(3, 4);
        postings1.add(6, 3);
        postings1.add(7, 5);
        postings1.add(8, 6);
        let mut postings2 = Postings::new();
        postings2.add(1, 3);
        postings2.add(3, 4);
        postings2.add(4, 5);
        postings2.add(7, 5);
        postings2.add(8, 6);

        let merged = merge_postings(postings1, postings2);
        assert_eq!(merged.postings, vec![Posting::new(1, 5),
                                         Posting::new(3, 8),
                                         Posting::new(7, 10),
                                         Posting::new(8, 12)]);
    }
}