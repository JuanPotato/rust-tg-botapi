use std::path::PathBuf;

use super::types;

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub timeout: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub allowed_updates: Option<String>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SetWebhook {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub certificate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub max_connections: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub allowed_updates: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatId {
    Id(i64),
    Username(String),
}

impl From<i64> for ChatId {
    fn from(id: i64) -> ChatId {
        ChatId::Id(id)
    }
}

impl From<String> for ChatId {
    fn from(id: String) -> ChatId {
        ChatId::Username(id)
    }
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SendMessage {
    pub chat_id: ChatId,
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub parse_mode: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_web_page_preview: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_notification: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct ForwardMessage {
    pub chat_id: ChatId,
    pub from_chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_notification: Option<bool>,
    pub message_id: i64,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SendPhoto {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub photo: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub file_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SendAudio {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub audio: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub file_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SendDocument {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub document: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub file_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SendSticker {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub sticker: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub file_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SendVideo {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub video: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub file_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub height: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SendVoice {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub voice: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub file_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SendLocation {
    pub chat_id: ChatId,
    pub latitude: f64,
    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SendVenue {
    pub chat_id: ChatId,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub foursquare_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SendContact {
    pub chat_id: ChatId,
    pub phone_number: String,
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct GetUserProfilePhotos {
    pub user_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub limit: Option<i64>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct GetFile {
    pub file_id: String,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct KickChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct LeaveChat {
    pub chat_id: ChatId,
}

impl LeaveChat {
    pub fn new(chat_id: ChatId) -> LeaveChat {
        LeaveChat { chat_id: chat_id }
    }
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct UnbanChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct GetChat {
    pub chat_id: ChatId,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct GetChatAdministrators {
    pub chat_id: ChatId,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct GetChatMembersCount {
    pub chat_id: ChatId,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct GetChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct AnswerCallbackQuery {
    pub callback_query_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub show_alert: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub cache_time: Option<i64>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct EditMessageText {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub inline_message_id: Option<String>,
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_web_page_preview: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct EditMessageCaption {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct EditMessageReplyMarkup {
    pub chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct AnswerInlineQuery {
    pub inline_query_id: String,
    pub results: Vec<types::InlineQueryResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub cache_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub is_personal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub next_offset: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub switch_pm_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub switch_pm_parameter: Option<String>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SendGame {
    pub chat_id: i64,
    pub game_short_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct SetGameScore {
    pub user_id: i64,
    pub score: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub force: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub disable_edit_message: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub inline_message_id: Option<String>,
}

#[derive(Debug, Builder, Serialize)]
#[builder(setter(into))]
pub struct GetGameHighScores {
    pub user_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default="None")]
    pub inline_message_id: Option<String>,
}
