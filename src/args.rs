extern crate serde_json;

use serde_json::value::Value;

use super::types;


#[derive(Debug)]
pub struct GetUpdates<'a> {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub timeout: Option<i64>,
    pub allowed_updates: Option<&'a [&'a str]>,
}

#[derive(Debug)]
pub struct SetWebhook<'a> {
    pub url: &'a str,
    pub certificate: Option<&'a str>,
    pub max_connections: Option<i64>,
    pub allowed_updates: Option<&'a [&'a str]>,
}

#[derive(Debug)]
pub struct SendMessage<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub text: &'a str,
    pub parse_mode: Option<&'a str>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>, // Will probably & ReplyMarkup's
}

#[derive(Debug)]
pub struct ForwardMessage<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub from_chat_id: Option<i64>,
    pub from_chat_username: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub message_id: i64,
}

#[derive(Debug)]
pub struct SendPhoto<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub photo: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug)]
pub struct SendAudio<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub audio: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub duration: Option<i64>,
    pub performer: Option<&'a str>,
    pub title: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug)]
pub struct SendDocument<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub document: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug)]
pub struct SendSticker<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub sticker: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug)]
pub struct SendVideo<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub video: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub duration: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub caption: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug)]
pub struct SendVoice<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub voice: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub duration: Option<i64>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug)]
pub struct SendLocation<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub latitude: f64,
    pub longitude: f64,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug)]
pub struct SendVenue<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub latitude: f64,
    pub longitude: f64,
    pub title: &'a str,
    pub address: &'a str,
    pub foursquare_id: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug)]
pub struct SendContact<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub phone_number: &'a str,
    pub first_name: &'a str,
    pub last_name: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug)]
pub struct GetUserProfilePhotos {
    pub user_id: i64,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug)]
pub struct GetFile<'a> {
    pub file_id: &'a str,
}

#[derive(Debug)]
pub struct KickChatMember<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub user_id: i64,
}

#[derive(Debug)]
pub struct LeaveChat<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
}

#[derive(Debug)]
pub struct UnbanChatMember<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub user_id: i64,
}

#[derive(Debug)]
pub struct GetChat<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
}

#[derive(Debug)]
pub struct GetChatAdministrators<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
}

#[derive(Debug)]
pub struct GetChatMembersCount<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
}

#[derive(Debug)]
pub struct GetChatMember<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub user_id: i64,
}

#[derive(Debug)]
pub struct AnswerCallbackQuery<'a> {
    pub callback_query_id: &'a str,
    pub text: Option<&'a str>,
    pub show_alert: Option<bool>,
    pub url: Option<&'a str>,
    pub cache_time: Option<i64>,
}

#[derive(Debug)]
pub struct EditMessageText<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<&'a str>,
    pub text: &'a str,
    pub parse_mode: Option<&'a str>,
    pub disable_web_page_preview: Option<bool>,
    pub reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug)]
pub struct EditMessageCaption<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug)]
pub struct EditMessageReplyMarkup<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<&'a str>,
    pub reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug)]
pub struct AnswerInlineQuery<'a> {
    pub inline_query_id: &'a str,
    pub results: Vec<types::InlineQueryResult>,
    pub cache_time: Option<i64>,
    pub is_personal: Option<bool>,
    pub next_offset: Option<&'a str>,
    pub switch_pm_text: Option<&'a str>,
    pub switch_pm_parameter: Option<&'a str>,
}

#[derive(Debug)]
pub struct SendGame<'a> {
    pub chat_id: i64,
    pub game_short_name: &'a str,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug)]
pub struct GetGameScore<'a> {
    pub user_id: i64,
    pub score: i64,
    pub force: Option<bool>,
    pub disable_edit_message: Option<bool>,
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<&'a str>,
}

#[derive(Debug)]
pub struct GetGameHighScores<'a> {
    pub user_id: i64,
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<&'a str>,
}
