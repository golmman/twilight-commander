extern crate ncurses;

use ncurses::*;
use path_tree::*;

mod path_tree;

fn main() {
    let mut path_node = PathNode::new("./tests/test_dirs");
    expand_dir(&mut path_node, &TreeIndex::new(Vec::new()));
    let mut test_entries = prettify(&path_node);

    /* Setup ncurses. */
    initscr();
    raw();
    start_color();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* Status/help info. */
    addstr("Use the arrow keys to move");
    mvprintw(LINES() - 1, 0, "Press F1 to exit");
    refresh();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    /* Start in the center. */
    let mut text_row = 0;
    let mut cursor_row = 0;
    text_row = create_win(text_row, cursor_row, &test_entries);

    let mut ch = getch();
    while ch != 113 {
        match ch {
            KEY_UP => {
                cursor_row -= 1;
                if cursor_row < 0 {
                    cursor_row = test_entries.len() as i32 - 1;
                }

                clear();
                text_row = create_win(text_row, cursor_row, &test_entries);
            }
            KEY_DOWN => {
                cursor_row += 1;
                if cursor_row >= test_entries.len() as i32 {
                    cursor_row = 0;
                }

                clear();
                text_row = create_win(text_row, cursor_row, &test_entries);
            }
            KEY_RIGHT => {
                let tree_index = flat_index_to_tree_index(&path_node, cursor_row as usize);
                expand_dir(&mut path_node, &tree_index);
                test_entries = prettify(&path_node);

                clear();
                text_row = create_win(text_row, cursor_row, &test_entries);
            }
            KEY_LEFT => {
                let tree_index = flat_index_to_tree_index(&path_node, cursor_row as usize);
                reduce_dir(&mut path_node, &tree_index);
                test_entries = prettify(&path_node);

                clear();
                text_row = create_win(text_row, cursor_row, &test_entries);
            }
            _ => {}
        }
        ch = getch();
    }

    endwin();
}

fn create_win(text_row: i32, cursor_row: i32, text_entries: &[String]) -> i32 {
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
