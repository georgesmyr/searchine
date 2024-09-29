# Searchine

<p align="center">
  <img src="./assets/logo.webp" alt="Alt text" width="300" height="300">
</p>

# Searchine

A simple local CLI document search engine built from scratch in Rust, using an inverted index for efficient search.
Searchine allows users to index and search through local document collections with ease.

The tool is evolving, but its current interface provides straightforward commands for initializing, indexing, and
searching document repositories.

## Key Features

- Initialize and manage local document repositories
- Efficient document search using an inverted index
- CLI-based, easy-to-use interface
- Supports indexing of multiple directories

---

## Getting Started

### Initialize a Searchine Index Repository

To create a new Searchine index repository, navigate to the directory containing the documents you want to index, and
run:

```bash
searchine init
```

Alternatively, you can specify the path to the directory:

```bash
searchine init <PATH>
```

If the initialization is successful, you'll see the following message:

```no_run
📂 Index created at: <FULL-PATH>/.searchine
```

### Index Collection

Once you've initialized the repository, you need to index the document collection. This step will store file paths,
assign document IDs, and track modification times. You can index the collection by running:

```bash
searchine index-collection
```

Or, if you're not in the directory containing the repository:

```bash
searchine index-collection <PATH>
```

A successful indexing operation will print:

```no_run
📚 Indexed collection at: <FULL-PATH>/.searchine
```

## Viewing the Document Collection

After indexing, you can list all indexed documents using the following command from within the repository:

```bash
searchine list-collection
```

Or specify the path to the repository:

```bash
searchine list-collection <PATH>
```

## Creating Inverted Index

To enable search functionality, you'll need to create an inverted index of your document collection. This can be
done in one step:

```bash
searchine index
```

Alternatively, specify the path to the repository:

```shell
searchine index <PATH>
```

If the collection hasn't been indexed yet, this command will first index the collection, then create the inverted]
index. Upon completion, you'll see:

```no_run
📋 Created index for: <FULL-PATH>
```

## Documents

`documents` is responsible for representing and loading documents to retrieve information from.

## Tokenize

`tokenize` is responsible for tokenizing the content of document, that is, breaking down the text into separate tokens.

## Index

`index` contains the functionality for indexing a stream of document for a collection of documents.

## Retrieve

`retrieve` is responsible for retrieving relevant documents for specific query.

# Roadmap

`searchine` is functional in its current state, but there's much more that can be done to elevate it into a quality
option. Below are some ideas for improvements, including new features and optimizations. As `searchine` is a learning
project for me, it's uncertain how many of these enhancements will be implemented. However, anyone interested in
contributing is welcome to explore these suggestions.

### Document Loading

- [ ] Add parsers
    - [ ] XML
    - [ ] PDF
    - [ ] Text (txt, md)
- [ ] Parse into documents

### Query processing

- [x] Tokenization
- [x] Token Encoding in tokenizing.
- [ ] Stop words
- [x] Stemming
- [x] Case folding
- [ ] Synonyms
- [ ] Spelling mistakes

### Inverted index & Postings

- [x] Postings
- [x] Postings: doc_id and term_freq as usize or u32? Answer: u32.
- [x] Pipeline with synchronous channels.
- [ ] Find the "optimum" size of the channel.
- [ ] Save index with VB encoding.
- [ ] Save index with Gamma encoding.
- [ ] Figure out how to store and load big indices.

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
- [ ] Async