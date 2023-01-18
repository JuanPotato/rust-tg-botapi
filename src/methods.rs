use crate::types::*;
use crate::helpers::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetWebhook {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWebhookInfo {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMe {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogOut {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Close {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessage {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardMessage {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub from_chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    pub message_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyMessage {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub from_chat_id: ChatId,
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPhoto {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub photo: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendAudio {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub audio: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendDocument {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub document: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendVideo {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub video: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendAnimation {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub animation: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendVoice {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub voice: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendVideoNote {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub video_note: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMediaGroup {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub media: Vec<InputMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendLocation {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMessageLiveLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopMessageLiveLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendVenue {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendContact {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPoll {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub question: String,
    pub options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendDice {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendChatAction {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserProfilePhotos {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFile {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbanChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestrictChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoteChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChatAdministratorCustomTitle {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub custom_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanChatSenderChat {
    pub chat_id: ChatId,
    pub sender_chat_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbanChatSenderChat {
    pub chat_id: ChatId,
    pub sender_chat_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChatPermissions {
    pub chat_id: ChatId,
    pub permissions: ChatPermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportChatInviteLink {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatInviteLink {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditChatInviteLink {
    pub chat_id: ChatId,
    pub invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeChatInviteLink {
    pub chat_id: ChatId,
    pub invite_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveChatJoinRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclineChatJoinRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChatPhoto {
    pub chat_id: ChatId,
    pub photo: InputFile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteChatPhoto {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChatTitle {
    pub chat_id: ChatId,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChatDescription {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinChatMessage {
    pub chat_id: ChatId,
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnpinChatMessage {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnpinAllChatMessages {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveChat {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChat {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAdministrators {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatMemberCount {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChatStickerSet {
    pub chat_id: ChatId,
    pub sticker_set_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteChatStickerSet {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetForumTopicIconStickers {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateForumTopic {
    pub chat_id: ChatId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditForumTopic {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseForumTopic {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReopenForumTopic {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteForumTopic {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnpinAllForumTopicMessages {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditGeneralForumTopic {
    pub chat_id: ChatId,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseGeneralForumTopic {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReopenGeneralForumTopic {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HideGeneralForumTopic {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnhideGeneralForumTopic {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetMyCommands {
    pub commands: Vec<BotCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMyCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMyCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChatMenuButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<MenuButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatMenuButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetMyDefaultAdministratorRights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMyDefaultAdministratorRights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMessageText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMessageCaption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMessageMedia {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub media: InputMedia,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMessageReplyMarkup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopPoll {
    pub chat_id: ChatId,
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessage {
    pub chat_id: ChatId,
    pub message_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendSticker {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub sticker: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStickerSet {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomEmojiStickers {
    pub custom_emoji_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadStickerFile {
    pub user_id: i64,
    pub png_sticker: InputFile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNewStickerSet {
    pub user_id: i64,
    pub name: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png_sticker: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgs_sticker: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webm_sticker: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<String>,
    pub emojis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddStickerToSet {
    pub user_id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png_sticker: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgs_sticker: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webm_sticker: Option<InputFile>,
    pub emojis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStickerPositionInSet {
    pub sticker: String,
    pub position: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStickerFromSet {
    pub sticker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStickerSetThumb {
    pub name: String,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerInlineQuery {
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerWebAppQuery {
    pub web_app_query_id: String,
    pub result: InlineQueryResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendInvoice {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: String,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceLink {
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: String,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerShippingQuery {
    pub shipping_query_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerPreCheckoutQuery {
    pub pre_checkout_query_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPassportDataErrors {
    pub user_id: i64,
    pub errors: Vec<PassportElementError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendGame {
    pub chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGameHighScores {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

