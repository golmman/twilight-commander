use crate::view::Composer;
use crate::view::Pager;
use std::io::Write;
use termion::{color, style};

// TODO: replace print! with write! to separate Printer from updater
//       its also possible to test with a mock stdout
impl<W: Write> Pager<W> {
    pub fn print_clear(&mut self) {
        write!(self, "{}", termion::clear::All).unwrap();
    }

    pub fn print_text_entry(&mut self, text_entry: &str, row: i32) {
        write!(
            self,
            "{}{}{}",
            termion::cursor::Goto(1, row as u16),
            Composer::truncate_string(text_entry, self.terminal_cols as usize),
            style::Reset
        )
        .unwrap();
    }

    pub fn print_text_entry_emphasized(&mut self, text_entry: &str, row: i32) {
        write!(
            self,
            "{}{}{}{}",
            termion::cursor::Goto(1, row as u16),
            color::Bg(color::Blue),
            Composer::truncate_string(text_entry, self.terminal_cols as usize),
            style::Reset
        )
        .unwrap();
    }

    pub fn print_header(&mut self, header_text: &str) {
        write!(
            self,
            "{}{}",
            termion::cursor::Goto(1, 1),
            Composer::truncate_string(header_text, self.terminal_cols as usize),
        )
        .unwrap();
    }

    pub fn print_debug_info(&mut self) {
        if !self.config.debug.enabled {
            return;
        }

        let padding_bot = self.config.debug.padding_bot;
        let padding_top = self.config.debug.padding_top;
        let spacing_bot = self.config.debug.spacing_bot;
        let spacing_top = self.config.debug.spacing_top;

        // line numbers
        for i in 0..self.terminal_rows {
            write!(self, "{} L{}", termion::cursor::Goto(50, 1 + i as u16), i.to_string()).unwrap();
        }

        // padding_top debug
        for i in 0..padding_bot {
            write!(
                self,
                "{}~~~ padding_bot",
                termion::cursor::Goto(30, (self.terminal_rows - (spacing_bot + i)) as u16)
            )
            .unwrap();
        }

        for i in 0..padding_top {
            write!(
                self,
                "{}~~~ padding_top",
                termion::cursor::Goto(30, (1 + spacing_top + i) as u16)
            )
            .unwrap();
        }

        // spacing_top debug
        for i in 0..spacing_bot {
            write!(
                self,
                "{}--- spacing_bot",
                termion::cursor::Goto(30, (self.terminal_rows - i) as u16)
            )
            .unwrap();
        }
        for i in 0..spacing_top {
            write!(self, "{}--- spacing_top", termion::cursor::Goto(30, 1 + i as u16)).unwrap();
        }

        // debug info
        let terminal_rows = self.terminal_rows;
        let terminal_cols = self.terminal_cols;
        let cursor_row = self.cursor_row;
        let text_row = self.text_row;

        write!(
            self,
            "{}rows: {}, cols: {}",
            termion::cursor::Goto(1, (self.terminal_rows - 1) as u16),
            terminal_rows,
            terminal_cols
        )
        .unwrap();
        write!(
            self,
            "{}cursor_row: {}, text_row: {}",
            termion::cursor::Goto(1, self.terminal_rows as u16),
            cursor_row,
            text_row
        )
        .unwrap();
    }
}
