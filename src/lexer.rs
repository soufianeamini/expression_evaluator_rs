#[derive(Debug, Clone)]
pub enum Token {
    Integer(i32),
    Plus,
    Minus,
    Star,
    Slash,
    Lparen,
    Rparen,
    Error(String),
}

pub fn lex(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut it = line.trim().chars().peekable();
    let mut str = String::new();
    while let Some(c) = it.next() {
        if c.is_numeric() {
            str.push(c);
            while let Some(n) = it.peek() {
                if n.is_numeric() {
                    str.push(it.next().unwrap());
                } else {
                    break;
                }
            }
            tokens.push(Token::Integer(str.parse().unwrap()));
            str.clear();
            continue;
        }
        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '(' => tokens.push(Token::Lparen),
            ')' => tokens.push(Token::Rparen),
            ' ' | '\t' => continue,
            _   => tokens.push(Token::Error(String::from(c))),
        }
    };
    tokens
}

pub fn resolve(tokens: Vec<Token>) -> Vec<Token> {
    let mut iter = tokens.iter().peekable();
    let mut resolved_tokens = Vec::new();
    let mut previous = None;

    while let Some(token) = iter.next() {
        match *token {
            Token::Lparen => {
                if let Some(prev) = previous {
                    if std::mem::discriminant(prev) == std::mem::discriminant(&Token::Integer(0)) {
                        resolved_tokens.push(Token::Star);
                    }
                }
            }
            Token::Rparen => {
                if let Some(itoken) = iter.peek() {
                    if std::mem::discriminant(*itoken) == std::mem::discriminant(&Token::Integer(0)) {
                        resolved_tokens.push((*token).clone());
                        resolved_tokens.push(Token::Star);
                        continue;
                    }
                }
            }
            _ => (),
        }
        previous = Some(token);
        resolved_tokens.push((*token).clone());
    }

    resolved_tokens
}
