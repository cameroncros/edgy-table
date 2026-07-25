use crate::line::{Line, NULL_LINE};
use crate::renderer::Renderer;
use crate::segment::Segment;

/// Empty cell.
pub static NULL_CELL: Cell = Cell { lines: vec![] };

/// Represents a cell.
/// Cells are broken up into multiple lines.
pub struct Cell {
    lines: Vec<Line>,
}

impl From<&str> for Cell {
    fn from(value: &str) -> Self {
        let lines = value
            .split("\n")
            .map(|l| Line::from(Segment::from(l)))
            .collect();
        Cell { lines }
    }
}

impl From<Vec<Segment>> for Cell {
    fn from(value: Vec<Segment>) -> Self {
        let mut cell = Cell::new();
        value
            .into_iter()
            .for_each(|segment: Segment| cell.add(segment));
        cell
    }
}

impl Cell {
    fn new() -> Self {
        Cell { lines: vec![] }
    }

    fn add<S: Into<Segment>>(&mut self, segment: S) {
        let splits = segment.into().split();

        splits.into_iter().enumerate().for_each(|(i, segment)| {
            if i == 0 {
                let last_line = match self.lines.last_mut() {
                    Some(l) => l,
                    None => self.lines.push_mut(Line::new()),
                };
                last_line.add(segment);
            } else {
                self.lines.push(Line::from(segment));
            }
        });
    }

    pub(crate) fn height(&self) -> usize {
        self.lines.len()
    }

    pub(crate) fn width(&self) -> usize {
        self.lines.iter().map(|l| l.width()).max().unwrap_or(0)
    }

    pub(crate) fn render_line(
        &self,
        renderer: &mut Renderer,
        i: usize,
        width: usize,
    ) -> std::io::Result<()> {
        self.lines
            .get(i)
            .unwrap_or(&NULL_LINE)
            .render(renderer, width)
    }
}
#[macro_export]
macro_rules! cell {
    ($value:expr) => {
        Cell::from($value)
    };
}
