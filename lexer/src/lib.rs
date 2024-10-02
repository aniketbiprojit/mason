use syntax::{match_identifier, match_operator, SyntaxKind, Token, TokenMetadata};

#[derive(Debug)]
pub struct Lexer {
    pub source: String,
    cursor: usize,
    row_start: usize,
    column: usize,
    row: usize,
    pub tokens: Vec<Token>,
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
        }
    }

    fn is_not_empty(&self) -> bool {
        self.cursor < self.source.len()
    }

    fn is_empty(&self) -> bool {
        !self.is_not_empty()
    }

    fn get_current_buffer(&self) -> &str {
        &self.source[self.cursor..]
    }
}

impl Lexer {
    fn get_current_char(&mut self) -> char {
        let current_char = self
            .source
            .chars()
            .nth(self.cursor)
            .expect("No character found");
        current_char
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

    fn _drop_line(&mut self) {
        let data = self.source[self.cursor..].find(|c| c == '\n');
        let data_length = data.unwrap();
        // self.directives
        //     .push(self.source[self.cursor..directive_length].to_string());

        self.cursor += data_length;
        self.column = 0;
        // self.row += 1;

        self.trim_left();
    }

    fn next_token(&mut self) -> Option<Token> {
        self.trim_left();

        if self.is_empty() {
            return None;
        }

        let current_char = self.get_current_char();

        if current_char.is_numeric() {
            let text_length = self
                .get_current_buffer()
                .find(|c: char| !c.is_numeric() && c != '.')
                .expect("unexpected character");

            let text = &self.get_current_buffer()[0..text_length].to_string();

            // check if it has more than one dot

            let places = text.chars().filter(|&c| c == '.').count();
            if &places > &1 {
                panic!(
                    "Expected numeric, value found `{}` {}:{}",
                    text, self.row, self.column
                )
            }

            self.cursor += text_length;
            let column = self.column;
            self.column += text_length;

            let kind;
            if places == 0 {
                kind = SyntaxKind::IntLiteral;
            } else {
                kind = SyntaxKind::FloatLiteral;
            }

            return Some(Token::new(
                match_identifier(&text).unwrap_or(TokenMetadata {
                    kind,
                    token_type: syntax::TokenType::Literal,
                    text: text.to_string(),
                }),
                self.row,
                column,
            ));
        }

        if current_char.is_alphabetic() || current_char == '_' {
            if current_char == '#' {
                self.cursor += 1;
                self.column += 1;
            }

            let mut text_length = self
                .get_current_buffer()
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .expect(
                    "unexpected character encountered - only alphanumeric characters are allowed",
                );
            if current_char == '#' {
                self.cursor -= 1;
                self.column -= 1;
                text_length += 1;
            }

            let text = &self.get_current_buffer()[0..text_length].to_string();

            self.cursor += text_length;
            let column = self.column;
            self.column += text_length;

            return Some(Token::new(
                match_identifier(&text).unwrap_or(TokenMetadata {
                    kind: SyntaxKind::StringLiteral,
                    token_type: syntax::TokenType::Literal,
                    text: text.to_string(),
                }),
                self.row,
                column,
            ));
        }

        let is_punctuation = match_operator(&self.get_current_buffer());

        if is_punctuation.is_some() {
            let metadata = is_punctuation.unwrap();

            self.cursor += metadata.text.len();
            let column = self.column;
            self.column += metadata.text.len();

            if metadata.kind == syntax::SyntaxKind::Comment {
                self._drop_line();
                return Some(Token::new(metadata, self.row, column));
            }

            return Some(Token::new(metadata, self.row, column));
        }

        if current_char == '"' {
            self.cursor += 1;
            self.column += 1;

            let text_length = self.get_current_buffer()[0..]
                .find(|c: char| c == '"')
                .expect("unclosed string literal");

            let text = &self.get_current_buffer()[0..text_length].to_string();

            self.cursor += text_length + 1;
            let column = self.column;
            self.column += text_length + 1;

            return Some(Token::new(
                TokenMetadata {
                    kind: SyntaxKind::StringLiteral,
                    token_type: syntax::TokenType::Literal,
                    text: text.to_string(),
                },
                self.row,
                column,
            ));
        }

        panic!(
            "unexpected character encountered: {}:{}:{}. \
            If inside a string, check for unclosed quotes. \
            Escaped characters are not supported.",
            current_char, self.row, self.column
        );
    }

    pub fn tokenize(&mut self) {
        let mut next_token = self.next_token();
        while next_token.is_some() {
            let token = next_token.unwrap();

            self.tokens.push(token);

            next_token = self.next_token();
        }
    }

    pub fn serial(&self) -> String {
        serde_json::to_string_pretty(&self.tokens).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_lexer() {
        let mut lexer = Lexer::new("#include <stdio.h>");

        lexer.tokenize();
        for token in &lexer.tokens {
            println!("{:?}", token.metadata);
        }

        println!("{}", lexer.serial());
    }
}
