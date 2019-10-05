use crate::pager::Pager;
use termion::{color, style};

impl Pager {
    pub fn print_text_entry(&self, text_entry: &str, row: i32) {
        print!(
            "{}{}{}",
            termion::cursor::Goto(1, row as u16),
            self.shorten_string_to_terminal_cols(text_entry),
            style::Reset
        );
    }

    pub fn print_text_entry_emphasized(&self, text_entry: &str, row: i32) {
        print!(
            "{}{}{}{}",
            termion::cursor::Goto(1, row as u16),
            color::Bg(color::Blue),
            self.shorten_string_to_terminal_cols(text_entry),
            style::Reset
        );
    }

    pub fn print_header(&self, header_text: &str) {
        print!(
            "{}{}",
            termion::cursor::Goto(1, 1),
            self.shorten_string_to_terminal_cols(header_text),
        );
    }

    pub fn print_debug_info(&self) {
        if !self.config.debug.enabled {
            return;
        }

        let padding_bot = self.config.debug.padding_bot;
        let padding_top = self.config.debug.padding_top;
        let spacing_bot = self.config.debug.spacing_bot;
        let spacing_top = self.config.debug.spacing_top;

        // line numbers
        for i in 0..self.terminal_rows {
            print!("{} L{}", termion::cursor::Goto(50, 1 + i as u16), i.to_string());
        }

        // padding_top debug
        for i in 0..padding_bot {
            print!(
                "{}~~~ padding_bot",
                termion::cursor::Goto(30, (self.terminal_rows - (spacing_bot + i)) as u16)
            );
        }

        for i in 0..padding_top {
            print!(
                "{}~~~ padding_top",
                termion::cursor::Goto(30, (1 + spacing_top + i) as u16)
            );
        }

        // spacing_top debug
        for i in 0..spacing_bot {
            print!(
                "{}--- spacing_bot",
                termion::cursor::Goto(30, (self.terminal_rows - i) as u16)
            );
        }
        for i in 0..spacing_top {
            print!("{}--- spacing_top", termion::cursor::Goto(30, 1 + i as u16));
        }

        // debug info
        print!(
            "{}rows: {}, cols: {}",
            termion::cursor::Goto(1, (self.terminal_rows - 1) as u16),
            self.terminal_rows,
            self.terminal_cols
        );
        print!(
            "{}cursor_row: {}, text_row: {}",
            termion::cursor::Goto(1, self.terminal_rows as u16),
            self.cursor_row,
            self.text_row
        );
    }
}
