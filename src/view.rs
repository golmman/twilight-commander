use crate::model::config::Config;
use std::io::stdout;
use std::io::Write;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

mod compose;
mod print;
mod scroll;
mod update;

pub struct Pager {
    config: Config,
    pub cursor_row: i32,
    stdout: RawTerminal<std::io::Stdout>,
    terminal_cols: i32,
    terminal_rows: i32,
    text_row: i32,
}

impl Pager {
    pub fn new(config: Config) -> Self {
        let mut stdout = stdout().into_raw_mode().unwrap();

        print!(
            "{}{}{}",
            termion::cursor::Hide,
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        );
        stdout.flush().unwrap();

        Self {
            config,
            cursor_row: 0,
            stdout,
            terminal_cols: 0,
            terminal_rows: 0,
            text_row: 0,
        }
    }
}
