use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Debug {
    #[serde(default = "Debug::default_enabled")]
    pub enabled: bool,

    #[serde(default = "Debug::default_padding_bot")]
    pub padding_bot: i32,

    #[serde(default = "Debug::default_padding_top")]
    pub padding_top: i32,

    #[serde(default = "Debug::default_spacing_bot")]
    pub spacing_bot: i32,

    #[serde(default = "Debug::default_spacing_top")]
    pub spacing_top: i32,
}

impl Default for Debug {
    fn default() -> Self {
        Self {
            enabled: Self::default_enabled(),
            padding_bot: Self::default_padding_bot(),
            padding_top: Self::default_padding_top(),
            spacing_bot: Self::default_spacing_bot(),
            spacing_top: Self::default_spacing_top(),
        }
    }
}

impl Debug {
    fn default_enabled() -> bool {
        false
    }

    fn default_padding_bot() -> i32 {
        3
    }

    fn default_padding_top() -> i32 {
        3
    }

    fn default_spacing_bot() -> i32 {
        0
    }

    fn default_spacing_top() -> i32 {
        1
    }
}
