use crate::segment::Segment;

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

    pub fn render(&self, width: usize) {
        let mut width_remaining = width;
        for segment in &self.segments {
            width_remaining -= segment.render(width_remaining)
        }
        if width_remaining > 0 {
            print!("{:width_remaining$}", " ");
        }
    }
}
