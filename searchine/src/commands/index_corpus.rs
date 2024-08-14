use std::collections::BTreeSet;
use std::io;
use std::path::Path;

use fingertips::fs::*;
use fingertips::index::corpus::*;

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
pub fn invoke(
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
