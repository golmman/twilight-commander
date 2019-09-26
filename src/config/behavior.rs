use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Behavior {
    #[serde(default = "Behavior::default_scrolling")]
    pub scrolling: String,
}

impl Behavior {
    pub fn default() -> Behavior {
        Behavior {
            scrolling: Self::default_scrolling(),
        }
    }

    fn default_scrolling() -> String {
        String::from("center")
    }
}