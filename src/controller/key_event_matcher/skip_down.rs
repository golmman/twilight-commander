use crate::controller::EventQueue;
use std::io::Write;

impl<W: Write> EventQueue<W> {
    pub fn do_skip_down(&mut self) -> Option<()> {
        self.update_pager(self.config.behavior.skip_amount);
        Some(())
    }
}
