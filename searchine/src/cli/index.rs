use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

use anyhow::Context;

use index::collection::*;
use index::doc::freq::DocumentFrequencyIndex;
use index::inverted::freq::FrequencyIndexer;

use loadocs::document::*;
use loadocs::read_to_string;

use tokenize::Tokenizer;

use crate::fs::*;

type Tokens = (usize, Vec<String>);

const CHANNEL_BOUND: usize = 1024;

/// Indexes the documents in the corpus.
pub fn invoke(
    repo_dir: impl AsRef<Path>,
    index_name: impl AsRef<Path>,
    verbose: bool,
) -> anyhow::Result<()> {
    let repo_dir = repo_dir.as_ref();

    // Instantiate tokenizer.
    let tokenizer = Tokenizer::default();

    // Instantiate corpus index.
    let dir_path = repo_dir
        .parent()
        .context(format!("Failed to get parent for: {}", repo_dir.display()))?;
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
        let mut doc_index = DocumentFrequencyIndex::new(document_id);
        doc_index.index_tokens(tokens);
        // Include the document index
        indexer.index(doc_index);
    }

    // Build index and store it to file.
    let index = indexer.build();
    index.write_to_file(repo_dir.join(index_name))?;
    let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x93, 0x8B]).unwrap_or_default();
    println_bold!("{emoji} Created index for: {}", dir_path.display());
    Ok(())
}

/// Part of a pipeline that loads documents.
fn load_docs<I>(
    paths: I,
    collection: CorpusIndex,
) -> (
    mpsc::Receiver<Document>,
    thread::JoinHandle<anyhow::Result<()>>,
)
where
    I: IntoIterator<Item=PathBuf> + Send + 'static,
{
    let (sender, receiver) = mpsc::sync_channel(CHANNEL_BOUND);
    let handle = thread::spawn(move || {
        for path in paths {
            // For each path, we map it to a document id with the collection index,
            // and load the document from file.
            let doc_id = collection
                .get_document_id(&path)
                .context(format!("Document {} is not in collection.", path.display()))?;
            println!("Loading document: {}", path.display());
            let document = Document::from_file(doc_id, &path)?;

            if sender.send(document).is_err() {
                eprintln!("Failed to read from {}", path.display());
            }
        }
        Ok(())
    });
    (receiver, handle)
}

fn tokenize_content(
    document_receiver: mpsc::Receiver<Document>,
) -> (
    mpsc::Receiver<Tokens>,
    thread::JoinHandle<anyhow::Result<()>>,
) {
    let (sender, receiver) = mpsc::sync_channel(CHANNEL_BOUND);
    let tokenizer = Tokenizer::default();
    let handle = thread::spawn(move || {
        for document in document_receiver {
            let tokens = (
                document.doc_id(),
                tokenizer.tokenize(document.page_content()),
            );
            if sender.send(tokens).is_err() {
                eprintln!("Failed to tokenize document {}", document.doc_id());
            }
        }
        Ok(())
    });
    (receiver, handle)
}

fn index_documents(
    tokens_receiver: mpsc::Receiver<(usize, Vec<String>)>,
) -> (
    mpsc::Receiver<DocumentFrequencyIndex>,
    thread::JoinHandle<anyhow::Result<()>>,
) {
    let (sender, receiver) = mpsc::sync_channel(CHANNEL_BOUND);
    let handle = thread::spawn(move || {
        for (doc_id, tokens) in tokens_receiver {
            let mut doc_index = DocumentFrequencyIndex::new(doc_id);
            doc_index.index_tokens(tokens);

            if sender.send(doc_index).is_err() {
                eprintln!("Failed to send index {}", doc_id)
            }
        }
        Ok(())
    });
    (receiver, handle)
}

pub fn invoke_par(
    repo_dir: impl AsRef<Path>,
    index_name: impl AsRef<Path>,
    verbose: bool,
) -> anyhow::Result<()> {
    // Get all paths
    let repo_dir = repo_dir.as_ref();
    let dir_path = repo_dir
        .parent()
        .context(format!("Failed to get parent for: {}", repo_dir.display()))?;
    let dir = Directory::new(dir_path)?;
    let dir = dir.iter_full_paths(verbose).collect::<BTreeSet<_>>();

    // // This is indexing collection from the scratch?
    let collection = CorpusIndex::from_paths(dir.clone())?;

    let (doc_receiver, h1) = load_docs(dir, collection);
    let (token_receiver, h2) = tokenize_content(doc_receiver);
    let (doc_index_receiver, h3) = index_documents(token_receiver);
    let mut indexer = FrequencyIndexer::new();
    for doc_index in doc_index_receiver {
        indexer.index(doc_index);
    }

    let r1 = h1.join().unwrap();
    let r2 = h2.join().unwrap();
    let r3 = h3.join().unwrap();

    r1?;
    r2?;
    r3?;

    // Build index and store it to file.
    let index = indexer.build();
    index.write_to_file(repo_dir.join(index_name))?;
    let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x93, 0x8B]).unwrap_or_default();
    println_bold!("{emoji} Created index for: {}", dir_path.display());

    Ok(())
}
