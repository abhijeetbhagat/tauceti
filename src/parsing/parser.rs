use super::tokenizer::Lexer;
use crate::trees::index_tree::IndexTree;
use std::sync::{Arc, Mutex};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    tree: Arc<Mutex<IndexTree<String>>>,
    id: u32,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tree: Arc<Mutex<IndexTree<String>>>, id: u32) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(source),
            tree,
            id,
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.lexer.next() {
            let mut tree = self.tree.lock().unwrap();
            tree.insert(token.into(), self.id);
        }
    }
}

#[test]
fn test_parsing() {}
