use crate::config::Config;
use termion::terminal_size;
use termion::{color, style};

pub struct Pager {
    config: Config,
    pub cursor_row: i32,
    terminal_cols: i32,
    terminal_rows: i32,
    text_row: i32,
}

impl Pager {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            cursor_row: 0,
            terminal_cols: 0,
            terminal_rows: 0,
            text_row: 0,
        }
    }

    fn update_terminal_size(&mut self) {
        let (terminal_cols_raw, terminal_rows_raw) = terminal_size().unwrap();
        self.terminal_cols = i32::from(terminal_cols_raw);
        self.terminal_rows = i32::from(terminal_rows_raw);
    }

    fn scroll_like_center(&self, cursor_row_delta: i32, text_entries_len: i32) -> i32 {
        let spacing_bot = self.config.debug.spacing_bot;
        let spacing_top = self.config.debug.spacing_top;

        let center_text_row = spacing_top - self.text_row + (self.terminal_rows - (spacing_bot + spacing_top)) / 2;
        let last_text_row = self.terminal_rows - (self.text_row + spacing_bot);

        // re-center a cursor row that is under the center (last text entry was visible)
        // in the case that a subdirectory is opened
        // in such a way that the bottom is not visible anymore
        if cursor_row_delta == 0
            && self.cursor_row - center_text_row > 0
            && self.cursor_row - center_text_row <= text_entries_len - last_text_row
        {
            return self.text_row - (self.cursor_row - center_text_row);
        }

        // cursor row is exactly centered
        if self.cursor_row - cursor_row_delta == center_text_row {
            // no need to keep it centered when we reach the top or bottom
            if self.text_row >= spacing_top && cursor_row_delta < 0 {
                return self.text_row;
            }
            if self.text_row + text_entries_len <= self.terminal_rows - spacing_bot && cursor_row_delta > 0 {
                return self.text_row;
            }

            // keep it centered
            return self.text_row - cursor_row_delta;
        }

        // cursor row is beyond vision -> move the text row the minimal amount to correct that
        if self.text_row + self.cursor_row < spacing_top {
            return spacing_top - self.cursor_row;
        } else if self.text_row + self.cursor_row > self.terminal_rows - (1 + spacing_bot) {
            return self.terminal_rows - (1 + spacing_bot + self.cursor_row);
        }

        self.text_row
    }

    fn scroll_like_editor(&self) -> i32 {
        let padding_bot = self.config.debug.padding_bot;
        let padding_top = self.config.debug.padding_top;
        let spacing_bot = self.config.debug.spacing_bot;
        let spacing_top = self.config.debug.spacing_top;

        if self.text_row + self.cursor_row < spacing_top + padding_top {
            return spacing_top + padding_top - self.cursor_row;
        } else if self.text_row + self.cursor_row > self.terminal_rows - (1 + spacing_bot + padding_bot) {
            return self.terminal_rows - (1 + spacing_bot + padding_bot + self.cursor_row);
        }

        self.text_row
    }

    fn update_cursor_row(&mut self, cursor_row_delta: i32, text_entries_len: i32) {
        self.cursor_row += cursor_row_delta;
        if self.cursor_row < 0 {
            self.cursor_row = text_entries_len - 1;
        }
        if self.cursor_row >= text_entries_len {
            self.cursor_row = 0;
        }
    }

    pub fn update(&mut self, cursor_row_delta: i32, text_entries: &[String], root_path: String) {
        self.update_terminal_size();

        let spacing_bot = self.config.debug.spacing_bot;
        let spacing_top = self.config.debug.spacing_top;

        let text_entries_len = text_entries.len() as i32;

        self.update_cursor_row(cursor_row_delta, text_entries_len);

        self.text_row = match self.config.behavior.scrolling.as_str() {
            "center" => self.scroll_like_center(cursor_row_delta, text_entries_len),
            "editor" => self.scroll_like_editor(),
            _ => 0,
        };

        let displayable_rows = self.terminal_rows - (spacing_bot + spacing_top);

        let first_index = spacing_top - self.text_row;

        // print rows
        for i in 0..displayable_rows {
            let index = first_index + i;

            if index >= 0 && index < text_entries.len() as i32 {
                let text_entry = &text_entries[index as usize];

                if index == self.cursor_row {
                    print!(
                        "{}{}{}{}",
                        termion::cursor::Goto(1, (1 + spacing_top + i) as u16),
                        color::Bg(color::Blue),
                        self.shorten_string_to_terminal_cols(text_entry),
                        style::Reset
                    );
                } else {
                    print!(
                        "{}{}{}",
                        termion::cursor::Goto(1, (1 + spacing_top + i) as u16),
                        self.shorten_string_to_terminal_cols(text_entry),
                        style::Reset
                    );
                }
            }
        }

        // print header
        print!(
            "{}{}",
            termion::cursor::Goto(1, 1),
            self.shorten_string_to_terminal_cols(&root_path),
        );

        self.print_debug_info();
    }

    fn shorten_string_to_terminal_cols(&self, string: &str) -> String {
        if self.terminal_cols > string.len() as i32 {
            return String::from(string);
        }

        let split_at = self.terminal_cols - 1;
        let mut shortened = String::from(string.split_at(split_at as usize).0);

        shortened.push('~');

        shortened
    }

    fn print_debug_info(&self) {
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
