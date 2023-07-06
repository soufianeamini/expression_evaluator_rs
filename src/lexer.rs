#[derive(Debug)]
pub enum Token {
    INTEGER(i32),
    OPERATOR(char),
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
            '+' => tokens.push(Token::OPERATOR(c)),
            '-' => tokens.push(Token::OPERATOR(c)),
            '*' => tokens.push(Token::OPERATOR(c)),
            '/' => tokens.push(Token::OPERATOR(c)),
            '(' => tokens.push(Token::LPAREN),
            ')' => tokens.push(Token::RPAREN),
            ' ' => continue,
            _   => tokens.push(Token::ERROR(String::from(c))),
        }
    };
    tokens
}
