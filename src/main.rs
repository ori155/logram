#![recursion_limit = "128"]
#![warn(clippy::all)]

use failure::Error;
use futures::{Future, Stream};
use std::process;
use tgbot::{methods::SendMessage, types::ParseMode, Api as TelegramApi};
use tokio;

mod cli;
mod config;
mod echo_id;
mod source;
mod utils;
use self::{config::Config, echo_id::echo_id, source::create_log_sources_stream};

fn run() -> Result<(), Error> {
    let matches = cli::matches();

    if let Some(matches) = matches.subcommand_matches("echo_id") {
        let token = matches.value_of("token").unwrap();
        return echo_id(token);
    }

    let config_filename = matches.value_of("config").unwrap();
    let config = Config::read(config_filename)?;

    let telegram = TelegramApi::new::<String, String>(config.telegram.token, None)?;
    let chat_id = config.telegram.chat_id.clone();

    let log_sources_stream = create_log_sources_stream(config.sources)?;

    let main_loop = log_sources_stream
        .then(move |result| {
            let text = match result {
                Ok(record) => format!("*{}*```\n{}```", record.title, record.body),
                Err(error) => format!("Error: {}", error),
            };

            Ok(text)
        })
        .map(move |text| SendMessage::new(chat_id.as_str(), text).parse_mode(ParseMode::Markdown))
        .for_each(move |method| telegram.execute(&method).map(|_| ()))
        .map_err(|error| eprintln!("Telegram error: {}", error));

    tokio::run(main_loop);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        process::exit(2);
    }
}
