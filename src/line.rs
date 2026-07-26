use crate::renderer::Renderer;
use crate::segment::Segment;
use std::io::Write;

pub(crate) static NULL_LINE: Line = Line { segments: vec![] };

/// A line from a cell.
#[derive(Clone)]
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
        self.segments.iter().map(super::segment::Segment::width).sum()
    }

    pub fn render(&self, renderer: &mut Renderer, width: usize, repeat: bool) -> std::io::Result<()> {
        let mut width_remaining = width;
        loop {
            for segment in &self.segments {
                width_remaining = width_remaining.saturating_sub(segment.render(renderer, width_remaining)?);
            }
            if !repeat || width_remaining == 0 || self.segments.is_empty() {
                break
            }
        }
        if width_remaining != 0 {
            renderer.write_fmt(format_args!("{:width_remaining$}", " "))?;
        }
        Ok(())
    }
}
