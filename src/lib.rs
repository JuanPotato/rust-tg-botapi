extern crate hyper;
extern crate multipart;
extern crate serde_json;

use hyper::net::Streaming;
use hyper::method::Method;
use hyper::client::Request;

use hyper::client::RequestBuilder;
use hyper::Client;

use multipart::client::Multipart;

use serde_json::value::Value;

use std::result::Result;
use std::sync::Arc;
use std::io::Read;
use std::{fmt, error};

pub mod types {
    extern crate serde_json;
    use serde_json::value::Value;
    
    include!(concat!(env!("OUT_DIR"), "/types.rs"));
}

pub mod args;

use types::*;

#[derive(Debug)]
enum BotError {
    Http (hyper::error::Error),
    Api {
        error_code: i64,
        description: String,
        parameters: Option<ResponseParameters>
    }
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BotError::Http(ref e) => {
                write!(f, "{}", e)
            },
            &BotError::Api { error_code, ref description, parameters: _ } => {
                write!(f, "Error {0}: {1}", error_code, description)
            }
        }
    }
}

impl error::Error for BotError {
    fn description(&self) -> &str {
        "Something unexpected occured while talking to the telegram bot api." // meh
    }
}

fn request(rb: RequestBuilder) -> Result<Value, BotError> {
    match rb.send() {
        Ok(mut res) => {
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
            let got_me: ApiResult = serde_json::from_str(&body).unwrap();
            if let Some(val) = got_me.result {
                Ok(val)
            } else {
                Err(BotError::Api{
                    error_code: got_me.error_code.unwrap(),
                    description: got_me.description.unwrap(),
                    parameters: got_me.parameters
                })
            }
        },
        Err(e) => {
            Err(BotError::Http(e))
        }
    }
}

#[derive(Debug)]
pub struct BotApi {
    base_url: String,
    client: Client,
}

impl BotApi {
    pub fn new(bot_token: &str) -> BotApi {
        BotApi {
            base_url: format!("https://api.telegram.org/bot{}/{{}}", bot_token),
            client: Client::new(),
        }
    }

    // Plan for function:
    // Have each function have a corresponding struct for its arguments
    // And have optional arguments wrapping in a Option<...>

    // TODO: Find a way to support channel usernames and chat_id
    // maybe two different arguments, just both being optional
    // and have a check that at least one was filled, make chat_id higher priority

    // TODO: support file_id and uploading files in the same manner^

    fn get_me(self) -> Result<User, BotError> {
        let url = self.base_url + "getMe";
        let res = request(self.client.get(&url));
        match res {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests { // These aren't going to be the actual tests, just a place for me to easily test things as I go along
    use super::*;
    #[test]
    fn it_works() {
        let token = "";
        let bot = BotApi::new(token);
        println!("{:?}", bot.get_me());
    }
}
