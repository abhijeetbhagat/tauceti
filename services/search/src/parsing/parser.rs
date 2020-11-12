use super::tokenizer::Lexer;
use crate::{trees::index_tree::IndexTree, utils::reader_enums::Token};
use async_std::sync::{Arc, RwLock};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    tree: Arc<RwLock<IndexTree<String, u32>>>,
    id: u32,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tree: Arc<RwLock<IndexTree<String, u32>>>, id: u32) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(source),
            tree,
            id,
        }
    }

    pub async fn parse(&mut self) {
        loop {
            match self.lexer.next() {
                Some(Token::Word(token)) => {
                    let mut tree = self.tree.write().await;
                    tree.insert(token, self.id);
                }
                Some(Token::Invalid) => continue,
                _ => break,
            }
        }
    }
}

#[async_std::test]
async fn test_parsing() -> std::io::Result<()> {
    let tree = Arc::new(RwLock::new(IndexTree::new()));
    let mut parser = Parser::new("cpp python java gandole", tree.clone(), 1);
    parser.parse().await;
    assert_eq!(tree.read().await.get("cpp").unwrap().len(), 1);
    assert_eq!(tree.read().await.get("python").unwrap().len(), 1);
    assert_eq!(tree.read().await.get("java").unwrap().len(), 1);
    Ok(())
}
