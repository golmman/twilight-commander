use crate::model::config::Config;
use crate::view::composer::Composer;
use log::info;
use std::io::Write;

pub mod composer;
mod print;
mod scroll;
mod update;

pub struct Pager<W: Write> {
    config: Config,
    pub cursor_row: i32,
    out: W,
    terminal_cols: i32,
    terminal_rows: i32,
    text_row: i32,
}

impl<W: Write> Pager<W> {
    pub fn new(config: Config, mut out: W) -> Self {
        info!("initializing pager");

        write!(
            out,
            "{}{}{}",
            termion::cursor::Hide,
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        Self {
            config,
            cursor_row: 0,
            out,
            terminal_cols: 0,
            terminal_rows: 0,
            text_row: 0,
        }
    }
}

impl<W: Write> Drop for Pager<W> {
    fn drop(&mut self) {
        write!(
            self,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show,
        )
        .unwrap();
    }
}

impl<W: Write> Write for Pager<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.out.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.out.flush()
    }
}
