use crate::lexer::Token;

#[derive(Debug)]
pub enum ASTRootNode {
    Program(ASTFunction),
}

#[derive(Debug)]
pub struct ASTFunction {
    pub name: String,
    pub body: ASTStatement,
}

#[derive(Debug)]
pub enum ASTStatement {
    Return(ASTExpression),
}

#[derive(Debug)]
pub enum ASTExpression {
    Constant(i32),
}

pub fn parse(tokens: Vec<Token>) -> ASTRootNode {
    let mut at = 0;

    return parse_program(&tokens, &mut at);
}

fn parse_program(tokens: &Vec<Token>, at: &mut usize) -> ASTRootNode {
    let function = parse_function(tokens, at);

    if *at < tokens.len() {
        panic!("Syntax error : extra stuff after the top-level construct");
    }

    return ASTRootNode::Program(function);
}

fn parse_function(tokens: &Vec<Token>, at: &mut usize) -> ASTFunction {
    expect(Token::Int, at, tokens);

    let identifier = parse_identifier(tokens, at);

    expect(Token::OpenParenthesis, at, tokens);
    expect(Token::Void, at, tokens);
    expect(Token::CloseParenthesis, at, tokens);

    expect(Token::OpenBrace, at, tokens);
    let statement = parse_statement(tokens, at);
    expect(Token::CloseBrace, at, tokens);

    return ASTFunction {
        name: identifier,
        body: statement,
    };
}

fn parse_identifier(tokens: &Vec<Token>, at: &mut usize) -> String {
    if let Token::Identifier(s) = &tokens[*at] {
        *at += 1;
        return s.to_owned();
    }

    panic!("Syntax error : expected identifier, got {:?}", tokens[*at]);
}

fn parse_statement(tokens: &Vec<Token>, at: &mut usize) -> ASTStatement {
    expect(Token::Return, at, tokens);

    let expr = parse_expression(tokens, at);

    expect(Token::Semicolon, at, tokens);

    return ASTStatement::Return(expr);
}

fn parse_expression(tokens: &Vec<Token>, at: &mut usize) -> ASTExpression {
    if let Token::Constant(c) = &tokens[*at] {
        *at += 1;
        return ASTExpression::Constant(*c);
    }

    panic!("Syntax error : expected expression, got {:?}", tokens[*at]);
}

fn expect(token: Token, at: &mut usize, tokens: &Vec<Token>) {
    assert!(
        token == tokens[*at],
        "Syntax error : expected {token:?}, got {:?}",
        tokens[*at]
    );

    *at += 1;
}
