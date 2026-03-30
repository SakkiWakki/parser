use crate::structs::{Grammar, Term};

trait Tokenizer {
    fn tokenize(&self, expr: &str) -> Result<Vec<Term>, TokenizeError>;
}

pub enum TokenizeError {
    UnexpectedChar(char),
}

struct GreedyTokenizer {
    grammar: Grammar,
}

/// Greedy by smallest substring from left to right that matches a terminal
impl Tokenizer for GreedyTokenizer {
    fn tokenize(&self, expr: &str) -> Result<Vec<Term>, TokenizeError> {
        let ret: Result<Vec<Term>, TokenizeError>;
        unimplemented!()
    }
}