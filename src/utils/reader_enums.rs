use serde::{Deserialize, Serialize};

/// Type of document
#[derive(Serialize, Deserialize, Debug)]
pub enum DocType {
    Text,
    PDF,
    Word,
    Raw,
}
