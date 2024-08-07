use crate::cli::{commands, Commands, SearchineCli};
use crate::postings::Posting;
use crate::postings::*;
use clap::Parser;

mod fs;
mod path;
mod cli;
mod tokenize;
mod postings;
mod index;
mod scores;


const XML_PATH: &str =
    "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/glVertexAttribDivisor.xhtml";
const XML_PATH_2: &str =
    "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/glActiveShaderProgram.xhtml";
const DIR_PATH: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/";

const SEARCHINE_PATH: &str = ".searchine";
const CORPUS_INDEX_FILENAME: &str = "corpus_index.json";
const VOCABULARY_FILENAME: &str = "vocabulary.json";


fn main() -> anyhow::Result<()> {
    let args = SearchineCli::parse();

    match args.command {
        Commands::Init { path } => {
            commands::init(path, SEARCHINE_PATH)?;
        }
        Commands::IndexCorpus { dir_path } => {
            if !commands::repo_exists(&dir_path, SEARCHINE_PATH) {
                eprintln!("Searchine index not found at: {}", dir_path);
                return Ok(());
            }
            commands::index_corpus(dir_path, SEARCHINE_PATH, CORPUS_INDEX_FILENAME)?;
        }
        Commands::ListCorpus { dir_path } => {
            if !commands::repo_exists(&dir_path, SEARCHINE_PATH) {
                eprintln!("Searchine index not found at: {}", dir_path);
                return Ok(());
            }
            commands::list_corpus(dir_path, SEARCHINE_PATH, CORPUS_INDEX_FILENAME)?;
        }
        Commands::CreateVocabulary {
            path,
            output: output_path,
        } => {
            if !commands::repo_exists(&path, SEARCHINE_PATH) {
                eprintln!("Searchine index not found at: {}", path);
                return Ok(());
            }
            commands::create_vocabulary(path, SEARCHINE_PATH, VOCABULARY_FILENAME)?;
        }
        Commands::Index { path } => {
            if !commands::repo_exists(&path, SEARCHINE_PATH) {
                eprintln!("Searchine index not found at: {}", path);
                return Ok(());
            }
        }
    }

    // Create blocks of documents
    // let dir = std::fs::read_dir(DIR_PATH)?;
    // let dir: Vec<DirEntry> = dir.map(|entry| entry.unwrap()).collect();
    // let paths: Vec<PathBuf> = dir.iter().map(|entry| entry.path()).collect();
    // let blocks = DocumentBlocks::from_entries(dir, 500 * 1024)?;
    // for block in blocks {
    //     println!("Block size: {}", block.size());
    // }

    Ok(())
}
