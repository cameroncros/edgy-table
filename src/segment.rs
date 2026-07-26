use crate::renderer::Renderer;
use ascii::AsciiChar;
use owo_colors::{AnsiColors, OwoColorize};
use std::io::Write;
use unicode_width::UnicodeWidthStr;

/// Represents a piece of text, with color applied to it.
#[derive(Clone)]
pub struct Segment {
    text: String,
    color: AnsiColors,
}

impl From<&str> for Segment {
    fn from(value: &str) -> Self {
        Self {
            text: value.to_string(),
            color: AnsiColors::Default,
        }
    }
}

fn sanitise_string(str: &str) -> String {
    let mut expanded = String::with_capacity(str.len());
    str.chars().enumerate().for_each(|(i, c)| {
        if c == '\t' {
            match i % 4 {
                0 => expanded += "    ",
                1 => expanded += "   ",
                2 => expanded += "  ",
                _ => expanded += " ",
            }
        } else if c == '\n' {
            expanded.push('\n');
        } else if let Ok(ac) = AsciiChar::from_ascii(c) {
            expanded.push(ac.as_printable_char());
        } else {
            expanded.push(c);
        }
    });
    expanded
}

impl Segment {
    #[must_use]
    pub fn new(str: &str, color: AnsiColors) -> Self {
        Self {
            text: sanitise_string(str),
            color,
        }
    }

    pub(crate) fn width(&self) -> usize {
        self.text.width()
    }

    pub(crate) fn render(
        &self,
        renderer: &mut Renderer,
        max_width: usize,
    ) -> std::io::Result<usize> {
        let str = format!("{:.max_width$}", self.text);
        if renderer.enable_color {
            renderer.write_all(str.color(self.color).to_string().as_bytes())?;
        } else {
            renderer.write_all(str.as_bytes())?;
        }
        Ok(str.width())
    }

    /// Splits a segment on new-line.
    pub(crate) fn split(self) -> Vec<Segment> {
        self.text
            .split('\n')
            .map(|s| Segment::new(s, self.color))
            .collect()
    }
}

/// Create a segment.
/// ```
/// # use edgy_table::{seg, segment::Segment};
/// # use owo_colors::AnsiColors;
/// seg!["String with default color"];
/// seg!["String with supplied color", AnsiColors::Red];
/// ```
#[macro_export]
macro_rules! seg {
    ($value:expr, $color:expr) => {
        $crate::segment::Segment::new($value, $color)
    };
    ($value:expr) => {
        $crate::segment::Segment::new($value, owo_colors::AnsiColors::Default)
    };
}

#[cfg(test)]
mod tests {
    use crate::segment::sanitise_string;
    use owo_colors::AnsiColors::Cyan;

    #[test]
    fn test_split() {
        let splits = seg!["Hello World", Cyan].split();
        assert_eq!(1, splits.len());
        assert_eq!("Hello World", splits[0].text);
        assert_eq!(Cyan, splits[0].color);

        let splits = seg!["Hello\nWorld", Cyan].split();
        assert_eq!(2, splits.len());
        assert_eq!("Hello", splits[0].text);
        assert_eq!(Cyan, splits[0].color);
        assert_eq!("World", splits[1].text);
        assert_eq!(Cyan, splits[1].color);

        let splits = seg!["\nWorld", Cyan].split();
        assert_eq!(2, splits.len());
        assert_eq!("", splits[0].text);
        assert_eq!(Cyan, splits[0].color);
        assert_eq!("World", splits[1].text);
        assert_eq!(Cyan, splits[1].color);

        let splits = seg!["\n", Cyan].split();
        assert_eq!(2, splits.len());
        assert_eq!("", splits[0].text);
        assert_eq!(Cyan, splits[0].color);
        assert_eq!("", splits[1].text);
        assert_eq!(Cyan, splits[1].color);

        let splits = seg!["\n\n", Cyan].split();
        assert_eq!(3, splits.len());
        assert_eq!("", splits[0].text);
        assert_eq!(Cyan, splits[0].color);
        assert_eq!("", splits[1].text);
        assert_eq!(Cyan, splits[1].color);
        assert_eq!("", splits[2].text);
        assert_eq!(Cyan, splits[2].color);
    }

    #[test]
    fn test_width() {
        assert_eq!(11, seg!["Hello World"].width());
        assert_eq!(13, seg!["Hello\tWorld"].width());
    }

    #[test]
    fn test_sanitise_string() {
        assert_eq!("Hello World", sanitise_string("Hello World"));
        assert_eq!("    Hello World", sanitise_string("\tHello World"));
        assert_eq!("H   ello World", sanitise_string("H\tello World"));
        assert_eq!("He  llo World", sanitise_string("He\tllo World"));
        assert_eq!("Hel lo World", sanitise_string("Hel\tlo World"));
        assert_eq!("Hell    o World", sanitise_string("Hell\to World"));
        assert_eq!("Hello   World", sanitise_string("Hello\tWorld"));
        assert_eq!("Hello   World", sanitise_string("Hello\tWorld"));
        assert_eq!("Hello␈World", sanitise_string("Hello\x08World"));

        let all_bytes: String = (0u8..255u8).map(char::from).collect();
        assert_eq!(
            "␀␁␂␃␄␅␆␇␈   \n␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~␡\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}¡¢£¤¥¦§¨©ª«¬\u{ad}®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþ",
            sanitise_string(&all_bytes)
        );
    }
}
