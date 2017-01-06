extern crate hyper;
extern crate multipart;
extern crate serde_json;

use hyper::net::Streaming;
use hyper::method::Method;
use hyper::client::Request;

use hyper::client::RequestBuilder;
use hyper::Client;
use hyper::Url;

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
pub mod builders;

use types::*;

#[derive(Debug)]
pub enum BotError {
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

fn request_multipart(multi: multipart::client::Multipart<hyper::client::Request<hyper::net::Streaming>>) -> Result<Value, BotError> {
    match multi.send() {
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
    base_url: Url,
    client: Client,
}

impl BotApi {
    pub fn new(bot_token: &str) -> BotApi {
        BotApi {
            base_url: format!("https://api.telegram.org/bot{}/", bot_token).parse().unwrap(),
            client: Client::new(),
        }
    }

    pub fn get_me(self) -> Result<User, BotError> {
        let url = self.base_url.join("getMe").unwrap();
        let res = request(self.client.get(url));
        match res {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn get_updates(&self, params: &args::GetUpdates) -> Result<Vec<Update>, BotError> {
        let url = self.base_url.join("getUpdates").unwrap();

        // All these unwraps make me uncomfortable, someone is going to yell
        // at me to get some good error handling. And they would be right
        // but I'm really lazy and I don't want to deal with this. I should
        // though because it's good practice and no real software should ever
        // just throw errors around like they don't matter.

        // Manage your errors

        let req = Request::new(Method::Post, url).unwrap();

        let mut multi = Multipart::from_request(req).unwrap();

        if let Some(offset) = params.offset {
            multi.write_text("offset", offset.to_string()).unwrap();
        }   
        
        if let Some(limit) = params.limit {
            multi.write_text("limit", limit.to_string()).unwrap();
        }   
        
        if let Some(timeout) = params.timeout {
            multi.write_text("timeout", timeout.to_string()).unwrap();
        }   
        
        // if let Some(allowed_updates) = params.allowed_updates {
        //     for allowed_update in allowed_updates {
        //         multi.write_text("allowed_updates[]", allowed_update).unwrap();
        //     }
        // } // Cannot move out of borrowed context, :\

        match request_multipart(multi) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    
    pub fn send_message(&self, params: &args::SendMessage) -> Result<Message, BotError> {
        let url = self.base_url.join("sendMessage").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        if let Some(chat_id) = params.chat_id {
            multi.write_text("chat_id", chat_id.to_string()).unwrap();
        } else
        if let Some(chat_username) = params.chat_username {
            multi.write_text("chat_id", chat_username).unwrap();
        }

        multi.write_text("text", params.text).unwrap();

        if let Some(parse_mode) = params.parse_mode {
            multi.write_text("parse_mode", parse_mode).unwrap();
        }

        if let Some(disable_web_page_preview) = params.disable_web_page_preview {
            multi.write_text("disable_web_page_preview", disable_web_page_preview.to_string()).unwrap();
        }

        if let Some(disable_notification) = params.disable_notification {
            multi.write_text("disable_notification", disable_notification.to_string()).unwrap();
        }

        if let Some(reply_to_message_id) = params.reply_to_message_id {
            multi.write_text("reply_to_message_id", reply_to_message_id.to_string()).unwrap();
        }

        // if let Some(reply_markup) = params.reply_markup {
        //     multi.write_text("reply_markup", reply_markup).unwrap();
        // }

        // TODO create macro or just a function that will take a serde_json::Value
        // and multi.write_text correctly all of its stuff. then some function for
        // arrays that is essentially just a loop


        match request_multipart(multi) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }


}

#[cfg(test)]
mod tests { // These aren't going to be the actual tests, just a place for me to easily test things as I go along
    use super::*;
    extern crate serde_json;
    #[test]
    fn it_works() {
        let token = "";
        let bot = BotApi::new(token);
        
        let mut offset = 0;
        
        for _ in 0..3 {
            let updates = bot.get_updates(&args::GetUpdates {
                                            offset: Some(offset),
                                            limit: None,
                                            timeout: Some(600),
                                            allowed_updates: None,
                                        }).unwrap();

            for update in updates {
                offset = update.update_id;

                if let Some(message) = update.message {
                    let from = message.from.unwrap();
                    let text = format!("Hi {}[{}]!", from.first_name,
                                                     from.id);

                    bot.send_message(&args::SendMessage {
                                        chat_id: Some(message.chat.id),
                                        chat_username: None,
                                        text: &text,
                                        parse_mode: Some("Markdown"),
                                        disable_web_page_preview: None,
                                        disable_notification: None,
                                        reply_to_message_id: Some(message.message_id),
                                        reply_markup: None,
                                    }).unwrap();
                }
            }
        }
    }
}
