use std::collections::BTreeSet;
use std::io;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

use fingertips::fs::*;
use fingertips::tokenize::*;

/// Creates a vocabulary from a directory of documents.
///
/// Each document is parsed to a string and tokenized. All the tokens
/// are collected and assigned a token ID.
///
/// # Arguments
///
/// * `repo_dir` - The path to the directory containing the documents.
/// * `vocabulary_file_name` - The file name where the vocabulary will be written.
pub fn invoke(
    repo_dir: impl AsRef<Path>,
    vocabulary_file_name: impl AsRef<Path>,
) -> io::Result<Vocabulary> {

    // Initialize tokenizer and vocabulary.
    let tokenizer = Builder::default().build();
    let mut vocab = Arc::new(Mutex::new(Vocabulary::new()));

    // For each directory entry, read the file and tokenize the content.
    // Add the tokens to the vocabulary.
    let repo_dir = repo_dir.as_ref();
    let base_dir = repo_dir.parent().unwrap_or_else(|| {
        panic!(
            "Could not find parent directory of repo path: {}",
            repo_dir.display()
        );
    });

    let dir = Directory::new(base_dir)?;
    let dir = dir.iter_full_paths().collect::<BTreeSet<_>>();
    dir.par_iter().for_each(|path| {
        let content = read_to_string(&path).unwrap();
        let tokens = tokenizer.tokenize(&content);
        let mut vocab = vocab.lock().unwrap();
        vocab.add_tokens(&tokens);
    });

    // Write the vocabulary to the output file.
    let output_path = repo_dir.join(vocabulary_file_name);
    let vocab = Arc::try_unwrap(vocab).expect("").into_inner().unwrap();
    vocab.write_to_file(output_path);

    let emoji = String::from_utf8(vec![0xF0, 0x9F, 0x93, 0x92]).unwrap_or_default();
    println_bold!("{emoji} Created vocabulary for: {}", base_dir.display());
    Ok(vocab)
}
