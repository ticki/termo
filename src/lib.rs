//#![feature(io)]

extern crate termion;

//use termion::{TermWrite};
use termion::style;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::{TermRead, Keys};
use termion::cursor;

pub use termion::color::Color;

use std::io::{self, Write};

pub struct Terminal<'lock> {
    stdout: RawTerminal<io::StdoutLock<'lock>>,
    stdin: Keys<io::Bytes<io::StdinLock<'lock>>>,
}

impl<'lock> Terminal<'lock> {
    pub fn new(stdout: &'lock io::Stdout, stdin: &'lock io::Stdin) -> Terminal<'lock> {
        Terminal {
            stdout: stdout.lock().into_raw_mode().unwrap(),
            stdin: stdin.lock().keys(),
        }
    }

    pub fn text<'term>(&'term mut self) -> TextBuilder<'term, 'lock> {
        TextBuilder {
            term: self,
            text: "",
            x: 0,
            y: 0,
            bold: false,
            italic: false,
        }
    }

    pub fn keys(&mut self) -> &mut Keys<io::Bytes<io::StdinLock<'lock>>> {
        &mut self.stdin
    }
}

pub struct TextBuilder<'term, 'lock: 'term> {
    term: &'term mut Terminal<'lock>,
    text: &'term str,
    x: u16,
    y: u16,
    bold: bool,
    italic: bool,
}

impl<'term, 'lock> TextBuilder<'term, 'lock> {
    pub fn text(&mut self, text: &'term str) -> &mut TextBuilder<'term, 'lock> {
        debug_assert!(self.text.is_empty(), "Setting the text multiple times.");
        self.text = text;
        self
    }

    pub fn pos(&mut self, x: u16, y: u16) -> &mut TextBuilder<'term, 'lock> {
        self.x = x;
        self.y = y;
        self
    }

    pub fn bold(&mut self) -> &mut TextBuilder<'term, 'lock> {
        self.bold = true;
        self
    }

    pub fn italic(&mut self) -> &mut TextBuilder<'term, 'lock> {
        self.italic = true;
        self
    }
}

impl<'term, 'lock> Drop for TextBuilder<'term, 'lock> {
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
