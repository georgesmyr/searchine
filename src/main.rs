use rust_stemmers::{Algorithm, Stemmer};

use crate::fs::Document;
use crate::index::FileIndexer;
use crate::tokenize::*;

mod index;
mod tokenize;
mod fs;

const XML_PATH: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/glStencilFunc.xhtml";
// const TEXT_PATH: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/readme.md";
// const DIR_PATH: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/";

fn main() -> anyhow::Result<()> {
    let content = Document::read_to_string(XML_PATH)?;
    let indexer = FileIndexer::new(SimpleTokenizer);
    let index = indexer.index(content);
    let stemmer = Stemmer::create(Algorithm::English);
    for (word, freq) in index.index {
        let word_s = stemmer.stem(&word).to_string();
        println!("{} -> {}: {}", word, word_s, freq);
    }


    Ok(())
}
