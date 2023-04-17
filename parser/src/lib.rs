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
    pub fn parse(&self) {
        {
            todo!("Parse the tokens into AST");
        }
    }
}
