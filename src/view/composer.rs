use crate::model::config::Config;
use crate::model::path_node::PathNode;
use log::info;

pub struct Composer {
    config: Config,
}

impl From<Config> for Composer {
    fn from(config: Config) -> Self {
        info!("initializing composer");
        Self { config }
    }
}

impl Composer {
    pub fn truncate_string(string: &str, desired_char_count: usize) -> String {
        if desired_char_count < 1 {
            return String::new();
        }

        if desired_char_count >= string.chars().count() {
            return String::from(string);
        }

        let truncated = match string.char_indices().nth(desired_char_count - 1)
        {
            None => string,
            Some((idx, _)) => &string[..idx],
        };

        format!("{}~", truncated)
    }

    pub fn compose_path_node(&self, path_node: &PathNode) -> Vec<String> {
        let mut result = Vec::new();

        self.compose_path_node_recursive(path_node, &mut result, 0);

        result
    }

    fn compose_path_node_recursive(
        &self,
        path_node: &PathNode,
        texts: &mut Vec<String>,
        depth: usize,
    ) {
        for child in &path_node.children {
            let dir_prefix = self.get_dir_prefix(child);
            let dir_suffix = self.get_dir_suffix(child);
            let indent = self.get_indent(depth);

            let text = format!(
                "{}{}{}{}",
                indent,
                dir_prefix,
                child.display_text.clone(),
                dir_suffix,
            );
            texts.push(text);
            self.compose_path_node_recursive(child, texts, depth + 1);
        }
    }

    fn get_dir_prefix(&self, path_node: &PathNode) -> String {
        let (err_char, expanded_char, reduced_char) =
            if self.config.composition.use_utf8 {
                ('⨯', '▼', '▶')
            } else {
                ('x', 'v', '>')
            };

        let expanded_indicator = if path_node.is_err {
            err_char
        } else if path_node.is_expanded {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_string_test() {
        let tc = Composer::truncate_string;
        assert_eq!(tc("hello world", 5), "hell~");
        assert_eq!(tc("hello world", 1), "~");
        assert_eq!(tc("hello world", 0), "");
        assert_eq!(tc("aaa▶bbb▶ccc", 8), "aaa▶bbb~");
        assert_eq!(tc("aaa▶bbb▶ccc", 6), "aaa▶b~");
        assert_eq!(tc("aaa▶bbb▶ccc", 4), "aaa~");
    }
}
