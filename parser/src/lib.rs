use syntax::Token;

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new<T: AsRef<Vec<Token>>>(tokens: T) -> Self {
        Self {
            tokens: tokens.as_ref().clone(),
        }
    }
}

impl Parser {
    pub fn parse(&mut self) {
        #[cfg(feature = "debug")]
        {
            for token in self.tokens.iter() {
                println!("{:?}", token);
            }
        }
    }
}
