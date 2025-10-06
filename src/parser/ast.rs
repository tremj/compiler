/*
supported grammar right now

<program> ::= <function>
<function> ::= "int" <id> "(" ")" "{" <statement> "}"
<statement> ::= "return" <exp> ";"
<exp> ::= <int>

*/

#[derive(Debug, PartialEq)]
pub enum SyntaxNodeType {
    Program,
    Function(String),
    Statement(String),
    Expression(i64),
}

#[derive(Debug, PartialEq)]
pub struct ASTNode {
    children: Vec<ASTNode>,
    node_type: SyntaxNodeType,
}

impl ASTNode {
    pub fn new(node_type: SyntaxNodeType) -> Self {
        ASTNode {
            node_type,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: ASTNode) {
        self.children.push(child);
    }

    pub fn num_child(&mut self) -> u32 {
        return self.children.len() as u32;
    }
}

#[derive(Debug, PartialEq)]
pub struct AST {
    pub root: ASTNode,
}

impl AST {
    pub fn new() -> Self {
        AST {
            root: ASTNode::new(SyntaxNodeType::Program),
        }
    }
}
