use crate::model::config::Config;
use crate::model::path_node::PathNode;
use crate::view::composer::Composer;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

impl Debug for PathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let composer = Composer::from(Config::new());

        let entries = composer.compose_path_node(self);

        for (index, entry) in entries.iter().enumerate() {
            writeln!(f, "{:4}|{}", index, entry)?;
        }

        Ok(())
    }
}
