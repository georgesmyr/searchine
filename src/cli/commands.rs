use std::collections::BTreeSet;
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

use tabwriter::TabWriter;
use rayon::prelude::*;

use crate::fs::Directory;
use crate::index::corpus::CorpusIndex;
use crate::index::im::{InMemoryDocumentIndexer, InMemoryIndex};
use crate::tokenize::{Builder, Encoder, Vocabulary};

/// Initializes a new searchine index repo.
///
/// # Arguments
///
/// * `dir_path` - The path to the directory where the index will be created.
/// * `searchine_path` - The path to the searchine index directory relative to `dir_path`.
pub fn init(dir_path: impl AsRef<Path>, searchine_path: impl AsRef<Path>) -> io::Result<()> {
    let dir_path = dir_path.as_ref();
    let index_path = dir_path.join(searchine_path);
    std::fs::create_dir_all(&index_path)?;
    let full_index_path = std::fs::canonicalize(&index_path)?;
    println!("Index created at: {}", full_index_path.display());
    Ok(())
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
/// * `repo_dir` - The path to the searchine repository.
/// * `corpus_index_file_name` - The name of the file where the corpus index will be written.
pub fn index_corpus(
    repo_dir: impl AsRef<Path>,
    corpus_index_file_name: impl AsRef<Path>,
) -> io::Result<()> {
    let repo_dir = repo_dir.as_ref();
    let dir_path = repo_dir.parent().expect("Could not derive directory path.");

    let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x93, 0x81]).unwrap_or_default();
    println!("{} Indexing corpus at: {}", emoji, repo_dir.display());

    let dir = Directory::new(dir_path)?;
    let paths = dir.iter_full_paths().collect::<BTreeSet<_>>();
    let corpus_index = CorpusIndex::from_paths(paths)?;
    corpus_index.write_to_file(repo_dir.join(corpus_index_file_name))?;
    Ok(())
}

/// Lists the indexed documents in the corpus, which are listed in the
/// corpus-index. The result is printed to the standard output as a table.
/// The first column is the path to the document, the second column is the
/// document ID, and the third column is the last modified time.
///
/// # Arguments
///
/// * `repo_dir` - The path to the searchine repository.
/// * `corpus_index_file_name` - The name of the file where the corpus index is stored.
///
/// # Errors
///
/// Returns an error if the corpus index file cannot be read.
pub fn list_docs(
    repo_dir: impl AsRef<Path>,
    corpus_index_file_name: impl AsRef<Path>,
) -> io::Result<()> {
    let index_path = repo_dir.as_ref().join(corpus_index_file_name);
    let base_path = repo_dir.as_ref().parent().unwrap();
    let mut corpus_index = CorpusIndex::from_file(index_path)?
        .into_iter()
        .collect::<BTreeSet<_>>();

    // Print out the indexed documents.
    let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x93, 0x9A]).unwrap_or_default();
    println!(
        "{} Documents in the corpus: {}\n",
        emoji,
        corpus_index.len()
    );
    let mut tab_writer = TabWriter::new(io::stdout());
    _ = writeln!(
        tab_writer,
        "\t{}\t{}\t{}",
        "Path", "Document ID", "Last Modified"
    );
    for (path, entry) in corpus_index {
        _ = writeln!(
            tab_writer,
            "\t{}\t{}\t{:?}",
            path.display(),
            entry.document_id,
            entry.modified
        );
    }
    tab_writer.flush()?;
    Ok(())
}

/// Creates a vocabulary from a directory of documents.
///
/// Each document is parsed to a string and tokenized. All the tokens
/// are collected and assigned a token ID.
///
/// # Arguments
///
/// * `repo_dir` - The path to the directory containing the documents.
/// * `vocabulary_file_name` - The file name where the vocabulary will be written.
pub fn create_vocabulary(
    repo_dir: impl AsRef<Path>,
    vocabulary_file_name: impl AsRef<Path>,
) -> io::Result<()> {
    // Initialize tokenizer and vocabulary.
    let tokenizer = Builder::default().build();
    let mut vocab = Arc::new(Mutex::new(Vocabulary::new()));

    // For each directory entry, read the file and tokenize the content.
    // Add the tokens to the vocabulary.
    let repo_dir = repo_dir.as_ref();
    let base_dir = repo_dir.parent().unwrap_or_else(|| {
        panic!(
            "Could not find parent directory of repo path: {}",
            repo_dir.display()
        );
    });
    println!("Creating vocabulary from: {}\n", base_dir.display());
    let dir = Directory::new(base_dir)?;
    let dir = dir.iter_full_paths().collect::<BTreeSet<_>>();
    dir.par_iter().for_each(|path| {
        let content = crate::fs::read_to_string(&path).unwrap();
        let tokens = tokenizer.tokenize(&content);
        let mut vocab = vocab.lock().unwrap();
        vocab.add_tokens(&tokens);
    });

    // Write the vocabulary to the output file.
    let output_path = repo_dir.join(vocabulary_file_name);
    println!("\nWriting vocabulary to: {}", output_path.display());
    let vocab = Arc::try_unwrap(vocab).expect("").into_inner().unwrap();
    vocab.write_to_disk(output_path);
    Ok(())
}

/// Indexes the documents in the corpus.
pub fn index(repo_dir: impl AsRef<Path>, index_name: impl AsRef<Path>) -> io::Result<()> {
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
        let content = crate::fs::read_to_string(&path).unwrap();
        let tokens = tokenizer.tokenize(&content);
        let document_id = corpus_index.get_document_id(&path).unwrap();
        let mut doc_indexer = InMemoryDocumentIndexer::new(document_id);
        doc_indexer.index_tokens(tokens);
        let doc_index = doc_indexer.finalize();
        println!("Indexed doc: {:?}", doc_index);
        index.insert(document_id, doc_index);
    };

    index.write_to_disk(repo_dir.join(index_name));
    Ok(())
}

pub fn find(repo_dir: impl AsRef<Path>, query: &str, top_n: usize) -> io::Result<()> {
    let repo_dir = repo_dir.as_ref();

    // let corpus_index_path = repo_dir.join("corpus-index.json");
    // let corpus_index = CorpusIndex::from_file(corpus_index_path)?;

    let vocabulary_path = repo_dir.join("vocabulary.json");
    let vocabulary = Vocabulary::from_file(vocabulary_path)?;
    let encoder = Encoder::from(vocabulary);
    let tokenizer = Builder::default().with_encoder(encoder).build();
    let query_tokens = tokenizer.tokenize(query);
    println!("Query tokens: {:?}", query_tokens);

    // let index_path = repo_dir.join("index.json");
    // let index = InMemoryIndex::<FrequencyPosting>::from_file(index_path)?;


    Ok(())
}