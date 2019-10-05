use crate::config::Config;

mod print;
mod scroll;
mod update;
mod util;

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
}
