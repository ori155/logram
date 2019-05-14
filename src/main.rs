#![recursion_limit = "128"]
#![warn(clippy::all)]

use failure::Error;
use futures::{
    future::{self, Either},
    Future, Stream,
};
use std::process;
use tokio;

mod cli;
mod config;
mod echo_id;
mod source;
mod telegram;
mod utils;
use self::{
    config::Config,
    echo_id::echo_id,
    source::{LogSource, LogSourceStream},
    telegram::Telegram,
};

fn make_log_sources_streams(sources: Vec<Box<LogSource>>) -> Vec<Box<LogSourceStream>> {
    sources.into_iter().map(LogSource::into_stream).collect()
}

fn run() -> Result<(), Error> {
    let matches = cli::matches();

    if let Some(matches) = matches.subcommand_matches("echo_id") {
        let token = matches.value_of("token").unwrap();
        return echo_id(token);
    }

    let config_filename = matches.value_of("config").unwrap();
    let config = Config::read(config_filename)?;

    let telegram = Telegram::new(config.telegram)?;
    let log_sources = source::init_log_sources(config.sources)?;

    let main_loop = future::lazy(move || {
        let log_streams = make_log_sources_streams(log_sources);
        let log_stream = utils::stream_select_all(log_streams);

        log_stream
            .then(move |result| match result {
                Ok(record) => Either::A(telegram.send_log_record(record)),
                Err(error) => {
                    eprintln!("Log source error: {}", error);
                    Either::B(telegram.send_error(error))
                }
            })
            .for_each(|_| Ok(()))
            .map_err(|error| eprintln!("Error: {}", error))
    });

    tokio::run(main_loop);

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        process::exit(2);
    }
}
