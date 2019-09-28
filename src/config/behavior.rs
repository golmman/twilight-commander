use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Behavior {
    #[serde(default = "Behavior::default_file_action")]
    pub file_action: String,

    #[serde(default = "Behavior::default_scrolling")]
    pub scrolling: String,
}

impl Behavior {
    pub fn default() -> Behavior {
        Behavior {
            file_action: Self::default_file_action(),
            scrolling: Self::default_scrolling(),
        }
    }

    fn default_file_action() -> String {
        String::from("true") // do nothing!
    }

    fn default_scrolling() -> String {
        String::from("center")
    }
}