// use std::collections::HashMap;

use crate::{error::LoxError, token::*, token_type::*};
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        // let mut had_error: Option<LoxError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    e.report("".to_string());
                }
            };
        }
        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }
    pub fn new(source: String) -> Self {
        // let mut keywords = HashMap::new();
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
            // keywords,
        }
    }
    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '-' => self.add_token(TokenType::Minus),
            '!' => {
                let tok = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tok)
            }
            '=' => {
                let tok = if self.is_match('=') {
                    TokenType::Equals
                } else {
                    TokenType::Assign
                };
                self.add_token(tok)
            }
            '>' => {
                let tok = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tok)
            }
            '<' => {
                let tok = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tok)
            }
            '/' => {
                // eat up all the characters in a line till the current doesnt reach a line break
                // [peek()] and we are not at the end of the line [is_at_end()]
                if self.is_match('/') {
                    while let Some(ch) = self.peek() {
                        if ch != '\n' {
                            self.advance();
                        } else {
                            self.add_token(TokenType::Slash)
                        }
                    }
                } else if self.is_match('*') {
                    while let Some(ch) = self.peek() {
                        match ch {
                            '*' => {
                                if self.peek_next() == Some('/') {
                                    self.advance();
                                    self.advance();
                                    println!("current after two advance: {}", self.current);
                                    break;
                                }
                            }
                            '\n' => {
                                self.line += 1;
                            }
                            _ => {}
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '\n' => {
                self.line += 1;
            }
            '\r' | '\t' | ' ' => {}
            '"' => {
                self.string()?;
            }
            '0'..='9' => {
                self.number()?;
            }
            _ => {
                if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier()?;
                }
            }
        }
        Ok(())
    }
    fn identifier(&mut self) -> Result<(), LoxError> {
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }
        let text: String = self.source[self.start..self.current].iter().collect();

        if let Some(ttype) = Scanner::keywords(text.as_str()) {
            self.add_token(ttype)
        } else {
            self.add_token(TokenType::Identifier)
        }
        Ok(())
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn advance(&mut self) -> char {
        let result = *self.source.get(self.current).unwrap();
        self.current += 1;
        result
    }
    fn keywords(check: &str) -> Option<TokenType> {
        match check {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_object(token_type, None)
    }
    fn add_token_object(&mut self, ttype: TokenType, literal: Option<Object>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, lexeme, literal, self.line));
    }
    fn is_match(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if *ch == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }
    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }
    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }
    fn string(&mut self) -> Result<(), LoxError> {
        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    // this will break the loop
                    // because an ending " was found
                    break;
                }
                '\n' => {
                    self.line += 1;
                }
                _ => {}
            }
            // while loop will not be continued after self.peek()
            // becomes None, but self.current() will be incremented
            self.advance();
        }
        if self.is_at_end() {
            return Err(LoxError::error(
                self.line,
                "Unterminated string".to_string(),
            ));
        }
        // in case there are further characters after the ending "
        self.advance();
        let substr = self.source.get(self.start + 1..self.current - 1);
        let value = substr.map(|s| s.iter().collect::<String>());
        if let Some(res) = value {
            self.add_token_object(TokenType::String, Some(Object::Str(res)));
        } else {
            return Err(LoxError::error(
                self.line,
                "Couldnt parse the value as a string from characters".to_string(),
            ));
        }
        Ok(())
    }
    fn number(&mut self) -> Result<(), LoxError> {
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }

        if let (Some('.'), Some(nxt)) = (self.peek(), self.peek_next()) {
            if nxt.is_ascii_digit() {
                self.advance();
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_digit() {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }
        let number = self
            .source
            // range is exclusive at the end so that means advance
            // is already on the character after the character at which the number ends
            .get(self.start..self.current)
            .unwrap()
            .iter()
            .collect::<String>()
            .parse::<f64>()
            .expect("Could not parse number");

        self.add_token_object(TokenType::Number, Some(Object::Num(number)));
        println!("Current: {}", self.current);
        Ok(())
    }
}
