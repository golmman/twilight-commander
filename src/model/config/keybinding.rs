use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Keybinding {
    #[serde(default = "Keybinding::default_quit")]
    pub quit: String,

    #[serde(default = "Keybinding::default_entry_up")]
    pub entry_up: String,

    #[serde(default = "Keybinding::default_entry_down")]
    pub entry_down: String,

    #[serde(default = "Keybinding::default_expand_dir")]
    pub expand_dir: String,

    #[serde(default = "Keybinding::default_collapse_dir")]
    pub collapse_dir: String,

    #[serde(default = "Keybinding::default_file_action")]
    pub file_action: String,

    #[serde(default = "Keybinding::default_reload")]
    pub reload: String,
}

impl Default for Keybinding {
    fn default() -> Self {
        Self {
            quit: Self::default_quit(),
            entry_up: Self::default_entry_up(),
            entry_down: Self::default_entry_down(),
            expand_dir: Self::default_expand_dir(),
            collapse_dir: Self::default_collapse_dir(),
            file_action: Self::default_file_action(),
            reload: Self::default_reload(),
        }
    }
}

impl Keybinding {
    fn default_quit() -> String {
        String::from("q")
    }

    fn default_entry_up() -> String {
        String::from("up")
    }

    fn default_entry_down() -> String {
        String::from("down")
    }

    fn default_expand_dir() -> String {
        String::from("right")
    }

    fn default_collapse_dir() -> String {
        String::from("left")
    }

    fn default_file_action() -> String {
        String::from("return")
    }

    fn default_reload() -> String {
        String::from("r")
    }
}
