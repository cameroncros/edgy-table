use crate::cell::{Cell, NULL_CELL};
use crate::renderer::Renderer;
use crate::table::ColStats;
use std::io::Write;

/// Represents a row of cells.
pub struct Row {
    max_height: usize,
    pub(crate) cells: Vec<Cell>,
}

impl Row {
    pub(crate) fn render(
        &self,
        renderer: &mut Renderer,
        col_stats: &[ColStats],
    ) -> std::io::Result<()> {
        let theme = renderer.theme.clone();
        for line in 0..self.max_height {
            if let Some(lw) = &theme.left_wall {
                lw.render_line(renderer, line, lw.width())?;
            }
            for (i, column) in col_stats.iter().enumerate() {
                if i != 0 && let Some(vw) = &theme.vertical_wall {
                    vw.render_line(renderer, line, vw.width())?;
                }
                let cell = self.cells.get(i).unwrap_or(&NULL_CELL);
                cell.render_line(renderer, line, column.max_width)?;
            }
            if let Some(rw) = &theme.right_wall {
                rw.render_line(renderer, line, rw.width())?;
            }
            renderer.write_all(b"\n")?;
        }
        Ok(())
    }
    
    pub(crate) fn render_raw(
        &self,
        renderer: &mut Renderer,
        col_stats: &[ColStats],
    ) -> std::io::Result<()> {
        for line in 0..self.max_height {
            for (i, column) in col_stats.iter().enumerate() {
                let cell = self.cells.get(i).unwrap_or(&NULL_CELL);
                cell.render_line(renderer, line, column.max_width)?;
            }
            renderer.write_all(b"\n")?;
        }
        Ok(())
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
