# Searchine

Searchine will be a search engine from your CLI!

## Notes

### Query processing

- [ ] Tokenization
- [ ] Stop words
- [x] Stemming
- [x] Case folding
- [ ] Synonyms
- [ ] Spelling mistakes

### Inverted index & Postings

- [x] Vocabulary
- [x] Postings
- [ ] Vectors vs Singly linked lists for postings
- [ ] Postings: doc_id and term_freq as usize or u32?

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

- [ ] Document Index with SHA1 of each document content

### Optimizations

- [ ] Compress files
- [ ] Use memory mapped files
- [ ] Parse XMLs faster (quick-xml?)