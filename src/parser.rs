use crate::Expression;
use crate::TokenType;
use crate::Token;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current_index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expression, String> {
        self.logical_or()
    }

    fn logical_or(&mut self) -> Result<Expression, String> {
        let mut left = self.logical_and()?;

        while self.has(TokenType::BinaryOp("||".to_string())) {
            self.advance();
            let right = self.logical_and()?;
            left = Expression::LOr(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn logical_and(&mut self) -> Result<Expression, String> {
        let mut left = self.bitwise_or()?;

        while self.has(TokenType::BinaryOp("&&".to_string())) {
            self.advance();
            let right = self.bitwise_or()?;
            left = Expression::LAnd(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn bitwise_or(&mut self) -> Result<Expression, String> {
        let mut left = self.bitwise_xor()?;

        while self.has(TokenType::BinaryOp("|".to_string())) {
            self.advance();
            let right = self.bitwise_xor()?;
            left = Expression::BOr(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn bitwise_xor(&mut self) -> Result<Expression, String> {
        let mut left = self.bitwise_and()?;

        while self.has(TokenType::BinaryOp("^".to_string())) {
            self.advance();
            let right = self.bitwise_and()?;
            left = Expression::Xor(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn bitwise_and(&mut self) -> Result<Expression, String> {
        let mut left = self.shift()?;

        while self.has(TokenType::BinaryOp("&".to_string())) {
            self.advance();
            let right = self.shift()?;
            left = Expression::BAnd(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn shift(&mut self) -> Result<Expression, String> {
        let mut left = self.additive()?;

        while self.has(TokenType::BinaryOp("<<".to_string())) || self.has(TokenType::BinaryOp(">>".to_string())) {
            let op_token = self.tokens[self.current_index].clone();
            self.advance();
            let right = self.additive()?;
            if op_token.text == "<<" {
                left = Expression::LeftShift(Box::new(left), Box::new(right));
            } else {
                left = Expression::RightShift(Box::new(left), Box::new(right));
            }
        }

        Ok(left)
    }

    fn additive(&mut self) -> Result<Expression, String> {
        let mut left = self.multiplicative()?;

        while self.has(TokenType::BinaryOp("+".to_string())) || self.has(TokenType::BinaryOp("-".to_string())) {
            let op_token = self.tokens[self.current_index].clone();
            self.advance();
            let right = self.multiplicative()?;
            if op_token.text == "+" {
                left = Expression::Add(Box::new(left), Box::new(right));
            } else {
                left = Expression::Subtract(Box::new(left), Box::new(right));
            }
        }

        Ok(left)
    }

    fn multiplicative(&mut self) -> Result<Expression, String> {
        let mut left = self.exponentiation()?;

        while self.has(TokenType::BinaryOp("*".to_string())) || self.has(TokenType::BinaryOp("/".to_string())) || self.has(TokenType::BinaryOp("%".to_string())) {
            let op_token = self.tokens[self.current_index].clone();
            self.advance();
            let right = self.exponentiation()?;
            if op_token.text == "*" {
                left = Expression::Multiply(Box::new(left), Box::new(right));
            } else if op_token.text == "/" {
                left = Expression::Divide(Box::new(left), Box::new(right));
            } else {
                left = Expression::Modulo(Box::new(left), Box::new(right));
            }
        }

        Ok(left)
    }

    fn exponentiation(&mut self) -> Result<Expression, String> {
        let mut left = self.unary()?;

        while self.has(TokenType::BinaryOp("^".to_string())) {
            self.advance();
            let right = self.unary()?;
            left = Expression::Exp(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expression, String> {
        if self.has(TokenType::UnaryOp("-".to_string())) {
            self.advance();
            let expr = self.unary()?;
            return Ok(Expression::Negate(Box::new(expr)));
        }
        if self.has(TokenType::UnaryOp("~".to_string())) {
            self.advance();
            let expr = self.unary()?;
            return Ok(Expression::BNot(Box::new(expr)));
        }
        if self.has(TokenType::UnaryOp("!".to_string())) {
            self.advance();
            let expr = self.unary()?;
            return Ok(Expression::LNot(Box::new(expr)));
        }
        if self.has(TokenType::UnaryOp("int".to_string())) {
            self.advance();
            let expr = self.unary()?;
            return Ok(Expression::FTI(Box::new(expr)));
        }
        if self.has(TokenType::UnaryOp("float".to_string())) {
            self.advance();
            let expr = self.unary()?;
            return Ok(Expression::ITF(Box::new(expr)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, String> {
        if self.has(TokenType::IntegerLiteral) {
            let token = self.tokens[self.current_index].clone();
            self.advance();
            return Ok(Expression::Integer(token.text.parse().unwrap()));
        }
        if self.has(TokenType::FloatLiteral) {
            let token = self.tokens[self.current_index].clone();
            self.advance();
            return Ok(Expression::Float(token.text.parse().unwrap()));
        }
        if self.has(TokenType::BooleanLiteral) {
            let token = self.tokens[self.current_index].clone();
            self.advance();
            return Ok(Expression::Boolean(token.text == "true"));
        }
        if self.has(TokenType::StringLiteral) {
            let token = self.tokens[self.current_index].clone();
            self.advance();
            return Ok(Expression::String(token.text));
        }
        if self.has(TokenType::ParenOpen) {
            self.advance();
            let expr = self.expression()?;
            if !self.has(TokenType::ParenClose) {
                return Err("Expected closing parenthesis".to_string());
            }
            self.advance();
            return Ok(expr);
        }
        Err("Unexpected token".to_string())
    }

    fn has(&mut self, token_type: TokenType) -> bool {
        if self.current_index >= self.tokens.len() {
            return false;
        }

        self.tokens[self.current_index].token_type == token_type
    }

    fn advance(&mut self) {
        if self.current_index < self.tokens.len() {
            self.current_index += 1;
        }
    }
}
