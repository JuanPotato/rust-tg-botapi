#![feature(question_mark)]
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

pub mod args {
    include!(concat!(env!("OUT_DIR"), "/args.rs"));
}

pub mod builders;

use types::*;

// TODO: Organize this mess and get rid of most unwraps

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

fn parse_request(response: Result<hyper::client::Response, hyper::Error>) -> Result<Value, BotError> {
    match response {
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

impl Clone for BotApi {
    fn clone(&self) -> BotApi {
        BotApi {
            base_url: self.base_url.clone(),
            client: Client::new(),
        }
    }
}

impl BotApi {
    pub fn new(bot_token: &str) -> BotApi {
        let url = format!("https://api.telegram.org/bot{}/", bot_token);

        BotApi {
            base_url: url.parse().unwrap(),
            client: Client::new(),
        }
    }

    pub fn get_me(&self) -> Result<User, BotError> {
        let url = self.base_url.join("getMe").unwrap();
        let res = parse_request(self.client.get(url).send());
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

        // I managed to have less places to have errors

        let body = serde_json::to_string(params).unwrap();

        let res = self.client.post(url)
                             .header(hyper::header::ContentType::json())
                             .body(&body);

        match parse_request(res.send()) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_message(&self, params: &args::SendMessage) -> Result<Message, BotError> {
        let url = self.base_url.join("sendMessage").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client.post(url)
                             .header(hyper::header::ContentType::json())
                             .body(&body);

        match parse_request(res.send()) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }
    
    pub fn forward_message(&self, params: &args::ForwardMessage) -> Result<Message, BotError> {
        let url = self.base_url.join("forwardMessage").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client.post(url)
                             .header(hyper::header::ContentType::json())
                             .body(&body);

        match parse_request(res.send()) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_photo(&self, params: &args::SendPhoto) -> Result<Message, BotError> {
        let url = self.base_url.join("sendPhoto").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        if let Some(chat_id) = params.chat_id {
            multi.write_text("chat_id", chat_id.to_string()).unwrap();
        } else if let Some(chat_username) = params.chat_username {
            multi.write_text("chat_id", chat_username).unwrap();
        }

        if let Some(photo) = params.photo {
            multi.write_file("photo", photo).unwrap();
        } else if let Some(file_id) = params.file_id {
            multi.write_text("file_id", file_id).unwrap();
        }

        if let Some(caption) = params.caption {
            multi.write_text("caption", caption).unwrap();
        }

        if let Some(disable_notification) = params.disable_notification {
            multi.write_text("disable_notification", disable_notification.to_string()).unwrap();
        }

        if let Some(reply_to_message_id) = params.reply_to_message_id {
            multi.write_text("reply_to_message_id", reply_to_message_id.to_string()).unwrap();
        }

        if let Some(reply_markup) = params.reply_markup {
            value_to_multi(&mut multi, "reply_markup", serde_json::to_value(reply_markup));
        }

        match parse_request(multi.send()) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_audio(&self, params: &args::SendAudio) -> Result<Message, BotError> {
        let url = self.base_url.join("sendAudio").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        if let Some(chat_id) = params.chat_id {
            multi.write_text("chat_id", chat_id.to_string()).unwrap();
        } else if let Some(chat_username) = params.chat_username {
            multi.write_text("chat_id", chat_username).unwrap();
        }

        if let Some(audio) = params.audio {
            multi.write_file("audio", audio).unwrap();
        } else if let Some(file_id) = params.file_id {
            multi.write_text("file_id", file_id).unwrap();
        }

        if let Some(caption) = params.caption {
            multi.write_text("caption", caption).unwrap();
        }

        if let Some(duration) = params.duration {
            multi.write_text("duration", duration.to_string()).unwrap();
        }

        if let Some(performer) = params.performer {
            multi.write_text("performer", performer).unwrap();
        }

        if let Some(title) = params.title {
            multi.write_text("title", title).unwrap();
        }

        if let Some(disable_notification) = params.disable_notification {
            multi.write_text("disable_notification", disable_notification.to_string()).unwrap();
        }

        if let Some(reply_to_message_id) = params.reply_to_message_id {
            multi.write_text("reply_to_message_id", reply_to_message_id.to_string()).unwrap();
        }

        if let Some(reply_markup) = params.reply_markup {
            value_to_multi(&mut multi, "reply_markup", serde_json::to_value(reply_markup));
        }

        match parse_request(multi.send()) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_document(&self, params: &args::SendDocument) -> Result<Message, BotError> {
        let url = self.base_url.join("sendDocument").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        if let Some(chat_id) = params.chat_id {
            multi.write_text("chat_id", chat_id.to_string()).unwrap();
        } else if let Some(chat_username) = params.chat_username {
            multi.write_text("chat_id", chat_username).unwrap();
        }

        if let Some(document) = params.document {
            multi.write_file("document", document).unwrap();
        } else if let Some(file_id) = params.file_id {
            multi.write_text("file_id", file_id).unwrap();
        }

        if let Some(caption) = params.caption {
            multi.write_text("caption", caption).unwrap();
        }

        if let Some(disable_notification) = params.disable_notification {
            multi.write_text("disable_notification", disable_notification.to_string()).unwrap();
        }

        if let Some(reply_to_message_id) = params.reply_to_message_id {
            multi.write_text("reply_to_message_id", reply_to_message_id.to_string()).unwrap();
        }

        if let Some(reply_markup) = params.reply_markup {
            value_to_multi(&mut multi, "reply_markup", serde_json::to_value(reply_markup));
        }

        match parse_request(multi.send()) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_sticker(&self, params: &args::SendSticker) -> Result<Message, BotError> {
        let url = self.base_url.join("sendSticker").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        if let Some(chat_id) = params.chat_id {
            multi.write_text("chat_id", chat_id.to_string()).unwrap();
        } else if let Some(chat_username) = params.chat_username {
            multi.write_text("chat_id", chat_username).unwrap();
        }

        if let Some(sticker) = params.sticker {
            multi.write_file("sticker", sticker).unwrap();
        } else if let Some(file_id) = params.file_id {
            multi.write_text("file_id", file_id).unwrap();
        }

        if let Some(disable_notification) = params.disable_notification {
            multi.write_text("disable_notification", disable_notification.to_string()).unwrap();
        }

        if let Some(reply_to_message_id) = params.reply_to_message_id {
            multi.write_text("reply_to_message_id", reply_to_message_id.to_string()).unwrap();
        }

        if let Some(reply_markup) = params.reply_markup {
            value_to_multi(&mut multi, "reply_markup", serde_json::to_value(reply_markup));
        }

        match parse_request(multi.send()) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_video(&self, params: &args::SendVideo) -> Result<Message, BotError> {
        let url = self.base_url.join("sendVideo").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        if let Some(chat_id) = params.chat_id {
            multi.write_text("chat_id", chat_id.to_string()).unwrap();
        } else if let Some(chat_username) = params.chat_username {
            multi.write_text("chat_id", chat_username).unwrap();
        }

        if let Some(video) = params.video {
            multi.write_file("video", video).unwrap();
        } else if let Some(file_id) = params.file_id {
            multi.write_text("file_id", file_id).unwrap();
        }

        if let Some(caption) = params.caption {
            multi.write_text("caption", caption).unwrap();
        }

        if let Some(width) = params.width {
            multi.write_text("width", width.to_string()).unwrap();
        }

        if let Some(height) = params.height {
            multi.write_text("height", height.to_string()).unwrap();
        }

        if let Some(duration) = params.duration {
            multi.write_text("duration", duration.to_string()).unwrap();
        }

        if let Some(disable_notification) = params.disable_notification {
            multi.write_text("disable_notification", disable_notification.to_string()).unwrap();
        }

        if let Some(reply_to_message_id) = params.reply_to_message_id {
            multi.write_text("reply_to_message_id", reply_to_message_id.to_string()).unwrap();
        }

        if let Some(reply_markup) = params.reply_markup {
            value_to_multi(&mut multi, "reply_markup", serde_json::to_value(reply_markup));
        }

        match parse_request(multi.send()) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_voice(&self, params: &args::SendVoice) -> Result<Message, BotError> {
        let url = self.base_url.join("sendVoice").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        if let Some(chat_id) = params.chat_id {
            multi.write_text("chat_id", chat_id.to_string()).unwrap();
        } else if let Some(chat_username) = params.chat_username {
            multi.write_text("chat_id", chat_username).unwrap();
        }

        if let Some(voice) = params.voice {
            multi.write_file("voice", voice).unwrap();
        } else if let Some(file_id) = params.file_id {
            multi.write_text("file_id", file_id).unwrap();
        }

        if let Some(caption) = params.caption {
            multi.write_text("caption", caption).unwrap();
        }

        if let Some(duration) = params.duration {
            multi.write_text("duration", duration.to_string()).unwrap();
        }

        if let Some(disable_notification) = params.disable_notification {
            multi.write_text("disable_notification", disable_notification.to_string()).unwrap();
        }

        if let Some(reply_to_message_id) = params.reply_to_message_id {
            multi.write_text("reply_to_message_id", reply_to_message_id.to_string()).unwrap();
        }

        if let Some(reply_markup) = params.reply_markup {
            value_to_multi(&mut multi, "reply_markup", serde_json::to_value(reply_markup));
        }

        match parse_request(multi.send()) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }
}

fn value_to_multi(multi: &mut Multipart<Request<Streaming>>, key: &str, val: Value) {
    match val {
        Value::Null => {
            multi.write_text(key, "null").unwrap();
        }
        Value::Bool(b) => {
            multi.write_text(key, b.to_string()).unwrap();
        }
        Value::I64(i) => {
            multi.write_text(key, i.to_string()).unwrap();
        }
        Value::U64(u) => {
            multi.write_text(key, u.to_string()).unwrap();
        }
        Value::F64(f) => {
            multi.write_text(key, f.to_string()).unwrap();
        }
        Value::String(s) => {
            multi.write_text(key, s.to_string()).unwrap();
        }
        Value::Array(a) => {
            let mut new_key = String::from(key);
            new_key.push_str("[[]]");

            let mut index = 0;
            for item in a {
                let final_key = match item {
                    Value::Array(_) | Value::Object(_) => {
                        new_key.replace("[]", &index.to_string())
                    }
                    _ => {
                        new_key.replace("[]", "")
                    }
                };

                value_to_multi(multi, &final_key, item);
                index += 1;
            }
        }
        Value::Object(map) => {
            let mut new_key = String::from(key);
            new_key.push_str("[{}]");

            for (map_key, map_value) in map {
                value_to_multi(multi, &new_key.replace("{}", &map_key), map_value);
            }
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

        let mut update_args = args::GetUpdates {
            offset: Some(0),
            limit: None,
            timeout: Some(600),
            allowed_updates: None,
        };

                
        'update_loop: loop {
            let updates = bot.get_updates(&update_args).unwrap();

            for update in updates {
                update_args.offset = Some(update.update_id + 1);
                println!("{:?}", update_args.offset);

                if let Some(message) = update.message {
                    let from = message.from.unwrap();

                    let message_text = message.text.unwrap_or(String::new());
                    let mut split_text = message_text.split_whitespace();

                    if let Some(cmd) = split_text.next() {
                        match cmd {
                            "/exit" => {
                                bot.send_message(&args::SendMessage {
                                    chat_id: Some(message.chat.id),
                                    chat_username: None,
                                    text: "Goodbye!",
                                    parse_mode: Some("Markdown"),
                                    disable_web_page_preview: None,
                                    disable_notification: None,
                                    reply_to_message_id: Some(message.message_id),
                                    reply_markup: None,
                                });
                                break 'update_loop;
                            }
                            "/start" | "/help" => {
                                bot.send_message(&args::SendMessage {
                                    chat_id: Some(message.chat.id),
                                    chat_username: None,
                                    text: "Hi, I'm a bot!",
                                    parse_mode: Some("Markdown"),
                                    disable_web_page_preview: None,
                                    disable_notification: None,
                                    reply_to_message_id: Some(message.message_id),
                                    reply_markup: None,
                                });
                            }
                            "/photo" => {
                                bot.send_photo(&args::SendPhoto {
                                    chat_id: Some(message.chat.id),
                                    chat_username: None,
                                    photo: Some("photo.png"),
                                    file_id: None,
                                    caption: Some("Yeahboi"),
                                    disable_notification: None,
                                    reply_to_message_id: Some(message.message_id),
                                    reply_markup: None,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        update_args.limit = Some(0);
        update_args.timeout = Some(0);
        bot.get_updates(&update_args);
        // Alright, so if you ever decide to have a function that terminates
        // your bot, make sure you have a check at the beginning of the loop
        // that makes sure you aren't processing old messages. You could also
        // just make a getUpdates at the end of the execution that just uses
        // the latest offset. This prevents you from having to reread any
        // updates that were in the update array you got that had the terminate
        // command. Because telegram only will stop sending you the update
        // after you have used an offset greater than its. So if you never make
        // another getUpdates, you will boot up, and terminate, forever. :)
    }
}
