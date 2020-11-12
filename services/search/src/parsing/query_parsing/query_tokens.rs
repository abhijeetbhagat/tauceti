#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {
    And,
    Or,
    Term,
    //Eos,
    LeftParen,
    RightParen,
    Space,
}
