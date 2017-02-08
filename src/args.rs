use std::borrow::Cow;

use super::types;

#[derive(Debug, Serialize)]
pub struct GetUpdates<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<&'a [&'a str]>,
}

#[derive(Debug, Serialize)]
pub struct SetWebhook<'a> {
    pub url: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<&'a [&'a str]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatId<'a> {
    Id(i64),
    Username(Cow<'a, str>),
}

#[derive(Debug, Serialize)]
pub struct SendMessage<'a> {
    pub chat_id: ChatId<'a>,
    pub text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>,
}

#[derive(Debug, Serialize)]
pub struct ForwardMessage<'a> {
    pub chat_id: ChatId<'a>,
    pub from_chat_id: ChatId<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    pub message_id: i64,
}

#[derive(Debug)]
pub struct SendPhoto<'a> {
    pub chat_id: ChatId<'a>,
    pub photo: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>,
}

#[derive(Debug)]
pub struct SendAudio<'a> {
    pub chat_id: ChatId<'a>,
    pub audio: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub duration: Option<i64>,
    pub performer: Option<&'a str>,
    pub title: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>,
}

#[derive(Debug)]
pub struct SendDocument<'a> {
    pub chat_id: ChatId<'a>,
    pub document: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>,
}

#[derive(Debug)]
pub struct SendSticker<'a> {
    pub chat_id: ChatId<'a>,
    pub sticker: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>,
}

#[derive(Debug)]
pub struct SendVideo<'a> {
    pub chat_id: ChatId<'a>,
    pub video: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub duration: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub caption: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>,
}

#[derive(Debug)]
pub struct SendVoice<'a> {
    pub chat_id: ChatId<'a>,
    pub voice: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub duration: Option<i64>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>,
}

#[derive(Debug, Serialize)]
pub struct SendLocation<'a> {
    pub chat_id: ChatId<'a>,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>,
}

#[derive(Debug, Serialize)]
pub struct SendVenue<'a> {
    pub chat_id: ChatId<'a>,
    pub latitude: f64,
    pub longitude: f64,
    pub title: &'a str,
    pub address: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>,
}

#[derive(Debug, Serialize)]
pub struct SendContact<'a> {
    pub chat_id: ChatId<'a>,
    pub phone_number: &'a str,
    pub first_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>,
}

#[derive(Debug, Serialize)]
pub struct GetUserProfilePhotos {
    pub user_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct GetFile<'a> {
    pub file_id: &'a str,
}

#[derive(Debug, Serialize)]
pub struct KickChatMember<'a> {
    pub chat_id: ChatId<'a>,
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct LeaveChat<'a> {
    pub chat_id: ChatId<'a>,
}

#[derive(Debug, Serialize)]
pub struct UnbanChatMember<'a> {
    pub chat_id: ChatId<'a>,
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct GetChat<'a> {
    pub chat_id: ChatId<'a>,
}

#[derive(Debug, Serialize)]
pub struct GetChatAdministrators<'a> {
    pub chat_id: ChatId<'a>,
}

#[derive(Debug, Serialize)]
pub struct GetChatMembersCount<'a> {
    pub chat_id: ChatId<'a>,
}

#[derive(Debug, Serialize)]
pub struct GetChatMember<'a> {
    pub chat_id: ChatId<'a>,
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct AnswerCallbackQuery<'a> {
    pub callback_query_id: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct EditMessageText<'a> {
    pub chat_id: ChatId<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<&'a str>,
    pub text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>, // InlineKeyboardMarkup
}

#[derive(Debug, Serialize)]
pub struct EditMessageCaption<'a> {
    pub chat_id: ChatId<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>, // InlineKeyboardMarkup
}

#[derive(Debug, Serialize)]
pub struct EditMessageReplyMarkup<'a> {
    pub chat_id: ChatId<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>, // InlineKeyboardMarkup
}

#[derive(Debug, Serialize)]
pub struct AnswerInlineQuery<'a> {
    pub inline_query_id: &'a str,
    pub results: &'a [types::InlineQueryResult<'a>],

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct SendGame<'a> {
    pub chat_id: i64,
    pub game_short_name: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup<'a>>, // InlineKeyboardMarkup
}

#[derive(Debug, Serialize)]
pub struct SetGameScore<'a> {
    pub user_id: i64,
    pub score: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct GetGameHighScores<'a> {
    pub user_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<&'a str>,
}
