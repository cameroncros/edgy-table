use crate::renderer::Renderer;
use crate::segment::Segment;
use std::io::Write;

pub(crate) static NULL_LINE: Line = Line { segments: vec![] };

/// A line from a cell.
pub struct Line {
    segments: Vec<Segment>,
}

impl From<Segment> for Line {
    fn from(value: Segment) -> Self {
        Self {
            segments: vec![value],
        }
    }
}

impl Line {
    pub fn new() -> Self {
        Line { segments: vec![] }
    }

    pub fn add(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    pub fn width(&self) -> usize {
        self.segments.iter().map(|l| l.width()).sum()
    }

    pub fn render(&self, renderer: &mut Renderer, width: usize) -> std::io::Result<()> {
        let mut width_remaining = width;
        for segment in &self.segments {
            width_remaining -= segment.render(renderer, width_remaining)?;
        }
        if width_remaining > 0 {
            renderer.write_fmt(format_args!("{:width_remaining$}", " "))?;
        }
        Ok(())
    }
}
