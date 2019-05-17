use serde::Deserialize;

use super::{
    docker::Config as DockerConfig, filesystem::Config as FilesystemConfig,
    journald::Config as JournaldConfig,
};

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
            enabled: false,
            inner: T::default(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct LogSourcesConfig {
    pub filesystem: LogSourceConfig<FilesystemConfig>,
    pub journald: LogSourceConfig<JournaldConfig>,
    pub docker: LogSourceConfig<DockerConfig>,
}
