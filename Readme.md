# Searchine

<p align="center">
  <img src="./assets/logo.webp" alt="Alt text" width="300" height="300">
</p>

Searchine is a simple (for now) local CLI document search engine implemented in Rust from scratch.
The tool is planned to change in the future but currently the interface is like so:

You can navigate a directory of documents you want to be indexed, and the tool creates a searchine
index repository.

```bash
searchine index
```

the output

Searchine will be a search engine from your CLI!

## Bugs

- [ ] Fix the document ids in the index to match the document ids in the collection.

## TODOs

### Document Loading

- [ ] Add parsers
    - [ ] XML
    - [ ] PDF
    - [ ] Text (txt, md)
- [ ] Parse into documents

### Query processing

- [ ] Tokenization
- [x] Token Encoding in tokenizing.
- [ ] Stop words
- [x] Stemming
- [x] Case folding
- [ ] Synonyms
- [ ] Spelling mistakes

### Inverted index & Postings

- [x] Postings
- [x] Postings: doc_id and term_freq as usize or u32? Answer: u32.
- [ ] Indexing pipeline channels
- [ ] Pipeline with synchronous channels

### Encoding

- [ ] VBEncoding
    - [ ] Estimate the necessary capacity of vector, for encoding.
    - [ ] Fix types or/and return appropriate errors
- [ ] Unary
    - [ ] Change encoding to start from 0? 0 is used nowhere I think.
    - [ ] Encode in bits, and use BitWriter, or bytes ?
- [ ] Gamma
    - [ ] Optimize offsetting bytes' calculation.
- [ ] Delta
- [ ] Golomb

### Boolean Search

- [ ] Take intersections efficiently
- [ ] Take unions efficiently
- [ ] Take negations efficiently
- [ ] Combine all of the above
- [ ] Implement a simple boolean search engine
- [ ] Phrases
- [ ] Proximity
- [ ] Wildcards
- [ ] Ranking

### Index Status

### Optimizations

- [ ] Compress files
- [ ] Use memory mapped files
- [ ] Parse XMLs faster (quick-xml?)
- [ ] Select top n results with heap or quick-select algorithm
- [ ] Make use of SIMD instructions?
- [ ] Deserialize corpus index to Inverted corpus index immediately.