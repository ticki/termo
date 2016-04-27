#![feature(io)]

extern crate termion;

use termion::{RawTerminal, Keys, Style, TermWrite, TermRead, IntoRawMode};

pub use termion::Color;

use std::io::{self, Write};

pub struct Terminal<'a> {
    stdout: RawTerminal<io::StdoutLock<'a>>,
    stdin: Keys<io::Chars<io::StdinLock<'a>>>,
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

    pub fn keys(&mut self) -> &mut Keys<io::Chars<io::StdinLock<'a>>> {
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
        self.term.stdout.goto(self.x, self.y).unwrap();

        if self.bold {
            self.term.stdout.style(Style::Bold).unwrap();
        }
        if self.italic {
            self.term.stdout.style(Style::Italic).unwrap();
        }

        self.term.stdout.write(self.text.as_bytes()).unwrap();
        self.term.stdout.reset().unwrap();
    }
}
