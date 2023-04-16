include!(concat!(env!("OUT_DIR"), "/syntax_enum.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_token() {
        let token = TokenMetadata {
            kind: SyntaxKind::Add,
            token_type: TokenType::Punctuation,
        };
        println!("{:?}", token);
    }
}
