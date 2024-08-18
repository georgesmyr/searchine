use fingertips::index::corpus::CorpusIndex;

pub mod create_vocabulary;
pub mod index;
pub mod index_corpus;
pub mod init;
pub mod list_corpus;
pub mod search;
pub mod status;

use std::path::PathBuf;

/// Fetches the paths of the files that have been removed from the directory,
/// compared to the corpus index.
pub fn fetch_removed_files(corpus_index: &CorpusIndex, dir: &Vec<PathBuf>) -> Vec<PathBuf> {
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
pub fn fetch_new_files(corpus_index: &CorpusIndex, dir: &Vec<PathBuf>) -> Vec<PathBuf> {
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
pub fn fetch_modified_files(corpus_index: &CorpusIndex, dir: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut modified_paths = vec![];
    for path in dir {
        if corpus_index.contains_path(path) {
            let metadata = path
                .metadata()
                .expect(format!("Failed to get metadata for: {}.", path.display()).as_str());
            let current_modified = metadata.modified().expect(
                format!("Failed to get last modified time for: {}.", path.display()).as_str(),
            );
            let index_modified = corpus_index.get_last_modified(path).expect(
                format!(
                    "Failed to get last modified time for: {} from the index.",
                    path.display()
                )
                    .as_str(),
            );
            if current_modified > index_modified {
                modified_paths.push(path.clone());
            }
        }
    }
    modified_paths
}
