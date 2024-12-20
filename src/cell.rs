use std::fmt;

#[derive(Clone, Debug)]
pub enum CellValue {
    String(String),
    Int(i64),
    Bool(bool),
    Float(f64),
}

impl Default for CellValue {
    fn default() -> Self {
        CellValue::Int(0)
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    value: CellValue, /* resolved from formula */
    formula: Option<String>,
}

impl Cell {
    pub fn new_empty() -> Self {
        Cell {
            value: CellValue::Int(0),
            formula: None,
        }
    }

    pub fn new(value: CellValue) -> Self {
        Cell {
            value,
            formula: None,
        }
    }

    pub fn set_value(&mut self, value: CellValue) {
        self.value = value;
    }

    pub fn set_formula(&mut self, formula: String) {
        self.formula = Some(formula);
    }

    pub fn get_value(&self) -> &CellValue {
        &self.value
    }

    pub fn get_formula(&self) -> Option<&String> {
        self.formula.as_ref()
    }

    pub fn evaluate(&self) -> String {
        match &self.value {
            CellValue::String(s) => s.clone(),
            CellValue::Int(i) => i.to_string(),
            CellValue::Bool(b) => b.to_string(),
            CellValue::Float(f) => f.to_string(),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.evaluate())
    }
}
