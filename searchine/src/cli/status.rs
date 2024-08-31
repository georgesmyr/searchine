use std::path::{Path, PathBuf};

use anyhow::Context;
use index::collection::CorpusIndex;

use crate::cli::utils::{fetch_modified_files, fetch_new_files, fetch_removed_files};
use crate::fs::Directory;

/// Checks for new files, removed files, and modified files.
pub fn invoke(
    repo_path: impl AsRef<Path>,
    index_file_name: &str,
    verbose: bool,
) -> anyhow::Result<()> {
    let repo_path = repo_path.as_ref();
    let index_path = repo_path.join(index_file_name);

    let corpus_index = CorpusIndex::from_file(&index_path).context(format!(
        "Could not read index file: {}",
        index_path.display()
    ))?;

    let dir_path = repo_path.parent().context(format!(
        "Could not get parent directory of the repo {}",
        repo_path.display()
    ))?;
    let dir = Directory::new(dir_path)?;
    let dir = dir.iter_full_paths(verbose).collect::<Vec<_>>();

    // Get the paths that are in the directory but not in the index.
    // Add them to the corpus index
    let new_paths = fetch_new_files(&corpus_index, &dir);

    // Get the paths that are in the index but not in the directory.
    // Remove them from the corpus index
    let removed_paths = fetch_removed_files(&corpus_index, &dir);

    // Get the paths that are both in the directory and in the index,
    // but they have different modified times.
    let modified_paths = fetch_modified_files(&corpus_index, &dir)?;

    // Display updates
    if new_paths.is_empty() && removed_paths.is_empty() && modified_paths.is_empty() {
        let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x91, 0x8C]).unwrap_or_default();
        println_bold!("{emoji} No changes detected. Index is up to date.");
    } else {
        display_removed_files(&removed_paths);
        display_new_files(&new_paths);
        display_modified_files(&modified_paths);
    }
    Ok(())
}

fn display_removed_files(removed_files: &Vec<PathBuf>) {
    for removed_file in removed_files {
        let emoji = String::from_utf8(vec![0xE2, 0x9C, 0x96]).unwrap_or_default();
        println_bold!("Documents removed from the index:");
        println_bold!("  (use \"searchine re-index\" to update the index)");
        println_red!("  {emoji} removed: {}", removed_file.display());
    }
    println!();
}

fn display_new_files(new_files: &Vec<PathBuf>) {
    for new_file in new_files {
        let emoji = String::from_utf8(vec![0xE2, 0x9C, 0x94]).unwrap_or_default();
        println_bold!("Documents to be added to the index:");
        println_bold!("  (use \"searchine re-index\" to update the index)");
        println_green!("  {emoji} added: {}", new_file.display());
    }
    println!();
}

fn display_modified_files(modified_files: &Vec<PathBuf>) {
    for modified_file in modified_files {
        let emoji = String::from_utf8(vec![0xE2, 0x9C, 0x8F]).unwrap_or_default();
        println_bold!("Documents to be updated in the index:");
        println_bold!("  (use \"searchine re-index\" to update the index)");
        println_red!("  {emoji} modified: {}", modified_file.display());
    }
    println!();
}
