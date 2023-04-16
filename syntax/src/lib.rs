include!(concat!(env!("OUT_DIR"), "/syntax_enum.rs"));

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

fn is_numeric(c: char) -> bool {
    c.is_numeric()
}

impl Token {
    pub fn is_whitespace(source: &str) -> usize {
        if source.starts_with(is_whitespace) {
            source.find(is_whitespace).unwrap_or(source.len()) as usize
        } else {
            return source.len();
        }
    }

    pub fn is_numeric(source: &str) -> usize {
        if source.starts_with(is_numeric) {
            source.find(is_numeric).unwrap() as usize
        } else {
            return source.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_token() {
        let token = Token {
            kind: SyntaxKind::Add,
            token_type: TokenType::Punctuation,
        };
        println!("{:?}", token);
    }
}
