use crate::controller::EventQueue;
use std::io::Write;
use log::info;

impl<W: Write> EventQueue<W> {
    pub fn do_file_action(&mut self) -> Option<()> {
        let tree_index = self
            .path_node_root
            .flat_index_to_tree_index(self.pager.cursor_row as usize);

        let child_node = self.path_node_root.get_child_path_node(&tree_index);

        if !child_node.is_dir {
            let file_path = &child_node.get_absolute_path();
            let file_action_replaced =
                self.config.behavior.file_action.replace("%s", file_path);

            info!("executing file action:\n{}", file_action_replaced);


            if self.config.behavior.quit_on_action {
                self.command_to_run_on_exit = Some(file_action_replaced);
                None
            } else {
                std::process::Command::new("bash")
                    .arg("-c")
                    .arg(file_action_replaced)
                    .spawn()
                    .unwrap();
                Some(())
            }
        }
        else
            {Some(())}
    }
}
