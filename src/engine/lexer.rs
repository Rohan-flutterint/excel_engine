use crate::engine::token::Token;

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek();
        self.pos += 1;
        c
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(c) = self.peek() {
            match c {
                '0'..='9' | '.' => tokens.push(self.number()),
                'A'..='Z' | 'a'..='z' => tokens.push(self.ident()),
                '+' => {
                    self.next();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    self.next();
                    tokens.push(Token::Minus);
                }
                '*' => {
                    self.next();
                    tokens.push(Token::Star);
                }
                '/' => {
                    self.next();
                    tokens.push(Token::Slash);
                }
                '(' => {
                    self.next();
                    tokens.push(Token::LParen);
                }
                ')' => {
                    self.next();
                    tokens.push(Token::RParen);
                }
                ':' => {
                    self.next();
                    tokens.push(Token::Colon);
                }
                ',' => {
                    self.next();
                    tokens.push(Token::Comma);
                }
                ' ' => {
                    self.next();
                }
                _ => {
                    self.next();
                }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }

    fn number(&mut self) -> Token {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                s.push(c);
                self.next();
            } else {
                break;
            }
        }
        Token::Number(s.parse().unwrap())
    }

    fn ident(&mut self) -> Token {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                s.push(c);
                self.next();
            } else {
                break;
            }
        }
        Token::Ident(s.to_uppercase())
    }
}
