use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Setup {
    #[serde(default = "Setup::default_working_dir")]
    pub working_dir: String,
}

impl Default for Setup {
    fn default() -> Self {
        Setup {
            working_dir: Self::default_working_dir(),
        }
    }
}

impl Setup {
    fn default_working_dir() -> String {
        String::from(".")
    }
}
