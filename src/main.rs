use crate::cli::{commands, Commands, SearchineCli};
use crate::postings::Posting;
use crate::postings::*;
use crate::path::find_repo_path;
use clap::Parser;

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
        Commands::Init { path } => {
            if let Some(repo_path) = find_repo_path(&path, SEARCHINE_PATH) {
                println!("Index already exists at: {}", repo_path.display());
                return Ok(());
            }
            commands::init(path, SEARCHINE_PATH)?;
        }
        Commands::IndexCorpus { dir_path } => {
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                commands::index_corpus(repo_path, CORPUS_INDEX_FILENAME)?;
            } else {
                println!("Index does not exist at: {}", dir_path);
            }
        }
        Commands::ListCorpus { dir_path } => {
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                commands::list_docs(repo_path, CORPUS_INDEX_FILENAME)?;
            } else {
                println!("Index does not exist at: {}", dir_path);
            }
        }
        Commands::CreateVocabulary {
            path,
        } => {
            if let Some(repo_path) = find_repo_path(&path, SEARCHINE_PATH) {
                commands::create_vocabulary(repo_path, VOCABULARY_FILENAME)?;
            } else {
                println!("Index does not exist at: {}", path);
            }
        }
        // Commands::Index { path } => {}
        _ => {}
    }

    Ok(())
}
