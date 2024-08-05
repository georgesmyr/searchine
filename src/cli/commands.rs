use std::io;
use std::path::Path;

use crate::tokenize::{Builder, Vocabulary};

/// Creates a vocabulary from a directory of documents.
pub fn create_vocab(path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> io::Result<()> {
    // Read the directory and create a tokenizer.
    let dir = std::fs::read_dir(path.as_ref())?;
    let tokenizer = Builder::default().build();
    let mut vocab = Vocabulary::new();

    // For each directory entry, read the file and tokenize the content.
    // Add the tokens to the vocabulary.
    for dir_entry in dir {
        let path = dir_entry?.path();
        println!("Parsing document: {}", path.display());
        let content = std::fs::read_to_string(&path)?;
        let tokens = tokenizer.tokenize(&content);
        vocab.add_tokens(&tokens);
    }

    // Write the vocabulary to the output file.
    vocab.write_to_disk(&output_path);
    Ok(())
}
