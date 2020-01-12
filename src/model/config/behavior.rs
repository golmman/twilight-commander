use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Behavior {
    #[serde(default = "Behavior::default_file_action")]
    pub file_action: String,

    #[serde(default = "Behavior::default_path_node_sort")]
    pub path_node_sort: String,

    #[serde(default = "Behavior::default_scrolling")]
    pub scrolling: String,

    #[serde(default = "Behavior::default_skip_amount")]
    pub skip_amount: i32,
}

impl Default for Behavior {
    fn default() -> Behavior {
        Behavior {
            file_action: Self::default_file_action(),
            path_node_sort: Self::default_path_node_sort(),
            scrolling: Self::default_scrolling(),
            skip_amount: Self::default_skip_amount(),
        }
    }
}

impl Behavior {
    fn default_file_action() -> String {
        String::from("true") // do nothing!
    }

    fn default_path_node_sort() -> String {
        String::from("dirs_top_simple")
    }

    fn default_scrolling() -> String {
        String::from("center")
    }

    fn default_skip_amount() -> i32 {
        5
    }
}
