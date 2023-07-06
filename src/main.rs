use std::io;

mod lexer;

mod parser {
    use std::collections::VecDeque;

    use crate::lexer::Token;
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
            unimplemented!()
        }

        fn factor(&mut self) -> Box<dyn Expression> {
            let mut expr = self.primary();

            while self.matchp(Token::STAR) || self.matchp(Token::SLASH) {
                let right = self.primary();
                let operator = self.previous.as_ref().unwrap();
                expr = Box::new(
                    expression::Binary {
                        left: expr,
                        operator: match operator {
                            Token::STAR => '*',
                            Token::SLASH => '/',
                            _ => unreachable!(),
                        },
                        right,
                    }
                );
            }

            expr
        }

        fn term(&mut self) -> Box<dyn Expression> {
            let expr = self.factor();

            expr
        }

        fn expression(&mut self) -> Box<dyn Expression> {
            self.term()
        }

        pub fn parse(&mut self) -> Box<dyn Expression> {
            self.expression()
        }
    }

    pub trait Expression {
        fn evaluate(&self) -> f64;
    }

    mod expression {
        use super::Expression;
        use crate::parser::Token;

        pub struct Literal {
            pub value: f64,
        }
        
        pub struct Binary {
            pub operator: char,
            pub left: Box<dyn Expression>,
            pub right: Box<dyn Expression>,
        }

        impl Expression for Literal {
            fn evaluate(&self) -> f64 {
                self.value
            }
        }

        impl Expression for Binary {
            fn evaluate(&self) -> f64 {
                match self.operator {
                    '+' => self.left.evaluate() + self.right.evaluate(),
                    '-' => self.left.evaluate() - self.right.evaluate(),
                    '*' => self.left.evaluate() * self.right.evaluate(),
                    '/' => self.left.evaluate() / self.right.evaluate(),
                    _ => 0.,
                }
            }
        }
    }
}

fn main() {
    loop {
        let mut line = String::new();
        match io::stdin()
            .read_line(&mut line) {
                Ok(_) => (),
                Err(_) => {
                    println!("Couldn't read line");
                    std::process::exit(5);
                }
            }
        if line.is_empty() {
            break;
        }
        let tokens = lexer::lex(&line);
        let mut parser = parser::Parser::new(tokens);
        parser.parse();
        // println!("{:?}", tokens);
    }
}
