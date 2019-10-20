use crate::view::Pager;
use std::io::Write;
use termion::terminal_size;

impl<W: Write> Pager<W> {
    fn update_terminal_size(&mut self) {
        let (terminal_cols_raw, terminal_rows_raw) = terminal_size().unwrap();
        self.terminal_cols = i32::from(terminal_cols_raw);
        self.terminal_rows = i32::from(terminal_rows_raw);
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

    pub fn update(&mut self, cursor_row_delta: i32, text_entries: &[String], header_text: String) {
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

        // clear screen
        self.print_clear();

        // print rows
        for i in 0..displayable_rows {
            let index = first_index + i;

            if index >= 0 && index < text_entries.len() as i32 {
                let text_entry = &text_entries[index as usize];

                if index == self.cursor_row {
                    self.print_text_entry_emphasized(text_entry, 1 + spacing_top + i)
                } else {
                    self.print_text_entry(text_entry, 1 + spacing_top + i);
                }
            }
        }

        let footer_text = format!("[{}/{}]", self.cursor_row + 1, text_entries_len);

        self.print_header(&header_text);
        self.print_footer(&footer_text);

        self.print_debug_info();

        self.flush().unwrap();
    }
}
