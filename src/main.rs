mod tree;

use tree::Expression;

fn main() {
    let expr = Expression::LeftShift(
        Box::new(Expression::Integer(2))
        ,
        Box::new(Expression::Add(
            Box::new(Expression::Integer(5)),
            Box::new(Expression::Multiply(
                Box::new(Expression::Integer(3)),
                Box::new(Expression::Integer(2)),
            )),
        ))
    );

    let serialized = expr.serialize();
    println!("Serialized: {}", serialized);

    match expr.evaluate() {
        Ok(result) => println!("Evaluation Result: {:?}", result),
        Err(e) => println!("Evaluation Error: {}", e),
    }
}
