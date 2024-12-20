mod tree;
mod grid;
mod cell;
mod lexer;
mod parser;

use grid::Grid;
use tree::Expression;
use cell::CellValue::Int;
use lexer::{Lexer, Token, TokenType};
use parser::Parser;

fn main() {
    let mut grid = Grid::new();

    let expr = Expression::Add(
        Box::new(Expression::LeftShift(
            Box::new(Expression::Integer(2)),
            Box::new(Expression::Add(
                Box::new(Expression::Integer(5)),
                Box::new(Expression::Multiply(
                    Box::new(Expression::Integer(3)),
                    Box::new(Expression::Integer(2)),
                )),
            )),
        )),
        Box::new(Expression::CellRValue(
            Box::new(Expression::Integer(100)),
            Box::new(Expression::Integer(1)),
        )),
    );
    

    let serialized = expr.serialize();
    println!("Serialized: {}", serialized);

    match expr.evaluate(&mut grid) {
        Ok(result) => println!("Evaluation Result: {:?}", result),
        Err(e) => println!("Evaluation Error: {}", e),
    }

    grid.set_cell_value(1, 1, Int(22));

    match expr.evaluate(&mut grid) {
        Ok(result) => println!("Evaluation Result: {:?}", result),
        Err(e) => println!("Evaluation Error: {}", e),
    }

    let input = "5 + 10 * (2.5 + 2) * 1000 + #[1, 1] \"hello world\" #[1, 2] [3, 5] [5000, 49999] >= > >>> ";

    let mut lexer = Lexer::new(input);

    let tokens = lexer.tokenize();
    for token in &tokens {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

match ast {
    Ok(expression) => {
        println!("Serialized: {}", expression.serialize());
        match expression.evaluate(&mut grid) {
            Ok(result) => println!("Evaluation Result: {:?}", result),
            Err(e) => println!("Evaluation Error: {}", e),
        }
    },
    Err(e) => {
        println!("Error parsing AST: {}", e);
    }
}
}
