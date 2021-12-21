#[macro_use]
extern crate serde_derive;

use std::{error, fmt};

use futures::SinkExt;
use futures::channel::mpsc::Receiver;
use reqwest::{Client, Response};
use reqwest::multipart::Form;

use tokio::time::Duration;

use serde::Deserialize;

pub mod better;
pub mod functions;
pub mod objects;

pub mod api {
    pub use crate::functions::*;
    pub use crate::objects::*;
}

#[allow(unused_mut, unused_variables)]
mod functions_impl;
#[allow(unused_mut, unused_variables)]
mod objects_impl;

use better::{ApiResult, FormSer};

#[derive(Debug)]
pub enum BotError {
    Http(reqwest::Error),
    Json(serde_json::error::Error),
    Api {
        error_code: i64,
        description: String,
        parameters: Option<objects::ResponseParameters>,
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
        "Something unexpected occurred while talking to the telegram bot api." // meh
    }
}

pub trait TgMethod {
    type ResponseType;
    const PATH: &'static str;
}

#[derive(Debug, Clone)]
pub struct Bot {
    token: String,
    client: Client,
}

impl Bot {
    pub fn new(bot_token: impl Into<String>) -> Bot {
        let client = Client::builder()
            .timeout(Duration::from_secs(5 * 60 + 30))
            .connect_timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        Bot {
            token: bot_token.into(),
            client: client,
        }
    }

    pub async fn send<R: for<'de> Deserialize<'de>, M: TgMethod<ResponseType=R> + FormSer>(
        &self, m: &M,
    ) -> Result<R, BotError> {
        let url = format!("https://api.telegram.org/bot{}/{}", self.token, M::PATH);

        let form = m.serialize("".into(), Form::new());

        let resp: Response = self.client
            .post(&url)
            .multipart(form)
            .send()
            .await?;

        let res: ApiResult<R> = resp.json().await?;
        res.into()
    }

    pub fn start_polling(&self) -> Receiver<better::Update> {
        let (mut tx, rx) = futures::channel::mpsc::channel(100);

        let bot = self.clone();

        tokio::spawn(async move {
            let mut req = functions::GetUpdates {
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
        });

        rx
    }
}
