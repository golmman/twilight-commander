use termion::event::Key;

#[derive(Clone, Debug)]
pub enum Event {
    Resize,
    Key(Key),
}
