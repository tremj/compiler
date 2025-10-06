use crate::lexer::lexer::Token;
use crate::parser::ast::*;

pub struct Parser<'a> {
    ast: AST,
    iter: std::slice::Iter<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        let iter = tokens.iter();
        Parser {
            ast: AST::new(),
            iter,
        }
    }

    // Top level parser for program
    pub fn parse(&mut self) -> bool {
        let token = self.iter.next().unwrap();

        let mut code: bool;

        match token {
            Token::Int => {
                code = self.parse();
                if !code {
                    return code;
                }
            }
            Token::Identifier(name) => {
                let res = self.get_function_args();
                code = res.1;
                if !code {
                    return code;
                }
                code = self.parse_function(res.0, name);
                if !code {
                    return code;
                }
                code = self.parse();
                if !code {
                    return code;
                }
            }
            Token::EOF => return true,
            _ => return false,
        }

        true
    }

    fn get_function_args(&mut self) -> (u8, bool) {
        if *self.iter.next().unwrap() != Token::LPar {
            return (0, false);
        } else if *self.iter.next().unwrap() != Token::RPar {
            return (0, false);
        }
        return (0, true);
    }

    fn parse_function(&mut self, _args: u8, name: &String) -> bool {
        if *self.iter.next().unwrap() != Token::LSquigly {
            return false;
        }
        let mut function_node = ASTNode::new(SyntaxNodeType::Function(name.clone()));

        let mut code: bool;

        loop {
            let token = self.iter.next().unwrap();
            match token {
                Token::RSquigly => {
                    code = true;
                    break;
                }
                Token::Return => {
                    code = self.parse_statement(&mut function_node, Token::Return);
                    if !code {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }

        if function_node.num_child() == 0 {
            return false;
        }

        self.ast.root.add_child(function_node);
        code
    }

    fn parse_statement(&mut self, function_node: &mut ASTNode, expression: Token) -> bool {
        let statement_string = match expression {
            Token::Return => { String::from("return") },
            _ => return false,
        };

        let mut statement_node = ASTNode::new(SyntaxNodeType::Statement(statement_string));

        loop {
            let token = self.iter.next().unwrap();
            match token {
                Token::IntVal(value) => {
                    let code = self.parse_int(&mut statement_node, value);
                    if !code {
                        return code;
                    }
                }
                Token::Semicolon => break,
                _ => return false,
            }
        }

        if statement_node.num_child() == 0 {
            return false;
        }

        function_node.add_child(statement_node);
        true
    }

    fn parse_int(&mut self, statement_node: &mut ASTNode, value: &String) -> bool {
        let int_value: i64 = value.parse().unwrap();
        let expression_node = ASTNode::new(SyntaxNodeType::Expression(int_value));
        statement_node.add_child(expression_node);
        true
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::lexer::lexer::Lexer;
    use std::fs;
    use crate::parser::ast::{AST, ASTNode};
    use crate::parser::ast::SyntaxNodeType::{Expression, Function, Statement};

    #[test]
    fn parser_test_valid_multi_digit() {
        let input = fs::read_to_string("./tests/stage_1/valid/multi_digit.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(no_err);

        let mut ast = AST::new();
        let expression_node = ASTNode::new(Expression(100));
        let mut statement_node = ASTNode::new(Statement(String::from("return")));
        statement_node.add_child(expression_node);
        let mut function_node = ASTNode::new(Function(String::from("main")));
        function_node.add_child(statement_node);
        ast.root.add_child(function_node);

        assert_eq!(ast, parser.ast);
    }

    #[test]
    fn parser_test_valid_newlines() {
        let input = fs::read_to_string("./tests/stage_1/valid/newlines.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(no_err);

        let mut ast = AST::new();
        let expression_node = ASTNode::new(Expression(0));
        let mut statement_node = ASTNode::new(Statement(String::from("return")));
        statement_node.add_child(expression_node);
        let mut function_node = ASTNode::new(Function(String::from("main")));
        function_node.add_child(statement_node);
        ast.root.add_child(function_node);

        assert_eq!(ast, parser.ast);
    }

    #[test]
    fn parser_test_valid_no_newlines() {
        let input = fs::read_to_string("./tests/stage_1/valid/no_newlines.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(no_err);

        let mut ast = AST::new();
        let expression_node = ASTNode::new(Expression(0));
        let mut statement_node = ASTNode::new(Statement(String::from("return")));
        statement_node.add_child(expression_node);
        let mut function_node = ASTNode::new(Function(String::from("main")));
        function_node.add_child(statement_node);
        ast.root.add_child(function_node);

        assert_eq!(ast, parser.ast);
    }

    #[test]
    fn parser_test_valid_return_0() {
        let input = fs::read_to_string("./tests/stage_1/valid/return_0.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(no_err);

        let mut ast = AST::new();
        let expression_node = ASTNode::new(Expression(0));
        let mut statement_node = ASTNode::new(Statement(String::from("return")));
        statement_node.add_child(expression_node);
        let mut function_node = ASTNode::new(Function(String::from("main")));
        function_node.add_child(statement_node);
        ast.root.add_child(function_node);

        assert_eq!(ast, parser.ast);
    }

    #[test]
    fn parser_test_valid_return_2() {
        let input = fs::read_to_string("./tests/stage_1/valid/return_2.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(no_err);

        let mut ast = AST::new();
        let expression_node = ASTNode::new(Expression(2));
        let mut statement_node = ASTNode::new(Statement(String::from("return")));
        statement_node.add_child(expression_node);
        let mut function_node = ASTNode::new(Function(String::from("main")));
        function_node.add_child(statement_node);
        ast.root.add_child(function_node);

        assert_eq!(ast, parser.ast);
    }

    #[test]
    fn parser_test_valid_spaces() {
        let input = fs::read_to_string("./tests/stage_1/valid/spaces.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(no_err);

        let mut ast = AST::new();
        let expression_node = ASTNode::new(Expression(0));
        let mut statement_node = ASTNode::new(Statement(String::from("return")));
        statement_node.add_child(expression_node);
        let mut function_node = ASTNode::new(Function(String::from("main")));
        function_node.add_child(statement_node);
        ast.root.add_child(function_node);

        assert_eq!(ast, parser.ast);
    }

    #[test]
    fn parser_test_invalid_missing_paren() {
        let input = fs::read_to_string("./tests/stage_1/invalid/missing_paren.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(!no_err);
    }

    #[test]
    fn parser_test_invalid_missing_retval() {
        let input = fs::read_to_string("./tests/stage_1/invalid/missing_retval.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(!no_err);
    }

    #[test]
    fn parser_test_invalid_no_brace() {
        let input = fs::read_to_string("./tests/stage_1/invalid/no_brace.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(!no_err);
    }

    #[test]
    fn parser_test_invalid_no_semicolon() {
        let input = fs::read_to_string("./tests/stage_1/invalid/no_semicolon.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(!no_err);
    }

    #[test]
    fn parser_test_invalid_no_space() {
        let input = fs::read_to_string("./tests/stage_1/invalid/no_space.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(!no_err);
    }

    #[test]
    fn parser_test_invalid_no_space2() {
        let input = fs::read_to_string("./tests/stage_1/invalid/no_space2.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(!no_err);
    }

    #[test]
    fn parser_test_invalid_wrong_case() {
        let input = fs::read_to_string("./tests/stage_1/invalid/wrong_case.c").unwrap();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let no_err = parser.parse();
        assert!(!no_err);
    }
}
