#[derive(Debug)]
pub struct Lexer {
    pub source: String,

    pos: usize,
}

impl Lexer {
    pub fn new<S: AsRef<str>>(input: S) -> Self {
        Self {
            source: input.as_ref().to_string(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) {
        println!("Tokenizing");
        for token in self.source.chars() {
            println!("{}", token);

            self.pos += 1;
        }
    }
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
