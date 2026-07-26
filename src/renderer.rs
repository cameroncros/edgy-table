use std::io::Result;
use std::io::{IsTerminal, Write, stdout};
use std::rc::Rc;
use std::string::FromUtf8Error;
use crate::theme::Theme;


pub struct Renderer {
    writer: Vec<u8>,
    pub(crate) theme: Rc<Theme>,
    pub(crate) enable_color: bool,
}

fn check_color() -> bool {
    if std::env::var("NO_COLOR").is_ok() {
        false
    } else if std::env::var("FORCE_COLOR").is_ok() {
        true
    } else {
        stdout().is_terminal()
    }
}

pub enum Color {
    Auto,
    On,
    Off
}

impl Renderer {
    #[must_use]
    pub fn new(color: &Color, theme: Theme) -> Self {
        let color = match color {
            Color::Auto => check_color(),
            Color::On => true,
            Color::Off => false,
        };

        Self {
            writer: Vec::with_capacity(1000),
            theme: Rc::new(theme),
            enable_color: color
        }
    }

    /// Write the rendered table to stdout.
    ///
    /// # Errors
    ///
    /// Returns errors from `stdout().write_all`
    pub fn to_stdout(&self) -> Result<()> {
        stdout().write_all(&self.writer)
    }

    /// Write the rendered table to a string.
    ///
    /// # Errors
    ///
    /// Returns errors from `String::from_utf8`
    pub fn to_string(&self) -> std::result::Result<String, FromUtf8Error> {
        String::from_utf8(self.writer.clone())
    }
}

impl Write for Renderer {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer::check_color;

    #[test]
    fn test_check_color() {
        unsafe {
            std::env::set_var("FORCE_COLOR", "1");
        }
        assert!(check_color());
        unsafe {
            std::env::remove_var("FORCE_COLOR");
            std::env::set_var("NO_COLOR", "1");
        }
        assert!(!check_color());
        unsafe {
            std::env::remove_var("FORCE_COLOR");
            std::env::remove_var("NO_COLOR");
        }
        let _ = check_color();
    }
}
