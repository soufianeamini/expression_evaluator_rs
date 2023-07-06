use std::io;

mod lexer;

mod parser {
    use crate::lexer::Token;
    pub struct Parser {
        tokens: Vec<Token>,
        current: Option<Token>,
    }

    impl Parser {
        pub fn new(tokens: Vec<Token>) -> Parser {
            Parser { tokens: tokens, current: None }
        }

        pub fn parse(&self) {}
    }

    trait Expression {
        fn evaluate(&self) -> f64;
    }

    mod expression {
        use super::Expression;

        struct Literal {
            value: f64,
        }
        
        struct Binary {
            operator: char,
            left: Box<dyn Expression>,
            right: Box<dyn Expression>,
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
        let parser = parser::Parser::new(tokens);
        parser.parse();
        // println!("{:?}", tokens);
    }
}
