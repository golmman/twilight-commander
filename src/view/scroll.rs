use crate::view::Pager;
use std::io::Write;

impl<W: Write> Pager<W> {
    fn get_index_overshoot(index_under_test: i32, index_now: i32, index_delta: i32) -> Option<i32> {
        let index_before = index_now - index_delta;

        // cross from below
        if index_before <= index_under_test && index_now >= index_under_test {
            return Some(index_now - index_under_test);
        }

        // cross from above
        if index_before >= index_under_test && index_now <= index_under_test {
            return Some(index_now - index_under_test);
        }

        None
    }

    // TODO: when jumping to the parent dir via collapse_dir, dont force center
    pub fn scroll_like_center(&self, cursor_row_delta: i32, text_entries_len: i32) -> i32 {
        let spacing_bot = self.config.debug.spacing_bot;
        let spacing_top = self.config.debug.spacing_top;

        let center_text_row = spacing_top - self.text_row + (self.terminal_rows - (spacing_bot + spacing_top)) / 2;
        let last_text_row = self.terminal_rows - (self.text_row + spacing_bot);

        // re-center a cursor row that is under the center (last text entry was visible)
        // in the case that a subdirectory is opened
        // in such a way that the bottom is not visible anymore
        if cursor_row_delta == 0
            && self.cursor_row - center_text_row > 0
            && self.cursor_row - center_text_row <= text_entries_len - last_text_row
        {
            return self.text_row - (self.cursor_row - center_text_row);
        }

        // cursor row is moved over the center
        if let Some(overshoot) = Self::get_index_overshoot(center_text_row, self.cursor_row, cursor_row_delta) {
            // no need to keep it centered when we reach the top or bottom
            if self.text_row >= spacing_top && cursor_row_delta < 0 {
                return self.text_row;
            }
            if self.text_row + text_entries_len <= self.terminal_rows - spacing_bot && cursor_row_delta > 0 {
                return self.text_row;
            }

            // keep it centered
            return self.text_row - overshoot;
        }

        // cursor row is beyond vision -> move the text row the minimal amount to correct that
        if self.text_row + self.cursor_row < spacing_top {
            return spacing_top - self.cursor_row;
        } else if self.text_row + self.cursor_row > self.terminal_rows - (1 + spacing_bot) {
            return self.terminal_rows - (1 + spacing_bot + self.cursor_row);
        }

        self.text_row
    }

    pub fn scroll_like_editor(&self) -> i32 {
        let padding_bot = self.config.debug.padding_bot;
        let padding_top = self.config.debug.padding_top;
        let spacing_bot = self.config.debug.spacing_bot;
        let spacing_top = self.config.debug.spacing_top;

        if self.text_row + self.cursor_row < spacing_top + padding_top {
            return spacing_top + padding_top - self.cursor_row;
        } else if self.text_row + self.cursor_row > self.terminal_rows - (1 + spacing_bot + padding_bot) {
            return self.terminal_rows - (1 + spacing_bot + padding_bot + self.cursor_row);
        }

        self.text_row
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::config::Config;

    fn prepare_pager() -> Pager<Vec<u8>> {
        let mut config = Config::default();
        config.debug.enabled = true;
        config.debug.padding_bot = 1;
        config.debug.padding_top = 1;
        config.debug.spacing_bot = 1;
        config.debug.spacing_top = 1;

        let out: Vec<u8> = Vec::new();
        let mut pager = Pager::new(config, out);

        pager.terminal_cols = 100;
        pager.terminal_rows = 10;

        pager
    }

    mod get_index_overshoot_tests {
        use super::*;

        #[test]
        fn overshoot_from_below() {
            let overshoot = Pager::<Vec<u8>>::get_index_overshoot(10, 11, 3);
            assert_eq!(Some(1), overshoot);
        }

        #[test]
        fn overshoot_from_above() {
            let overshoot = Pager::<Vec<u8>>::get_index_overshoot(10, 7, -4);
            assert_eq!(Some(-3), overshoot);
        }

        #[test]
        fn no_overshoot_from_below() {
            let overshoot = Pager::<Vec<u8>>::get_index_overshoot(10, 7, 2);
            assert_eq!(None, overshoot);
        }

        #[test]
        fn no_overshoot_from_above() {
            let overshoot = Pager::<Vec<u8>>::get_index_overshoot(10, 14, -3);
            assert_eq!(None, overshoot);
        }
    }

    mod scroll_like_center_tests {
        use super::*;

        #[test]
        fn scroll_like_center_cursor_top_test() {
            let text_row = {
                let pager = prepare_pager();
                pager.scroll_like_center(1, 17)
            };

            assert_eq!(1, text_row);
        }

        #[test]
        fn scroll_like_center_text_moves_up1_test() {
            let text_row = {
                let mut pager = prepare_pager();
                pager.cursor_row = 5;
                pager.scroll_like_center(1, 17)
            };

            assert_eq!(0, text_row);
        }

        #[test]
        fn scroll_like_center_text_moves_up2_test() {
            let text_row = {
                let mut pager = prepare_pager();
                pager.cursor_row = 6;
                pager.scroll_like_center(1, 17)
            };

            assert_eq!(-1, text_row);
        }

        #[test]
        fn scroll_like_center_text_moves_down_test() {
            let text_row = {
                let mut pager = prepare_pager();
                pager.cursor_row = 6;
                pager.scroll_like_center(-1, 17)
            };

            assert_eq!(0, text_row);
        }

        #[test]
        fn scroll_like_center_cursor_bot_test() {
            let text_row = {
                let mut pager = prepare_pager();
                pager.cursor_row = 9;
                pager.scroll_like_center(-1, 17)
            };

            assert_eq!(-1, text_row);
        }

        #[test]
        fn scroll_like_center_cursor_bot_no_delta_test() {
            let text_row = {
                let mut pager = prepare_pager();
                pager.cursor_row = 9;
                pager.scroll_like_center(0, 17)
            };

            assert_eq!(-4, text_row);
        }
    }

    mod scroll_like_editor_tests {
        use super::*;

        #[test]
        fn scroll_like_editor_cursor_top_test() {
            let text_row = {
                let pager = prepare_pager();
                pager.scroll_like_editor()
            };

            assert_eq!(2, text_row);
        }

        #[test]
        fn scroll_like_editor_text_moves_up1_test() {
            let text_row = {
                let mut pager = prepare_pager();
                pager.cursor_row = 5;
                pager.scroll_like_editor()
            };

            assert_eq!(0, text_row);
        }

        #[test]
        fn scroll_like_editor_text_moves_up2_test() {
            let text_row = {
                let mut pager = prepare_pager();
                pager.cursor_row = 6;
                pager.scroll_like_editor()
            };

            assert_eq!(0, text_row);
        }

        #[test]
        fn scroll_like_editor_text_moves_down_test() {
            let text_row = {
                let mut pager = prepare_pager();
                pager.cursor_row = 6;
                pager.scroll_like_editor()
            };

            assert_eq!(0, text_row);
        }

        #[test]
        fn scroll_like_editor_cursor_bot_test() {
            let text_row = {
                let mut pager = prepare_pager();
                pager.cursor_row = 9;
                pager.scroll_like_editor()
            };

            assert_eq!(-2, text_row);
        }

        #[test]
        fn scroll_like_editor_cursor_bot_no_delta_test() {
            let text_row = {
                let mut pager = prepare_pager();
                pager.cursor_row = 9;
                pager.scroll_like_editor()
            };

            assert_eq!(-2, text_row);
        }
    }
}
