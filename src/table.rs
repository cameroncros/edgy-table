use crate::renderer::Renderer;
use crate::row::Row;
use std::cmp::max;

pub(crate) struct ColStats {
    pub(crate) max_width: usize,
}

/// Main structure for building a table.
#[derive(Default)]
pub struct Table {
    col_stats: Vec<ColStats>,
    rows: Vec<Row>,
}

impl From<Vec<Row>> for Table {
    fn from(value: Vec<Row>) -> Self {
        let mut table = Table::new();
        value.into_iter().for_each(|r| table.add_row(r));
        table
    }
}


impl Table {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Write the table.
    pub fn render(&self, renderer: &mut Renderer) -> std::io::Result<()> {
        for row in &self.rows {
            row.render(renderer, &self.col_stats)?;
        }
        Ok(())
    }

    /// Add a row to the table.
    pub(crate) fn add_row(&mut self, new_row: Row) {
        new_row.cells.iter().enumerate().for_each(|(i, cell)| {
            let col_stat = match self.col_stats.get_mut(i) {
                None => self.col_stats.push_mut(ColStats { max_width: 0 }),
                Some(cs) => cs,
            };
            col_stat.max_width = max(col_stat.max_width, cell.width());
        });

        self.rows.push(new_row)
    }
}

/// Creates a `Table` from list of `row`s.
///
/// ```
/// # use edgy_table::{seg, row, cell, table};
/// # use edgy_table::{cell::Cell};
/// # use owo_colors::AnsiColors;
/// let christmas_cell = Cell::from(vec![
///     seg!["\t"],
///     seg!["H", AnsiColors::Red],
///     seg!["E", AnsiColors::Green],
///     seg!["L", AnsiColors::Blue],
///     seg!["L", AnsiColors::Green],
///     seg!["O", AnsiColors::Red],
/// ]);
///
/// let christmas_cell2 = Cell::from(vec![
///     seg!["H", AnsiColors::Red],
///     seg!["E", AnsiColors::Green],
///     seg!["L", AnsiColors::Blue],
///     seg!["L", AnsiColors::Green],
///     seg!["O", AnsiColors::Red],
///     seg!["\n", AnsiColors::Red],
///     seg!["W", AnsiColors::Magenta],
///     seg!["O", AnsiColors::Cyan],
///     seg!["R", AnsiColors::Yellow],
///     seg!["L", AnsiColors::Cyan],
///     seg!["D", AnsiColors::Magenta],
/// ]);
///
/// let table = table![
///     row![christmas_cell, cell!["Hello\n\n\n\tWorld"],],
///     row![cell!["Ｈｅｌｌｏ, ｗｏｒｌｄ!"], christmas_cell2,]
/// ];
///```
#[macro_export]
macro_rules! table {
    ($($elem:expr),+ $(,)?) => {
        $crate::table::Table::from(vec![$($elem),+])
    };
}
