use crate::controller::EventQueue;
use std::io::Write;

impl<W: Write> EventQueue<W> {
    pub fn do_skip_up(&mut self) -> Option<()> {
        self.update_pager(-self.config.behavior.skip_amount);
        Some(())
    }
}

// todo: 1) scoll center down most 2) wait top or bottom