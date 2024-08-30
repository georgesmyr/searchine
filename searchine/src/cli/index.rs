use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Context;

use tokenize::Tokenizer;
use loadocs::read_to_string;
use fingertips::collection::*;
use fingertips::inverted::freq::FrequencyIndexer;
use fingertips::doc::freq::DocumentFrequencyIndexer;

use crate::fs::*;

/// Indexes the documents in the corpus.
pub fn invoke(repo_dir: impl AsRef<Path>, index_name: impl AsRef<Path>, verbose: bool) -> anyhow::Result<()> {
    let repo_dir = repo_dir.as_ref();

    // Instantiate tokenizer.
    let tokenizer = Tokenizer::default();

    // Instantiate corpus index.
    let dir_path = repo_dir.parent().context(format!("Failed to get parent for: {}", repo_dir.display()))?;
    let dir = Directory::new(dir_path)?;
    let dir = dir.iter_full_paths(verbose).collect::<BTreeSet<_>>();
    let corpus_index = CorpusIndex::from_paths(dir)?;

    let mut indexer = FrequencyIndexer::new();
    for (path, _) in &corpus_index {
        // Get document ID
        // println!("Indexing {}", path.display());
        let document_id = corpus_index
            .get_document_id(path)
            .context("Failed to get document ID")?;
        // Load document
        let content = read_to_string(path).expect("Failed to parse document.");
        // Tokenize document
        let tokens = tokenizer.tokenize(&content);
        // Instantiate a document indexer with specified document ID, and index tokens
        let mut doc_indexer = DocumentFrequencyIndexer::new(document_id);
        doc_indexer.index_tokens(tokens);
        // Build document index
        let doc_index = doc_indexer.build();
        // Include the document index
        indexer.index(doc_index);
    }

    // Build index and store it to file.
    let index = indexer.build();
    index.to_file(repo_dir.join(index_name))?;
    let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x93, 0x8B]).unwrap_or_default();
    println_bold!("{emoji} Created index for: {}", dir_path.display());
    Ok(())
}
