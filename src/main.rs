mod spreadsheet {
    pub mod cell;
    pub mod grid;
}

use spreadsheet::grid::Grid;
use spreadsheet::cell::CellValue;

fn main() {
    let mut grid = Grid::new();

    grid.set_cell_value(0, 0, CellValue::Int(1));
    grid.set_cell_value(0, 1, CellValue::String("hello world".to_string()));
    grid.set_cell_value(1, 0, CellValue::Bool(true));
    grid.set_cell_value(1, 1, CellValue::Float(1.23));

    grid.set_cell_formula(3, 3, "=A1 + B1".to_string());

    grid.render();
}