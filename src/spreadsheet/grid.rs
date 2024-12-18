use crate::spreadsheet::cell::Cell;
use crate::spreadsheet::cell::CellValue;

const ROWS: usize = 20;
const COLS: usize = 20;

pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new() -> Self {
        let cells = vec![vec![Cell::new_empty(); COLS]; ROWS];
        Grid { cells }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        if row < ROWS && col < COLS {
            Some(&self.cells[row][col])
        } else {
            None
        }
    }

    pub fn get_mut_cell(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        if row < ROWS && col < COLS {
            Some(&mut self.cells[row][col])
        } else {
            None
        }
    }

    pub fn set_cell_value(&mut self, row: usize, col: usize, value: CellValue) {
        if row < ROWS && col < COLS {
            self.cells[row][col].set_value(value);
        }
    }

    pub fn set_cell_formula(&mut self, row: usize, col: usize, formula: String) {
        if row < ROWS && col < COLS {
            self.cells[row][col].set_formula(formula);
        }
    }

    /* debug printer, replace with curses later */
    pub fn render(&self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                let cell = &self.cells[row][col];
                print!("{:<12}", cell);
            }
            println!();
        }
    }
}
