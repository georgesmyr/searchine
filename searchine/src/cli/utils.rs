use std::path::PathBuf;

use anyhow::Context;

use index::collection::CorpusIndex;

/// Fetches the paths of the files that have been removed from the directory,
/// compared to the corpus index.
pub fn fetch_removed_files(corpus_index: &CorpusIndex, dir: &[PathBuf]) -> Vec<PathBuf> {
    let mut removed_paths = vec![];
    for (index_path, _) in corpus_index {
        if !dir.contains(index_path) {
            removed_paths.push(index_path.clone());
        }
    }
    removed_paths
}

/// Fetches the paths of the files that have been added to the directory,
/// compared to the corpus index.
pub fn fetch_new_files(corpus_index: &CorpusIndex, dir: &[PathBuf]) -> Vec<PathBuf> {
    let mut new_paths = vec![];
    for entry in dir {
        if !corpus_index.contains_path(entry) {
            new_paths.push(entry.clone());
        }
    }
    new_paths
}

/// Fetches the paths of the files that have been modified in the directory,
/// compared to the corpus index.
pub fn fetch_modified_files(
    corpus_index: &CorpusIndex,
    dir: &[PathBuf],
) -> anyhow::Result<Vec<PathBuf>> {
    let mut modified_paths = vec![];
    for path in dir {
        if corpus_index.contains_path(path) {
            let metadata = path
                .metadata()
                .context(format!("Failed to get metadata for: {}.", path.display()))?;
            let current_modified = metadata.modified().context(format!(
                "Failed to get last modified time for: {}.",
                path.display()
            ))?;
            let index_modified = corpus_index.get_last_modified(path).context(format!(
                "Failed to get last modified time for: {} from the index.",
                path.display()
            ))?;
            if current_modified > index_modified {
                modified_paths.push(path.clone());
            }
        }
    }
    Ok(modified_paths)
}
