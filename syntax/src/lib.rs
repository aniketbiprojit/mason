use serde::{Deserialize, Serialize};

include!(concat!(env!("OUT_DIR"), "/syntax_enum.rs"));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub metadata: TokenMetadata,
    pub location: Location,
}

impl Token {
    pub fn new(metadata: TokenMetadata, row: usize, column: usize) -> Self {
        Self {
            metadata,
            location: Location { row, column },
        }
    }
}
