use crate::controller::EventQueue;
use std::io::Write;

impl<W: Write> EventQueue<W> {
    pub fn do_entry_up(&mut self) -> Option<()> {
        self.pager
            .update(-1, &self.text_entries, self.path_node.get_absolute_path());
        Some(())
    }
}
