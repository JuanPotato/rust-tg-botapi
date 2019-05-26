#![feature(async_await)]
#[macro_use]
extern crate serde_derive;

use std::{error, fmt};
use std::sync::Arc;

use futures::{Future, FutureExt, SinkExt, TryFutureExt};
use futures::channel::mpsc::Receiver;
use futures::compat::Future01CompatExt;
use reqwest::r#async::{Client, Response};
use serde::Deserialize;

use types::*;

pub mod types;
pub mod methods;


#[derive(Debug)]
pub enum BotError {
    Http(reqwest::Error),
    Json(serde_json::error::Error),
    Api {
        error_code: i64,
        description: String,
        parameters: Option<ResponseParameters>,
    },
}

impl From<serde_json::error::Error> for BotError {
    fn from(err: serde_json::error::Error) -> BotError {
        BotError::Json(err)
    }
}

impl From<reqwest::Error> for BotError {
    fn from(err: reqwest::Error) -> BotError {
        BotError::Http(err)
    }
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BotError::Http(ref e) => write!(f, "{}", e),
            BotError::Json(ref e) => write!(f, "{}", e),
            BotError::Api {
                error_code,
                ref description,
                ..
            } => write!(f, "Error {0}: {1}", error_code, description),
        }
    }
}

impl error::Error for BotError {
    fn description(&self) -> &str {
        "Something unexpected occured while talking to the telegram bot api." // meh
    }
}

pub trait TgMethod {
    type ResponseType;
    const PATH: &'static str;
}

pub trait Captures<'a> {}

impl<'a, T> Captures<'a> for T {}


#[derive(Debug, Clone)]
pub struct Bot {
    token: String,
    client: Arc<Client>,
}

impl Bot {
    pub fn new(bot_token: impl Into<String>) -> Bot {
        Bot {
            token: bot_token.into(),
            client: Arc::new(Client::new()),
        }
    }

    pub fn send<'a: 'c, 'b: 'c, 'c, R: for<'de> Deserialize<'de>, M: TgMethod<ResponseType=R> + serde::Serialize>(
        &'a self, m: &'b M,
    ) -> impl Future<Output=Result<R, BotError>> + Captures<'a> + Captures<'b> + 'c {
        async move {
            let url = format!("https://api.telegram.org/bot{}/{}", self.token, M::PATH);

            let mut resp: Response = self.client.post(&url).json(m).send().compat().await.unwrap();
            let res: ApiResult<R> = resp.json().compat().await?;

            res.into()
        }
    }

    pub fn start_polling(&self) -> Receiver<Update> {
        let (mut tx, rx) = futures::channel::mpsc::channel(100);

        let bot = self.clone();

        tokio::spawn(async move {
            let mut req = methods::GetUpdates {
                offset: Some(0),
                limit: None,
                timeout: Some(5 * 60),
                allowed_updates: None,
            };

            loop {
                let updates = bot.send(&req).await;

                for update in updates.unwrap() {
                    req.offset = Some(update.update_id + 1);
                    tx.send(update).await.unwrap();
                }
            }
        }.boxed().unit_error().compat());

        rx
    }
}

