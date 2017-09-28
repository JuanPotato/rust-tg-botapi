#[macro_use]
extern crate derive_builder;
extern crate hyper;
extern crate multipart;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper_native_tls;
#[macro_use]
extern crate debug_stub_derive;

use std::{error, fmt};
use std::result::Result;
use std::io::Read;
use std::time::Duration;
use std::path::PathBuf;

use hyper::net::{Streaming, HttpsConnector};
use hyper::method::Method;
use hyper::client::Request;

use hyper::{Client, Url};

use hyper_native_tls::NativeTlsClient;

use multipart::client::Multipart;

use serde_json::{Map, Value};

pub mod types;
pub mod args;

use types::*;

// TODO: Organize this mess and get rid of most unwraps

macro_rules! api_method {
    ($func_name:ident, $method_name:expr, $return_type:ty) => (
        pub fn $func_name(&self) -> Result<$return_type, BotError> {
            let url = self.base_url.join($method_name).unwrap();
            let res = self.client.get(url).send();
            
            match parse_request(res, self.debug) {
                Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
                Err(e) => Err(e),
            }
        }
    );

    ($func_name:ident, $method_name:expr, $arg_type:ty, $return_type:ty) => (
        pub fn $func_name(&self, params: &$arg_type) -> Result<$return_type, BotError> {
            let url = self.base_url.join($method_name).unwrap();
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
    );
}

macro_rules! api_method_msgorbool {
    ($func_name:ident, $method_name:expr, $arg_type:ty) => (
        pub fn $func_name(&self, params: &$arg_type) -> Result<MessageOrBool, BotError> {
            let url = self.base_url.join($method_name).unwrap();
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
    );
}

macro_rules! api_method_multi {
    ($func_name:ident, $method_name:expr, $arg_type:ty, $return_type:ty, $attr:ident) => (
        pub fn $func_name(&self, params: &$arg_type) -> Result<$return_type, BotError> {
            let url = self.base_url.join($method_name).unwrap();
            let req = Request::with_connector(Method::Post, url, &self.connector).unwrap();
            let mut multi = Multipart::from_request(req).unwrap();

            if let Some(ref $attr) = params.$attr {
                multi.write_file(stringify!($attr), $attr).unwrap();
            } else if let Some(ref file_id) = params.file_id {
                multi.write_text(stringify!($attr), file_id).unwrap();
            }

            value_to_root_multi(&mut multi, serde_json::to_value(params).unwrap());

            match parse_request(multi.send(), self.debug) {
                Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
                Err(e) => Err(e),
            }
        }
    );

    ($func_name:ident, $method_name:expr, $arg_type:ty, $return_type:ty, $attr:ident;) => (
        pub fn $func_name(&self, params: &$arg_type) -> Result<$return_type, BotError> {
            let url = self.base_url.join($method_name).unwrap();
            let req = Request::with_connector(Method::Post, url, &self.connector).unwrap();
            let mut multi = Multipart::from_request(req).unwrap();

            multi.write_file(stringify!($attr), &params.$attr).unwrap();

            value_to_root_multi(&mut multi, serde_json::to_value(params).unwrap());

            match parse_request(multi.send(), self.debug) {
                Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
                Err(e) => Err(e),
            }
        }
    );
}

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
        match *self {
            BotError::Http(ref e) => write!(f, "{}", e),
            BotError::Api { error_code, ref description, .. } => {
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


#[derive(DebugStub)]
pub struct BotApi {
    base_url: Url,
    client: Client,
    #[debug_stub="HttpsConnector"]
    connector: hyper::net::HttpsConnector<hyper_native_tls::NativeTlsClient>,
    debug: bool,
}

impl BotApi {
    fn new_bot(bot_token: &str, debug: bool) -> BotApi {
        let url = format!("https://api.telegram.org/bot{}/", bot_token);
        // TODO validate this token

        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let mut client = Client::with_connector(connector);

        client.set_read_timeout(Some(Duration::new(60, 0)));


        let main_ssl = NativeTlsClient::new().unwrap();
        let main_connector = HttpsConnector::new(main_ssl);

        BotApi {
            base_url: url.parse().unwrap(),
            client: client,
            connector: main_connector,
            debug: debug,
        }
    }

    pub fn new(bot_token: &str) -> BotApi {
        BotApi::new_bot(bot_token, false)
    }

    pub fn new_debug(bot_token: &str) -> BotApi {
        BotApi::new_bot(bot_token, true)
    }

    api_method!(get_me, "getMe", User);

    api_method!(get_updates, "getUpdates", args::GetUpdates, Vec<Update>);

    api_method!(send_message, "sendMessage", args::SendMessage, Message);

    api_method!(forward_message, "forwardMessage", args::ForwardMessage, Message);

    api_method_multi!(send_photo, "sendPhoto", args::SendPhoto, Message, photo);

    api_method_multi!(send_audio, "sendAudio", args::SendAudio, Message, audio);

    api_method_multi!(send_document, "sendDocument", args::SendDocument, Message, document);

    api_method_multi!(send_sticker, "sendSticker", args::SendSticker, Message, sticker);

    api_method_multi!(send_video, "sendVideo", args::SendVideo, Message, video);

    api_method_multi!(send_voice, "sendVoice", args::SendVoice, Message, voice);

    api_method!(get_user_profile_photos, "getUserProfilePhotos", args::GetUserProfilePhotos, UserProfilePhotos);

    api_method!(get_file, "getFile", args::GetFile, File);

    api_method!(kick_chat_member, "kickChatMember", args::KickChatMember, bool);

    api_method!(leave_chat, "leaveChat", args::LeaveChat, bool);

    api_method!(unban_chat_member, "unbanChatMember", args::UnbanChatMember, bool);

    api_method!(get_chat, "getChat", args::GetChat, Chat);

    api_method!(get_chat_administrators, "getChatAdministrators", args::GetChatAdministrators, Vec<ChatMember>);

    api_method!(get_chat_members_count, "getChatMembersCount", args::GetChatMembersCount, i64);

    api_method!(get_chat_member, "getChatMember", args::GetChatMember, ChatMember);

    api_method!(answer_callback_query, "answerCallbackQuery", args::AnswerCallbackQuery, bool);

    api_method_msgorbool!(edit_message_text, "editMessageText", args::EditMessageText);

    api_method_msgorbool!(edit_message_caption, "editMessageCaption", args::EditMessageCaption);
    
    api_method_msgorbool!(edit_message_reply_markup, "editMessageReplyMarkup", args::EditMessageReplyMarkup);

    api_method!(answer_inline_query, "answerInlineQuery", args::AnswerInlineQuery, bool);

    api_method!(send_game, "sendGame", args::SendGame, Message);

    api_method_msgorbool!(set_game_score, "setGameScore", args::SetGameScore);

    api_method!(get_game_high_scores, "getGameHighScores", args::GetGameHighScores, Vec<GameHighScore>);
    
    api_method_multi!(add_sticker_to_set, "addStickerToSet", args::AddStickerToSet, bool, png_sticker);

    api_method!(answer_pre_checkout_query, "answerPreCheckoutQuery", args::AnswerPreCheckoutQuery, bool);

    api_method!(answer_shipping_query, "answerShippingQuery", args::AnswerShippingQuery, bool);

    api_method_multi!(create_new_sticker_set, "createNewStickerSet", args::CreateNewStickerSet, bool, png_sticker);

    api_method!(delete_chat_photo, "deleteChatPhoto", args::DeleteChatPhoto, bool);

    api_method!(delete_message, "deleteMessage", args::DeleteMessage, bool);

    api_method!(delete_sticker_from_set, "deleteStickerFromSet", args::DeleteStickerFromSet, bool);

    api_method!(delete_webhook, "deleteWebhook", bool);

    api_method!(export_chat_invite_link, "exportChatInviteLink", args::ExportChatInviteLink, String);
    
    api_method!(get_sticker_set, "getStickerSet", args::GetStickerSet, StickerSet);

    api_method!(pin_chat_message, "pinChatMessage", args::PinChatMessage, bool);

    api_method!(promote_chat_member, "promoteChatMember", args::PromoteChatMember, bool);

    api_method!(restrict_chat_member, "restrictChatMember", args::RestrictChatMember, bool);

    api_method!(send_contact, "sendContact", args::SendContact, Message);

    api_method!(send_invoice, "sendInvoice", args::SendInvoice, Message);

    api_method!(send_location, "sendLocation", args::SendLocation, Message);

    api_method!(send_venue, "sendVenue", args::SendVenue, Message);

    api_method_multi!(send_video_note, "sendVideoNote", args::SendVideoNote, Message, video_note);

    api_method!(set_chat_description, "setChatDescription", args::SetChatDescription, bool);

    api_method_multi!(set_chat_photo, "setChatPhoto", args::SetChatPhoto, bool, photo;);

    api_method!(set_chat_title, "setChatTitle", args::SetChatTitle, bool);

    api_method!(set_sticker_position_in_set, "setStickerPositionInSet", args::SetStickerPositionInSet, bool);

    pub fn set_webhook(&self, params: &args::SetWebhook) -> Result<bool, BotError> {
        let url = self.base_url.join("setWebhook").unwrap();
        let req = Request::with_connector(Method::Post, url, &self.connector).unwrap();
        let mut multi = Multipart::from_request(req).unwrap();

        if let Some(ref certificate) = params.certificate {
            multi.write_file("certificate", certificate).unwrap();
        }

        value_to_root_multi(&mut multi, serde_json::to_value(params).unwrap());

        match parse_request(multi.send(), self.debug) {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    api_method!(unpin_chat_message, "unpinChatMessage", args::UnpinChatMessage, bool);

    api_method_multi!(upload_sticker_file, "uploadStickerFile", args::UploadStickerFile, File, png_sticker;);
}

fn value_to_multi(multi: &mut Multipart<Request<Streaming>>, key: &str, value: Value) {
    match value {
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

            for (index, item) in a.into_iter().enumerate() {
                let final_key = match item {
                    Value::Array(_) | Value::Object(_) => new_key.replace("[]", &index.to_string()),
                    _ => new_key.replace("[]", ""),
                };

                value_to_multi(multi, &final_key, item);
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

fn value_to_root_multi(multi: &mut Multipart<Request<Streaming>>, value: Value) {
    match value {
        Value::Object(map) => {
            for (map_key, map_value) in map {
                value_to_multi(multi, &map_key, map_value);
            }
        }

        _ => { panic!("This will never occur") } // TODO: prove myself wrong
    }
}
