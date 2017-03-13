use types::ReplyMarkup;
use types::InlineQueryResult;

#[derive(Debug, Clone, new, Serialize)]
pub struct GetUpdates<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub timeout: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub allowed_updates: Option<&'a [&'a str]>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SetWebhook<'a> {
    pub url: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub certificate: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub max_connections: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub allowed_updates: Option<&'a [&'a str]>,
}

#[derive(Debug, Clone, From, Copy, Serialize)]
#[serde(untagged)]
pub enum ChatId<'a> {
    Id(i64),
    Username(&'a str),
}

impl<'a> ToString for ChatId<'a> {
    fn to_string(&self) -> String {
        match *self {
            ChatId::Id(id) => id.to_string(),
            ChatId::Username(username) => username.to_owned(),
        }
    }
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SendMessage<'a> {
    pub chat_id: ChatId<'a>,
    pub text: &'a str,
    #[new(default)]
    pub parse_mode: Option<&'a str>,
    #[new(default)]
    pub disable_web_page_preview: Option<bool>,
    #[new(default)]
    pub disable_notification: Option<bool>,
    #[new(default)]
    pub reply_to_message_id: Option<i64>,
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct ForwardMessage<'a> {
    pub chat_id: ChatId<'a>,
    pub from_chat_id: ChatId<'a>,
    #[new(default)]
    pub disable_notification: Option<bool>,
    pub message_id: i64,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SendPhoto<'a> {
    pub chat_id: ChatId<'a>,
    #[new(default)]
    pub photo: Option<&'a str>,
    #[new(default)]
    pub file_id: Option<&'a str>,
    #[new(default)]
    pub caption: Option<&'a str>,
    #[new(default)]
    pub disable_notification: Option<bool>,
    #[new(default)]
    pub reply_to_message_id: Option<i64>,
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SendAudio<'a> {
    pub chat_id: ChatId<'a>,
    #[new(default)]
    pub audio: Option<&'a str>,
    #[new(default)]
    pub file_id: Option<&'a str>,
    #[new(default)]
    pub caption: Option<&'a str>,
    #[new(default)]
    pub duration: Option<i64>,
    #[new(default)]
    pub performer: Option<&'a str>,
    #[new(default)]
    pub title: Option<&'a str>,
    #[new(default)]
    pub disable_notification: Option<bool>,
    #[new(default)]
    pub reply_to_message_id: Option<i64>,
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SendDocument<'a> {
    pub chat_id: ChatId<'a>,
    #[new(default)]
    pub document: Option<&'a str>,
    #[new(default)]
    pub file_id: Option<&'a str>,
    #[new(default)]
    pub caption: Option<&'a str>,
    #[new(default)]
    pub disable_notification: Option<bool>,
    #[new(default)]
    pub reply_to_message_id: Option<i64>,
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SendSticker<'a> {
    pub chat_id: ChatId<'a>,
    #[new(default)]
    pub sticker: Option<&'a str>,
    #[new(default)]
    pub file_id: Option<&'a str>,
    #[new(default)]
    pub disable_notification: Option<bool>,
    #[new(default)]
    pub reply_to_message_id: Option<i64>,
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SendVideo<'a> {
    pub chat_id: ChatId<'a>,
    #[new(default)]
    pub video: Option<&'a str>,
    #[new(default)]
    pub file_id: Option<&'a str>,
    #[new(default)]
    pub duration: Option<i64>,
    #[new(default)]
    pub width: Option<i64>,
    #[new(default)]
    pub height: Option<i64>,
    #[new(default)]
    pub caption: Option<&'a str>,
    #[new(default)]
    pub disable_notification: Option<bool>,
    #[new(default)]
    pub reply_to_message_id: Option<i64>,
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SendVoice<'a> {
    pub chat_id: ChatId<'a>,
    #[new(default)]
    pub voice: Option<&'a str>,
    #[new(default)]
    pub file_id: Option<&'a str>,
    #[new(default)]
    pub caption: Option<&'a str>,
    #[new(default)]
    pub duration: Option<i64>,
    #[new(default)]
    pub disable_notification: Option<bool>,
    #[new(default)]
    pub reply_to_message_id: Option<i64>,
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SendLocation<'a> {
    pub chat_id: ChatId<'a>,
    pub latitude: f64,
    pub longitude: f64,
    #[new(default)]
    pub disable_notification: Option<bool>,
    #[new(default)]
    pub reply_to_message_id: Option<i64>,
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SendVenue<'a> {
    pub chat_id: ChatId<'a>,
    pub latitude: f64,
    pub longitude: f64,
    pub title: &'a str,
    pub address: &'a str,
    #[new(default)]
    pub foursquare_id: Option<&'a str>,
    #[new(default)]
    pub disable_notification: Option<bool>,
    #[new(default)]
    pub reply_to_message_id: Option<i64>,
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SendContact<'a> {
    pub chat_id: ChatId<'a>,
    pub phone_number: &'a str,
    pub first_name: &'a str,
    #[new(default)]
    pub last_name: Option<&'a str>,
    #[new(default)]
    pub disable_notification: Option<bool>,
    #[new(default)]
    pub reply_to_message_id: Option<i64>,
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct GetUserProfilePhotos {
    pub user_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct GetFile<'a> {
    pub file_id: &'a str,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct KickChatMember<'a> {
    pub chat_id: ChatId<'a>,
    pub user_id: i64,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct LeaveChat<'a> {
    pub chat_id: ChatId<'a>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct UnbanChatMember<'a> {
    pub chat_id: ChatId<'a>,
    pub user_id: i64,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct GetChat<'a> {
    pub chat_id: ChatId<'a>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct GetChatAdministrators<'a> {
    pub chat_id: ChatId<'a>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct GetChatMembersCount<'a> {
    pub chat_id: ChatId<'a>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct GetChatMember<'a> {
    pub chat_id: ChatId<'a>,
    pub user_id: i64,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct AnswerCallbackQuery<'a> {
    pub callback_query_id: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub text: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub show_alert: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub url: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub cache_time: Option<i64>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct EditMessageText<'a> {
    pub chat_id: ChatId<'a>,
    #[new(default)]
    pub message_id: Option<i64>,
    #[new(default)]
    pub inline_message_id: Option<&'a str>,
    pub text: &'a str,
    #[new(default)]
    pub parse_mode: Option<&'a str>,
    #[new(default)]
    pub disable_web_page_preview: Option<bool>,
    // InlineKeyboardMarkup
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct EditMessageCaption<'a> {
    pub chat_id: ChatId<'a>,
    #[new(default)]
    pub message_id: Option<i64>,
    #[new(default)]
    pub inline_message_id: Option<&'a str>,
    #[new(default)]
    pub caption: Option<&'a str>,
    // InlineKeyboardMarkup
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct EditMessageReplyMarkup<'a> {
    pub chat_id: ChatId<'a>,
    #[new(default)]
    pub message_id: Option<i64>,
    #[new(default)]
    pub inline_message_id: Option<&'a str>,
    // InlineKeyboardMarkup
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct AnswerInlineQuery<'a> {
    pub inline_query_id: &'a str,
    pub results: Vec<InlineQueryResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub cache_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub is_personal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub next_offset: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub switch_pm_text: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub switch_pm_parameter: Option<&'a str>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SendGame<'a> {
    pub chat_id: i64,
    pub game_short_name: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub reply_to_message_id: Option<i64>,

    // InlineKeyboardMarkup
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct SetGameScore<'a> {
    pub user_id: i64,
    pub score: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub force: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub disable_edit_message: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub inline_message_id: Option<&'a str>,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct GetGameHighScores<'a> {
    pub user_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub inline_message_id: Option<&'a str>,
}
