use crate::controller::EventQueue;
use crate::model::event::Key;
use std::io::Write;

mod collapse_dir;
mod entry_down;
mod entry_up;
mod expand_dir;
mod file_action;
mod quit;
mod reload;

impl<W: Write> EventQueue<W> {
    #[rustfmt::skip]
    pub fn match_key_event(&mut self, key: Key) -> Option<()> {
        let ck = self.config.keybinding.clone();

        if key == Key::from(ck.collapse_dir) { self.do_collapse_dir() }
        else if key == Key::from(ck.entry_down) { self.do_entry_down() }
        else if key == Key::from(ck.entry_up) { self.do_entry_up() }
        else if key == Key::from(ck.expand_dir) { self.do_expand_dir() }
        else if key == Key::from(ck.file_action) { self.do_file_action() }
        else if key == Key::from(ck.quit) { self.do_quit() }
        else if key == Key::from(ck.reload) { self.do_reload() }
        else { Some(()) }
    }

    fn update_pager(&mut self, cursor_delta: i32) {
        self.pager
            .update(cursor_delta, &self.text_entries, self.path_node.get_absolute_path());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::config::Config;
    use crate::model::path_node::PathNode;
    use crate::view::composer::Composer;
    use crate::view::Pager;

    fn prepare_event_queue() -> EventQueue<Vec<u8>> {
        let config = Config::default();

        let composer = Composer::new(config.clone());
        let pager = Pager::new(config.clone(), Vec::new());
        let path_node = PathNode::from(config.setup.working_dir.clone());

        EventQueue::new(config, composer, pager, path_node)
    }

    #[test]
    fn match_key_event_default_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::from("nonsense"))
        };

        assert!(result.is_some());
    }

    #[test]
    fn match_key_event_quit_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::from(event_queue.config.keybinding.quit.clone()))
        };

        assert!(result.is_none());
    }

    #[test]
    fn match_key_event_reload_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::from(event_queue.config.keybinding.reload.clone()))
        };

        assert!(result.is_some());
    }

    #[test]
    fn match_key_event_file_action_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::from(event_queue.config.keybinding.file_action.clone()))
        };

        assert!(result.is_some());
    }

    #[test]
    fn match_key_event_entry_up_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::from(event_queue.config.keybinding.entry_up.clone()))
        };

        assert!(result.is_some());
    }

    #[test]
    fn match_key_event_entry_down_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::from(event_queue.config.keybinding.entry_down.clone()))
        };

        assert!(result.is_some());
    }

    #[test]
    fn match_key_event_collapse_dir_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::from(event_queue.config.keybinding.collapse_dir.clone()))
        };

        assert!(result.is_some());
    }

    #[test]
    fn match_key_event_expand_dir_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::from(event_queue.config.keybinding.expand_dir.clone()))
        };

        assert!(result.is_some());
    }
}
