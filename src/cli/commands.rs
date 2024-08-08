use crate::index::corpus::CorpusIndex;
use crate::index::im::{InMemoryDocumentIndexer, InMemoryIndexer};
use crate::postings::FrequencyPosting;
use crate::tokenize::{Builder, Encoder, Vocabulary};
use std::io;
use std::path::Path;

/// Initializes a new searchine index repo.
///
/// # Arguments
///
/// * `dir_path` - The path to the directory where the index will be created.
/// * `searchine_path` - The path to the searchine index directory relative to `dir_path`.
///
/// # Examples
///
/// ```no_run
/// use searchine::cli::commands;
/// commands::init("/path/to/dir", ".searchine");
/// ```
pub fn init(dir_path: impl AsRef<Path>, searchine_path: impl AsRef<Path>) -> io::Result<()> {
    let dir_path = dir_path.as_ref();
    let index_path = dir_path.join(searchine_path);

    if index_path.exists() {
        let full_index_path = std::fs::canonicalize(&index_path)?;
        println!("Index already exists at: {}", full_index_path.display());
    } else {
        std::fs::create_dir_all(&index_path)?;
        let full_index_path = std::fs::canonicalize(&index_path)?;
        println!("Index created at: {}", full_index_path.display());
    }
    Ok(())
}

/// Checks if a searchine index exists at the specified path.
///
/// # Arguments
///
/// * `dir_path` - The path to the directory where the index is located.
/// * `searchine_path` - The path to the searchine index directory relative to `dir_path`.
pub fn repo_exists(dir_path: impl AsRef<Path>, searchine_path: impl AsRef<Path>) -> bool {
    let dir_path = dir_path.as_ref();
    let index_path = dir_path.join(searchine_path);
    index_path.exists()
}

/// Indexes a corpus of documents.
///
/// The corpus is a list of documents in the directory. Each document
/// is assigned a document ID and the last time the document was indexed.
///
/// The index is then used as a cache of the up to date indexed documents.
/// If the last modified time of the document is later than the last indexing
/// time, then the index is out of date.
///
/// # Arguments
///
/// * `dir_path` - The path to the directory containing the corpus.
/// * `searchine_path` - The path to the searchine index directory relative to `dir_path`.
/// * `corpus_index_file_name` - The name of the file where the corpus index will be written.
pub fn index_corpus(
    dir_path: impl AsRef<Path>,
    searchine_path: impl AsRef<Path>,
    corpus_index_file_name: impl AsRef<Path>,
) -> io::Result<()> {
    let dir_path = dir_path.as_ref();
    let output_path = dir_path.join(searchine_path).join(corpus_index_file_name);

    let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x93, 0x81]);
    if let Ok(emoji) = emoji {
        println!("{}  Indexing corpus at: {}", emoji, dir_path.display());
    } else {
        println!("Indexing corpus at: {}", dir_path.display());
    }

    let dir = std::fs::read_dir(dir_path)?;
    let dir = dir.collect::<Result<Vec<_>, _>>()?;
    let corpus_index = CorpusIndex::try_from(dir)?;
    corpus_index.write_to_file(output_path)?;
    Ok(())
}

/// Lists the ...
pub fn list_corpus(
    dir_path: impl AsRef<Path>,
    searchine_path: impl AsRef<Path>,
    corpus_index_file_name: impl AsRef<Path>,
) -> io::Result<()> {
    let dir = std::fs::read_dir(dir_path.as_ref())?;
    let dir = dir.collect::<Result<Vec<_>, _>>()?;
    let corpus_index = CorpusIndex::try_from(dir)?;
    let corpus_index = corpus_index.into_iter().collect::<Vec<_>>();
    for (path, entry) in corpus_index {
        println!(
            "{} : {} : {:?}",
            path.display(),
            entry.document_id,
            entry.modified
        );
    }
    Ok(())
}

// pub fn status(corpus_path: impl AsRef<Path>) -> io::Result<()> {
//     if corpus_path.as_ref().exists() {
//         let corpus_index = CorpusIndex::read_from_disk(corpus_path)?;
//         for (path, entry) in corpus_index {}
//     }
// }

/// Creates a vocabulary from a directory of documents.
///
/// Each document is parsed to a string and tokenized. All the tokens
/// are collected and assigned a token ID.
///
/// # Arguments
///
/// * `dir_path` - The path to the directory containing the documents.
/// * `output_path` - The path to the output file where the vocabulary will be written.
pub fn create_vocabulary(
    dir_path: impl AsRef<Path>,
    searchine_path: impl AsRef<Path>,
    vocabulary_file_name: impl AsRef<Path>,
) -> io::Result<()> {
    // Read the directory and create a tokenizer.
    let dir = std::fs::read_dir(dir_path.as_ref())?;
    let tokenizer = Builder::default().build();
    let mut vocab = Vocabulary::new();

    // For each directory entry, read the file and tokenize the content.
    // Add the tokens to the vocabulary.
    for dir_entry in dir {
        let path = dir_entry?.path();
        println!("Parsing document: {}", path.display());
        let content = std::fs::read_to_string(&path)?;
        let tokens = tokenizer.tokenize(&content);
        vocab.add_tokens(&tokens);
    }

    // Write the vocabulary to the output file.
    let output_path = dir_path
        .as_ref()
        .join(searchine_path)
        .join(vocabulary_file_name);
    vocab.write_to_disk(&output_path);
    Ok(())
}

pub fn index(dir_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> io::Result<()> {
    let vocab = Vocabulary::from_file("vocab.json")?;
    let encoder = Encoder::from(vocab);
    let tokenizer = Builder::default().with_encoder(encoder).build();
    let mut indexer = InMemoryIndexer::<FrequencyPosting>::new();

    // Read the directory
    let dir = std::fs::read_dir(dir_path)?;
    let dir = dir.collect::<Result<Vec<_>, _>>()?;
    let corpus_index = crate::index::corpus::CorpusIndex::try_from(dir)?;
    for (path, entry) in &corpus_index {
        let content = crate::fs::Document::read_to_string(&path)?;
        let tokens = tokenizer.tokenize(&content);
        let document_id = corpus_index.get_document_id(&path).unwrap();
        let mut doc_indexer = InMemoryDocumentIndexer::<FrequencyPosting>::new(document_id);
        doc_indexer.index_tokens(tokens);
        let doc_index = doc_indexer.finalize();
        indexer.insert(document_id, doc_index);
    }

    let index = indexer.finalize();
    for (doc_id, doc_index) in index.index {
        println!(
            "Document ID: {} -> Index Length: {}",
            doc_id,
            doc_index.n_terms()
        );
    }
    // index.write_to_disk("index.json")?;
    Ok(())
}
