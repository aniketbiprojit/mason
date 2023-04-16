use syntax::Token;

pub struct Parser<'a> {
    pub tokens: &'a Vec<Token>,
    pub directives: &'a Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, directives: &'a Vec<String>) -> Self {
        Self { tokens, directives }
    }
}

impl<'a> Parser<'a> {
    pub fn parse(&self) {
        #[cfg(feature = "debug")]
        {
            for directive in self.directives {
                println!("directive: {:?}", directive);
            }

            for token in self.tokens {
                println!("{:?}", token);
            }
        }
    }
}
