type TEvent = termion::event::Event;
type TKey = termion::event::Key;

#[derive(Clone, Debug, PartialEq)]
pub struct Key {
    inner: termion::event::Event,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Resize,
    Key(Key),
}

impl From<TEvent> for Key {
    fn from(t_event: TEvent) -> Key {
        Key { inner: t_event }
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Key {
        Key::from(convert_str_to_termion_event(s))
    }
}

impl From<String> for Key {
    fn from(s: String) -> Key {
        Key::from(convert_str_to_termion_event(&s))
    }
}

// TODO: add tests
fn convert_str_to_termion_event(s: &str) -> TEvent {
    if s.len() == 1 {
        return TEvent::Key(TKey::Char(s.chars().last().unwrap()));
    }

    if s.starts_with("alt+") && s.len() == 5 {
        return TEvent::Key(TKey::Alt(s.chars().last().unwrap()));
    }

    if s.starts_with("ctrl+") && s.len() == 6 {
        return TEvent::Key(TKey::Ctrl(s.chars().last().unwrap()));
    }

    match s {
        // f keys
        "f1" => TEvent::Key(TKey::F(1)),
        "f2" => TEvent::Key(TKey::F(2)),
        "f3" => TEvent::Key(TKey::F(3)),
        "f4" => TEvent::Key(TKey::F(4)),
        "f5" => TEvent::Key(TKey::F(5)),
        "f6" => TEvent::Key(TKey::F(6)),
        "f7" => TEvent::Key(TKey::F(7)),
        "f8" => TEvent::Key(TKey::F(8)),
        "f9" => TEvent::Key(TKey::F(9)),
        "f10" => TEvent::Key(TKey::F(10)),
        "f11" => TEvent::Key(TKey::F(11)),
        "f12" => TEvent::Key(TKey::F(12)),

        // special keys
        "backspace" => TEvent::Key(TKey::Backspace),
        "left" => TEvent::Key(TKey::Left),
        "right" => TEvent::Key(TKey::Right),
        "up" => TEvent::Key(TKey::Up),
        "down" => TEvent::Key(TKey::Down),
        "home" => TEvent::Key(TKey::Home),
        "end" => TEvent::Key(TKey::End),
        "page_up" => TEvent::Key(TKey::PageUp),
        "page_down" => TEvent::Key(TKey::PageDown),
        "delete" => TEvent::Key(TKey::Delete),
        "insert" => TEvent::Key(TKey::Insert),
        "esc" => TEvent::Key(TKey::Esc),
        "return" => TEvent::Key(TKey::Char('\n')),
        "tab" => TEvent::Key(TKey::Char('\t')),

        // special key combinations

        // arrow keys
        "ctrl+left" => TEvent::Unsupported(vec![27, 91, 49, 59, 53, 68]),
        "ctrl+right" => TEvent::Unsupported(vec![27, 91, 49, 59, 53, 67]),
        "ctrl+up" => TEvent::Unsupported(vec![27, 91, 49, 59, 53, 65]),
        "ctrl+down" => TEvent::Unsupported(vec![27, 91, 49, 59, 53, 66]),
        "shift+left" => TEvent::Unsupported(vec![27, 91, 49, 59, 50, 68]),
        "shift+right" => TEvent::Unsupported(vec![27, 91, 49, 59, 50, 67]),
        "shift+up" => TEvent::Unsupported(vec![27, 91, 49, 59, 50, 65]),
        "shift+down" => TEvent::Unsupported(vec![27, 91, 49, 59, 50, 66]),
        "alt+shift+left" => TEvent::Unsupported(vec![27, 91, 49, 59, 52, 68]),
        "alt+shift+right" => TEvent::Unsupported(vec![27, 91, 49, 59, 52, 67]),
        "alt+shift+up" => TEvent::Unsupported(vec![27, 91, 49, 59, 52, 65]),
        "alt+shift+down" => TEvent::Unsupported(vec![27, 91, 49, 59, 52, 66]),
        "shift+alt+left" => TEvent::Unsupported(vec![27, 91, 49, 59, 52, 68]),
        "shift+alt+right" => TEvent::Unsupported(vec![27, 91, 49, 59, 52, 67]),
        "shift+alt+up" => TEvent::Unsupported(vec![27, 91, 49, 59, 52, 65]),
        "shift+alt+down" => TEvent::Unsupported(vec![27, 91, 49, 59, 52, 66]),

        // default
        _ => TEvent::Unsupported(Vec::new()),
    }
}
