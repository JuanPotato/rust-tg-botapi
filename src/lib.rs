#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate multipart;
extern crate serde_json;

use hyper::net::Streaming;
use hyper::method::Method;
use hyper::client::Request;

use hyper::Client;
use hyper::Url;

use multipart::client::Multipart;

use serde_json::value::Value;

use std::result::Result;
use std::io::Read;
use std::{fmt, error};
use std::time::Duration;

pub mod types;
pub mod args;
pub mod builders;

use types::*;

// TODO: Organize this mess and get rid of most unwraps

#[derive(Debug)]
pub enum BotError {
    Http(hyper::error::Error),
    Api {
        error_code: i64,
        description: String,
        parameters: Option<ResponseParameters>,
    },
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BotError::Http(ref e) => write!(f, "{}", e),
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

fn parse_request(respon_result: Result<hyper::client::Response, hyper::Error>,
                 debug: bool)
                 -> Result<Value, BotError> {
    match respon_result {
        Ok(mut response) => {
            let mut body = String::new();
            response.read_to_string(&mut body).unwrap();
            if debug {
                println!("{}", body);
            }
            let result: ApiResult = serde_json::from_str(&body).unwrap();
            if let Some(val) = result.result {
                Ok(val)
            } else {
                Err(BotError::Api {
                        error_code: result.error_code.unwrap(),
                        description: result.description.unwrap(),
                        parameters: result.parameters,
                    })
            }
        }
        Err(e) => Err(BotError::Http(e)),
    }
}


#[derive(Debug)]
pub struct BotApi {
    base_url: Url,
    client: Client,
    debug: bool,
}

impl BotApi {
    fn new_bot(bot_token: &str, debug: bool) -> BotApi {
        let url = format!("https://api.telegram.org/bot{}/", bot_token);
        // TODO validate this token

        let mut c = Client::new();
        c.set_read_timeout(Some(Duration::new(60, 0)));

        BotApi {
            base_url: url.parse().unwrap(),
            client: c,
            debug: debug,
        }
    }

    pub fn new(bot_token: &str) -> BotApi {
        BotApi::new_bot(bot_token, false)
    }

    pub fn new_debug(bot_token: &str) -> BotApi {
        BotApi::new_bot(bot_token, true)
    }

