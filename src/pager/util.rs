use crate::pager::Pager;

impl Pager {
    pub fn shorten_string_to_terminal_cols(&self, string: &str) -> String {
        if self.terminal_cols > string.len() as i32 {
            return String::from(string);
        }

        let split_at = self.terminal_cols - 1;
        let mut shortened = String::from(string.split_at(split_at as usize).0);

        shortened.push('~');

        shortened
    }
}
