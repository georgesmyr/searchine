/// A vocabulary that maps tokens to IDs and vice versa.
///
/// The vocabulary is used to map tokens to IDs and vice versa.
///
/// # Examples
///
/// ```
/// use crate::vocab::Vocabulary;
///
/// let mut vocab = Vocabulary::new();
/// vocab.add_token("hello");
/// vocab.add_token("world");
///
/// assert_eq!(vocab.get_id("hello"), Some(0));
/// assert_eq!(vocab.get_id("world"), Some(1));
///
/// assert_eq!(vocab.get_token(0), Some(&"hello".to_string()));
/// assert_eq!(vocab.get_token(1), Some(&"world".to_string()));
/// ```
pub struct Vocabulary {
    pub token_to_id: std::collections::HashMap<String, usize>,
    pub id_to_token: std::collections::HashMap<usize, String>,
}

impl Vocabulary {
    /// Creates a new vocabulary.
    pub fn new() -> Self {
        Self {
            token_to_id: std::collections::HashMap::new(),
            id_to_token: std::collections::HashMap::new(),
        }
    }

    /// Adds a token to the vocabulary.
    ///
    /// If the token already exists in the vocabulary, it will not be added again.
    /// Otherwise, the token will be added to the vocabulary and assigned an ID.
    pub fn add_token(&mut self, token: &str) {
        if self.token_to_id.contains_key(token) {
            return;
        }

        let id = self.token_to_id.len();
        self.token_to_id.insert(token.to_string(), id);
        self.id_to_token.insert(id, token.to_string());
    }

    /// Returns the ID of a token if it exists in the vocabulary.
    pub fn get_id(&self, token: &str) -> Option<usize> {
        self.token_to_id.get(token).copied()
    }

    /// Returns the token of an ID if it exists in the vocabulary.
    pub fn get_token(&self, id: usize) -> Option<&String> {
        self.id_to_token.get(&id)
    }
}
