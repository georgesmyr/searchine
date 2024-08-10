use clap::Parser;

use crate::cli::{Commands, SearchineCli};
use crate::path::find_repo_path;

mod cli;
mod fs;
mod index;
mod path;
mod scores;
mod tokenize;
mod commands;

const SEARCHINE_PATH: &str = ".searchine";
const CORPUS_INDEX_FILENAME: &str = "corpus_index.json";
const VOCABULARY_FILENAME: &str = "vocabulary.json";
const INDEX_FILENAME: &str = "index.json";

fn main() -> anyhow::Result<()> {
    let args = SearchineCli::parse();

    match args.command {
        Commands::Init { dir_path } => {
            let dir_path = dir_path.unwrap_or(".".to_string());
            let dir_path = std::fs::canonicalize(dir_path)?;
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                eprintln!("searchine repo already exists at: {}", repo_path.display());
                return Ok(());
            }
            commands::init::invoke(dir_path, SEARCHINE_PATH)?;
        }
        Commands::IndexCorpus { dir_path } => {
            let dir_path = dir_path.unwrap_or(".".to_string());
            let dir_path = std::fs::canonicalize(dir_path)?;
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                commands::index_corpus::invoke(repo_path, CORPUS_INDEX_FILENAME)?;
            } else {
                eprintln!("Index does not exist at: {}", dir_path.display());
            }
        }
        Commands::ListCorpus { dir_path } => {
            let dir_path = dir_path.unwrap_or(".".to_string());
            let dir_path = std::fs::canonicalize(dir_path)?;
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                if repo_path.join(CORPUS_INDEX_FILENAME).exists() {
                    commands::list_corpus::invoke(repo_path, CORPUS_INDEX_FILENAME)?;
                } else {
                    eprintln!("Corpus index does not exist at: {}", dir_path.display());
                    eprintln!("Run `searchine index-corpus` to create the corpus index.");
                }
            } else {
                eprintln!("Index does not exist at: {}", dir_path.display());
            }
        }
        Commands::CreateVocabulary { dir_path } => {
            let dir_path = dir_path.unwrap_or(".".to_string());
            let dir_path = std::fs::canonicalize(dir_path)?;
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                commands::create_vocabulary::invoke(repo_path, VOCABULARY_FILENAME)?;
            } else {
                eprintln!("Index does not exist at: {}", dir_path.display());
            }
        }
        Commands::Index { dir_path } => {
            let dir_path = dir_path.unwrap_or(".".to_string());
            let dir_path = std::fs::canonicalize(dir_path)?;
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                if !repo_path.join(CORPUS_INDEX_FILENAME).exists() {
                    let _ = commands::index_corpus::invoke(&repo_path, CORPUS_INDEX_FILENAME);
                }
                if !repo_path.join(VOCABULARY_FILENAME).exists() {
                    let _ = commands::create_vocabulary::invoke(&repo_path, VOCABULARY_FILENAME);
                }
                commands::index::invoke(repo_path, INDEX_FILENAME)?;
            } else {
                eprintln!("Index does not exist at: {}", dir_path.display());
            }
        }
        Commands::Search {
            query,
            dir_path,
            top_n,
        } => {
            let dir_path = dir_path.unwrap_or(".".to_string());
            let dir_path = std::fs::canonicalize(dir_path)?;
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                if !repo_path.join(INDEX_FILENAME).exists() {
                    let _ = commands::index::invoke(&repo_path, INDEX_FILENAME);
                }
                let top_n = top_n.unwrap_or(10);
                commands::search::invoke(repo_path, &query, top_n)?;
            } else {
                eprintln!("Index does not exist at: {}", dir_path.display());
            }
        }
        _ => {}
    }

    Ok(())
}
