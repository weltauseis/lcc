#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Constant(i32),
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
    // keywords
    Void,
    Return,
    Int,
}

pub fn lex(source_string: String) -> Vec<Token> {
    let source: Vec<char> = source_string.chars().collect();
    let mut at = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while at < source.len() {
        let c = source[at];

        if c.is_whitespace() {
            at += 1;
            continue;
        }

        match c {
            '_' | 'a'..='z' | 'A'..='Z' => {
                // maybe identifier or keyword

                let identifer_or_keyword = {
                    let mut s = String::new();
                    let mut i = 0;

                    while (at + i) < source.len()
                        && (source[at + i].is_alphanumeric() || source[at + i] == '_')
                    {
                        s.push(source[at + i]);
                        i += 1;
                    }

                    s
                };
                match identifer_or_keyword.as_str() {
                    "int" => {
                        tokens.push(Token::Int);
                        at += 3;
                    }
                    "void" => {
                        tokens.push(Token::Void);
                        at += 4;
                    }
                    "return" => {
                        tokens.push(Token::Return);
                        at += 6;
                    }
                    identifier => {
                        tokens.push(Token::Identifier(identifier.to_owned()));
                        at += identifier.len();
                    }
                }
            }
            '0'..='9' => {
                // maybe integer constant
                let string_constant = {
                    let mut s = String::new();
                    let mut i = 0;

                    while (at + i) < source.len() && (source[at + i].is_numeric()) {
                        s.push(source[at + i]);
                        i += 1;

                        // check if the constant is followed by a alphabetic character,
                        // in order to reject identifiers like "1foo"
                        if source[at + i].is_alphabetic() {
                            panic!("identifiers can't start with numbers !")
                        }
                    }

                    s
                };

                let constant = string_constant.parse::<i32>().unwrap();
                tokens.push(Token::Constant(constant));
                at += string_constant.len();
            }
            '(' => {
                tokens.push(Token::OpenParenthesis);
                at += 1;
            }
            ')' => {
                tokens.push(Token::CloseParenthesis);
                at += 1;
            }
            '{' => {
                tokens.push(Token::OpenBrace);
                at += 1;
            }
            '}' => {
                tokens.push(Token::CloseBrace);
                at += 1;
            }
            ';' => {
                tokens.push(Token::Semicolon);
                at += 1;
            }
            _ => {
                panic!("Unknown character \'{c}\' at {at}.");
            }
        }
    }

    return tokens;
}
