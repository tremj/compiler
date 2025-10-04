use std::fs;

pub mod generator;
pub mod lexer;
pub mod parser;

fn main() {
    let input = fs::read_to_string("./tests/stage_1/invalid/no_space2.c").unwrap();
    let mut lex = lexer::lexer::Lexer::new(input);

    loop {
        let tok = lex.next_token();
        println!("{:?}", tok);
        if tok == lexer::lexer::Token::EOF {
            break;
        }
    }
}
