use crate::controller::EventQueue;
use std::io::Write;

impl<W: Write> EventQueue<W> {
    pub fn do_quit(&mut self) -> Option<()> {
        None
    }
}
