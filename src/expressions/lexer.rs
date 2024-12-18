#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Eq,
    BinaryOp(String),
    UnaryOp(String),
    Literal(Literal),
    CellReference(String, usize),
    ParenOpen,
    ParenClose,
    Comma,
    EOF,
}

pub enum Literal {
    String(String),
    Int(i32),
    Bool(bool),
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
        self.pos += self.peek().map_or(0, |c| c.len_utf8());
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

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        tokens.push(Token::EOF);
        tokens
    }
}
