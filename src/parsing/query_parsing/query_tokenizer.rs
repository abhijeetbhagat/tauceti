use super::query_tokens::Token;
/// A lexer to read tokens in a query
pub struct QueryLexer<'a> {
    query: &'a str,
    cur_pos: usize,
    cur_string: String,
    cur_tok: Option<Token>,
}

impl<'a> QueryLexer<'a> {
    /// Creates a new `QueryLexer` that holds reference
    ///
    /// to the query.
    pub fn new(query: &str) -> QueryLexer {
        QueryLexer {
            query,
            cur_pos: 0,
            cur_tok: None,
            cur_string: String::new(),
        }
    }

    /// Converts a string token to a `Token`
    fn to_token(token: &str) -> Option<Token> {
        match token {
            "(" => Some(Token::LeftParen),
            ")" => Some(Token::RightParen),
            "and" => Some(Token::And),
            "or" => Some(Token::Or),
            "" => None,
            _ => Some(Token::Term),
        }
    }

    #[inline]
    pub fn get_cur_tok(&self) -> Option<Token> {
        self.cur_tok
    }

    #[inline]
    /// Gets the next character in the stream
    pub fn get_cur_string(&self) -> String {
        self.cur_string.clone()
    }

    #[inline]
    /// Gets the next character in the stream
    fn get_char(&mut self) -> char {
        let c = self.query.as_bytes().get(self.cur_pos);
        self.cur_pos += 1;
        *c.unwrap() as char
    }

    #[inline]
    /// Check whether an end-of-stream has been reached
    pub fn eos(&self) -> bool {
        self.cur_pos >= self.query.len()
    }
}

impl<'a> Iterator for QueryLexer<'a> {
    type Item = Token;
    /// Gets the next token in the query
    fn next(&mut self) -> Option<Self::Item> {
        self.cur_string.truncate(0);
        while !self.eos() {
            let c = self.get_char();
            match c {
                'a'..='z' => self.cur_string.push(c),
                '(' => {
                    if !self.cur_string.is_empty() {
                        self.cur_pos -= 1;
                        break;
                    }
                    self.cur_tok = Some(Token::LeftParen);
                    return self.cur_tok;
                }
                ')' => {
                    if !self.cur_string.is_empty() {
                        self.cur_pos -= 1;
                        break;
                    }
                    self.cur_tok = Some(Token::RightParen);
                    return self.cur_tok;
                }
                ' ' => {
                    if !self.cur_string.is_empty() {
                        self.cur_pos -= 1;
                        break;
                    }

                    self.cur_tok = Some(Token::Space);
                    return self.cur_tok;
                }
                _ => continue,
            }
        }

        self.cur_tok = Self::to_token(&self.cur_string);
        self.cur_tok
    }
}

#[cfg(test)]
mod tests {
    use super::{QueryLexer, Token};
    use std::iter::Iterator;

    #[test]
    fn test_query_and_tokenizing() {
        let mut lexer = QueryLexer::new("cpp and python");
        assert_eq!(lexer.next(), Some(Token::Term));
        assert_eq!(lexer.next(), Some(Token::Space));
        assert_eq!(lexer.next(), Some(Token::And));
        assert_eq!(lexer.next(), Some(Token::Space));
        assert_eq!(lexer.next(), Some(Token::Term));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_query_or_tokenizing() {
        let lexer = QueryLexer::new("cpp or python");
        let tokens: Vec<Token> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Token::Term,
                Token::Space,
                Token::Or,
                Token::Space,
                Token::Term
            ]
        );
    }

    #[test]
    fn test_query_and_or_tokenizing() {
        let lexer = QueryLexer::new("cpp or python and java");
        let tokens: Vec<Token> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Token::Term,
                Token::Space,
                Token::Or,
                Token::Space,
                Token::Term,
                Token::Space,
                Token::And,
                Token::Space,
                Token::Term,
            ]
        );
    }

    #[test]
    fn test_query_parens_tokenizing() {
        let lexer = QueryLexer::new("()");
        let tokens: Vec<Token> = lexer.collect();
        assert_eq!(tokens, vec![Token::LeftParen, Token::RightParen,]);
    }

    #[test]
    fn test_query_expr_grouping_tokenizing() {
        let lexer = QueryLexer::new("(python)");
        let tokens: Vec<Token> = lexer.collect();
        assert_eq!(
            tokens,
            vec![Token::LeftParen, Token::Term, Token::RightParen,]
        );
    }

    #[test]
    fn test_query_expr_grouping_tokenizing2() {
        let lexer = QueryLexer::new("(python and cpp)");
        let tokens: Vec<Token> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::Term,
                Token::Space,
                Token::And,
                Token::Space,
                Token::Term,
                Token::RightParen,
            ]
        );
    }

    #[test]
    fn test_query_expr_grouping_tokenizing3() {
        let lexer = QueryLexer::new("(python and cpp) or java");
        let tokens: Vec<Token> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::Term,
                Token::Space,
                Token::And,
                Token::Space,
                Token::Term,
                Token::RightParen,
                Token::Space,
                Token::Or,
                Token::Space,
                Token::Term,
            ]
        );
    }

    #[test]
    fn test_query_expr_grouping_tokenizing4() {
        let lexer = QueryLexer::new("(python and cpp) or (java and perl)");
        let tokens: Vec<Token> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::Term,
                Token::Space,
                Token::And,
                Token::Space,
                Token::Term,
                Token::RightParen,
                Token::Space,
                Token::Or,
                Token::Space,
                Token::LeftParen,
                Token::Term,
                Token::Space,
                Token::And,
                Token::Space,
                Token::Term,
                Token::RightParen,
            ]
        );
    }
}
