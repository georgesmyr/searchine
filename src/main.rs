use crate::index::*;
use crate::io::*;
use crate::tokenize::*;

mod index;
mod tokenize;
mod io;
mod fs;

const XML_PATH: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/glStencilFunc.xhtml";
const TEXT_PATH: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/readme.md";
const DIR_PATH: &str = "/Users/georgesmyridis/Desktop/Projects/docs.gl/gl4/";


fn main() -> anyhow::Result<()> {
    let content = FileReader::read(XML_PATH)?;
    let indexer = FileIndexer::new(SimpleTokenizer);
    let index = indexer.index(content);
    println!("{:?}", index.get("testing"));

    Ok(())
}
