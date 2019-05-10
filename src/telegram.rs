use failure::Error;
use futures::Future;
use serde_derive::Deserialize;
use tgbot::{methods::SendMessage, types::ParseMode, Api};

use crate::source::LogRecord;

#[derive(Debug, Deserialize)]
pub struct Config {
    token: String,
    chat_id: String,
}

pub struct Telegram {
    api: Api,
    chat_id: String,
}

impl Telegram {
    pub fn new(config: Config) -> Result<Self, Error> {
        let api = Api::new::<_, String>(config.token, None)?;
        let chat_id = config.chat_id;

        Ok(Telegram { api, chat_id })
    }
    fn send(&self, text: String) -> impl Future<Item = (), Error = Error> {
        let method = SendMessage::new(self.chat_id.as_str(), text).parse_mode(ParseMode::Markdown);

        self.api.execute(&method).map(|_| ()).from_err()
    }
    pub fn send_log_record(&self, record: LogRecord) -> impl Future<Item = (), Error = Error> {
        let text = format!("*{}*```\n{}```", record.title, record.body);

        self.send(text)
    }
    pub fn send_error(&self, error: Error) -> impl Future<Item = (), Error = Error> {
        let text = format!("Error: {}", error);

        self.send(text)
    }
}
