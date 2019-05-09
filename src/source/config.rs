use serde::Deserialize;

use super::{fs::Config as FsConfig, journald::Config as JournaldConfig};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LogSourceConfig<T> {
    pub enabled: bool,

    #[serde(flatten)]
    pub inner: T,
}
impl<T: Default> Default for LogSourceConfig<T> {
    fn default() -> Self {
        LogSourceConfig {
            enabled: true,
            inner: T::default(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct LogSourcesConfig {
    pub fs: LogSourceConfig<FsConfig>,
    pub journald: LogSourceConfig<JournaldConfig>,
}
