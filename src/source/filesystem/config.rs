use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub entries: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            entries: Vec::new(),
        }
    }
}
