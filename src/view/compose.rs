use crate::model::path_node::PathNode;
use crate::view::Pager;

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

    pub fn compose_path_node(&self, path_node: &PathNode) -> Vec<String> {
        let mut result = Vec::new();

        self.compose_path_node_recursive(path_node, &mut result, 0);

        result
    }

    fn compose_path_node_recursive(&self, path_node: &PathNode, texts: &mut Vec<String>, depth: usize) {
        for child in &path_node.children {
            let dir_prefix = self.get_dir_prefix(child);
            let dir_suffix = self.get_dir_suffix(child);
            let indent = self.get_indent(depth);

            let text = format!("{}{}{}{}", indent, dir_prefix, child.display_text.clone(), dir_suffix,);
            texts.push(text);
            self.compose_path_node_recursive(child, texts, depth + 1);
        }
    }

    fn get_dir_prefix(&self, path_node: &PathNode) -> String {
        let expanded_char = if self.config.composition.use_utf8 { '▼' } else { 'v' };

        let reduced_char = if self.config.composition.use_utf8 { '▶' } else { '>' };

        let expanded_indicator = if path_node.is_expanded {
            expanded_char
        } else {
            reduced_char
        };

        if path_node.is_dir {
            format!("{} ", expanded_indicator)
        } else {
            String::from("  ")
        }
    }

    fn get_dir_suffix(&self, path_node: &PathNode) -> String {
        if path_node.is_dir {
            String::from("/")
        } else {
            String::from("")
        }
    }

    fn get_indent(&self, depth: usize) -> String {
        let indent_char = if !self.config.composition.show_indent {
            ' '
        } else if self.config.composition.use_utf8 {
            '·'
        } else {
            '-'
        };
        let indent = " ".repeat(self.config.composition.indent as usize - 1);

        format!("{}{}", indent_char, indent).repeat(depth)
    }
}
