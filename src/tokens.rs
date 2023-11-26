
#[derive(Debug)]
pub enum Token {
    Add,
    Sub,
    Div,
    Mpl,
    Equ,
    Pnt,
    Char(char),
    Nl,
    Ws,
}

pub fn parse_to_tokens(data: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    for char in data.chars() {
        if char == '\n' {
            tokens.push(Token::Nl);
            continue;
        }
        if char.is_whitespace() {
            tokens.push(Token::Ws);
            continue;
        }
        match char {
            '+' => tokens.push(Token::Add),
            '-' => tokens.push(Token::Sub),
            '/' => tokens.push(Token::Div),
            '*' => tokens.push(Token::Mpl),
            '=' => tokens.push(Token::Equ),
            '.' => tokens.push(Token::Pnt),
            _ => tokens.push(Token::Char(char))
        }
    }

    tokens
}
