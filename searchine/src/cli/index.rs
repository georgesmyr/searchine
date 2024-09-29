use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{sync_channel, Receiver};
use std::thread::{spawn, JoinHandle};

use anyhow::Context;

use documents::{Document, DocumentId};
use index::collection::*;
use index::doc::freq::DocumentFrequencyIndex;
use index::inverted::freq::FrequencyIndex;
use tokenize::{Token, Tokenizer};

use crate::config::{CHANNEL_BOUND, INDEX_FILENAME, VOCABULARY_FILENAME};
use crate::fs::Directory;

type TokenizedDocument = (DocumentId, Vec<Token>);

/// Part of a pipeline that loads documents. Each document in the collection is loaded
/// and sent over a channel for tokenizing.
fn load_docs<I>(
    paths: I,
    collection: Collection,
) -> (Receiver<Document>, JoinHandle<anyhow::Result<()>>)
where
    I: IntoIterator<Item = PathBuf> + Send + 'static,
{
    let (sender, receiver) = sync_channel(CHANNEL_BOUND);
    let handle = spawn(move || {
        for path in paths {
            // For each path, we map it to a document id with the collection index,
            // and load the document from file.
            let doc_id = collection
                .get_document_id(&path)
                .context(format!("Document {} is not in collection.", path.display()))?;
            // println!("Loading document: {}", path.display());
            let document = Document::from_file(doc_id, &path)?;

            if sender.send(document).is_err() {
                eprintln!("Failed to read from {}", path.display());
            }
        }
        Ok(())
    });
    (receiver, handle)
}

/// Part of a pipeline that tokenizes the contents of a document. Each received
/// document is broken into tokens. The document ID and the stream of tokens are
/// sent over a channel to a thread that indexes the documents.
fn tokenize_content(
    document_receiver: Receiver<Document>,
) -> (
    Receiver<TokenizedDocument>,
    JoinHandle<anyhow::Result<Tokenizer>>,
) {
    let (sender, receiver) = sync_channel(CHANNEL_BOUND);
    let mut tokenizer = Tokenizer::default();
    let handle = spawn(move || {
        for document in document_receiver {
            let tokens = (
                document.doc_id(),
                tokenizer.tokenize(document.page_content()),
            );
            if sender.send(tokens).is_err() {
                eprintln!("Failed to tokenize document {}", document.doc_id());
            }
        }
        Ok(tokenizer)
    });
    (receiver, handle)
}

/// Part of a pipeline that creates an index for each document. The received
/// pair of document ID and token stream are turned into an index. The resulting
/// document indices are sent over a channel and joint in the main thread that
/// merges them into an overall index.
fn index_documents(
    tokens_receiver: Receiver<(u32, Vec<Token>)>,
) -> (
    Receiver<DocumentFrequencyIndex>,
    JoinHandle<anyhow::Result<()>>,
) {
    let (sender, receiver) = sync_channel(CHANNEL_BOUND);
    let handle = spawn(move || {
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

/// Indexes a directory of documents with a pipeline.
pub fn invoke_par(repo_dir: impl AsRef<Path>, verbose: bool) -> anyhow::Result<()> {
    // Get all paths
    let repo_dir = repo_dir.as_ref();
    let dir_path = repo_dir
        .parent()
        .context(format!("Failed to get parent for: {}", repo_dir.display()))?;
    let dir = Directory::new(dir_path)?;
    let dir = dir.iter_full_paths(verbose).collect::<BTreeSet<_>>();

    // This is indexing collection from the scratch?
    let collection = Collection::from_paths(dir.clone())?;

    let (doc_receiver, h1) = load_docs(dir, collection);
    let (token_receiver, h2) = tokenize_content(doc_receiver);
    let (doc_index_receiver, h3) = index_documents(token_receiver);
    let mut index = FrequencyIndex::new();
    for doc_index in doc_index_receiver {
        index.index(doc_index);
    }

    let r1 = h1.join().unwrap();
    let r2 = h2.join().unwrap();
    let r3 = h3.join().unwrap();

    r1?;
    let tokenizer = r2?;
    tokenizer.into_file(repo_dir.join(VOCABULARY_FILENAME))?;
    r3?;

    // Build index and store it to file.
    index.into_file(repo_dir.join(INDEX_FILENAME))?;
    let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x93, 0x8B]).unwrap_or_default();
    println_bold!("{emoji} Created index for: {}", dir_path.display());

    Ok(())
}
