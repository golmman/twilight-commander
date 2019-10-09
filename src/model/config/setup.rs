use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Setup {
    #[serde(default = "Setup::default_working_dir")]
    pub working_dir: String,
}

impl Setup {
    pub fn default() -> Setup {
        Setup {
            working_dir: Self::default_working_dir(),
        }
    }

    fn default_working_dir() -> String {
        String::from(".")
    }
}
