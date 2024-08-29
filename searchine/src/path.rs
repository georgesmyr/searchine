use std::cmp::Ordering;
use std::ffi::OsStr;
use std::path::{Path, PathBuf, StripPrefixError};

/// Checks if a directory is contained in a directory with specified name.
/// If it is, returns the path to the repo. Otherwise, returns `None`.
///
/// # Arguments
///
/// * `path` - The path which is checked to be contained in the repo.
/// * `repo_path` - The path to the `.searchine` directory.
///
/// # Examples
/// ```no_run
/// use std::path::Path;
/// use fingertips::path::find_repo_path;
///
/// let target_dir = "target_dir";
/// let dir_path = Path::new("/Users/some_user/target_dir/some_dir/");
/// let repo_path = Path::new("/Users/some_user/target_dir/").to_path_buf();
///
/// assert_eq!(find_repo_path(dir_path, target_dir), Some(repo_path));
/// ```
pub fn find_repo_path(path: impl AsRef<Path>, repo_dir_name: impl AsRef<Path>) -> Option<PathBuf> {
    let path = std::fs::canonicalize(path).ok()?;
    let mut path = path.as_path();
    if dir_contains(&path, &repo_dir_name) {
        return Some(path.join(repo_dir_name));
    }
    while let Some(parent) = path.parent() {
        path = parent;
        if dir_contains(&path, &repo_dir_name) {
            return Some(path.join(repo_dir_name));
        }
    }
    None
}

/// Formats the directory path that is optionally specified.
/// If the path is specified, it is canonicalized and returned.
/// If the path is not specified, the current directory is
/// canonicalized and returned.
pub fn canonicalize_dir_path(dir_path: Option<String>) -> PathBuf {
    let dir_path = dir_path.unwrap_or(".".to_string());
    std::fs::canonicalize(dir_path).expect("Failed to canonicalize the specified path.")
}

/// Returns the relative path of `path` relative to `base_path`.
///
/// # Arguments
///
/// * `path` - The path to be made relative.
/// * `base_path` - The base path to which the `path` will be made relative.
///
/// # Returns
///
/// * `Result<PathBuf, StripPrefixError>` - Returns `Ok(PathBuf)` with the
///   relative path if successful, otherwise returns `Err(StripPrefixError)`.
///
/// # Examples
///
/// ```
/// use std::path::{Path, PathBuf};
/// use fingertips::path::get_relative_path;
///
/// let base_path = Path::new("/Users/some_user/target_dir/");
/// let path = Path::new("/Users/some_user/target_dir/some_dir/");
/// let expected_path = PathBuf::from("some_dir/");
/// assert_eq!(get_relative_path(path, base_path), Ok(expected_path));
/// ```
pub fn get_relative_path(
    path: impl AsRef<Path>,
    base_path: impl AsRef<Path>,
) -> Result<PathBuf, StripPrefixError> {
    match path.as_ref().strip_prefix(base_path) {
        Ok(path) => Ok(path.to_path_buf()),
        Err(e) => Err(e),
    }
}

/// Checks if a directory contains a file or directory with the specified name.
fn dir_contains(dir: impl AsRef<Path>, name: impl AsRef<Path>) -> bool {
    dir.as_ref().join(name).exists()
}

/// Compares the base name of two entries.
///
/// This function compares the base name of two entries, taking into account
/// their modes. It returns an `Ordering` value that indicates the relative
/// order of the entries.
///
/// # Arguments
///
/// * `name1` - The base name of the first entry.
/// * `name2` - The base name of the second entry.
/// * `is_dir1` - The mode of the first entry.
/// * `is_dir2` - The mode of the second entry.
///
/// # Returns
///
/// An `Ordering` value indicating the relative order of the entries.
pub fn compare_base_name(name1: &OsStr, name2: &OsStr, is_dir1: bool, is_dir2: bool) -> Ordering {
    let name1 = name1.as_encoded_bytes();
    let name2 = name2.as_encoded_bytes();
    let common_len = std::cmp::min(name1.len(), name2.len());

    match name1[..common_len].cmp(&name2[..common_len]) {
        Ordering::Equal => {}
        ord => return ord,
    }

    // If we are past the match expression, then the names are the same up to their common length.
    // If the lengths are the same, then the names are the same.
    if name1.len() == name2.len() {
        return Ordering::Equal;
    }

    // Check if we have reached the end of the name. If not, get the next character.
    // Otherwise, if the entry is a directory add the '/' character for the sake of comparing.
    // If the entry is not a directory, then we have reached the end of the name.
    fn get_next_char(name: &[u8], is_dir: bool, len: usize) -> Option<u8> {
        match name.get(len).copied() {
            Some(c) => Some(c),
            None if is_dir => Some(b'/'),
            None => None,
        }
    }
    let c1 = get_next_char(&name1, is_dir1, common_len);
    let c2 = get_next_char(&name2, is_dir2, common_len);
    c1.cmp(&c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_relative_path() {
        let base_path = Path::new("/Users/some_user/target_dir/");
        let path = Path::new("/Users/some_user/target_dir/some_dir/");
        let expected_path = PathBuf::from("some_dir/");
        assert_eq!(get_relative_path(path, base_path), Ok(expected_path));
    }
}
