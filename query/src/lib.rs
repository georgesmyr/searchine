use std::collections::HashMap;

use tokenize::Token;

/// Structure that stores the token and its count. /// /// For example, after we have tokenized a query, we can
/// create a token counter for more efficient relevance scoring.
#[derive(Debug)]
pub struct Query {
    inner: HashMap<Token, u32>,
}

impl Query {
    /// Returns the count of a specific token.
    pub fn term_count(&self, token: &Token) -> u32 {
        *self.inner.get(token).unwrap_or(&0)
    }
}

impl<I> From<I> for Query
where
    I: IntoIterator<Item=Token>,
{
    /// Creates token counter from a stream of tokens.
    fn from(tokens: I) -> Self {
        let mut tokens_counter = HashMap::new();
        for token in tokens {
            *tokens_counter.entry(token).or_insert(0) += 1;
        }
        Query {
            inner: tokens_counter,
        }
    }
}

impl IntoIterator for Query {
    type Item = (Token, u32);
    type IntoIter = std::collections::hash_map::IntoIter<Token, u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use tokenize::Tokenizer;

    use super::*;

    #[test]
    fn test_construction() {
        let text = "I want it. I got it.".to_string();
        let mut tokenizer = Tokenizer::default();
        let tokens = tokenizer.tokenize(text);
        let token_counts = Query::from(tokens);
        assert_eq!(token_counts.term_count(&"i".to_string()), 2);
        assert_eq!(token_counts.term_count(&"want".to_string()), 1);
        assert_eq!(token_counts.term_count(&"it".to_string()), 2);
        assert_eq!(token_counts.term_count(&"got".to_string()), 1);
        assert_eq!(token_counts.term_count(&"ai".to_string()), 0);
    }
}
