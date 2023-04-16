use syntax::{match_identifier, match_operator, SyntaxKind, TokenMetadata};

#[derive(Debug)]
struct Location {
    row: usize,
    column: usize,
}

#[derive(Debug)]
struct Token {
    text: String,
    metadata: TokenMetadata,
    location: Location,
}

impl Token {
    fn new(text: String, metadata: TokenMetadata, row: usize, column: usize) -> Self {
        Self {
            text,
            metadata,
            location: Location { row, column },
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    pub source: String,
    cursor: usize,
    row_start: usize,
    column: usize,
    row: usize,
    tokens: Vec<Token>,
    directives: Vec<String>,
}

impl Lexer {
    pub fn new<S: AsRef<str>>(input: S) -> Self {
        Self {
            source: input.as_ref().to_string(),
            cursor: 0,
            row_start: 0,
            column: 0,
            row: 0,
            tokens: Vec::new(),
            directives: Vec::new(),
        }
    }

    fn is_not_empty(&self) -> bool {
        self.cursor < self.source.len()
    }

    fn get_current_char(&mut self) -> char {
        self.source
            .chars()
            .nth(self.cursor)
            .expect("No character found")
    }

    fn trim_left(&mut self) {
        while self.is_not_empty() && self.get_current_char().is_whitespace() {
            let current = self.get_current_char();
            self.cursor += 1;
            self.column += 1;
            if current == '\n' {
                self.row_start = self.cursor;
                self.row += 1;
                self.column = 0;
            }
        }
    }

    fn drop_line(&mut self) {
        let data = self.source.find(|c| c == '\n');
        let directive_length = data.unwrap();
        self.directives
            .push(self.source[self.cursor..directive_length].to_string());

        self.cursor += directive_length;
        self.column += directive_length;
        self.row += 1;

        self.trim_left();
    }

    fn get_current_buffer(&self) -> String {
        self.source[self.cursor..].to_string()
    }

    fn next_token(&mut self) -> Option<Token> {
        self.trim_left();

        while self.is_not_empty() && self.get_current_char() == '#' {
            println!("directive line encountered {}", self.row);
            self.drop_line();
        }

        let current_char = self.get_current_char();
        if current_char.is_alphabetic() {
            let text_length = self
                .get_current_buffer()
                .find(|c: char| !c.is_alphanumeric())
                .expect("unexpected character");

            let text = &self.get_current_buffer()[0..text_length].to_string();

            self.cursor += text_length;
            let column = self.column;
            self.column += text_length;

            return Some(Token::new(
                text.to_string(),
                match_identifier(&text).unwrap_or(TokenMetadata {
                    kind: SyntaxKind::StringLiteral,
                    token_type: syntax::TokenType::Literal,
                }),
                self.row,
                column,
            ));
        }

        let is_punctuation = match_operator(&current_char.to_string());

        if is_punctuation.is_some() {
            self.cursor += 1;
            let column = self.column;
            self.column += 1;
            return Some(Token::new(
                current_char.to_string(),
                is_punctuation.unwrap(),
                self.row,
                column,
            ));
        }

        return None;
    }

    pub fn tokenize(&mut self) {
        let mut next_token = self.next_token();
        while next_token.is_some() {
            let token = next_token.unwrap();
            println!("next token {:?} {}", token.location, token.text);

            self.tokens.push(token);

            next_token = self.next_token();
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
