use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub units: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config { units: Vec::new() }
    }
}