    pub fn get_me(&self) -> Result<User, BotError> {
        let url = self.base_url.join("getMe").unwrap();
        match parse_request(self.client.get(url).send(), self.debug) {
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

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_message(&self, params: &args::SendMessage) -> Result<Message, BotError> {
        let url = self.base_url.join("sendMessage").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn forward_message(&self, params: &args::ForwardMessage) -> Result<Message, BotError> {
        let url = self.base_url.join("forwardMessage").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_photo(&self, params: &args::SendPhoto) -> Result<Message, BotError> {
        let url = self.base_url.join("sendPhoto").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        multi.write_text("chat_id", params.chat_id.to_string()).unwrap();

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

        if let Some(ref reply_markup) = params.reply_markup {
            value_to_multi(&mut multi,
                           "reply_markup",
                           serde_json::to_value(reply_markup).unwrap());
        }

        match parse_request(multi.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_audio(&self, params: &args::SendAudio) -> Result<Message, BotError> {
        let url = self.base_url.join("sendAudio").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        multi.write_text("chat_id", params.chat_id.to_string()).unwrap();

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

        if let Some(ref reply_markup) = params.reply_markup {
            value_to_multi(&mut multi,
                           "reply_markup",
                           serde_json::to_value(reply_markup).unwrap());
        }

        match parse_request(multi.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_document(&self, params: &args::SendDocument) -> Result<Message, BotError> {
        let url = self.base_url.join("sendDocument").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        multi.write_text("chat_id", params.chat_id.to_string()).unwrap();

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

        if let Some(ref reply_markup) = params.reply_markup {
            value_to_multi(&mut multi,
                           "reply_markup",
                           serde_json::to_value(reply_markup).unwrap());
        }

        match parse_request(multi.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_sticker(&self, params: &args::SendSticker) -> Result<Message, BotError> {
        let url = self.base_url.join("sendSticker").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        multi.write_text("chat_id", params.chat_id.to_string()).unwrap();

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

        if let Some(ref reply_markup) = params.reply_markup {
            value_to_multi(&mut multi,
                           "reply_markup",
                           serde_json::to_value(reply_markup).unwrap());
        }

        match parse_request(multi.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_video(&self, params: &args::SendVideo) -> Result<Message, BotError> {
        let url = self.base_url.join("sendVideo").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        multi.write_text("chat_id", params.chat_id.to_string()).unwrap();

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

        if let Some(ref reply_markup) = params.reply_markup {
            value_to_multi(&mut multi,
                           "reply_markup",
                           serde_json::to_value(reply_markup).unwrap());
        }

        match parse_request(multi.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_voice(&self, params: &args::SendVoice) -> Result<Message, BotError> {
        let url = self.base_url.join("sendVoice").unwrap();
        let req = Request::new(Method::Post, url).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        multi.write_text("chat_id", params.chat_id.to_string()).unwrap();

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

        if let Some(ref reply_markup) = params.reply_markup {
            value_to_multi(&mut multi,
                           "reply_markup",
                           serde_json::to_value(reply_markup).unwrap());
        }

        match parse_request(multi.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn get_user_profile_photos(&self,
                                   params: &args::GetUserProfilePhotos)
                                   -> Result<UserProfilePhotos, BotError> {
        let url = self.base_url.join("getUserProfilePhotos").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn get_file(&self, params: &args::GetFile) -> Result<File, BotError> {
        let url = self.base_url.join("getFile").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn kick_chat_member(&self, params: &args::KickChatMember) -> Result<bool, BotError> {
        let url = self.base_url.join("kickChatMember").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn leave_chat(&self, params: &args::LeaveChat) -> Result<bool, BotError> {
        let url = self.base_url.join("leaveChat").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn unban_chat_member(&self, params: &args::UnbanChatMember) -> Result<bool, BotError> {
        let url = self.base_url.join("unbanChatMember").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn get_chat(&self, params: &args::GetChat) -> Result<Chat, BotError> {
        let url = self.base_url.join("getChat").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn get_chat_administrators(&self,
                                   params: &args::GetChatAdministrators)
                                   -> Result<Vec<ChatMember>, BotError> {
        let url = self.base_url.join("getChatAdministrators").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn get_chat_members_count(&self,
                                  params: &args::GetChatMembersCount)
                                  -> Result<i64, BotError> {
        let url = self.base_url.join("getChatMembersCount").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn get_chat_member(&self, params: &args::GetChatMember) -> Result<ChatMember, BotError> {
        let url = self.base_url.join("getChatMember").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn answer_callback_query(&self,
                                 params: &args::AnswerCallbackQuery)
                                 -> Result<bool, BotError> {
        let url = self.base_url.join("answerCallbackQuery").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn edit_message_text(&self,
                             params: &args::EditMessageText)
                             -> Result<MessageOrBool, BotError> {
        let url = self.base_url.join("editMessageText").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => {
                match val {
                    Value::Bool(b) => Ok(MessageOrBool::B(b)),
                    Value::Object(_) => {
                        Ok(MessageOrBool::M(serde_json::value::from_value(val).unwrap()))
                    }
                    _ => Ok(MessageOrBool::B(false)),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn edit_message_caption(&self,
                                params: &args::EditMessageCaption)
                                -> Result<MessageOrBool, BotError> {
        let url = self.base_url.join("editMessageCaption").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => {
                match val {
                    Value::Bool(b) => Ok(MessageOrBool::B(b)),
                    Value::Object(_) => {
                        Ok(MessageOrBool::M(serde_json::value::from_value(val).unwrap()))
                    }
                    _ => Ok(MessageOrBool::B(false)),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn edit_message_reply_markup(&self,
                                     params: &args::EditMessageReplyMarkup)
                                     -> Result<MessageOrBool, BotError> {
        let url = self.base_url.join("editMessageReplyMarkup").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => {
                match val {
                    Value::Bool(b) => Ok(MessageOrBool::B(b)),
                    Value::Object(_) => {
                        Ok(MessageOrBool::M(serde_json::value::from_value(val).unwrap()))
                    }
                    _ => Ok(MessageOrBool::B(false)),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn answer_inline_query(&self, params: &args::AnswerInlineQuery) -> Result<bool, BotError> {
        let url = self.base_url.join("answerInlineQuery").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn send_game(&self, params: &args::SendGame) -> Result<Message, BotError> {
        let url = self.base_url.join("sendGame").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn set_game_score(&self, params: &args::SetGameScore) -> Result<MessageOrBool, BotError> {
        let url = self.base_url.join("setGameScore").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
            Ok(val) => {
                match val {
                    Value::Bool(b) => Ok(MessageOrBool::B(b)),
                    Value::Object(_) => {
                        Ok(MessageOrBool::M(serde_json::value::from_value(val).unwrap()))
                    }
                    _ => Ok(MessageOrBool::B(false)),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_game_high_scores(&self,
                                params: &args::GetGameHighScores)
                                -> Result<Vec<GameHighScore>, BotError> {
        let url = self.base_url.join("getGameHighScores").unwrap();
        let body = serde_json::to_string(params).unwrap();

        let res = self.client
            .post(url)
            .header(hyper::header::ContentType::json())
            .body(&body);

        match parse_request(res.send(), self.debug) {
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
        Value::Number(n) => {
            multi.write_text(key, n.to_string()).unwrap();
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
                    Value::Array(_) | Value::Object(_) => new_key.replace("[]", &index.to_string()),
                    _ => new_key.replace("[]", ""),
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
