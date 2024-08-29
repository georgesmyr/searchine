use std::collections::BTreeSet;
use std::io::{self, Write};
use std::path::Path;

use tabwriter::TabWriter;
use fingertips::collection::*;

use crate::fs::Directory;

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
pub fn index(
    repo_dir: impl AsRef<Path>,
    corpus_index_file_name: impl AsRef<Path>,
) -> io::Result<()> {
    let repo_dir = repo_dir.as_ref();
    let dir_path = repo_dir.parent().expect("Could not derive directory path.");

    let dir = Directory::new(dir_path)?;
    let paths = dir.iter_full_paths().collect::<BTreeSet<_>>();
    let corpus_index = CorpusIndex::from_paths(paths)?;
    corpus_index.write_to_file(repo_dir.join(corpus_index_file_name))?;

    let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x93, 0x9A]).unwrap_or_default();
    println_bold!("{} Indexed corpus at: {}", emoji, repo_dir.display());

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
pub fn list(
    repo_dir: impl AsRef<Path>,
    corpus_index_file_name: impl AsRef<Path>,
) -> io::Result<()> {
    let index_path = repo_dir.as_ref().join(corpus_index_file_name);
    let base_path = repo_dir.as_ref().parent().unwrap();
    let corpus_index = CorpusIndex::from_file(index_path)?
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
            entry.document_id(),
            entry.modified()
        );
    }
    tab_writer.flush()?;
    Ok(())
}
