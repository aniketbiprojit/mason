use syntax::Token;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub directives: Vec<String>,
}

impl Parser {
    pub fn new<T: AsRef<Vec<Token>>, U: AsRef<Vec<String>>>(tokens: T, directives: U) -> Self {
        Self {
            tokens: tokens.as_ref().clone(),
            directives: directives.as_ref().clone(),
        }
    }
}

impl Parser {
    pub fn parse(&self) {
        {
            for directive in self.directives.iter() {
                println!("directive: {:?}", directive);
            }

            for token in self.tokens.iter() {
                println!("{:?}", token);
            }
        }
    }
}
