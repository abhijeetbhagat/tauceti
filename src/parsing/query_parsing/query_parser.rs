use super::query_tokenizer::QueryLexer;
use super::query_tokens::Token;
use crate::parsing::query_ast::QueryExpression;
use crate::utils::error_structs::TaucetiError;

type ParensStack = Vec<Token>;

/// A query parser
pub struct QueryParser<'a> {
    lexer: QueryLexer<'a>,
    parens_stack: ParensStack,
}

impl<'a> QueryParser<'a> {
    /// Creates a new `QueryParser`
    pub fn new(query: &str) -> QueryParser {
        QueryParser {
            lexer: QueryLexer::new(query),
            parens_stack: ParensStack::new(),
        }
    }

    /// Parses a query and returns a boxed `QueryExpression`
    ///
    /// "(python or cpp) and rust" translates to:
    ///
    ///                     and
    ///                   /     \
    ///                 or      rust
    ///               /    \
    ///           python    cpp
    ///
    pub fn parse(&mut self) -> Result<Box<QueryExpression>, TaucetiError> {
        let mut expr = Err(TaucetiError::QueryParseError);
        while let Some(token) = self.lexer.next() {
            match token {
                Token::And | Token::Or | Token::Term | Token::LeftParen | Token::RightParen => {
                    expr = self.expr();
                    if !self.parens_stack.is_empty() {
                        return Err(TaucetiError::QueryParseError);
                    }
                }
                Token::Space => continue,
                _ => break,
            }
        }
        expr
    }

    /// Main entry point of the parsing logic
    fn expr(&mut self) -> Result<Box<QueryExpression>, TaucetiError> {
        match self.lexer.get_cur_tok() {
            Some(Token::And) => Ok(Box::new(QueryExpression::And(
                self.expr().unwrap(),
                self.expr().unwrap(),
            ))),
            Some(Token::Or) => Ok(Box::new(QueryExpression::Or(
                self.expr().unwrap(),
                self.expr().unwrap(),
            ))),
            Some(Token::Term) => Ok(Box::new(QueryExpression::Term(self.lexer.get_cur_string()))),
            //Token::Eos => {}
            Some(Token::LeftParen) => {
                self.parens_stack.push(Token::LeftParen);
                self.expr()
            }
            Some(Token::RightParen) => {
                self.parens_stack.pop();
                self.expr()
            }
            _ => Err(TaucetiError::QueryParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::QueryParser;
    use crate::parsing::query_ast::QueryExpression;

    #[test]
    fn test_parsing() {
        let mut parser = QueryParser::new("cpp");
        let result = parser.parse();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(*result, QueryExpression::Term("cpp".into()));
    }
}
