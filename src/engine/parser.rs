use crate::engine::{ast::Expr, lexer::Lexer, token::Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let tokens = Lexer::new(input).tokenize();
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn next(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        self.pos += 1;
        t
    }

    pub fn parse_expr(&mut self) -> Expr {
        self.parse_bp(0)
    }

    fn parse_bp(&mut self, min_bp: u8) -> Expr {
        let mut lhs = match self.next() {
            Token::Number(n) => Expr::Number(n),
            Token::Ident(name) => {
                if matches!(self.peek(), Token::LParen) {
                    self.next();
                    let mut args = vec![];
                    while !matches!(self.peek(), Token::RParen) {
                        args.push(self.parse_expr());
                        if matches!(self.peek(), Token::Comma) {
                            self.next();
                        }
                    }
                    self.next();
                    Expr::Func(name, args)
                } else if matches!(self.peek(), Token::Colon) {
                    self.next();
                    if let Token::Ident(end) = self.next() {
                        Expr::Range(name, end)
                    } else {
                        Expr::Cell(name)
                    }
                } else {
                    Expr::Cell(name)
                }
            }
            _ => Expr::Number(0.0),
        };

        loop {
            let op = match self.peek() {
                Token::Plus => '+',
                Token::Minus => '-',
                Token::Star => '*',
                Token::Slash => '/',
                _ => break,
            };

            let (l_bp, r_bp) = match op {
                '+' | '-' => (1, 2),
                '*' | '/' => (3, 4),
                _ => (0, 0),
            };

            if l_bp < min_bp {
                break;
            }

            // Prevent advancing past EOF (Excel-safe behavior)
            if matches!(self.peek(), Token::EOF) {
                return lhs; // stop parsing gracefully
            }

            self.next();
            let rhs = self.parse_bp(r_bp);

            lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
        }

        lhs
    }
}

pub fn parse_safe(input: &str) -> Expr {
    let mut parser = Parser::new(input);
    std::panic::catch_unwind(move || parser.parse_expr()).unwrap_or(Expr::Number(0.0))
}
