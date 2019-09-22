use ncurses::*;

pub fn init_pager() {
    initscr();
    raw();
    start_color();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    refresh();
}

pub fn update_pager(text_row: i32, cursor_row: i32, text_entries: &[String]) -> i32 {
    const SPACING: i32 = 5;
    const PADDING: i32 = 3;

    let displayable_rows = LINES() - (2 * SPACING);
    let mut new_text_row = text_row;
    let mut color_pair;

    if text_row + cursor_row < SPACING + PADDING {
        new_text_row += SPACING + PADDING - (text_row + cursor_row);
    } else if text_row + cursor_row > LINES() - (1 + SPACING + PADDING) {
        new_text_row -= text_row + cursor_row - (LINES() - (1 + SPACING + PADDING));
    }

    let first_index = SPACING - new_text_row;
    let last_index = first_index + displayable_rows;

    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    init_pair(2, COLOR_WHITE, COLOR_BLUE);

    // spacing debug
    for i in 0..LINES() {
        mvaddstr(i, 50, format!("{}", i).as_str());
    }

    for i in 0..SPACING {
        mvaddstr(i, 30, "--- SPACING");
    }

    for i in 0..SPACING {
        mvaddstr(LINES() - (1 + i), 30, "--- SPACING");
    }

    // padding debug
    for i in 0..PADDING {
        mvaddstr(SPACING + i, 30, "~~~ PADDING");
    }

    for i in 0..PADDING {
        mvaddstr(LINES() - (SPACING + 1 + i), 30, "~~~ PADDING");
    }

    for i in 0..displayable_rows {
        let index = first_index + i;

        if index == cursor_row {
            color_pair = 2;
        } else {
            color_pair = 1;
        }

        if index >= 0 && index < text_entries.len() as i32 {
            attron(COLOR_PAIR(color_pair));
            mvaddstr(SPACING + i, 2, &text_entries[index as usize]);
            attroff(COLOR_PAIR(color_pair));
        }
    }

    mvaddstr(
        LINES() - 3,
        0,
        format!("cursor_row: {}", cursor_row).as_str(),
    );
    mvaddstr(
        LINES() - 2,
        0,
        format!("first_index: {}, last_index: {}", first_index, last_index).as_str(),
    );
    mvaddstr(
        LINES() - 1,
        0,
        format!("text_row: {}, new_text_row: {}", text_row, new_text_row).as_str(),
    );

    new_text_row
}
