//#![feature(io)]

extern crate termion;

//use termion::{TermWrite};
use termion::style;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::{TermRead, Keys};
use termion::cursor;

pub use termion::color::Color;

use std::io::{self, Write};

pub struct Terminal<'a> {
    stdout: RawTerminal<io::StdoutLock<'a>>,
    stdin: Keys<io::Bytes<io::StdinLock<'a>>>,
}

impl<'a> Terminal<'a> {
    pub fn new(stdout: &'a io::Stdout, stdin: &'a io::Stdin) -> Terminal<'a> {
        Terminal {
            stdout: stdout.lock().into_raw_mode().unwrap(),
            stdin: stdin.lock().keys(),
        }
    }

    pub fn text(&'a mut self) -> TextBuilder<'a> {
        TextBuilder {
            term: self,
            text: "",
            x: 0,
            y: 0,
            bold: false,
            italic: false,
        }
    }

    pub fn keys(&mut self) -> &mut Keys<io::Bytes<io::StdinLock<'a>>> {
        &mut self.stdin
    }
}

pub struct TextBuilder<'a> {
    term: &'a mut Terminal<'a>,
    text: &'a str,
    x: u16,
    y: u16,
    bold: bool,
    italic: bool,
}

impl<'a> TextBuilder<'a> {
    pub fn text(&mut self, text: &'a str) -> &mut TextBuilder<'a> {
        debug_assert!(self.text.is_empty(), "Setting the text multiple times.");
        self.text = text;
        self
    }

    pub fn pos(&mut self, x: u16, y: u16) -> &mut TextBuilder<'a> {
        self.x = x;
        self.y = y;
        self
    }

    pub fn bold(&mut self) -> &mut TextBuilder<'a> {
        self.bold = true;
        self
    }

    pub fn italic(&mut self) -> &mut TextBuilder<'a> {
        self.italic = true;
        self
    }
}

impl<'a> Drop for TextBuilder<'a> {
    fn drop(&mut self) {
        debug_assert!(!self.text.is_empty(), "Text not set.");
        write!(self.term.stdout, "{}", cursor::Goto(self.x, self.y)).unwrap();

        if self.bold {
            write!(self.term.stdout, "{}", style::Bold).unwrap();
        }
        if self.italic {
            write!(self.term.stdout, "{}", style::Italic).unwrap();
        }

        write!(self.term.stdout, "{}", self.text).unwrap();
        write!(self.term.stdout, "{}", style::Reset).unwrap();
    }
}
