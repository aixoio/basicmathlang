use std::process;

#[derive(Debug)]
pub enum Token {
    Add,
    Sub,
    Div,
    Mpl,
    Pow,
    Mod,
    Equ,
    Pnt,
    Char(char),
    Nl,
    Ws,
    Min,
}

pub fn parse_to_tokens(data: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let chars = data.chars().collect::<Vec<char>>();

    let mut skip_line = false;

    for (index, char) in chars.iter().enumerate() {
        if *char == '\n' {
            if skip_line {
                skip_line = false;
                continue;
            }
            tokens.push(Token::Nl);
            continue;
        }
        if skip_line {
            continue;
        }
        if char.is_whitespace() {
            tokens.push(Token::Ws);
            continue;
        }
        match char {
            '#' => skip_line = true,
            '+' => tokens.push(Token::Add),
            '-' => {
                let next = chars.get(index + 1);
                if let Some(c) = next {
                    if c.is_whitespace() {
                        tokens.push(Token::Sub);
                    } else {
                        tokens.push(Token::Min);
                    }
                } else {
                    eprintln!("Syntax error");
                    process::exit(1);
                }
            },
            '/' => tokens.push(Token::Div),
            '*' => tokens.push(Token::Mpl),
            '=' => tokens.push(Token::Equ),
            '.' => tokens.push(Token::Pnt),
            '^' => tokens.push(Token::Pow),
            '%' => tokens.push(Token::Mod),
            _ => tokens.push(Token::Char(*char))
        }
    }

    tokens
}
