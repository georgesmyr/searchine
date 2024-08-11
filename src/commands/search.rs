use std::io;
use std::io::Write;
use std::path::Path;

use crate::index::corpus::*;
use crate::index::im::*;
use crate::scores::*;
use crate::tokenize::*;

pub fn invoke(repo_dir: impl AsRef<Path>, query: &str, top_n: usize) -> io::Result<()> {
    let repo_dir = repo_dir.as_ref();

    let vocabulary_path = repo_dir.join("vocabulary.json");
    let vocabulary = Vocabulary::from_file(vocabulary_path)?;
    let encoder = Encoder::from(vocabulary);
    let tokenizer = Builder::default().with_encoder(encoder).build();
    let query_tokens = tokenizer.tokenize(query);

    let index_path = repo_dir.join("index.json");
    let index = InMemoryIndex::from_file(index_path)?;

    let mut scores: Vec<(usize, f64)> = Vec::with_capacity(index.n_docs());
    for doc in index.index.keys() {
        let mut score: f64 = 0.0;
        for token in &query_tokens {
            let n_docs_containing = index.n_docs_containing(token);
            let n_docs = index.n_docs();
            let idf = calc_idf(n_docs_containing, n_docs);

            let term_count = index.index[doc].term_count(token);
            let total_count = index.index[doc].count();
            let tf = calc_tf(term_count, total_count);

            score += calc_tf_idf(tf, idf);
        }
        scores.push((*doc, score));
    }
    let top_n_results = get_top_n(scores, top_n);

    let corpus_index_path = repo_dir.join("corpus_index.json");
    let inv_corpus_index = InvertedCorpusIndex::from_file(corpus_index_path)?;
    let top_n_results = top_n_results
        .iter()
        .map(|(doc, score)| (inv_corpus_index.get_path(*doc).unwrap(), *score))
        .collect::<Vec<_>>();

    let mut tw = tabwriter::TabWriter::new(io::stdout()).padding(2);
    writeln!(tw, "\t{}\t{}\t{}", "No.", "Path", "Score")?;
    for (i, (path, score)) in top_n_results.iter().enumerate() {
        writeln!(tw, "\t{}\t{}\t{}", i, path.display(), score)?;
    }
    tw.flush()?;

    Ok(())
}

/// Takes the top n elements from a vector of elements. Edit this to reach
/// an optimized result.
fn get_top_n(mut elements: Vec<(usize, f64)>, top_n: usize) -> Vec<(usize, f64)> {
    elements.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    elements.into_iter().take(top_n).collect()
}
