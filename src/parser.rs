use std::collections::VecDeque;
use crate::lexer::Token;
use expression::Expression;

mod expression;

pub struct Parser {
    tokens: VecDeque<Token>,
    previous: Option<Token>,
    error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into(),
            error: false,
            previous: None,
        }
    }

    fn matchp(&mut self, tok_type: Token) -> bool {
        if let Some(token) = self.tokens.front() {
            if std::mem::discriminant(token) == std::mem::discriminant(&tok_type) {
                self.previous = self.tokens.pop_front();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, tok_type: Token) {
        if let Some(token) = self.tokens.front() {
            if std::mem::discriminant(token) == std::mem::discriminant(&tok_type) {
                self.tokens.pop_front();
            } else {
                self.error = true;
                eprintln!("Error: expected token: {:?}, found: {:?}.", tok_type, token);
            }
            return;
        }
        self.error = true;
        eprintln!("Error: expected token: {:?}, found: nothing.", tok_type);
    }

    fn primary(&mut self) -> Box<dyn Expression> {
        if self.matchp(Token::Integer(0)) {
            return Box::new(
                match self.previous.as_ref().unwrap() {
                    Token::Integer(value) => expression::Literal {value: f64::from(*value)},
                    _ => unreachable!(),
                }
            )
        }
        if self.matchp(Token::Lparen) {
            let expr = self.expression();
            self.consume(Token::Rparen);
            expr
        } else {
            self.error = true;
            let nope;
            let token = match self.tokens.front() {
                Some(val) => val,
                None => {
                    nope = Token::Error(String::from("newline"));
                    &nope
                }
            };
            eprintln!("Error: Unexpected token: {:?}", token);
            Box::new(expression::Literal {value: 0.})
        }
    }

    fn factor(&mut self) -> Box<dyn Expression> {
        let mut expr = self.primary();

        while self.matchp(Token::Star) || self.matchp(Token::Slash) {
            let operator = match self.previous.as_ref().unwrap() {
                Token::Star => '*',
                Token::Slash => '/',
                _ => unreachable!(),
            };
            let right = self.primary();
            expr = Box::new(
                expression::Binary {
                    left: expr,
                    operator,
                    right,
                }
            );
        }

        expr
    }

    fn term(&mut self) -> Box<dyn Expression> {
        let mut expr = self.factor();

        while self.matchp(Token::Plus) || self.matchp(Token::Minus) {
            let operator = match self.previous.as_ref().unwrap() {
                Token::Plus => '+',
                Token::Minus => '-',
                _ => unreachable!(),
            };
            let right = self.factor();
            expr = Box::new(
                expression::Binary {
                    left: expr,
                    operator,
                    right,
                }
            );
        }

        expr
    }

    fn expression(&mut self) -> Box<dyn Expression> {
        self.term()
    }

    pub fn parse(&mut self) -> Box<dyn Expression> {
        self.error = false;
        let expr = self.expression();
        if let Some(tok) = self.tokens.front() {
            self.error = true;
            eprintln!("Error: Unexpected token: {:?}", tok);
        }

        expr
    }

    pub fn was_successful(&self) -> bool {
        !self.error
    }
}
