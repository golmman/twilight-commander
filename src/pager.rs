use crate::config::Config;
use ncurses::*;

pub struct Pager {
    config: Config,
    pub cursor_row: i32,
    text_row: i32,
}

impl Pager {
    pub fn new(config: Config) -> Self {
        initscr();
        raw();
        start_color();
        keypad(stdscr(), true);
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        refresh();

        Self {
            config,
            text_row: 0,
            cursor_row: 0,
        }
    }

    fn scroll_like_center(&self, cursor_row_delta: i32, text_entries_len: i32) -> i32 {
        let spacing_bot = self.config.debug.spacing_bot;
        let spacing_top = self.config.debug.spacing_top;
        let center_text_row =
            spacing_top - self.text_row + (LINES() - (spacing_bot + spacing_top)) / 2;
        let last_text_row = LINES() - (self.text_row + spacing_bot);

        // re-center a cursor row that is under the center in the case that a subdirectory is opened
        // in such a way that the bottom is not visible afterwards
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
            if self.text_row + text_entries_len <= LINES() - spacing_bot && cursor_row_delta > 0 {
                return self.text_row;
            }

            // keep it centered
            return self.text_row - cursor_row_delta;
        }

        // cursor row is beyond vision -> move the text row the minimal amount to correct that
        if self.text_row + self.cursor_row < spacing_top {
            return spacing_top - self.cursor_row;
        } else if self.text_row + self.cursor_row > LINES() - (1 + spacing_bot) {
            return LINES() - (1 + spacing_bot + self.cursor_row);
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
        } else if self.text_row + self.cursor_row > LINES() - (1 + spacing_bot + padding_bot) {
            return LINES() - (1 + spacing_bot + padding_bot + self.cursor_row);
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
        let padding_bot = self.config.debug.padding_bot;
        let padding_top = self.config.debug.padding_top;
        let spacing_bot = self.config.debug.spacing_bot;
        let spacing_top = self.config.debug.spacing_top;

        let text_entries_len = text_entries.len() as i32;

        self.update_cursor_row(cursor_row_delta, text_entries_len);

        self.text_row = match self.config.behavior.scrolling.as_str() {
            "center" => self.scroll_like_center(cursor_row_delta, text_entries_len),
            "editor" => self.scroll_like_editor(),
            _ => 0,
        };

        let displayable_rows = LINES() - (spacing_bot + spacing_top);

        let first_index = spacing_top - self.text_row;
        let last_index = first_index + displayable_rows;

        init_pair(1, COLOR_WHITE, COLOR_BLACK);
        init_pair(2, COLOR_WHITE, COLOR_BLUE);

        // print rows
        for i in 0..displayable_rows {
            let index = first_index + i;
            let color_pair = if index == self.cursor_row { 2 } else { 1 };

            if index >= 0 && index < text_entries.len() as i32 {
                attron(COLOR_PAIR(color_pair));
                mvaddstr(spacing_top + i, 2, &text_entries[index as usize]);
                attroff(COLOR_PAIR(color_pair));
            }
        }

        // print header
        let header_split_at = std::cmp::max(0, root_path.len() as i32 - COLS() + 1);
        mvaddstr(0, 0, &root_path.split_at(header_split_at as usize).1);

        // print debug info
        if self.config.debug.enabled {
            // line numbers
            for i in 0..LINES() {
                mvaddstr(i, 50, format!("{}", i).as_str());
            }

            // padding_top debug
            for i in 0..padding_bot {
                mvaddstr(LINES() - (spacing_bot + 1 + i), 30, "~~~ padding_bot");
            }

            for i in 0..padding_top {
                mvaddstr(spacing_top + i, 30, "~~~ padding_top");
            }

            // spacing_top debug
            for i in 0..spacing_bot {
                mvaddstr(LINES() - (1 + i), 30, "--- spacing_bot");
            }
            for i in 0..spacing_top {
                mvaddstr(i, 30, "--- spacing_top");
            }

            // debug info
            mvaddstr(
                LINES() - 3,
                0,
                format!("cursor_row: {}", self.cursor_row).as_str(),
            );
            mvaddstr(
                LINES() - 2,
                0,
                format!("first_index: {}, last_index: {}", first_index, last_index).as_str(),
            );
            mvaddstr(
                LINES() - 1,
                0,
                format!("text_row: {}", self.text_row).as_str(),
            );
        }
    }
}
