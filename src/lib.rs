#[macro_use]
extern crate serde_derive;

use std::{error, fmt};
use std::sync::Arc;

use futures::{Future, SinkExt};
use futures::channel::mpsc::Receiver;
use reqwest::{Client, Response};
use reqwest::multipart::{self, Form};
use serde::Deserialize;
use serde_json::Value;

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
        "Something unexpected occurred while talking to the telegram bot api." // meh
    }
}

pub trait TgMethod {
    type ResponseType;
    const PATH: &'static str;
    const USE_MULTIPART: bool;
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

            let resp: Response = if !M::USE_MULTIPART {
                self.client
                    .post(&url)
                    .json(m)
                    .send()
                    .await
                    .unwrap()
            } else {
                let json_value: Value = serde_json::to_value(m).unwrap();
                let form = value_to_form(&json_value);

                self.client
                    .post(&url)
                    .multipart(form)
                    .send()
                    .await
                    .unwrap()
            };

            let res: ApiResult<R> = resp.json().await?;
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
        });

        rx
    }
}

pub fn value_to_form(json: &Value) -> Form {
    let mut form = Form::new();

    for (key, value) in json.as_object().unwrap() {
        form = add_value_to_form(form, key.to_string(), value);
    }

    form
}

fn add_value_to_form(mut form: Form, key: String, json_val: &Value) -> Form {
    match json_val {
        Value::Null => {
            form.text(key, "null")
        },

        Value::Bool(b) => {
            form.text(key, b.to_string())
        },

        Value::Number(n) => {
            form.text(key, n.to_string())
        },

        Value::String(s) => {
            form.text(key, s.to_string())
        },

        Value::Array(a) => {
            for (i, elem) in a.iter().enumerate() {
                let arr_key = if elem.is_object() || elem.is_array() {
                    format!("{}[{}]", key, i)
                } else {
                    format!("{}[]", key)
                };

                form = add_value_to_form(form, arr_key, elem);
            }

            form
        },

        Value::Object(o) => {
            if let Some(file_path) = o.get("_path") {
                form = add_file_to_form(form, key, file_path.as_str().unwrap()).unwrap();
            } else {
                for (k, v) in o {
                    let val_key = format!("{}[{}]", key, k);

                    if key.ends_with("_path") {
                        let file_path = v.as_str().unwrap().trim_end_matches("_path");
                        form = add_file_to_form(form, val_key, file_path).unwrap();
                    } else {
                        form = add_value_to_form(form, val_key, v);
                    }
                }
            }

            form
        }
    }
}

use std::io::Read;
use std::fs::File;
use std::io;
use std::path::PathBuf;

fn add_file_to_form(form: Form, key: String, file_path: &str) -> io::Result<Form> {
    let path = PathBuf::from(file_path);

    let mut file = File::open(file_path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    let mut part = multipart::Part::bytes(bytes);
    let filename: String = path.file_name().unwrap().to_str().unwrap().to_string();
    part = part.file_name(filename);

    Ok(form.part(key, part))
}
