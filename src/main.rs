use crate::ast::AST;
use crate::ast::lexer::Lexer;
use crate::ast::parser::Parser;

mod ast;
fn main() {
    let input = "7 + 3 * (10 / (12 / (3 + 1) - 1))";

    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    println!("{:?}", tokens);

    let mut ast: AST = AST::new();
    let mut parser = Parser::new();

    while let Some(stmt) = parser.next_statement() {
        ast.add_statement(stmt);
    }
    ast.visualize();
}
