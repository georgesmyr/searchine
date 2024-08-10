use std::collections::BTreeSet;
use std::io::{self, Write};
use std::path::Path;

use tabwriter::TabWriter;

use crate::index::corpus::*;

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
pub fn invoke(
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
