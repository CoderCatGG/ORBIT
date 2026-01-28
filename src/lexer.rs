#[derive(Clone, Debug)]
pub enum TokenType {
    ParensOpen,
    ParensClosed,
    BracesOpen,
    BracesClosed,
    Arrow,
    Colon,
    Semicolon,
    Newline,
    Whitespace,
    Identifier(String),
    Integer(String),
    Dot,
    Plus,
    Minus,
    Mul,
    Div,
    Remainder,
    And,
    Or,
    Xor,
    Not,
    Equals,
    NotEquals,
    GreaterThan,
    GreaterEquals,
    LesserThan,
    LesserEquals,
    If,
    Else,
    Loop,
    Function,
    Assign,
    Let,
    While,
    Debug,
}

#[derive(Clone, Debug)]
pub struct DataToken {
    pub ty: TokenType,
    pub pos: (usize /* Line */, usize /* Coloumn */),
}

macro_rules! token {
    ($t:ident, $ln:expr, $ci:expr, $ty:ident) => {
        $t.push(DataToken {
            ty: TokenType::$ty,
            pos: ($ln+1, $ci),
        })
    };
}

pub fn to_tokens(text: &str) -> Vec<DataToken> {
    text.lines().enumerate().flat_map(|(line_num, line)| {
        let mut tokens: Vec<DataToken> = Vec::new();

        let mut char_idx = 0;
        let mut chars = line.chars().peekable();

        while let Some(curr_char) = chars.next() {
            char_idx += 1;
            
            match curr_char {
                '(' => token!(tokens, line_num, char_idx, ParensOpen),
                ')' => token!(tokens, line_num, char_idx, ParensClosed),
                '{' => token!(tokens, line_num, char_idx, BracesOpen),
                '}' => token!(tokens, line_num, char_idx, BracesClosed),
                ':' => token!(tokens, line_num, char_idx, Colon),
                ';' => token!(tokens, line_num, char_idx, Semicolon),
                '.' => token!(tokens, line_num, char_idx, Dot),
                '+' => token!(tokens, line_num, char_idx, Plus),
                '*' => token!(tokens, line_num, char_idx, Mul),
                '/' => token!(tokens, line_num, char_idx, Div),
                '%' => token!(tokens, line_num, char_idx, Remainder),
                '^' => token!(tokens, line_num, char_idx, Xor),
                '=' => if chars.peek() == Some(&'=') {
                    token!(tokens, line_num, char_idx, Equals);
                    chars.next();
                    char_idx += 1;
                } else {
                    token!(tokens, line_num, char_idx, Assign)
                },
                '-' => if chars.peek() == Some(&'>') {
                    token!(tokens, line_num, char_idx, Arrow);
                    chars.next();
                    char_idx += 1;
                } else {
                    token!(tokens, line_num, char_idx, Minus)
                }
                '!' => if chars.peek() == Some(&'=') {
                    token!(tokens, line_num, char_idx, NotEquals);
                    chars.next();
                    char_idx += 1;
                } else {
                    token!(tokens, line_num, char_idx, Not)
                },
                '>' => if chars.peek() == Some(&'=') {
                    token!(tokens, line_num, char_idx, GreaterEquals);
                    chars.next();
                    char_idx += 1;
                } else {
                    token!(tokens, line_num, char_idx, GreaterThan)
                },
                '<' => if chars.peek() == Some(&'<') {
                    token!(tokens, line_num, char_idx, LesserEquals);
                    chars.next();
                    char_idx += 1;
                } else {
                    token!(tokens, line_num, char_idx, LesserThan)
                },
                '&' => if chars.peek() == Some(&'&') {
                    token!(tokens, line_num, char_idx, And);
                    chars.next();
                    char_idx += 1;
                } else {
                    panic!("Invalid `And` Symbol, expected `&&`, got `{:?}`", chars.peek())
                },
                '|' => if chars.peek() == Some(&'|') {
                    token!(tokens, line_num, char_idx, Or);
                    chars.next();
                    char_idx += 1;
                } else {
                    panic!("Invalid `Or` Symbol, expected `||`, got `{:?}`", chars.peek())
                },
                unidentified => if unidentified.is_whitespace() {
                    token!(tokens, line_num, char_idx, Whitespace)
                } else {
                    let op = if unidentified.is_ascii_digit() {
                        char::is_ascii_digit
                    } else {
                        char::is_ascii_alphabetic
                    };

                    let i = char_idx - 1;
                    let mut j = i;

                    #[allow(clippy::while_let_on_iterator)]
                    while let Some(c) = chars.peek() {
                        if !op(c) || c.is_whitespace() {
                            break;
                        } else {
                            chars.next();
                            char_idx += 1;
                            j += 1;
                        }
                    }
                    j += 1;

                    let text = &line[i..j];

                    if unidentified.is_numeric() {
                        tokens.push(DataToken { pos: (line_num+1, i+1), ty: TokenType::Integer(text.to_string()) })
                    } else if unidentified.is_ascii_alphabetic() {
                        match text {
                            "if" => token!(tokens, line_num, i+1, If),
                            "else" => token!(tokens, line_num, i+1, Else),
                            "loop" => token!(tokens, line_num, i+1, Loop),
                            "fn" => token!(tokens, line_num, i+1, Function),
                            "let" => token!(tokens, line_num, i+1, Let),
                            "while" => token!(tokens, line_num, i+1, While),
                            "debug" => token!(tokens, line_num, i+1, Debug),
                            ident => tokens.push(DataToken { pos: (line_num+1, i+1), ty: TokenType::Identifier(ident.to_string()) })
                        }
                    }
                },
            }
        }

        tokens.push(DataToken { ty: TokenType::Newline, pos: (line_num + 1, char_idx + 1)});

        tokens
    }).collect::<Vec<DataToken>>()
}
