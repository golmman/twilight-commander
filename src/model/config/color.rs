use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Color {
    #[serde(default = "Color::default_background")]
    pub background: String,

    #[serde(default = "Color::default_foreground")]
    pub foreground: String,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

impl Color {
    fn default_background() -> String {
        String::from("000000")
    }

    fn default_foreground() -> String {
        String::from("FFFFFF")
    }
}
