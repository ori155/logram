use failure::Error;
use futures::{
    future::{self, Either},
    stream, Future, Stream,
};
use reqwest::r#async::Client as AsyncClient;
use serde::Deserialize;
use url::Url;

pub mod types;
use self::types::{Message, Response, Update};

pub struct TelegramApi {
    client: AsyncClient,
    base_url: Url,
}

impl TelegramApi {
    pub fn new(token: &str) -> Result<Self, Error> {
        let client = AsyncClient::new();

        let url = format!("https://api.telegram.org/bot{}/", token);
        let base_url = Url::parse(&url)?;

        Ok(TelegramApi { client, base_url })
    }
    fn request<T>(
        &self,
        method: &str,
        params: &[(&str, &str)],
    ) -> impl Future<Item = T, Error = Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let mut url = match self.base_url.join(method) {
            Ok(url) => url,
            Err(error) => return Either::B(future::err(Error::from(error))),
        };
        url.query_pairs_mut().extend_pairs(params);

        let fut = self
            .client
            .post(url)
            .send()
            .and_then(|mut response| response.json::<Response<T>>())
            .map_err(Error::from)
            .and_then(|response| response.into_result());

        Either::A(fut)
    }
    pub fn updates(self) -> impl Stream<Item = Vec<Update>, Error = Error> {
        stream::unfold(0, move |last_update_id| {
            Some(self.get_updates(last_update_id).and_then(move |updates| {
                let last = updates
                    .last()
                    .map(|update| update.update_id + 1)
                    .unwrap_or(last_update_id);

                Ok((updates, last))
            }))
        })
    }
    pub fn get_updates(&self, offset: i64) -> impl Future<Item = Vec<Update>, Error = Error> {
        let offset_str = format!("{}", offset);
        let params = [("offset", offset_str.as_str()), ("timeout", "10")];

        self.request("getUpdates", &params)
    }
    pub fn send_message(
        &self,
        chat_id: &str,
        text: &str,
    ) -> impl Future<Item = Message, Error = Error> {
        let params = [
            ("text", text),
            ("chat_id", chat_id),
            ("parse_mode", "markdown"),
        ];

        self.request("sendMessage", &params)
    }
}
