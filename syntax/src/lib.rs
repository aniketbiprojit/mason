include!(concat!(env!("OUT_DIR"), "/syntax_enum.rs"));

#[derive(Debug, Clone)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub metadata: TokenMetadata,
    pub location: Location,
}

impl Token {
    pub fn new(text: String, metadata: TokenMetadata, row: usize, column: usize) -> Self {
        Self {
            text,
            metadata,
            location: Location { row, column },
        }
    }
}
