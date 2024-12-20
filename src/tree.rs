#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    CellAddress(Box<Expression>, Box<Expression>),

    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
    Exp(Box<Expression>, Box<Expression>),
    Negate(Box<Expression>),

    LAnd(Box<Expression>, Box<Expression>),
    LOr(Box<Expression>, Box<Expression>),
    LNot(Box<Expression>),

    CellLValue(Box<Expression>),
    CellRValue(Box<Expression>),

    BAnd(Box<Expression>, Box<Expression>),
    BOr(Box<Expression>, Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),
    BNot(Box<Expression>),
    LeftShift(Box<Expression>, Box<Expression>),
    RightShift(Box<Expression>, Box<Expression>),

    Equals(Box<Expression>, Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    LessThanEq(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    GreaterThanEq(Box<Expression>, Box<Expression>),

    FTI(Box<Expression>),
    ITF(Box<Expression>),

    Max(Vec<Expression>),
    Min(Vec<Expression>),
    Mean(Vec<Expression>),
    Sum(Vec<Expression>),
}

impl Expression {
    pub fn serialize(&self) -> String {
        match self {
            Expression::Integer(value) => value.to_string(),
            Expression::Float(value) => value.to_string(),
            Expression::Boolean(value) => value.to_string(),
            Expression::String(value) => value.to_string(),
            Expression::CellAddress(col, row) => {
                format!("{}, {}", col.serialize(), row.serialize())
            }

            Expression::Add(lhs, rhs) => format!("({} + {})", lhs.serialize(), rhs.serialize()),
            Expression::Subtract(lhs, rhs) => {
                format!("({} - {})", lhs.serialize(), rhs.serialize())
            }
            Expression::Multiply(lhs, rhs) => {
                format!("({} * {})", lhs.serialize(), rhs.serialize())
            }
            Expression::Divide(lhs, rhs) => format!("({} / {})", lhs.serialize(), rhs.serialize()),
            Expression::Modulo(lhs, rhs) => format!("({} % {})", lhs.serialize(), rhs.serialize()),
            Expression::Exp(lhs, rhs) => format!("({} ^ {})", lhs.serialize(), rhs.serialize()),
            Expression::Negate(expr) => format!("-{}", expr.serialize()),

            Expression::LAnd(lhs, rhs) => format!("({} && {})", lhs.serialize(), rhs.serialize()),
            Expression::LOr(lhs, rhs) => format!("({} || {})", lhs.serialize(), rhs.serialize()),
            Expression::LNot(expr) => format!("!{}", expr.serialize()),

            Expression::CellLValue(celladdr) => format!("([{}])", celladdr.serialize()),
            Expression::CellRValue(celladdr) => format!("(#[{}])", celladdr.serialize()),

            Expression::BAnd(lhs, rhs) => format!("({} & {})", lhs.serialize(), rhs.serialize()),
            Expression::BOr(lhs, rhs) => format!("({} | {})", lhs.serialize(), rhs.serialize()),
            Expression::Xor(lhs, rhs) => format!("({} ^ {})", lhs.serialize(), rhs.serialize()),
            Expression::BNot(expr) => format!("(~{})", expr.serialize()),
            Expression::LeftShift(lhs, rhs) => {
                format!("({} << {})", lhs.serialize(), rhs.serialize())
            }
            Expression::RightShift(lhs, rhs) => {
                format!("({} >> {})", lhs.serialize(), rhs.serialize())
            }

            Expression::Equals(lhs, rhs) => format!("({} == {})", lhs.serialize(), rhs.serialize()),
            Expression::NotEquals(lhs, rhs) => {
                format!("({} != {})", lhs.serialize(), rhs.serialize())
            }
            Expression::LessThan(lhs, rhs) => {
                format!("({} < {})", lhs.serialize(), rhs.serialize())
            }
            Expression::LessThanEq(lhs, rhs) => {
                format!("({} <= {})", lhs.serialize(), rhs.serialize())
            }
            Expression::GreaterThan(lhs, rhs) => {
                format!("({} > {})", lhs.serialize(), rhs.serialize())
            }
            Expression::GreaterThanEq(lhs, rhs) => {
                format!("({} >= {})", lhs.serialize(), rhs.serialize())
            }

            Expression::FTI(expr) => format!("(int({}))", expr.serialize()),
            Expression::ITF(expr) => format!("(float({}))", expr.serialize()),

            Expression::Max(expressions) => {
                let serialized: Vec<String> = expressions.iter().map(|e| e.serialize()).collect();
                format!("max({})", serialized.join(", "))
            }
            Expression::Min(expressions) => {
                let serialized: Vec<String> = expressions.iter().map(|e| e.serialize()).collect();
                format!("min({})", serialized.join(", "))
            }
            Expression::Mean(expressions) => {
                let serialized: Vec<String> = expressions.iter().map(|e| e.serialize()).collect();
                format!("avg({})", serialized.join(", "))
            }
            Expression::Sum(expressions) => {
                let serialized: Vec<String> = expressions.iter().map(|e| e.serialize()).collect();
                format!("sum({})", serialized.join(", "))
            }
        }
    }

    pub fn evaluate(&self) -> Result<Expression, String> {
        match self {
            Expression::Integer(_)
            | Expression::Float(_)
            | Expression::Boolean(_)
            | Expression::String(_) => Ok(self.clone()),

            Expression::CellAddress(_, _) => Ok(Expression::String("CellAddr".to_string())),

            Expression::Add(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Integer(l + r))
                    }
                    (Expression::Float(l), Expression::Float(r)) => Ok(Expression::Float(l + r)),
                    (Expression::Integer(l), Expression::Float(r)) => {
                        Ok(Expression::Float(l as f64 + r))
                    }
                    (Expression::Float(l), Expression::Integer(r)) => {
                        Ok(Expression::Float(l + r as f64))
                    }
                    _ => Err("Incompatible types for addition".to_string()),
                }
            }

            Expression::Subtract(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Integer(l - r))
                    }
                    (Expression::Float(l), Expression::Float(r)) => Ok(Expression::Float(l - r)),
                    (Expression::Integer(l), Expression::Float(r)) => {
                        Ok(Expression::Float(l as f64 - r))
                    }
                    (Expression::Float(l), Expression::Integer(r)) => {
                        Ok(Expression::Float(l - r as f64))
                    }
                    _ => Err("Incompatible types for subtraction".to_string()),
                }
            }

            Expression::Multiply(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Integer(l * r))
                    }
                    (Expression::Float(l), Expression::Float(r)) => Ok(Expression::Float(l * r)),
                    (Expression::Integer(l), Expression::Float(r)) => {
                        Ok(Expression::Float(l as f64 * r))
                    }
                    (Expression::Float(l), Expression::Integer(r)) => {
                        Ok(Expression::Float(l * r as f64))
                    }
                    _ => Err("Incompatible types for multiplication".to_string()),
                }
            }

            Expression::Divide(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        if r == 0 {
                            Err("Divide by zero error".to_string())
                        } else {
                            Ok(Expression::Integer(l / r))
                        }
                    }
                    (Expression::Float(l), Expression::Float(r)) => {
                        if r == 0.0 {
                            Err("Divide by zero error".to_string())
                        } else {
                            Ok(Expression::Float(l / r))
                        }
                    }
                    (Expression::Integer(l), Expression::Float(r)) => {
                        if r == 0.0 {
                            Err("Divide by zero error".to_string())
                        } else {
                            Ok(Expression::Float(l as f64 / r))
                        }
                    }
                    (Expression::Float(l), Expression::Integer(r)) => {
                        if r == 0 {
                            Err("Divide by zero error".to_string())
                        } else {
                            Ok(Expression::Float(l / r as f64))
                        }
                    }
                    _ => Err("Incompatible types for division".to_string()),
                }
            }

            Expression::Modulo(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        if r == 0 {
                            Err("Modulo by zero error".to_string())
                        } else {
                            Ok(Expression::Integer(l % r))
                        }
                    }
                    _ => Err("Modulo operation only valid on integers".to_string()),
                }
            }

            Expression::Exp(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Integer(l.pow(r as u32)))
                    }
                    (Expression::Float(l), Expression::Float(r)) => {
                        Ok(Expression::Float(l.powf(r)))
                    }
                    (Expression::Integer(l), Expression::Float(r)) => {
                        Ok(Expression::Float((l as f64).powf(r)))
                    }
                    (Expression::Float(l), Expression::Integer(r)) => {
                        Ok(Expression::Float(l.powf(r as f64)))
                    }
                    _ => Err("Incompatible types for exponentiation".to_string()),
                }
            }

            Expression::Negate(expr) => {
                let evaluated_expr = expr.evaluate()?;
                match evaluated_expr {
                    Expression::Integer(i) => Ok(Expression::Integer(-i)),
                    Expression::Float(f) => Ok(Expression::Float(-f)),
                    _ => Err("Negate operation only valid on numeric types".to_string()),
                }
            }

            Expression::LAnd(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Boolean(l), Expression::Boolean(r)) => {
                        Ok(Expression::Boolean(l && r))
                    }
                    _ => Err("Logical AND only valid on boolean values".to_string()),
                }
            }

            Expression::LOr(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Boolean(l), Expression::Boolean(r)) => {
                        Ok(Expression::Boolean(l || r))
                    }
                    _ => Err("Logical OR only valid on boolean values".to_string()),
                }
            }

            Expression::LNot(expr) => {
                let expr = expr.evaluate()?;
                match expr {
                    Expression::Boolean(b) => Ok(Expression::Boolean(!b)),
                    _ => Err("Logical NOT only valid on booleans".to_string()),
                }
            }

            Expression::CellLValue(celladdr) => Ok(Expression::String("[cell]".to_string())),
            Expression::CellRValue(celladdr) => Ok(Expression::String("#[cell]".to_string())),

            Expression::BAnd(lhs, rhs) => {
                let left = lhs.evaluate()?;
                let right = rhs.evaluate()?;

                match (left, right) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Integer(l & r))
                    }
                    _ => Err("Incompatible types for Bitwise AND".to_string()),
                }
            }

            Expression::BOr(lhs, rhs) => {
                let left = lhs.evaluate()?;
                let right = rhs.evaluate()?;

                match (left, right) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Integer(l | r))
                    }
                    _ => Err("Incompatible types for Bitwise OR".to_string()),
                }
            }

            Expression::Xor(lhs, rhs) => {
                let left = lhs.evaluate()?;
                let right = rhs.evaluate()?;

                match (left, right) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Integer(l ^ r))
                    }
                    _ => Err("Incompatible types for Bitwise XOR".to_string()),
                }
            }

            Expression::BNot(expr) => {
                let evaluated = expr.evaluate()?;
                match evaluated {
                    Expression::Integer(i) => Ok(Expression::Integer(!i)),
                    _ => Err("Incompatible type for Bitwise NOT".to_string()),
                }
            }

            Expression::LeftShift(lhs, rhs) => {
                let left = lhs.evaluate()?;
                let right = rhs.evaluate()?;

                match (left, right) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Integer(l << r))
                    }
                    _ => Err("Incompatible types for Left Shift".to_string()),
                }
            }

            Expression::RightShift(lhs, rhs) => {
                let left = lhs.evaluate()?;
                let right = rhs.evaluate()?;

                match (left, right) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Integer(l >> r))
                    }
                    _ => Err("Incompatible types for Right Shift".to_string()),
                }
            }

            Expression::Equals(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Boolean(l == r))
                    }
                    (Expression::Float(l), Expression::Float(r)) => Ok(Expression::Boolean(l == r)),
                    (Expression::Boolean(l), Expression::Boolean(r)) => {
                        Ok(Expression::Boolean(l == r))
                    }
                    (Expression::String(l), Expression::String(r)) => {
                        Ok(Expression::Boolean(l == r))
                    }
                    _ => Err("Incompatible types for equality comparison".to_string()),
                }
            }

            Expression::NotEquals(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Boolean(l != r))
                    }
                    (Expression::Float(l), Expression::Float(r)) => Ok(Expression::Boolean(l != r)),
                    (Expression::Boolean(l), Expression::Boolean(r)) => {
                        Ok(Expression::Boolean(l != r))
                    }
                    (Expression::String(l), Expression::String(r)) => {
                        Ok(Expression::Boolean(l != r))
                    }
                    _ => Err("Incompatible types for inequality comparison".to_string()),
                }
            }

            Expression::LessThan(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Boolean(l < r))
                    }
                    (Expression::Float(l), Expression::Float(r)) => Ok(Expression::Boolean(l < r)),
                    _ => Err("Incompatible types for less-than comparison".to_string()),
                }
            }

            Expression::LessThanEq(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Boolean(l <= r))
                    }
                    (Expression::Float(l), Expression::Float(r)) => Ok(Expression::Boolean(l <= r)),
                    _ => Err("Incompatible types for less-than-or-equal comparison".to_string()),
                }
            }

            Expression::GreaterThan(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Boolean(l > r))
                    }
                    (Expression::Float(l), Expression::Float(r)) => Ok(Expression::Boolean(l > r)),
                    _ => Err("Incompatible types for greater-than comparison".to_string()),
                }
            }

            Expression::GreaterThanEq(lhs, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                match (lhs, rhs) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        Ok(Expression::Boolean(l >= r))
                    }
                    (Expression::Float(l), Expression::Float(r)) => Ok(Expression::Boolean(l >= r)),
                    _ => Err("Incompatible types for greater-than-or-equal comparison".to_string()),
                }
            }

            Expression::FTI(expr) => {
                let evaluated_expr = expr.evaluate()?;
                match evaluated_expr {
                    Expression::Integer(i) => Ok(Expression::Float(i as f64)),
                    _ => Err("FTI operation only valid on integers".to_string()),
                }
            }

            Expression::ITF(expr) => {
                let evaluated_expr = expr.evaluate()?;
                match evaluated_expr {
                    Expression::Float(f) => Ok(Expression::Integer(f as i64)),
                    _ => Err("ITF operation only valid on floats".to_string()),
                }
            }

            Expression::Max(expressions) => {
                let evaluated: Result<Vec<Expression>, String> =
                    expressions.iter().map(|e| e.evaluate()).collect();
                let evaluated = evaluated?;
                let mut max_value = &evaluated[0];
                for expr in &evaluated[1..] {
                    match (max_value, expr) {
                        (Expression::Integer(l), Expression::Integer(r)) => {
                            if r > l {
                                max_value = expr;
                            }
                        }
                        (Expression::Float(l), Expression::Float(r)) => {
                            if r > l {
                                max_value = expr;
                            }
                        }
                        _ => return Err("Incompatible types in Max".to_string()),
                    }
                }
                Ok(max_value.clone())
            }

            Expression::Min(expressions) => {
                let evaluated: Result<Vec<Expression>, String> =
                    expressions.iter().map(|e| e.evaluate()).collect();
                let evaluated = evaluated?;
                let mut min_value = &evaluated[0];
                for expr in &evaluated[1..] {
                    match (min_value, expr) {
                        (Expression::Integer(l), Expression::Integer(r)) => {
                            if r < l {
                                min_value = expr;
                            }
                        }
                        (Expression::Float(l), Expression::Float(r)) => {
                            if r < l {
                                min_value = expr;
                            }
                        }
                        _ => return Err("Incompatible types in Min".to_string()),
                    }
                }
                Ok(min_value.clone())
            }

            Expression::Mean(expressions) => {
                let evaluated: Result<Vec<Expression>, String> =
                    expressions.iter().map(|e| e.evaluate()).collect();
                let evaluated = evaluated?;
                let sum = evaluated.iter().try_fold(0.0, |acc, e| match e {
                    Expression::Integer(i) => Ok(acc + *i as f64),
                    Expression::Float(f) => Ok(acc + *f),
                    _ => Err("Incompatible types in Mean".to_string()),
                })?;
                let mean = sum / evaluated.len() as f64;
                Ok(Expression::Float(mean))
            }

            Expression::Sum(expressions) => {
                let evaluated: Result<Vec<Expression>, String> =
                    expressions.iter().map(|e| e.evaluate()).collect();
                let evaluated = evaluated?;
                let sum = evaluated.iter().try_fold(0, |acc, e| match e {
                    Expression::Integer(i) => Ok(acc + *i),
                    Expression::Float(f) => Ok(acc + *f as i64),
                    _ => Err("Incompatible types in Sum".to_string()),
                })?;
                Ok(Expression::Integer(sum))
            }
        }
    }
}
