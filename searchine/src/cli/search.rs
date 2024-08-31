use std::io::{self, Write};
use std::path::Path;

use tokenize::{Tokenizer, TokenCounts};
use index::collection::InvertedCollection;
use index::inverted::freq::FrequencyIndex;
use index::inverted::Index;
use index::score::*;


pub fn invoke(repo_dir: impl AsRef<Path>, query: &str, top_n: usize) -> anyhow::Result<()> {
    let repo_dir = repo_dir.as_ref();

    // Instantiate tokenizer.
    let tokenizer = Tokenizer::default();
    // Tokenize the query.
    let query_tokens = tokenizer.tokenize(query);
    let query = TokenCounts::from(query_tokens);

    // Load inverted index from file.
    let index_path = repo_dir.join("index.json");
    let index = FrequencyIndex::from_file(index_path)?;

    // Iterate over all tokens in the query and score the documents they appear in.
    let mut scores = DocumentsScores::new();
    for (term, count) in query {
        for doc_id in index.doc_ids_containing(&term) {
            let score = (count as f64) * index.calc_tf_idf(doc_id, &term);
            scores.add_score(doc_id, score);
        }
    }
    let top_n_results = scores.get_top_n(top_n);
    println!("{:?}", top_n_results);

    let collection_path = repo_dir.join("collection.json");
    let inv_collection = InvertedCollection::from_file(collection_path)?;
    let top_n_results = top_n_results
        .iter()
        .map(|(doc_id, score)| (inv_collection.get_path(**doc_id).unwrap(), *score))
        .collect::<Vec<_>>();

    let mut tw = tabwriter::TabWriter::new(io::stdout()).padding(2);
    writeln!(tw, "\tNo\tPath\tScore")?;
    for (i, (path, score)) in top_n_results.iter().enumerate() {
        writeln!(tw, "\t{i}\t{}\t{score}", path.display())?;
    }
    tw.flush()?;

    Ok(())
}
