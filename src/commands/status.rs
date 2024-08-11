use std::io;
use std::path::{Path, PathBuf};

use crate::fs::Directory;
use crate::index::corpus::CorpusIndex;
use crate::fmt::*;

use crate::commands::{fetch_modified_files, fetch_new_files, fetch_removed_files};

pub fn invoke(repo_path: impl AsRef<Path>, index_file_name: &str) -> io::Result<()> {
    let repo_path = repo_path.as_ref();
    let index_path = repo_path.join(index_file_name);

    let mut corpus_index = CorpusIndex::from_file(&index_path)
        .expect(format!("Could not read index file: {}", index_path.display()).as_str());

    let dir_path = repo_path.parent().expect(
        format!(
            "Could not get parent directory of the repo {}",
            repo_path.display()
        )
            .as_str(),
    );
    let dir = Directory::new(dir_path)?;
    let dir = dir.iter_full_paths().collect::<Vec<_>>();

    // Get the paths that are in the directory but not in the index.
    // Add them to the corpus index
    // TODO: update the index.
    let new_paths = fetch_new_files(&corpus_index, &dir);

    // Get the paths that are in the index but not in the directory.
    // Remove them from the corpus index
    // TODO: update the index.
    let removed_paths = fetch_removed_files(&corpus_index, &dir);

    // Get the paths that are both in the directory and in the index,
    // but they have different modified times.
    // TODO: update the index.
    let modified_paths = fetch_modified_files(&corpus_index, &dir);

    // Display updates
    if new_paths.is_empty() && removed_paths.is_empty() && modified_paths.is_empty() {
        println!("{}", "No changes detected. Corpus-index up to date.");
    } else {
        display_removed_files(&removed_paths);
        display_new_files(&new_paths);
        display_modified_files(&modified_paths);
    }
    Ok(())
}


fn display_removed_files(removed_files: &Vec<PathBuf>) {
    for removed_file in removed_files {
        println!("{}", fmt_red(format!("\t\tremoved: {}", removed_file.display()).as_str()));
    }
    println!();
}

fn display_new_files(new_files: &Vec<PathBuf>) {
    for new_file in new_files {
        println!("{}", fmt_green(format!("\t\tadded: {}", new_file.display()).as_str()));
    }
    println!();
}

fn display_modified_files(modified_files: &Vec<PathBuf>) {
    for modified_file in modified_files {
        println!("{}", fmt_red(format!("\t\tmodified: {}", modified_file.display()).as_str()));
    }
    println!();
}
