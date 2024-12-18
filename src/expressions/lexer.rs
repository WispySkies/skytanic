#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Eq,
    BinaryOp(String),
    UnaryOp(String),
    Literal(Literal),
    CellReference(String),
    Identifier(String),
    ParenOpen,
    ParenClose,
    Comma,
    EOF,
    Error(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Int(i32),
    // Bool(String),
    Float(f64),
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn parse_number(&mut self) -> Token {
        let start_pos = self.pos;
        let mut has_dot = false;
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else if c == '.' && !has_dot {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        let value = &self.input[start_pos..self.pos];
        if has_dot {
            Token::Literal(Literal::Float(value.parse().unwrap()))
        } else {
            Token::Literal(Literal::Int(value.parse().unwrap()))
        }
    }

    fn parse_string(&mut self) -> Token {
        self.advance();
        let start_pos = self.pos;
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            self.advance();
        }
        let value = &self.input[start_pos..self.pos];
        self.advance();
        Token::Literal(Literal::String(value.to_string()))
    }

    fn parse_cell_reference(&mut self) -> Token {
        let mut cell_value = String::new();

        if self.peek() == Some('#') {
            self.advance();
            cell_value.push('#');
        }

        self.advance();
        while let Some(c) = self.peek() {
            if c == ',' || c == ']' {
                break;
            }
            cell_value.push(c);
            self.advance();
        }
        self.advance();

        Token::CellReference(cell_value)
    }

    fn parse_identifier(&mut self) -> Token {
        let start_pos = self.pos;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        Token::Identifier(self.input[start_pos..self.pos].to_string())
    }

    fn parse_operator(&mut self) -> Option<Token> {
        let start_pos = self.pos;
        if let Some(c) = self.peek() {
            match c {
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return Some(Token::BinaryOp("==".to_string()));
                    }
                    return Some(Token::Eq);
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return Some(Token::BinaryOp("<=".to_string()));
                    } else if self.peek() == Some('<') {
                        self.advance();
                        return Some(Token::BinaryOp("<<".to_string()));
                    }
                    return Some(Token::BinaryOp("<".to_string()));
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return Some(Token::BinaryOp(">=".to_string()));
                    } else if self.peek() == Some('>') {
                        self.advance();
                        return Some(Token::BinaryOp(">>".to_string()));
                    }
                    return Some(Token::BinaryOp(">".to_string()));
                }
                '!' => {
                    self.advance();
                    return Some(Token::UnaryOp("!".to_string()));
                }
                '&' => {
                    self.advance();
                    return Some(Token::BinaryOp("&".to_string()));
                }
                '|' => {
                    self.advance();
                    return Some(Token::BinaryOp("|".to_string()));
                }
                '+' | '-' | '*' | '/' | '%' | '^' => {
                    self.advance();
                    return Some(Token::BinaryOp(self.input[start_pos..self.pos].to_string()));
                }
                _ => {
                    return None;
                }
            }
        }
        None
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.peek() {
            self.skip_whitespace();

            if c.is_digit(10) {
                tokens.push(self.parse_number());
            } else if c == '"' {
                tokens.push(self.parse_string());
            } else if c == '[' || c == '#' {
                tokens.push(self.parse_cell_reference());
            } else if c == '=' {
                self.advance();
                tokens.push(Token::Eq);
            } else if c == '(' {
                self.advance();
                tokens.push(Token::ParenOpen);
            } else if c == ')' {
                self.advance();
                tokens.push(Token::ParenClose);
            } else if c == ',' {
                self.advance();
                tokens.push(Token::Comma);
            } else if c.is_alphabetic() || c == '_' {
                tokens.push(self.parse_identifier());
            } else {
                if let Some(op) = self.parse_operator() {
                    tokens.push(op);
                } else {
                    let start_pos = self.pos;
                    self.advance();
                    tokens.push(Token::Error(self.input[start_pos..self.pos].to_string()));
                }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }
}
