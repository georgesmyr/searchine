use crate::cli::{commands, Commands, SearchineCli};
use crate::path::find_repo_path;
use crate::postings::Posting;
use crate::postings::*;
use clap::Parser;
use walkdir;

mod cli;
mod fs;
mod index;
mod path;
mod postings;
mod scores;
mod tokenize;

const DIR_PATH: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/";

const SEARCHINE_PATH: &str = ".searchine";
const CORPUS_INDEX_FILENAME: &str = "corpus_index.json";
const VOCABULARY_FILENAME: &str = "vocabulary.json";

fn main() -> anyhow::Result<()> {
    let args = SearchineCli::parse();

    match args.command {
        Commands::Init { dir_path } => {
            let dir_path = dir_path.unwrap_or(".".to_string());
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                eprintln!("searchine repo already exists at: {}", repo_path.display());
                return Ok(());
            }
            commands::init(dir_path, SEARCHINE_PATH)?;
        }
        Commands::IndexCorpus { dir_path } => {
            let dir_path = dir_path.unwrap_or(".".to_string());
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                commands::index_corpus(repo_path, CORPUS_INDEX_FILENAME)?;
            } else {
                eprintln!("Index does not exist at: {}", dir_path);
            }
        }
        Commands::ListCorpus { dir_path } => {
            let dir_path = dir_path.unwrap_or(".".to_string());
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                if repo_path.join(CORPUS_INDEX_FILENAME).exists() {
                    commands::list_docs(repo_path, CORPUS_INDEX_FILENAME)?;
                } else {
                    eprintln!("Corpus index does not exist at: {}", dir_path);
                    eprintln!("Run `searchine index-corpus` to create the corpus index.");
                }
            } else {
                eprintln!("Index does not exist at: {}", dir_path);
            }
        }
        Commands::CreateVocabulary { dir_path } => {
            let dir_path = dir_path.unwrap_or(".".to_string());
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                commands::create_vocabulary(repo_path, VOCABULARY_FILENAME)?;
            } else {
                eprintln!("Index does not exist at: {}", dir_path);
            }
        }
        // Commands::Index { dir_path } => {
        //     let dir_path = dir_path.unwrap_or(".".to_string());
        //     if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
        //         commands::index(repo_path)?;
        //     } else {
        //         eprintln!("Index does not exist at: {}", dir_path);
        //     }
        // }
        _ => {}
    }

    Ok(())
}
