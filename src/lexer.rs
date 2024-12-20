use std::str::Chars;

#[derive(Debug, Clone)]
pub enum TokenType {
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    BooleanLiteral,
    CellReference,
    Identifier,
    Eq,
    BinaryOp(String),
    UnaryOp(String),
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    Comma,
    EOF,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub text: String,
    pub start_index: usize,
    pub end_index: usize,
}

impl Token {
    pub fn new(token_type: TokenType, text: String, start_index: usize, end_index: usize) -> Self {
        Token {
            token_type,
            text,
            start_index,
            end_index,
        }
    }
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
    current_index: usize,
    start_index: usize,
    text: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current_char: None,
            current_index: 0,
            start_index: 0,
            text: input,
        };
        lexer.advance(); // Initialize the first character
        lexer
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
        if let Some(c) = self.current_char {
            self.current_index += c.len_utf8();
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.clone().next()
    }

    fn capture<F>(&mut self, mut predicate: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if predicate(c) {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.current_char {
            self.start_index = self.current_index;

            let token = match c {
                '0'..='9' => self.lex_number(),
                '"' => self.lex_string(),
                '=' => {
                    self.advance();
                    Token::new(TokenType::Eq, "=".to_string(), self.start_index, self.current_index)
                }
                '+' | '-' | '*' | '/' | '%' | '^' | '&' | '|' => {
                    self.lex_binary_op(c)
                }
                '~' => {
                    self.advance();
                    Token::new(TokenType::UnaryOp("~".to_string()), "~".to_string(), self.start_index, self.current_index)
                }
                '<' | '>' | '!' => self.lex_comparison(),
                '(' => {
                    self.advance();
                    Token::new(TokenType::ParenOpen, "(".to_string(), self.start_index, self.current_index)
                }
                ')' => {
                    self.advance();
                    Token::new(TokenType::ParenClose, ")".to_string(), self.start_index, self.current_index)
                }
                ',' => {
                    self.advance();
                    Token::new(TokenType::Comma, ",".to_string(), self.start_index, self.current_index)
                }
                '#' => {
                    self.advance();
                    Token::new(TokenType::CellReference, "#".to_string(), self.start_index, self.current_index)
                }
                '[' => {
                    self.advance();
                    Token::new(TokenType::BracketOpen, "[".to_string(), self.start_index, self.current_index)
                }
                ']' => {
                    self.advance();
                    Token::new(TokenType::BracketClose, "]".to_string(), self.start_index, self.current_index)
                }
                c if c.is_whitespace() => {
                    self.advance();
                    continue;
                }
                c if c.is_alphabetic() || c == '_' => self.lex_identifier_or_boolean(),
                _ => {
                    self.advance();
                    Token::new(TokenType::Unknown, c.to_string(), self.start_index, self.current_index)
                }
            };
            tokens.push(token);
        }

        tokens.push(Token::new(TokenType::EOF, "".to_string(), self.current_index, self.current_index));
        tokens
    }

    fn lex_number(&mut self) -> Token {
        let mut number = self.capture(|c| c.is_digit(10));
        if let Some('.') = self.current_char {
            self.advance();
            number.push('.');
            number.push_str(&self.capture(|c| c.is_digit(10)));
            Token::new(TokenType::FloatLiteral, number, self.start_index, self.current_index)
        } else {
            Token::new(TokenType::IntegerLiteral, number, self.start_index, self.current_index)
        }
    }

    fn lex_string(&mut self) -> Token {
        self.advance();
        let content = self.capture(|c| c != '"');
        if self.current_char == Some('"') {
            self.advance();
        }
        Token::new(TokenType::StringLiteral, content, self.start_index, self.current_index)
    }

    fn lex_binary_op(&mut self, op: char) -> Token {
        self.advance();
        Token::new(TokenType::BinaryOp(op.to_string()), op.to_string(), self.start_index, self.current_index)
    }

    fn lex_comparison(&mut self) -> Token {
        let first_char = self.current_char.unwrap();
        self.advance();
        if self.current_char == Some('=') {
            let op = format!("{}=", first_char);
            self.advance();
            Token::new(
                TokenType::BinaryOp(op.clone()),
                op,
                self.start_index,
                self.current_index,
            )
        } else if first_char == '<' && self.current_char == Some('<') {
            self.advance();
            Token::new(TokenType::BinaryOp("<<".to_string()), "<<".to_string(), self.start_index, self.current_index)
        } else if first_char == '>' && self.current_char == Some('>') {
            self.advance();
            Token::new(TokenType::BinaryOp(">>".to_string()), ">>".to_string(), self.start_index, self.current_index)
        } else {
            Token::new(TokenType::BinaryOp(first_char.to_string()), first_char.to_string(), self.start_index, self.current_index)
        }
    }

    fn lex_identifier_or_boolean(&mut self) -> Token {
        let text = self.capture(|c| c.is_alphanumeric() || c == '_');
        let token_type = match text.as_str() {
            "true" | "false" => TokenType::BooleanLiteral,
            _ => TokenType::Identifier,
        };
        Token::new(token_type, text, self.start_index, self.current_index)
    }
}