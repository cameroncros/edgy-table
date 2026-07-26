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
        for r in value {
            table.add_row(r);
        }
        table
    }
}


impl Table {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn render_top(&self, renderer: &mut Renderer) -> std::io::Result<()> {
        let mut border_col_stats = vec![];

        let mut cells = vec![];
        if let Some(tl) = &renderer.theme.top_left {
            cells.push(tl.clone());
            border_col_stats.push(ColStats { max_width: tl.width()});
        }

        for (i, cell) in self.col_stats.iter().enumerate() {
            if i != 0 && let Some(ti) = &renderer.theme.top_intersection {
                cells.push(ti.clone());
                border_col_stats.push(ColStats { max_width: ti.width()});
            }
            if let Some(tb) = &renderer.theme.top_border {
                cells.push(tb.clone());
                border_col_stats.push(ColStats { max_width: cell.max_width});
            }
        }

        if let Some(tr) = &renderer.theme.top_right {
            cells.push(tr.clone());
            border_col_stats.push(ColStats { max_width: tr.width()});
        }

        if !cells.is_empty() {
            let row = Row::from(cells);
            row.render_raw(renderer, &border_col_stats)?;
        }

        Ok(())
    }

    fn render_middle(&self, renderer: &mut Renderer) -> std::io::Result<()> {
        let mut border_col_stats = vec![];

        let mut cells = vec![];
        if let Some(tl) = &renderer.theme.left_intersection {
            cells.push(tl.clone());
            border_col_stats.push(ColStats { max_width: tl.width()});
        }

        for (i, cell) in self.col_stats.iter().enumerate() {
            if i != 0 && let Some(ti) = &renderer.theme.intersection {
                cells.push(ti.clone());
                border_col_stats.push(ColStats { max_width: ti.width()});
            }
            if let Some(tb) = &renderer.theme.horizontal_wall {
                cells.push(tb.clone());
                border_col_stats.push(ColStats { max_width: cell.max_width});
            }
        }

        if let Some(tr) = &renderer.theme.right_intersection {
            cells.push(tr.clone());
            border_col_stats.push(ColStats { max_width: tr.width()});
        }

        if !cells.is_empty() {
            let row = Row::from(cells);
            row.render_raw(renderer, &border_col_stats)?;
        }

        Ok(())
    }

    fn render_bottom(&self, renderer: &mut Renderer) -> std::io::Result<()> {
        let mut border_col_stats = vec![];

        let mut cells = vec![];
        if let Some(tl) = &renderer.theme.bottom_left {
            cells.push(tl.clone());
            border_col_stats.push(ColStats { max_width: tl.width()});
        }

        for (i, cell) in self.col_stats.iter().enumerate() {
            if i != 0 && let Some(ti) = &renderer.theme.bottom_intersection {
                cells.push(ti.clone());
                border_col_stats.push(ColStats { max_width: ti.width()});
            }
            if let Some(tb) = &renderer.theme.bottom_border {
                cells.push(tb.clone());
                border_col_stats.push(ColStats { max_width: cell.max_width});
            }
        }

        if let Some(tr) = &renderer.theme.bottom_right {
            cells.push(tr.clone());
            border_col_stats.push(ColStats { max_width: tr.width()});
        }

        if !cells.is_empty() {
            let row = Row::from(cells);
            row.render_raw(renderer, &border_col_stats)?;
        }

        Ok(())
    }

    /// Write the table.
    ///
    /// # Errors
    ///
    /// Bubbles up errors from `write_fmt`
    pub fn render(&self, renderer: &mut Renderer) -> std::io::Result<()> {
        self.render_top(renderer)?;
        for (i, row) in self.rows.iter().enumerate() {
            if i != 0 {
                self.render_middle(renderer)?;
            }
            row.render(renderer, &self.col_stats)?;
        }
        self.render_bottom(renderer)
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

        self.rows.push(new_row);
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
