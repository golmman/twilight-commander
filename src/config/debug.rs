use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Debug {
    #[serde(default = "Debug::default_enabled")]
    pub enabled: bool,

    #[serde(default = "Debug::default_padding")]
    pub padding: i32,

    #[serde(default = "Debug::default_spacing")]
    pub spacing: i32,
}

impl Debug {
    pub fn default() -> Self {
        Self {
            enabled: Self::default_enabled(),
            padding: Self::default_padding(),
            spacing: Self::default_spacing(),
        }
    }

    fn default_enabled() -> bool {
        false
    }

    fn default_padding() -> i32 {
        3
    }

    fn default_spacing() -> i32 {
        0
    }
}
