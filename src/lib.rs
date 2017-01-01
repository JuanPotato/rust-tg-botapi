extern crate hyper;
extern crate serde_json;

use hyper::{Client, Url};
use hyper::error;

use std::result::Result;
use std::sync::Arc;
use std::io::Read;

include!(concat!(env!("OUT_DIR"), "/types.rs"));
include!(concat!(env!("OUT_DIR"), "/method_return_types.rs"));

#[derive(Debug)]
pub struct BotApi<'a> {
    token: &'a str,
    client: Client,
}

#[derive(Debug)]
enum BotError { // Will make this into an actual Error
    Http (hyper::error::Error),
    Api {
        error_code: i64,
        description: String,
        parameters:  Option<ResponseParameters>
    }
}

impl<'a> BotApi<'a> {
    pub fn new(bot_token: &str) -> BotApi {
        BotApi {
            token: bot_token,
            client: Client::new(),
        }
    }

    fn get_me(self) -> Result<User, BotError> { // Will make a generic request function
        let url = &format!("https://api.telegram.org/bot{}/{}", self.token, "getMe");
        let res = self.client.get(url).send();
        match res {
            Ok(mut res) => {
                let mut body = String::new();
                res.read_to_string(&mut body).unwrap();
                let got_me: GetMeResult = serde_json::from_str(&body).unwrap();
                if let Some(user) = got_me.result {
                    Ok(user)
                } else {
                    Err(BotError::Api{
                        error_code: got_me.error_code.unwrap(), // We are using unwrap because its impossible for there not to be code and descriptions
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
