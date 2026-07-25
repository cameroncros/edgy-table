use std::io::Result;
use std::io::{IsTerminal, Write, stdout};
use std::string::FromUtf8Error;

pub struct Renderer {
    writer: Vec<u8>,
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

impl Renderer {
    pub fn auto_color() -> Self {
        Self {
            writer: Vec::new(),
            enable_color: check_color(),
        }
    }

    pub fn no_color() -> Self {
        Self {
            writer: Vec::new(),
            enable_color: false,
        }
    }

    pub fn force_color() -> Self {
        Self {
            writer: Vec::new(),
            enable_color: true,
        }
    }

    pub fn to_stdout(&self) -> Result<()> {
        stdout().write_all(&self.writer)
    }

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
