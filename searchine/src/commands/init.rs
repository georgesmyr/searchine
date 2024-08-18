use std::io;
use std::path::Path;

/// Initializes a new searchine index repo.
///
/// # Arguments
///
/// * `dir_path` - The path to the directory where the index will be created.
/// * `searchine_path` - The path to the searchine index directory relative to `dir_path`.
pub fn invoke(dir_path: impl AsRef<Path>, searchine_path: impl AsRef<Path>) -> io::Result<()> {
    let dir_path = dir_path.as_ref();
    let index_path = dir_path.join(searchine_path);
    std::fs::create_dir_all(&index_path)?;
    let full_index_path = std::fs::canonicalize(&index_path)?;
    let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x93, 0x82]).unwrap_or_default();
    println_bold!("{emoji} Index created at: {}", full_index_path.display());
    Ok(())
}
