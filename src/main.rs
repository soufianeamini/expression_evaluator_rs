use std::io;

mod lexer;

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
        println!("{:?}", tokens);
    }
}
