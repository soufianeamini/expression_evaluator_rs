#[derive(Debug, Clone)]
pub enum Token {
    INTEGER(i32),
    PLUS,
    MINUS,
    STAR,
    SLASH,
    LPAREN,
    RPAREN,
    ERROR(String),
}

pub fn lex(line: &String) -> Vec<Token> {
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
            tokens.push(Token::INTEGER(str.parse().unwrap()));
            str.clear();
            continue;
        }
        match c {
            '+' => tokens.push(Token::PLUS),
            '-' => tokens.push(Token::MINUS),
            '*' => tokens.push(Token::STAR),
            '/' => tokens.push(Token::SLASH),
            '(' => tokens.push(Token::LPAREN),
            ')' => tokens.push(Token::RPAREN),
            ' ' | '\t' => continue,
            _   => tokens.push(Token::ERROR(String::from(c))),
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
            Token::LPAREN => {
                if let Some(prev) = previous {
                    if std::mem::discriminant(prev) == std::mem::discriminant(&Token::INTEGER(0)) {
                        resolved_tokens.push(Token::STAR);
                    }
                }
            }
            Token::RPAREN => {
                if let Some(itoken) = iter.peek() {
                    if std::mem::discriminant(*itoken) == std::mem::discriminant(&Token::INTEGER(0)) {
                        resolved_tokens.push((*token).clone());
                        resolved_tokens.push(Token::STAR);
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
