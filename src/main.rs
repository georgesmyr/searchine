use crate::fs::*;
use crate::index::spimi::*;
use crate::postings::Posting;
use crate::postings::*;
use crate::tokenize::*;
use crate::vocab::Vocabulary;

mod fs;
mod index;
mod postings;
mod scores;
mod tokenize;
mod vocab;

const XML_PATH: &str =
    "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/glVertexAttribDivisor.xhtml";
const XML_PATH_2: &str =
    "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/glActiveShaderProgram.xhtml";
const DIR_PATH: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/";

fn main() -> anyhow::Result<()> {
    let tokenizer = SimpleTokenizer::new();
    let mut gindex_hm = std::collections::HashMap::<usize, InMemoryDocumentIndex<FrequencyPosting>>::new();

    let dir = std::fs::read_dir(DIR_PATH)?;
    for (i, direntry) in dir.enumerate().take(2) {
        let path = direntry?.path();
        println!("Indexing document: {}", path.display());
        let content = Document::read_to_string(&path)?;
        let tokens = tokenizer.tokenize(&content);
        let mut indexer = InMemoryDocumentIndexer::<FrequencyPosting>::new(i);
        indexer.index_tokens(tokens);
        let index = indexer.finalize();
        gindex_hm.insert(i, index);
    }

    let gindex = InMemoryIndex { index: gindex_hm };

    for (id, doc_idx) in &gindex.index {
        println!("Document ID: {}", id);
        for (word, _) in doc_idx.clone().into_iter() {
            println!("    {} -> {}", word, gindex.score_tf_idf(*id, &word));
        }
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
