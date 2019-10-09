use crate::model::config::Config;
use crate::view::composer::Composer;
use std::io::stdout;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

pub mod composer;
mod print;
mod scroll;
mod update;

pub struct Pager {
    config: Config,
    pub composer: Composer,
    pub cursor_row: i32,
    pub stdout: RawTerminal<std::io::Stdout>,
    terminal_cols: i32,
    terminal_rows: i32,
    text_row: i32,
}

impl Pager {
    pub fn new(config: Config) -> Self {
        let composer_config = config.clone();
        let pager_config = config.clone();

        // Should be used with caution in tests as cargo seems to initialize its own conflicting "raw mode"
        let stdout = stdout().into_raw_mode().unwrap();

        print!(
            "{}{}{}",
            termion::cursor::Hide,
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        );

        Self {
            config: pager_config,
            composer: Composer::new(composer_config),
            cursor_row: 0,
            stdout,
            terminal_cols: 0,
            terminal_rows: 0,
            text_row: 0,
        }
    }
}

impl Drop for Pager {
    fn drop(&mut self) {
        print!(
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show,
        );
    }
}
