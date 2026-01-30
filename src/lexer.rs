use phf::phf_map;

#[derive(Copy, Clone, Debug)]
pub enum Seperator {
    ParensOpen,
    ParensClosed,
    BracesOpen,
    BracesClosed,
    Arrow,
    Colon,
    Semicolon,
    Newline,
    Whitespace,
}

#[derive(Copy, Clone, Debug)]
pub enum Operator {
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
    Assign,
}

#[derive(Copy, Clone, Debug)]
pub enum Keyword {
    If,
    Else,
    Loop,
    Function,
    Let,
    While,
    Debug,
}

static KEYWORD_ATLAS: phf::Map<&str, Keyword> = phf_map!(
    "if" => Keyword::If,
    "else" => Keyword::Else,
    "loop" => Keyword::Loop,
    "fn" => Keyword::Function,
    "let" => Keyword::Let,
    "while" => Keyword::While,
    "debug" => Keyword::Debug,
);

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Token {
    Identifier(String),
    Integer(String),
    Seperator(Seperator),
    Operator(Operator),
    Keyword(Keyword),
}

#[derive(Clone, Debug)]
pub struct DataToken {
    pub ty: Token,
    pub pos: (usize /* Line */, usize /* Coloumn */),
}

macro_rules! operator {
    ($t:ident, $ln:expr, $ci:expr, $ty:ident) => {
        $t.push(DataToken {
            ty: Token::Operator(Operator::$ty),
            pos: ($ln+1, $ci),
        })
    };
}

macro_rules! seperator {
    ($t:ident, $ln:expr, $ci:expr, $ty:ident) => {
        $t.push(DataToken {
            ty: Token::Seperator(Seperator::$ty),
            pos: ($ln+1, $ci),
        })
    };
}

static SINGLE_CHAR_ATLAS: phf::Map<char, Token> = phf_map!(
    '(' => Token::Seperator(Seperator::ParensOpen),
    ')' => Token::Seperator(Seperator::ParensClosed),
    '{' => Token::Seperator(Seperator::BracesOpen),
    '}' => Token::Seperator(Seperator::BracesClosed),
    ':' => Token::Seperator(Seperator::Colon),
    ';' => Token::Seperator(Seperator::Semicolon),
    '.' => Token::Operator(Operator::Dot),
    '+' => Token::Operator(Operator::Plus),
    '*' => Token::Operator(Operator::Mul),
    '/' => Token::Operator(Operator::Div),
    '%' => Token::Operator(Operator::Remainder),
    '^' => Token::Operator(Operator::Xor),
);

pub fn to_tokens(text: &str) -> Vec<DataToken> {
    text.lines().enumerate().flat_map(|(line_num, line)| {
        let mut tokens: Vec<DataToken> = Vec::new();

        let mut char_idx = 0;
        let mut chars = line.chars().peekable();

        while let Some(curr_char) = chars.next() {
            char_idx += 1;
            
            if let Some(token) = SINGLE_CHAR_ATLAS.get(&curr_char) {
                tokens.push(DataToken { ty: token.clone(), pos: (line_num+1, char_idx) })
            } else {
                match curr_char {
                    '=' => if chars.peek() == Some(&'=') {
                        operator!(tokens, line_num, char_idx, Equals);
                        chars.next();
                        char_idx += 1;
                    } else {
                        operator!(tokens, line_num, char_idx, Assign)
                    },
                    '-' => if chars.peek() == Some(&'>') {
                        seperator!(tokens, line_num, char_idx, Arrow);
                        chars.next();
                        char_idx += 1;
                    } else {
                        operator!(tokens, line_num, char_idx, Minus)
                    }
                    '!' => if chars.peek() == Some(&'=') {
                        operator!(tokens, line_num, char_idx, NotEquals);
                        chars.next();
                        char_idx += 1;
                    } else {
                        operator!(tokens, line_num, char_idx, Not)
                    },
                    '>' => if chars.peek() == Some(&'=') {
                        operator!(tokens, line_num, char_idx, GreaterEquals);
                        chars.next();
                        char_idx += 1;
                    } else {
                        operator!(tokens, line_num, char_idx, GreaterThan)
                    },
                    '<' => if chars.peek() == Some(&'<') {
                        operator!(tokens, line_num, char_idx, LesserEquals);
                        chars.next();
                        char_idx += 1;
                    } else {
                        operator!(tokens, line_num, char_idx, LesserThan)
                    },
                    '&' => if chars.peek() == Some(&'&') {
                        operator!(tokens, line_num, char_idx, And);
                        chars.next();
                        char_idx += 1;
                    } else {
                        panic!("Invalid `And` Symbol, expected `&&`, got `{:?}`", chars.peek())
                    },
                    '|' => if chars.peek() == Some(&'|') {
                        operator!(tokens, line_num, char_idx, Or);
                        chars.next();
                        char_idx += 1;
                    } else {
                        panic!("Invalid `Or` Symbol, expected `||`, got `{:?}`", chars.peek())
                    },
                    unidentified => if unidentified.is_whitespace() {
                        seperator!(tokens, line_num, char_idx, Whitespace)
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
                            tokens.push(DataToken { pos: (line_num+1, i+1), ty: Token::Integer(text.to_string()) })
                        } else if unidentified.is_ascii_alphabetic() {
                            if let Some(keyword) = KEYWORD_ATLAS.get(text) {
                                tokens.push(DataToken { pos: (line_num+1, i+1), ty: Token::Keyword(*keyword) });
                            } else {
                                tokens.push(DataToken { pos: (line_num+1, i+1), ty: Token::Identifier(text.to_string()) })
                            }
                        }
                    }
                }
            }
        }

        seperator!(tokens, line_num, char_idx+1, Newline);

        tokens
    }).collect::<Vec<DataToken>>()
}
