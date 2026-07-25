use crate::cell::{Cell, NULL_CELL};
use crate::table::ColStats;

/// Represents a row of cells.
pub struct Row {
    max_height: usize,
    pub(crate) cells: Vec<Cell>,
}

impl Row {
    pub(crate) fn render(&self, col_stats: &[ColStats]) {
        for line in 0..self.max_height {
            for (i, column) in col_stats.iter().enumerate() {
                let cell = self.cells.get(i).unwrap_or(&NULL_CELL);
                cell.render_line(line, column.max_width);
                print!("  ");
            }
            println!()
        }
    }
}

impl From<Vec<Cell>> for Row {
    fn from(cells: Vec<Cell>) -> Self {
        let max_height = cells.iter().map(Cell::height).max().unwrap();
        Self { max_height, cells }
    }
}

/// Creates a `Row` from a list of `Cell`s.
#[macro_export]
macro_rules! row {
    ($($elem:expr),+ $(,)?) => {
        $crate::row::Row::from(vec![$($elem),+])
    };
}
