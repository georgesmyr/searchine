use std::path::{Path, PathBuf, StripPrefixError};

/// Checks if a directory is contained in a directory with specified name.
/// If it is, returns the path to the repo. Otherwise, returns `None`.
///
/// # Arguments
///
/// * `from_path` - The path which is checked to be contained in the repo.
/// * `repo_path` - The path to the `.searchine` directory.
///
/// # Examples
/// ```
/// use std::path::Path;
/// use crate::path::get_repo_path;
///
/// let target_dir = "target_dir";
/// let dir_path = Path::new("/Users/some_user/target_dir/some_dir/");
/// let repo_path = Path::new("/Users/some_user/target_dir/").to_path_buf();
///
/// assert_eq!(get_repo_path(dir_path, target_dir), Some(repo_path));
/// ```
pub fn get_repo_path(from_path: impl AsRef<Path>, repo_path: impl AsRef<Path>) -> Option<PathBuf> {
    let mut current_path = from_path.as_ref();

    while let Some(parent) = current_path.parent() {
        if current_path.ends_with(&repo_path) {
            return Some(current_path.to_path_buf());
        }
        current_path = parent;
    }
    None
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
/// use std::path::Path;
/// use crate::path::get_relative_path;
///
/// let base_path = Path::new("/Users/some_user/target_dir/");
/// let path = Path::new("/Users/some_user/target_dir/some_dir/");
/// let expected_path = PathBuf::from("some_dir/");
/// assert_eq!(get_relative_path(path, base_path), Ok(expected_path));
/// ```
pub fn get_relative_path(path: impl AsRef<Path>, base_path: impl AsRef<Path>) -> Result<PathBuf, StripPrefixError> {
    match path.as_ref().strip_prefix(base_path) {
        Ok(path) => Ok(path.to_path_buf()),
        Err(e) => Err(e),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_repo_path() {
        let target_dir = "target_dir";
        let positive_path = Path::new("/Users/some_user/target_dir/some_dir/");
        let negative_path = Path::new("/Users/some_user/some_dir/");
        let expected_path = Path::new("/Users/some_user/target_dir/").to_path_buf();
        assert_eq!(get_repo_path(positive_path, target_dir), Some(expected_path.clone()));
        assert_ne!(get_repo_path(negative_path, target_dir), Some(expected_path));
    }

    #[test]
    fn test_get_relative_path() {
        let base_path = Path::new("/Users/some_user/target_dir/");
        let path = Path::new("/Users/some_user/target_dir/some_dir/");
        let expected_path = PathBuf::from("some_dir/");
        assert_eq!(get_relative_path(path, base_path), Ok(expected_path));
    }
}