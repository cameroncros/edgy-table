use crate::line::{Line, NULL_LINE};
use crate::renderer::Renderer;
use crate::segment::Segment;

/// Empty cell.
pub static NULL_CELL: Cell = Cell { lines: vec![], repeat: false };

/// Represents a cell.
/// Cells are broken up into multiple lines.
#[derive(Clone)]
pub struct Cell {
    pub(crate) repeat: bool,
    lines: Vec<Line>,
}

impl From<&str> for Cell {
    fn from(value: &str) -> Self {
        let mut cell = Cell::new();
        cell.add(Segment::from(value));
        cell
    }
}

impl From<Vec<Segment>> for Cell {
    fn from(value: Vec<Segment>) -> Self {
        let mut cell = Cell::new();
        for segment in value { cell.add(segment); }
        cell
    }
}

impl Cell {
    fn new() -> Self {
        Cell { lines: vec![], repeat: false }
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
        self.lines.iter().map(super::line::Line::width).max().unwrap_or(0)
    }

    pub(crate) fn render_line(
        &self,
        renderer: &mut Renderer,
        i: usize,
        width: usize,
    ) -> std::io::Result<()> {
        if self.repeat {
            self.lines.get(i % self.lines.len()).unwrap_or(&NULL_LINE).render(renderer,width,self.repeat)
        } else {
            self.lines
                .get(i)
                .unwrap_or(&NULL_LINE)
                .render(renderer, width, self.repeat)
        }
    }
}
#[macro_export]
macro_rules! cell {
    ($value:expr) => {
        Cell::from($value)
    };
}

#[macro_export]
macro_rules! border_cell {
    ($value:expr) => {
        {
            let mut cell = Cell::from($value);
            cell.repeat = true;
            cell
        }
    };
}
