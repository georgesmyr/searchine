use std::collections::BTreeSet;
use std::io;
use std::path::Path;

use fingertips::fs::*;
use fingertips::index::corpus::*;
use fingertips::index::im::*;
use fingertips::tokenize::*;

/// Indexes the documents in the corpus.
pub fn invoke(repo_dir: impl AsRef<Path>, index_name: impl AsRef<Path>) -> io::Result<()> {
    let repo_dir = repo_dir.as_ref();
    let vocab_path = repo_dir.join("vocabulary.json");
    let vocabulary = Vocabulary::from_file(vocab_path)?;
    let encoder = Encoder::from(vocabulary);
    let tokenizer = Builder::default().with_encoder(encoder).build();

    let dir_path = repo_dir.parent().unwrap();
    let dir = Directory::new(dir_path)?;
    let dir = dir.iter_full_paths().collect::<BTreeSet<_>>();
    let corpus_index = CorpusIndex::from_paths(dir)?;

    let mut index = InMemoryIndex::new();
    for (path, _) in &corpus_index {
        let content = read_to_string(&path).unwrap();
        let tokens = tokenizer.tokenize(&content);
        let mut doc_indexer = InMemoryDocumentIndexer::new();
        doc_indexer.index_tokens(tokens);
        let doc_index = doc_indexer.finalize();
        let document_id = corpus_index.get_document_id(&path).unwrap();
        index.insert(document_id, doc_index);
    }
    index.write_to_disk(repo_dir.join(index_name));
    let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x93, 0x8B]).unwrap_or_default();
    println_bold!("{emoji} Created index for: {}", dir_path.display());
    Ok(())
}
