#[derive(Debug, PartialEq)]
struct Posting {
    doc_id: usize,
    term_freq: usize,
}

impl Posting {
    /// Creates a new `Posting` with the given document ID and term frequency.
    fn new(doc_id: usize, term_freq: usize) -> Self {
        Self { doc_id, term_freq }
    }
}

struct Postings {
    postings: Vec<Posting>,
}

impl Postings {
    /// Creates a new `Postings` with an empty list of postings.
    fn new() -> Self {
        Self { postings: Vec::new() }
    }

    /// Adds a posting to the postings, by specifying the document ID and term frequency.
    fn add(&mut self, doc_id: usize, term_freq: usize) {
        self.postings.push(Posting { doc_id, term_freq });
    }

    /// Adds a posting to the postings, by specifying the posting.
    fn add_posting(&mut self, posting: Posting) {
        self.postings.push(posting);
    }
}

fn merge_postings(postings1: Postings, postings2: Postings) -> Postings {
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
        println!("p1 = {:?}", p1);
        println!("p2 = {:?}", p2);
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
        let mut postings2 = Postings::new();
        postings2.add(1, 3);
        postings2.add(3, 4);
        postings2.add(4, 5);

        let merged = merge_postings(postings1, postings2);
        assert_eq!(merged.postings, vec![Posting::new(1, 5), Posting::new(3, 8)]);
    }
}