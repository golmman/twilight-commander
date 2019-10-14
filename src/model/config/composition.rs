use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Composition {
    #[serde(default = "Composition::default_indent")]
    pub indent: i32,

    #[serde(default = "Composition::default_show_indent")]
    pub show_indent: bool,

    #[serde(default = "Composition::default_use_utf8")]
    pub use_utf8: bool,
}

impl Default for Composition {
    fn default() -> Composition {
        Composition {
            indent: Self::default_indent(),
            show_indent: Self::default_show_indent(),
            use_utf8: Self::default_use_utf8(),
        }
    }
}

impl Composition {
    fn default_indent() -> i32 {
        2
    }

    fn default_show_indent() -> bool {
        false
    }

    fn default_use_utf8() -> bool {
        true
    }
}
