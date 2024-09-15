use crate::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    Program(Box<ASTNode>),
    Function { name: String, body: Box<ASTNode> },
    Statement(Box<ASTNode>), // all statements are "return exp" for now
    Expression(i32),         // all expressions are just constant ints for now
}

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut at = 0;

    return parse_program(&tokens, &mut at);
}

fn parse_program(tokens: &Vec<Token>, at: &mut usize) -> ASTNode {
    let function = parse_function(tokens, at);

    if *at < tokens.len() {
        panic!("Syntax error : extra stuff after the top-level construct");
    }

    return ASTNode::Program(Box::new(function));
}

fn parse_function(tokens: &Vec<Token>, at: &mut usize) -> ASTNode {
    expect(Token::Int, at, tokens);

    let identifier = parse_identifier(tokens, at);

    expect(Token::OpenParenthesis, at, tokens);
    expect(Token::Void, at, tokens);
    expect(Token::CloseParenthesis, at, tokens);

    expect(Token::OpenBrace, at, tokens);
    let statement = parse_statement(tokens, at);
    expect(Token::CloseBrace, at, tokens);

    return ASTNode::Function {
        name: identifier,
        body: Box::new(statement),
    };
}

fn parse_identifier(tokens: &Vec<Token>, at: &mut usize) -> String {
    if let Token::Identifier(s) = &tokens[*at] {
        *at += 1;
        return s.to_owned();
    }

    panic!("Syntax error : expected identifier, got {:?}", tokens[*at]);
}

fn parse_statement(tokens: &Vec<Token>, at: &mut usize) -> ASTNode {
    expect(Token::Return, at, tokens);

    let expr = parse_expression(tokens, at);

    expect(Token::Semicolon, at, tokens);

    return ASTNode::Statement(Box::new(ASTNode::Expression(expr)));
}

fn parse_expression(tokens: &Vec<Token>, at: &mut usize) -> i32 {
    if let Token::Constant(c) = &tokens[*at] {
        *at += 1;
        return *c;
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
