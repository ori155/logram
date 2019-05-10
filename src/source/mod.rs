use failure::Error;
use futures::{stream, Stream};

mod config;
pub mod fs;
pub mod journald;
pub use self::config::LogSourcesConfig;
use self::{fs::FsLogSource, journald::JournaldLogSource};

#[derive(Debug, PartialEq)]
pub struct LogRecord {
    pub title: String,
    pub body: String,
}

pub type LogSourceStream = Stream<Item = LogRecord, Error = Error> + Send;

pub trait LogSource {
    fn into_stream(self) -> Box<LogSourceStream>;
}

pub fn create_stream(config: LogSourcesConfig) -> Result<Box<LogSourceStream>, Error> {
    let mut log_sources_stream = Box::new(stream::empty()) as Box<LogSourceStream>;

    if config.fs.enabled {
        let fs = FsLogSource::new(config.fs.inner)?;
        let fs_stream = fs.into_stream();

        log_sources_stream = Box::new(log_sources_stream.select(fs_stream));
    }

    if config.journald.enabled {
        let journald = JournaldLogSource::new(config.journald.inner)?;
        let journald_stream = journald.into_stream();

        log_sources_stream = Box::new(log_sources_stream.select(journald_stream));
    }

    Ok(log_sources_stream)
}
