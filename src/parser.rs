use crate::tokens::Token;


#[derive(Debug)]
pub enum ParsedTokens {
    Number(f64),
    Add,
    Sub,
    Mpl,
    Div,
    Print,
    Equal,
    Var(String),
}

impl ParsedTokens {
    pub fn get_number_value(&self) -> Option<&f64> {
        match self {
            Self::Number(n) => Some(n),
            _ => None,
        }
    }
}

fn str_keyword(str: &String) -> Option<ParsedTokens> {
    if str == "print" {
        return Some(ParsedTokens::Print);
    }
    None
}

pub fn parse_tokens(direct_tokens: &Vec<Token>) -> Vec<ParsedTokens> {
    let mut tokens = Vec::new();
    let mut buffer: Vec<&Token> = Vec::new();

    for (index, token) in direct_tokens.iter().enumerate() {
        match token {
            Token::Char(c) => {
                if c.is_numeric() {
                    buffer.push(token);

                    let mut string = String::new();
                    for buff in &buffer {
                        string.push(match buff {
                            Token::Char(c) => c.clone(),
                            Token::Pnt => '.',
                            _ => 0 as char,
                        });
                    }

                    if let Some(t) = direct_tokens.get(index + 1) {
                        if let Token::Ws = t {
                            let number: f64 = string.parse().expect("Cannot parse data");
                            tokens.push(ParsedTokens::Number(number));

                            buffer = Vec::new();
                        }
                    }
                    
                } else {
                    buffer.push(token);

                    let mut string = String::new();
                    for buff in &buffer {
                        string.push(match buff {
                            Token::Char(c) => c.clone(),
                            _ => 0 as char,
                        })
                    }
                    
                    if let Some(t) = direct_tokens.get(index + 1) {
                        if let Token::Nl | Token::Ws = t {

                            match str_keyword(&string) {
                                Some(tok) => tokens.push(tok),
                                None => tokens.push(ParsedTokens::Var(string)),
                            }

                            buffer = Vec::new();
                        }
                    }

                }
            },
            Token::Add => tokens.push(ParsedTokens::Add),
            Token::Sub => tokens.push(ParsedTokens::Sub),
            Token::Div => tokens.push(ParsedTokens::Div),
            Token::Mpl => tokens.push(ParsedTokens::Mpl),
            Token::Equ => tokens.push(ParsedTokens::Equal),
            Token::Pnt => buffer.push(&Token::Pnt),
            Token::Nl => {},
            Token::Ws => {},
        }
    }

    tokens
}
