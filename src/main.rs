use crate::fs::*;
use crate::postings::*;
use crate::postings::Posting;
use crate::index::spimi::*;
use crate::tokenize::*;
use crate::vocab::Vocabulary;

mod fs;
mod index;
mod tokenize;
mod types;
mod vocab;
mod postings;

const XML_PATH: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/glVertexAttribDivisor.xhtml";
const XML_PATH_2: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/glActiveShaderProgram.xhtml";
// const TEXT_PATH: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/readme.md";
// const DIR_PATH: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/";

fn main() -> anyhow::Result<()> {
    // let content = Document::read_to_string("/Users/georgesmyridis/Desktop/Projects/searchine/src/file1.txt")?;
    // let content2 = Document::read_to_string("/Users/georgesmyridis/Desktop/Projects/searchine/src/file2.txt")?;
    // let tokenizer = SimpleTokenizer::new();
    // let tokens = tokenizer.tokenize(&content);
    // let tokens2 = tokenizer.tokenize(&content2);
    //
    // let mut vocab = Vocabulary::new();
    // for token in (&tokens).into_iter().chain((&tokens2).into_iter()) {
    //     vocab.add_token(token);
    // }
    //
    // for (token, token_id) in vocab.token_to_id {
    //     println!("{} -> {}", token, token_id);
    // }

    // Index the documents in each block
    let content = Document::read_to_string(XML_PATH)?;
    let tokenizer = SimpleTokenizer::new();
    let tokens = tokenizer.tokenize(&content);
    let mut index = InMemoryDocumentIndexer::<FrequencyPosting>::new(0);
    index.index_tokens(tokens);
    let index = index.finalize();
    let mut iter = index.into_iter().collect::<Vec<_>>();
    iter.sort_by_key(|(_, f)| (*f).term_count());
    iter.reverse();
    for (t, f) in iter {
        println!("{} -> {:?}", t, f.term_count());
    }


    // let content2 = Document::read_to_string("/Users/georgesmyridis/Desktop/Projects/searchine/src/file2.txt")?;
    // let tokens2 = tokenizer.tokenize(&content2);
    // let mut index2 = InMemoryDocumentIndexer::<FrequencyPosting>::new(1);
    // index2.index_tokens(tokens2);
    // let index2 = index2.finalize();
    //
    // let mut index_all = InMemoryIndex::new();
    // index_all.add_document_index(index);
    // index_all.add_document_index(index2);
    //
    // for (work, posting) in &index_all.index {
    //     println!("{} -> {:?}", work, posting);
    // }
    //
    // println!("{:?}", index_all.get_frequency(0, "world"));
    // println!("{:?}", index_all.get_frequency(0, "world"));
    // println!("{:?}", index_all.get_freq_all("world"));
    // println!("{:?}", index_all.get_score(0, "world"));

    // let index = index::docs::DocumentIndex::read_from_disk("index.json")?;
    // for (path, id) in index.index.iter() {
    //     println!("{} -> {}", path.display(), id);
    // }

    // Create blocks of documents
    // let dir = std::fs::read_dir(DIR_PATH)?;
    // let dir: Vec<DirEntry> = dir.map(|entry| entry.unwrap()).collect();
    // let paths: Vec<PathBuf> = dir.iter().map(|entry| entry.path()).collect();
    // let blocks = DocumentBlocks::from_entries(dir, 500 * 1024)?;
    // for block in blocks {
    //     println!("Block size: {}", block.size());
    // }

    // let stemmer = Stemmer::create(Algorithm::English);
    // for (word, freq) in index.index {
    //     let word_s = stemmer.stem(&word).to_string();
    //     println!("{} -> {}: {}", word, word_s, freq);
    // }
    Ok(())
}
