#[macro_use]
extern crate serde;

use std::{error, fmt};

use futures::channel::mpsc::Receiver;
use futures::SinkExt;
use reqwest::multipart::Form;
use reqwest::{Client, Response};

use tokio::time::Duration;

use serde::Deserialize;

mod helpers;
#[allow(rustdoc::invalid_html_tags, rustdoc::bare_urls)]
mod methods;
#[allow(rustdoc::invalid_html_tags, rustdoc::bare_urls)]
mod types;

pub mod api {
    pub use crate::helpers::*;
    pub use crate::methods::*;
    pub use crate::types::*;
}

#[allow(unused_mut, unused_variables)]
mod methods_impl;
#[allow(unused_mut, unused_variables)]
mod types_impl;

use helpers::ApiResult;

#[derive(Debug)]
pub enum BotError {
    Http(reqwest::Error),
    Json(serde_json::error::Error),
    Api {
        error_code: i64,
        description: String,
        parameters: Option<types::ResponseParameters>,
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

    fn to_form(&self) -> Form;
}

pub(crate) trait TgObject {
    fn add_file(&self, form: Form) -> Form {
        form
    }
}

impl<T: TgObject> TgObject for Option<T> {
    fn add_file(&self, form: Form) -> Form {
        match self {
            Some(s) => s.add_file(form),
            None => form,
        }
    }
}

impl<T: TgObject> TgObject for Vec<T> {
    fn add_file(&self, mut form: Form) -> Form {
        for s in self {
            form = s.add_file(form);
        }

        form
    }
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

    pub async fn send<R: for<'de> Deserialize<'de>, M: TgMethod<ResponseType = R>>(
        &self,
        m: &M,
    ) -> Result<R, BotError> {
        let url = format!("https://api.telegram.org/bot{}/{}", self.token, M::PATH);

        let form = m.to_form();

        let resp: Response = self.client.post(&url).multipart(form).send().await?;

        let res: ApiResult<R> = resp.json().await?;
        res.into()
    }

    pub fn start_polling(&self) -> Receiver<helpers::Update> {
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
        });

        rx
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}
