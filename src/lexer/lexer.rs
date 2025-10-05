#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    LSquigly,  // {
    RSquigly,  // }
    LPar,      // (
    RPar,      // )
    Semicolon, // ;

    Int,    // int
    Return, // return
    If,     // if
    Else,   // else

    Equal,  // =
    GThan,  // >
    GEThan, // >=
    LThan,  // <
    LEThan, // <=

    EOF,

    Identifier(String), // identifier

    IntVal(String),
}

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };
        lex.read_char();

        lex
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'{' => Token::LSquigly,
            b'}' => Token::RSquigly,
            b'(' => Token::LPar,
            b')' => Token::RPar,
            b';' => Token::Semicolon,
            b'=' => Token::Equal,
            b'<' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::LEThan
                } else {
                    Token::LThan
                }
            }
            b'>' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::GEThan
                } else {
                    Token::GThan
                }
            }
            0 => Token::EOF,
            b'0'..=b'9' => Token::IntVal(self.read_int()),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                let tok = match ident.as_str() {
                    "int" => Token::Int,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,

                    _ => Token::Identifier(ident),
                };

                tok
            }

            _ => panic!("Not yet implemented"),
        };

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_int(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() && self.peek().is_ascii_digit() {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..=self.position]).to_string();
    }

    fn peek(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while (self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.ch.is_ascii_digit())
            && (self.peek().is_ascii_alphabetic()
                || self.peek() == b'_'
                || self.peek().is_ascii_digit())
        {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..=self.position]).to_string();
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        loop {
            let tok = self.next_token();
            tokens.push(tok.clone());
            if tok == Token::EOF {
                break;
            }
        }

        tokens
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn lexer_test_single_digit() {
        let input = "int main() {
                return 3;
            }";

        let mut lex = Lexer::new(String::from(input));
        let tokens = vec![
            Token::Int,
            Token::Identifier(String::from("main")),
            Token::LPar,
            Token::RPar,
            Token::LSquigly,
            Token::Return,
            Token::IntVal(String::from("3")),
            Token::Semicolon,
            Token::RSquigly,
        ];

        for token in tokens {
            let next_token = lex.next_token();
            println!("expected: {:?}, received: {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
    }

    #[test]
    fn lexer_test_multiple_digits() {
        let input = "int main() {
                return 135675424;
            }";

        let mut lex = Lexer::new(String::from(input));
        let tokens = vec![
            Token::Int,
            Token::Identifier(String::from("main")),
            Token::LPar,
            Token::RPar,
            Token::LSquigly,
            Token::Return,
            Token::IntVal(String::from("135675424")),
            Token::Semicolon,
            Token::RSquigly,
        ];

        for token in tokens {
            let next_token = lex.next_token();
            println!("expected: {:?}, received: {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
    }

    #[test]
    fn lexer_test_if_else() {
        let input = "<= > = >= if else";
        let mut lex = Lexer::new(String::from(input));

        let tokens = vec![
            Token::LEThan,
            Token::GThan,
            Token::Equal,
            Token::GEThan,
            Token::If,
            Token::Else,
        ];

        for token in tokens {
            let next_token = lex.next_token();
            println!("expected: {:?}, received: {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
    }
}
