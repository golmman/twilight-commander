use crate::config::Config;
use ncurses::*;

pub struct Pager {
    config: Config,
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

        Self { config }
    }

    pub fn update(&self, text_row: i32, cursor_row: i32, text_entries: &[String], root_path: String) -> i32 {
        let padding_bot = self.config.debug.padding_bot;
        let padding_top = self.config.debug.padding_top;
        let spacing_bot = self.config.debug.spacing_bot;
        let spacing_top = self.config.debug.spacing_top;

        let displayable_rows = LINES() - (spacing_bot + spacing_top);
        let mut new_text_row = text_row;
        let mut color_pair;

        if text_row + cursor_row < spacing_top + padding_top {
            new_text_row += spacing_top + padding_top - (text_row + cursor_row);
        } else if text_row + cursor_row > LINES() - (1 + spacing_bot + padding_bot) {
            new_text_row -= text_row + cursor_row - (LINES() - (1 + spacing_bot + padding_bot));
        }

        let first_index = spacing_top - new_text_row;
        let last_index = first_index + displayable_rows;

        init_pair(1, COLOR_WHITE, COLOR_BLACK);
        init_pair(2, COLOR_WHITE, COLOR_BLUE);

        // print rows
        for i in 0..displayable_rows {
            let index = first_index + i;

            if index == cursor_row {
                color_pair = 2;
            } else {
                color_pair = 1;
            }

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
                format!("cursor_row: {}", cursor_row).as_str(),
            );
            mvaddstr(
                LINES() - 2,
                0,
                format!("first_index: {}, last_index: {}", first_index, last_index).as_str(),
            );
            mvaddstr(
                LINES() - 1,
                0,
                format!("text_row: {}, new_text_row: {}", text_row, new_text_row).as_str(),
            );
        }

        new_text_row
    }

    fn print_debug_info() {}
}
