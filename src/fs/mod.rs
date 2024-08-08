pub mod blocks;
pub mod docs;

pub use docs::DocumentType;

use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};


/// Canonicalizes the given path.
///
/// This function attempts to resolve the given path to an absolute path.
/// If the path can be successfully canonicalized, it returns the canonicalized path.
/// If the path cannot be canonicalized, it returns `None`.
pub fn canonicalize(path: impl AsRef<Path>) -> Option<PathBuf> {
    if let Ok(path) = std::fs::canonicalize(path) {
        Some(path)
    } else {
        None
    }
}

/// Reads the directory as the given path and returns an iterator of
/// directory entries, after filtering them.
///
/// # Arguments
///
/// * `path` - The path to the directory to be read.
///
/// # Returns
///
/// * `impl Iterator<Item=DirEntry>` - An iterator of directory entries.
pub fn read_dir(
    path: impl AsRef<Path>,
) -> Vec<DirEntry> {
    // Read the directory entries and filter them.
    let mut dir = WalkDir::new(path)
        .into_iter()
        .filter_entry(|entry| !is_ignored(entry))
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .collect::<Vec<_>>();

    // Sort the directory entries by their base name.
    dir.sort_unstable_by(|a, b|
    crate::path::compare_base_name(a.path().as_os_str(), b.path().as_os_str(), false, false)
    );
    dir
}

/// Checks if a directory entry is ignored. Currently, here only the
/// hidden files and directories are ignored.
///
/// To add further filters to the directory entries, edit this function.
///
/// # Arguments
///
/// * `entry` - A reference to the `DirEntry` to be checked.
///
/// # Returns
///
/// * `bool` - Returns `true` if the directory entry is ignored, otherwise returns `false`.
fn is_ignored(entry: &DirEntry) -> bool {
    if is_hidden(entry) {
        return true;
    } else if entry.file_type().is_file() {
        return !is_supported_file_type(entry);
    } else {
        false
    }
}

/// Checks if a directory entry is hidden.
///
/// A directory entry is considered hidden if its file name starts with a dot (`.`).
///
/// # Arguments
///
/// * `entry` - A reference to the `DirEntry` to be checked.
///
/// # Returns
///
/// * `bool` - Returns `true` if the directory entry is hidden, otherwise returns `false`.
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

/// Checks if a directory entry is a supported file type.
///
/// A file type is considered supported if its extension matches one of the
/// supported document types.
///
/// # Arguments
///
/// * `entry` - A reference to the `DirEntry` to be checked.
///
/// # Returns
///
/// * `bool` - Returns `true` if the directory entry is a supported file type,
///   otherwise returns `false`.
fn is_supported_file_type(entry: &DirEntry) -> bool {
    let path = entry.path();
    let extension = path.extension();
    if let Some(extension) = extension {
        DocumentType::from_extension(extension).is_some()
    } else {
        false
    }
}
