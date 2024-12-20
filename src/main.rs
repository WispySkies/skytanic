mod tree;
mod grid;
mod cell;

use grid::Grid;
use tree::Expression;
use cell::CellValue::Int;

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
}
