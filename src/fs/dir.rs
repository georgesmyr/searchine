use std::io;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

use crate::fs::DocumentType;
use crate::path::get_relative_path;

/// A struct representing a directory in the repository.
///
/// The `Directory` struct is used to manage the files that will be
/// considered for indexing, walking the directory. The files that
/// are ignored are:
///
/// * Hidden files and directories.
/// * Files that are currently not supported for parsing.
/// * TODO: Add more filters.
///
/// Additionally, it provides methods for presenting the paths to
/// the files as relative to the current working directory.
#[derive(Debug)]
pub struct Directory {
    path: PathBuf,
    repo: PathBuf,
    cwd: PathBuf,
}

impl Directory {
    /// Creates a new `Directory` struct by specifying the path to
    /// the directory. The current working directory is retrieved
    /// from the environment and stored in the struct.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the directory.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the current working directory value is invalid.
    /// Possible cases:
    ///
    /// * Current directory does not exist.
    /// * There are insufficient permissions to access the current directory.
    ///
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf().canonicalize()?;
        let repo = path.join(".searchine").canonicalize()?;
        let cwd = std::env::current_dir()?;
        Ok(Self { path, repo, cwd })
    }

    /// Returns an iterator of the paths to the files in the directory,
    /// relative to the current working directory.
    ///
    /// Hidden directories and files (starting with a dot `.`) are ignored,
    /// and entries that cause errors are skipped printing an error message.
    pub fn iter_relative_paths(&self) -> impl Iterator<Item = PathBuf> {
        WalkDir::new(&self.path)
            .into_iter()
            .filter_entry(|entry| !is_hidden(entry))
            .filter_map(|entry| match entry {
                Ok(entry) => Some(entry.path().to_path_buf()),
                Err(err) => {
                    eprintln!("ERROR: Skipping entry because: {}", err);
                    None
                }
            })
            .filter(|path| !is_ignored(path))
    }

    /// Returns an iterator of the full paths to the files in the directory.
    ///
    /// Hidden directories and files (starting with a dot `.`) are ignored,
    /// and entries that cause errors are skipped printing an error message.
    pub fn iter_full_paths(&self) -> impl Iterator<Item = PathBuf> {
        self.iter_relative_paths()
            .filter_map(|path| path.canonicalize().ok())
    }

    /// Returns an iterator of the paths to the files in the directory,
    /// relative to the repository root.
    ///
    /// The directory entries are filtered by the `is_ignored` function,
    /// and entries that cause errors are skipped printing an error message.
    pub fn iter_repo_paths(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.iter_relative_paths()
            .filter_map(|path| get_relative_path(&path, &self.path).ok())
    }
}

/// Checks if a directory entry is hidden, i.e. if its name starts with
/// a dot (`.`).
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
fn is_supported_file_type(path: impl AsRef<Path>) -> bool {
    let extension = path.as_ref().extension();
    if let Some(extension) = extension {
        DocumentType::from_extension(extension).is_some()
    } else {
        false
    }
}

/// Checks if a directory entry is ignored. A directory entry is ignored if
/// it is a directory, or an unsupported file type.
fn is_ignored(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    if path.is_file() {
        if is_supported_file_type(path) {
            false
        } else {
            eprintln!("WARNING: Ignoring unsupported file: {}", path.display());
            true
        }
    } else {
        true
    }
}
