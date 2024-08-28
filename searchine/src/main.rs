mod cli;
mod fs;
mod path;

use clap::Parser;
use std::path::PathBuf;

use crate::cli::{Commands, SearchineCli};
use crate::path::find_repo_path;


#[macro_use]
mod fmt;
mod commands;

const SEARCHINE_PATH: &str = ".searchine";
const CORPUS_INDEX_FILENAME: &str = "collection.json";
const INDEX_FILENAME: &str = "index.json";

fn main() -> anyhow::Result<()> {
    let args = SearchineCli::parse();

    match args.command {
        // Initializes a new searchine index repository if one does not already exist
        // at the specified directory path. If it already exists, then nothing is done.
        Commands::Init { dir_path } => {
            let dir_path = fmt_dir_path(dir_path);
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                println_bold!("searchine repo already exists at: {}", repo_path.display());
                return Ok(());
            }
            commands::init::invoke(dir_path, SEARCHINE_PATH)?;
        }
        // Indexes a corpus of documents at the specified directory path.
        Commands::IndexCorpus { dir_path } => {
            let dir_path = fmt_dir_path(dir_path);
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                commands::index_corpus::invoke(repo_path, CORPUS_INDEX_FILENAME)?;
            } else {
                println_bold!("Index does not exist at: {}", dir_path.display());
            }
        }
        Commands::ListCorpus { dir_path } => {
            let dir_path = fmt_dir_path(dir_path);
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                if repo_path.join(CORPUS_INDEX_FILENAME).exists() {
                    commands::list_corpus::invoke(repo_path, CORPUS_INDEX_FILENAME)?;
                } else {
                    println_bold!("Corpus index does not exist at: {}", dir_path.display());
                    println_bold!("Run `searchine index-corpus` to create the corpus index.");
                }
            } else {
                println_bold!("Index does not exist at: {}", dir_path.display());
            }
        }
        Commands::Index { dir_path } => {
            let dir_path = fmt_dir_path(dir_path);
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                if !repo_path.join(CORPUS_INDEX_FILENAME).exists() {
                    let _ = commands::index_corpus::invoke(&repo_path, CORPUS_INDEX_FILENAME);
                }
                commands::index::invoke(repo_path, INDEX_FILENAME)?;
            } else {
                println_bold!("Index does not exist at: {}", dir_path.display());
            }
        }
        Commands::Status { dir_path } => {
            let dir_path = fmt_dir_path(dir_path);
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                commands::status::invoke(repo_path, CORPUS_INDEX_FILENAME)?;
            } else {
                println_bold!("Index does not exist at: {}", dir_path.display());
            }
        }
        Commands::Search {
            query,
            dir_path,
            top_n,
        } => {
            let dir_path = fmt_dir_path(dir_path);
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                if !repo_path.join(INDEX_FILENAME).exists() {
                    let _ = commands::index::invoke(&repo_path, INDEX_FILENAME);
                }
                let top_n = top_n.unwrap_or(10);
                commands::search::invoke(repo_path, &query, top_n)?;
            } else {
                println_bold!("Index does not exist at: {}", dir_path.display());
            }
        }
        _ => {}
    }

    Ok(())
}

/// Formats the directory path that is optionally specified.
/// If the path is specified, it is canonicalized and returned.
/// If the path is not specified, the current directory is
/// canonicalized and returned.
fn fmt_dir_path(dir_path: Option<String>) -> PathBuf {
    let dir_path = dir_path.unwrap_or(".".to_string());
    std::fs::canonicalize(dir_path).expect("Failed to canonicalize the specified path.")
}
