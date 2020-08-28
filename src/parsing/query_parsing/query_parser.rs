use super::query_tokenizer::QueryLexer;
use super::query_tokens::Token;
use crate::parsing::query_ast::QueryExpression;
use crate::utils::error_structs::TaucetiError;

type ParensStack = Vec<Token>;
type ExprStack = Vec<Box<QueryExpression>>;

/// A query parser
pub struct QueryParser<'a> {
    lexer: QueryLexer<'a>,
    parens_stack: ParensStack,
    expr_stack: ExprStack,
}

impl<'a> QueryParser<'a> {
    /// Creates a new `QueryParser`
    pub fn new(query: &str) -> QueryParser {
        QueryParser {
            lexer: QueryLexer::new(query),
            parens_stack: ParensStack::new(),
            expr_stack: ExprStack::new(),
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
        while let Some(token) = self.lexer.next() {
            match token {
                Token::And | Token::Or | Token::Term | Token::LeftParen | Token::RightParen => {
                    let expr = self.expr()?;
                    self.expr_stack.push(expr);
                    if !self.parens_stack.is_empty() {
                        return Err(TaucetiError::QueryParseError);
                    }
                }
                Token::Space => continue,
            }
        }

        let expr = self.expr_stack.pop().unwrap();
        if !self.expr_stack.is_empty() {
            return Err(TaucetiError::QueryParseError);
        }

        Ok(expr)
    }

    /// Main entry point of the parsing logic
    fn expr(&mut self) -> Result<Box<QueryExpression>, TaucetiError> {
        match self.lexer.get_cur_tok() {
            Some(Token::And) => {
                let op1 = self.expr_stack.pop().ok_or(TaucetiError::QueryParseError)?;
                Ok(Box::new(QueryExpression::And(op1, self.expr()?)))
            }
            Some(Token::Or) => {
                let op1 = self.expr_stack.pop().ok_or(TaucetiError::QueryParseError)?;
                Ok(Box::new(QueryExpression::Or(op1, self.expr()?)))
            }
            Some(Token::Term) => self.parse_term_expr(),
            //Token::Eos => {}
            Some(Token::LeftParen) => {
                self.parens_stack.push(Token::LeftParen);
                self.expr()
            }
            Some(Token::RightParen) => {
                self.parens_stack.pop();
                self.expr()
            }
            Some(Token::Space) => {
                self.lexer.next();
                self.expr()
            }
            _ => Err(TaucetiError::QueryParseError),
        }
    }

    fn parse_term_expr(&mut self) -> Result<Box<QueryExpression>, TaucetiError> {
        let op1 = Box::new(QueryExpression::Term(self.lexer.get_cur_string()));
        loop {
            match self.lexer.next() {
                Some(Token::And) => {
                    self.lexer.next(); // Move to the next token
                    return Ok(Box::new(QueryExpression::And(op1, self.expr()?)));
                }
                Some(Token::Or) => {
                    self.lexer.next(); // Move to the next token
                    return Ok(Box::new(QueryExpression::Or(op1, self.expr()?)));
                }
                Some(Token::RightParen) => {
                    self.lexer.next(); // Move to the next token
                    self.parens_stack.pop();
                    return Ok(op1);
                }
                Some(Token::Term) | Some(Token::LeftParen) => {
                    return Err(TaucetiError::QueryParseError)
                }
                Some(Token::Space) => continue,
                None => return Ok(op1),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::QueryParser;
    use crate::parsing::query_ast::QueryExpression;

    #[test]
    fn test_parsing_single_term() {
        let mut parser = QueryParser::new("cpp");
        let result = parser.parse();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(*result, QueryExpression::Term("cpp".into()));
    }

    #[test]
    fn test_parsing_and_expr() {
        let mut parser = QueryParser::new("cpp and python");
        let result = parser.parse();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            *result,
            QueryExpression::And(
                Box::new(QueryExpression::Term("cpp".into())),
                Box::new(QueryExpression::Term("python".into()))
            )
        );
    }

    #[test]
    fn test_parsing_or_expr() {
        let mut parser = QueryParser::new("cpp or python");
        let result = parser.parse();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            *result,
            QueryExpression::Or(
                Box::new(QueryExpression::Term("cpp".into())),
                Box::new(QueryExpression::Term("python".into()))
            )
        );
    }

    #[test]
    fn test_parsing_or_and_expr() {
        let mut parser = QueryParser::new("cpp or python and java");
        let result = parser.parse();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            *result,
            QueryExpression::Or(
                Box::new(QueryExpression::Term("cpp".into())),
                Box::new(QueryExpression::And(
                    Box::new(QueryExpression::Term("python".into())),
                    Box::new(QueryExpression::Term("java".into()))
                )),
            )
        );
    }

    #[test]
    fn test_parsing_invalid() {
        let mut parser = QueryParser::new("cpp or ");
        let result = parser.parse();
        assert!(result.is_err());
    }
}
