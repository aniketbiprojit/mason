use std::fmt;

use serde::{Deserialize, Serialize};

include!(concat!(env!("OUT_DIR"), "/syntax_enum.rs"));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

impl fmt::Debug for TokenMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.token_type == TokenType::Literal {
            match self.kind {
                SyntaxKind::StringLiteral => {
                    write!(f, "{:?} \"{}\" {}", self.kind, self.text, self.text)
                }
                SyntaxKind::IntLiteral => {
                    write!(f, "NUMBER {} {}.0", self.text, self.text)
                }
                SyntaxKind::FloatLiteral => {
                    let float_value = self.text.parse::<f32>().unwrap();
                    if float_value.fract() == 0.0 {
                        write!(f, "NUMBER {} {}.0", self.text, float_value)
                    } else {
                        write!(f, "NUMBER {} {}", self.text, float_value)
                    }
                }
                _ => panic!("Invalid position"),
            }
        } else {
            write!(f, "{:?} {} {}", self.kind, self.text, "null")
        }
    }
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
