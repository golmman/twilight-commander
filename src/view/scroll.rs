use crate::view::Pager;

impl Pager {
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

        // cursor row is exactly centered
        if self.cursor_row - cursor_row_delta == center_text_row {
            // no need to keep it centered when we reach the top or bottom
            if self.text_row >= spacing_top && cursor_row_delta < 0 {
                return self.text_row;
            }
            if self.text_row + text_entries_len <= self.terminal_rows - spacing_bot && cursor_row_delta > 0 {
                return self.text_row;
            }

            // keep it centered
            return self.text_row - cursor_row_delta;
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
