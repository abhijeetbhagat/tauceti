use serde::{Deserialize, Serialize};

/// Type of document
#[derive(Serialize, Deserialize, Debug)]
pub enum DocType {
    Text,
    PDF,
    Word,
    Raw,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    /// A word of interest
    Word(String),
    /// This isn't a word
    Invalid,
    /// End of stream
    Eos,
}
