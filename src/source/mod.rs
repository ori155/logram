use failure::Error;
use futures::Stream;

mod config;
mod docker;
mod filesystem;
mod journald;
pub use self::config::LogSourcesConfig;
use self::{docker::DockerLogSource, filesystem::FilesystemLogSource, journald::JournaldLogSource};

#[derive(Debug, PartialEq)]
pub struct LogRecord {
    pub title: String,
    pub body: String,
}

pub type LogSourceStream = Stream<Item = LogRecord, Error = Error> + Send;

pub trait LogSource: Send {
    fn into_stream(self: Box<Self>) -> Box<LogSourceStream>;
}

pub fn init_log_sources(config: LogSourcesConfig) -> Result<Vec<Box<LogSource>>, Error> {
    let mut sources = Vec::new();

    if config.filesystem.enabled {
        let filesystem = FilesystemLogSource::new(config.filesystem.inner)?;
        let source = Box::new(filesystem) as Box<LogSource>;

        sources.push(source);
    }

    if config.journald.enabled {
        let journald = JournaldLogSource::new(config.journald.inner)?;
        let source = Box::new(journald) as Box<LogSource>;

        sources.push(source);
    }

    if config.docker.enabled {
        let docker = DockerLogSource::new(config.docker.inner)?;
        let source = Box::new(docker) as Box<LogSource>;

        sources.push(source);
    }

    Ok(sources)
}
