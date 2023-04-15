#[derive(Debug)]
pub struct Lexer {
    pub source: String,
}

impl Lexer {
    pub fn new<S: AsRef<str>>(input: S) -> Self {
        Self {
            source: input.as_ref().to_string(),
        }
    }

    pub fn tokenize(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_lexer() {
        let lexer = Lexer::new("");
        println!("{:?}", lexer);
    }
}
