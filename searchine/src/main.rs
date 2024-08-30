#[macro_use]
mod fmt;
mod cli;
mod fs;
mod path;

use clap::Parser;

use crate::cli::{Commands, SearchineCli};
use crate::path::{find_repo_path, canonicalize_dir_path};

const SEARCHINE_PATH: &str = ".searchine";
const COLLECTION_FILENAME: &str = "collection.json";
const INDEX_FILENAME: &str = "index.json";

fn main() -> anyhow::Result<()> {
    let args = SearchineCli::parse();

    match args.command {
        // Initializes a new searchine index repository if one does not already exist
        // at the specified directory path. If it already exists, then nothing is done.
        Commands::Init { dir_path } => {
            let dir_path = canonicalize_dir_path(dir_path)?;
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                println_bold!("searchine repo already exists at: {}", repo_path.display());
                return Ok(());
            }
            cli::init::invoke(dir_path, SEARCHINE_PATH)?;
        }
        // Indexes a corpus of documents at the specified directory path.
        Commands::IndexCollection { dir_path } => {
            let dir_path = canonicalize_dir_path(dir_path)?;
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                cli::collection::index(repo_path, COLLECTION_FILENAME, true)?;
            } else {
                println_bold!("Index repository does not exist at: {}", dir_path.display());
            }
        }
        Commands::ListCollection { dir_path } => {
            let dir_path = canonicalize_dir_path(dir_path)?;
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                if repo_path.join(COLLECTION_FILENAME).exists() {
                    cli::collection::list(repo_path, COLLECTION_FILENAME)?;
                } else {
                    println_bold!("Collection does not exist at: {}", dir_path.display());
                    println_bold!("Run `searchine index-collection` to create the collection index.");
                }
            } else {
                println_bold!("Index repository does not exist at: {}", dir_path.display());
            }
        }
        Commands::Index { dir_path } => {
            let dir_path = canonicalize_dir_path(dir_path)?;
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                if !repo_path.join(COLLECTION_FILENAME).exists() {
                    let _ = cli::collection::index(&repo_path, COLLECTION_FILENAME, false);
                }
                cli::index::invoke(repo_path, INDEX_FILENAME, true)?;
            } else {
                println_bold!("Index repository does not exist at: {}", dir_path.display());
            }
        }
        Commands::Status { dir_path } => {
            let dir_path = canonicalize_dir_path(dir_path)?;
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                cli::status::invoke(repo_path, COLLECTION_FILENAME, false)?;
            } else {
                println_bold!("Index repository does not exist at: {}", dir_path.display());
            }
        }
        Commands::Search {
            query,
            dir_path,
            top_n,
        } => {
            let dir_path = canonicalize_dir_path(dir_path)?;
            if let Some(repo_path) = find_repo_path(&dir_path, SEARCHINE_PATH) {
                if !repo_path.join(INDEX_FILENAME).exists() {
                    println_bold!("{} {}",
                        "Index repository has not been indexed.",
                        "Run `searchine index` to index the repository."
                    );
                    return Ok(());
                }
                let top_n = top_n.unwrap_or(10);
                cli::search::invoke(repo_path, &query, top_n)?;
            } else {
                println_bold!("Index repository does not exist at: {}", dir_path.display());
            }
        }
    }

    Ok(())
}

