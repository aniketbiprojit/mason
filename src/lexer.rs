#[derive(Debug)]
pub(crate) struct Lexer {
    pub source: String,
}

impl Lexer {
    pub(crate) fn new<'a>(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }
}
