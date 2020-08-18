use crate::trees::trie::Trie;
use crate::utils::reader_enums::Token;

pub struct Lexer<'a> {
    cur_pos: usize,
    source: &'a str,
    keywords: Trie,
}

impl<'a> Lexer<'a> {
    /// Creates a lexer that holds a reference to the source
    ///
    /// Builds a dictionary of valid words.
    pub fn new(source: &str) -> Lexer {
        let mut keywords = Trie::new();
        keywords.insert("c");
        keywords.insert("cpp");
        keywords.insert("python");
        keywords.insert("java");

        Lexer {
            cur_pos: 0,
            source,
            keywords,
        }
    }

    /// Gets the next valid owned word
    pub fn next(&mut self) -> Option<Token> {
        match self.read_word() {
            Token::Word(word) if self.keywords.is_present(&word) => Some(Token::Word(word)),
            Token::Word(_) | Token::Invalid => Some(Token::Invalid),
            Token::Eos => None,
        }
    }

    /// Gets the next owned word
    fn read_word(&mut self) -> Token {
        let mut string = String::new();
        while !self.eos() {
            let c = self.get_char();
            match c {
                'a'..='z' => string.push(c),
                ' ' => return Token::Word(string),
                _ => continue,
            }
        }
        if string.is_empty() {
            Token::Eos
        } else {
            Token::Word(string)
        }
    }

    #[inline]
    /// Checks if we've reached end-of-stream
    fn eos(&self) -> bool {
        self.cur_pos >= self.source.len()
    }

    #[inline]
    /// Gets the next character in the stream
    fn get_char(&mut self) -> char {
        let c = self.source.as_bytes().get(self.cur_pos);
        self.cur_pos += 1;
        *c.unwrap() as char
    }
}

#[test]
fn test_tokenization() {
    let mut lexer = Lexer::new("\n\ncpp python irrelevant java");
    assert_eq!(lexer.next(), Some(Token::Word("cpp".into())));
    assert_eq!(lexer.next(), Some(Token::Word("python".into())));
    assert_eq!(lexer.next(), Some(Token::Invalid));
    assert_eq!(lexer.next(), Some(Token::Word("java".into())));
    assert_eq!(lexer.next(), None);
    assert_eq!(lexer.next(), None);
}
