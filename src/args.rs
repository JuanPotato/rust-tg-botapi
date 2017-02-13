use super::types;

#[derive(Debug, Builder, Serialize)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<String>,
}

#[derive(Debug, Builder, Serialize)]
pub struct SetWebhook {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
pub struct SendMessage {
    pub chat_id: ChatId,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
pub struct ForwardMessage {
    pub chat_id: ChatId,
    pub from_chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    pub message_id: i64,
}

#[derive(Debug, Builder, Serialize)]
pub struct SendPhoto {
    pub chat_id: ChatId,
    pub photo: Option<String>,
    pub file_id: Option<String>,
    pub caption: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
pub struct SendAudio {
    pub chat_id: ChatId,
    pub audio: Option<String>,
    pub file_id: Option<String>,
    pub caption: Option<String>,
    pub duration: Option<i64>,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
pub struct SendDocument {
    pub chat_id: ChatId,
    pub document: Option<String>,
    pub file_id: Option<String>,
    pub caption: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
pub struct SendSticker {
    pub chat_id: ChatId,
    pub sticker: Option<String>,
    pub file_id: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
pub struct SendVideo {
    pub chat_id: ChatId,
    pub video: Option<String>,
    pub file_id: Option<String>,
    pub duration: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub caption: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
pub struct SendVoice {
    pub chat_id: ChatId,
    pub voice: Option<String>,
    pub file_id: Option<String>,
    pub caption: Option<String>,
    pub duration: Option<i64>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
pub struct SendLocation {
    pub chat_id: ChatId,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
pub struct SendVenue {
    pub chat_id: ChatId,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
pub struct SendContact {
    pub chat_id: ChatId,
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Builder, Serialize)]
pub struct GetUserProfilePhotos {
    pub user_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Builder, Serialize)]
pub struct GetFile {
    pub file_id: String,
}

#[derive(Debug, Builder, Serialize)]
pub struct KickChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Builder, Serialize)]
pub struct LeaveChat {
    pub chat_id: ChatId,
}

#[derive(Debug, Builder, Serialize)]
pub struct UnbanChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Builder, Serialize)]
pub struct GetChat {
    pub chat_id: ChatId,
}

#[derive(Debug, Builder, Serialize)]
pub struct GetChatAdministrators {
    pub chat_id: ChatId,
}

#[derive(Debug, Builder, Serialize)]
pub struct GetChatMembersCount {
    pub chat_id: ChatId,
}

#[derive(Debug, Builder, Serialize)]
pub struct GetChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Builder, Serialize)]
pub struct AnswerCallbackQuery {
    pub callback_query_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
}

#[derive(Debug, Builder, Serialize)]
pub struct EditMessageText {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug, Builder, Serialize)]
pub struct EditMessageCaption {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug, Builder, Serialize)]
pub struct EditMessageReplyMarkup {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug, Builder, Serialize)]
pub struct AnswerInlineQuery {
    pub inline_query_id: String,
    pub results: Vec<types::InlineQueryResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<String>,
}

impl AnswerInlineQuery {
    pub fn new(id: String, results: Vec<types::InlineQueryResult>) -> AnswerInlineQuery {
        AnswerInlineQuery {
            inline_query_id: id,
            results: results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }
}

#[derive(Debug, Builder, Serialize)]
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
pub struct SetGameScore {
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
    pub inline_message_id: Option<String>,
}

#[derive(Debug, Builder, Serialize)]
pub struct GetGameHighScores {
    pub user_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
