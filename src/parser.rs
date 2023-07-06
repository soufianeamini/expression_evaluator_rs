use std::collections::VecDeque;
use crate::lexer::Token;

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
        return false;
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
        eprintln!("Error: expected token: {:?}, found: nothing.", tok_type);
    }

    fn primary(&mut self) -> Box<dyn Expression> {
        if self.matchp(Token::INTEGER(0)) {
            return Box::new(
                match self.previous.as_ref().unwrap() {
                    Token::INTEGER(value) => expression::Literal {value: f64::from(*value)},
                    _ => unreachable!(),
                }
            )
        }
        if self.matchp(Token::LPAREN) {
            let expr = self.expression();
            self.consume(Token::RPAREN);
            return expr;
        } else {
            self.error = true;
            eprintln!("Error: Unexpected token: {:?}", self.tokens.front().unwrap());
            return Box::new(expression::Literal {value: 0.});
        }
    }

    fn factor(&mut self) -> Box<dyn Expression> {
        let mut expr = self.primary();

        while self.matchp(Token::STAR) || self.matchp(Token::SLASH) {
            let operator = match self.previous.as_ref().unwrap() {
                Token::STAR => '*',
                Token::SLASH => '/',
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

        while self.matchp(Token::PLUS) || self.matchp(Token::MINUS) {
            let operator = match self.previous.as_ref().unwrap() {
                Token::PLUS => '+',
                Token::MINUS => '-',
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
        self.expression()
    }

    pub fn was_successful(&self) -> bool {
        return !self.error;
    }
}

pub trait Expression {
    fn evaluate(&self) -> f64;
}