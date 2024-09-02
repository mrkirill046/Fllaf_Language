use crate::token::Token;
use std::str::FromStr;

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.current_pos >= self.input.len() {
            return Token::EndOfFile;
        }

        let current_char = self.current_char();

        if current_char == '"' {
            return self.consume_string();
        } else if current_char.is_digit(10) {
            return self.consume_number();
        } else if current_char.is_alphabetic() {
            return self.consume_identifier();
        } else if current_char == '+' {
            self.advance();
            return Token::Plus;
        } else if current_char == '-' {
            self.advance();
            return Token::Minus;
        } else if current_char == '*' {
            self.advance();
            return Token::Multiply;
        } else if current_char == '/' {
            self.advance();

            return if self.current_char() == '/' {
                self.consume_line_comment()
            } else if self.current_char() == '*' {
                self.consume_block_comment()
            } else {
                Token::Divide
            }
        } else if current_char == '.' {
            self.advance();
            return Token::Dot;
        } else if current_char == '(' {
            self.advance();
            return Token::LParen;
        } else if current_char == ')' {
            self.advance();
            return Token::RParen;
        } else if current_char == ';' {
            self.advance();
            return Token::Semicolon;
        } else {
            panic!("Unexpected character: {}", current_char);
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn current_char(&self) -> char {
        self.input[self.current_pos..].chars().next().unwrap_or('\0')
    }

    fn advance(&mut self) {
        self.current_pos += self.current_char().len_utf8();
    }

    fn consume_string(&mut self) -> Token {
        self.advance();

        let start = self.current_pos;

        while self.current_char() != '"' {
            if self.current_char() == '\0' {
                panic!("Unterminated string literal");
            }
            self.advance();
        }

        let end = self.current_pos;

        self.advance();

        Token::StringLiteral(self.input[start..end].to_string())
    }

    fn consume_number(&mut self) -> Token {
        let start = self.current_pos;

        while self.current_char().is_digit(10) {
            self.advance();
        }

        let number_str = &self.input[start..self.current_pos];

        Token::Number(i64::from_str(number_str).unwrap())
    }

    fn consume_identifier(&mut self) -> Token {
        let start = self.current_pos;

        while self.current_char().is_alphanumeric() || self.current_char() == '_' {
            self.advance();
        }

        let ident_str = &self.input[start..self.current_pos];

        match ident_str {
            "System" => Token::System,
            "log" => Token::Log,
            _ => Token::Identifier(ident_str.to_string()),
        }
    }

    fn consume_line_comment(&mut self) -> Token {
        self.advance();
        self.advance();

        let start = self.current_pos;

        while self.current_char() != '\n' && self.current_char() != '\0' {
            self.advance();
        }

        Token::Comment(self.input[start..self.current_pos].to_string())
    }

    fn consume_block_comment(&mut self) -> Token {
        self.advance();
        self.advance();

        let start = self.current_pos;

        while !(self.current_char() == '*' && self.next_char() == '/') {
            if self.current_char() == '\0' {
                panic!("Unterminated block comment");
            }

            self.advance();
        }

        self.advance();
        self.advance();

        let end = self.current_pos - 2;
        Token::BlockComment(self.input[start..end].to_string())
    }

    fn next_char(&self) -> char {
        self.input[self.current_pos..].chars().nth(1).unwrap_or('\0')
    }
}
