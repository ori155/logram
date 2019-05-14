use chrono::Utc;
use failure::Error;
use futures::{sink::Sink, sync::mpsc::UnboundedSender, Future, Stream};
use shiplift::{
    builder::{ContainerListOptions, EventFilter, EventFilterType, EventsOptions, LogsOptions},
    Docker,
};
use tokio::{self, runtime::Runtime};

use crate::{
    source::{LogRecord, LogSource, LogSourceStream},
    utils,
};

mod config;
pub use self::config::Config;

fn check_connection(docker: &Docker) -> Result<(), Error> {
    let mut runtime = Runtime::new()?;
    runtime.block_on(docker.ping())?;

    Ok(())
}

fn pipe_logs(
    stream: impl Stream<Item = LogRecord, Error = Error>,
    channel: UnboundedSender<Result<LogRecord, Error>>,
) -> impl Future<Item = (), Error = Error> {
    stream.map(Ok).forward(channel).from_err().map(|_| ())
}

fn send_errors(
    future: impl Future<Item = (), Error = Error>,
    channel: UnboundedSender<Result<LogRecord, Error>>,
) -> impl Future<Item = (), Error = ()> {
    future
        .or_else(|error| channel.send(Err(error)).map_err(Error::from).map(|_| ()))
        .map_err(|_| ())
}

#[derive(Clone)]
pub struct DockerLogSource {
    docker: Docker,
}

impl DockerLogSource {
    pub fn new(_config: Config) -> Result<Self, Error> {
        let docker = Docker::new();
        check_connection(&docker)?;

        Ok(DockerLogSource { docker })
    }
    fn containers_names(&self) -> impl Future<Item = Vec<String>, Error = Error> {
        let options = ContainerListOptions::builder().build();

        self.docker
            .containers()
            .list(&options)
            .from_err()
            .map(|containers| {
                containers
                    .into_iter()
                    .map(|container| container.names[0].clone())
                    .collect()
            })
    }
    fn container_logs(&self, name: String) -> impl Stream<Item = LogRecord, Error = Error> {
        let now = Utc::now().timestamp();
        let options = LogsOptions::builder()
            .stdout(true)
            .stderr(true)
            .follow(true)
            .since(now)
            .build();

        self.docker
            .containers()
            .get(&name)
            .logs(&options)
            .from_err()
            .map(move |chunk| LogRecord {
                title: name.clone(),
                body: chunk.as_string_lossy(),
            })
    }
    fn new_containers(&self) -> impl Stream<Item = String, Error = Error> {
        let filter = EventFilter::Type(EventFilterType::Container);
        let options = EventsOptions::builder().filter(vec![filter]).build();

        self.docker
            .events(&options)
            .from_err()
            .filter_map(move |event| {
                if &event.typ == "container" && &event.action == "start" {
                    event.actor.attributes.get("name").cloned()
                } else {
                    None
                }
            })
    }
}

impl LogSource for DockerLogSource {
    fn into_stream(self: Box<Self>) -> Box<LogSourceStream> {
        let (tx, rx) = utils::result_channel();
        let self_clone = self.clone();
        let tx_clone = tx.clone();
        let tx_errors = tx.clone();

        let watch_logs = self.containers_names().and_then(move |names| {
            let log_streams: Vec<_> = names
                .into_iter()
                .map(|name| self.container_logs(name))
                .collect();

            pipe_logs(utils::stream_select_all(log_streams), tx)
        });

        let watch_new_containers = self_clone.new_containers().for_each(move |name| {
            let log_stream = self_clone.container_logs(name);

            pipe_logs(log_stream, tx_clone.clone())
        });

        tokio::spawn(send_errors(watch_logs, tx_errors.clone()));
        tokio::spawn(send_errors(watch_new_containers, tx_errors));

        Box::new(rx)
    }
}
