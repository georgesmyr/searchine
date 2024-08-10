use std::collections::BTreeSet;
use std::io;
use std::path::Path;

use crate::fs::Directory;
use crate::index::corpus::CorpusIndex;
use crate::index::im::{InMemoryDocumentIndexer, InMemoryIndex};
use crate::path::get_relative_path;
use crate::postings::FrequencyPosting;
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
/// corpus-index.
pub fn list_docs(
    repo_dir: impl AsRef<Path>,
    corpus_index_file_name: impl AsRef<Path>,
) -> io::Result<()> {
    let index_path = repo_dir.as_ref().join(corpus_index_file_name);
    let base_path = repo_dir.as_ref().parent().unwrap();

    let mut corpus_index = CorpusIndex::from_file(index_path)?
        .into_iter()
        .collect::<BTreeSet<_>>();

    let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x93, 0x9A]).unwrap_or_default();
    println!(
        "{} Documents in the corpus: {:?}",
        emoji,
        corpus_index.len()
    );
    for (path, entry) in corpus_index {
        println!(
            "{} {} : {} : {:?}",
            emoji,
            path.display(),
            entry.document_id,
            entry.modified
        );
    }
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
    let mut vocab = Vocabulary::new();

    // For each directory entry, read the file and tokenize the content.
    // Add the tokens to the vocabulary.
    let repo_dir = repo_dir.as_ref();
    let base_dir = repo_dir.parent().unwrap_or_else(|| {
        panic!("Could not find parent directory of repo path: {}", repo_dir.display());
    });
    println!("Creating vocabulary from: {}\n", base_dir.display());
    let dir = Directory::new(base_dir)?;
    let dir = dir.iter_full_paths().collect::<BTreeSet<_>>();
    for path in dir {
        let rel_path = get_relative_path(&path, &base_dir).unwrap();
        println!("- Parsing document: {}", rel_path.display());

        let content = crate::fs::docs::read_to_string(&path)?;
        let tokens = tokenizer.tokenize(&content);
        vocab.add_tokens(&tokens);
    }

    // Write the vocabulary to the output file.
    let output_path = repo_dir.join(vocabulary_file_name);
    println!("\nWriting vocabulary to: {}", output_path.display());
    vocab.write_to_disk(output_path);
    Ok(())
}

// pub fn index(dir_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> io::Result<()> {
//     let vocab = Vocabulary::from_file("vocab.json")?;
//     let encoder = Encoder::from(vocab);
//     let tokenizer = Builder::default().with_encoder(encoder).build();
//     let mut index = InMemoryIndex::<FrequencyPosting>::new();
//
//     // Read the directory
//     let dir = std::fs::read_dir(dir_path)?;
//     let dir = dir.collect::<Result<Vec<_>, _>>()?;
//     let corpus_index = crate::index::corpus::CorpusIndex::try_from(dir)?;
//     for (path, entry) in &corpus_index {
//         let content = crate::fs::Document::read_to_string(&path)?;
//         let tokens = tokenizer.tokenize(&content);
//         let document_id = corpus_index.get_document_id(&path).unwrap();
//         let mut doc_indexer = InMemoryDocumentIndexer::<FrequencyPosting>::new(document_id);
//         doc_indexer.index_tokens(tokens);
//         let doc_index = doc_indexer.finalize();
//         index.insert(doc_index);
//     }
//
//     for (doc_id, doc_index) in index.index {
//         println!(
//             "Document ID: {} -> Index Length: {}",
//             doc_id,
//             doc_index.n_terms()
//         );
//     }
//     // index.write_to_disk("index.json")?;
//     Ok(())
// }
