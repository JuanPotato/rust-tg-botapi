use std::path::PathBuf;

use crate::types::InputMediaPhoto;
use crate::{types, TgMethod};
use types::{ChatId, InputFile, ParseMode};

#[derive(Debug, Serialize)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>, // TODO: enum
}

impl TgMethod for GetUpdates {
    type ResponseType = Vec<types::Update>;
    const PATH: &'static str = "getUpdates";
}

#[derive(Debug, Serialize)]
pub struct SetWebhook {
    pub url: String,
    pub certificate: InputFile,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct DeleteWebhook;

#[derive(Debug, Serialize)]
pub struct GetWebhookInfo;

#[derive(Debug, Serialize)]
pub struct GetMe;

impl TgMethod for GetMe {
    type ResponseType = types::User;
    const PATH: &'static str = "getMe";
}

#[derive(Debug, Serialize)]
pub struct SendMessage {
    pub chat_id: ChatId,
    pub text: String,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

impl TgMethod for SendMessage {
    type ResponseType = types::Message;
    const PATH: &'static str = "sendMessage";
}

impl SendMessage {
    pub fn new(chat_id: impl Into<ChatId>, text: impl Into<String>) -> SendMessage {
        SendMessage {
            chat_id: chat_id.into(),
            text: text.into(),
            parse_mode: ParseMode::None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ForwardMessage {
    pub chat_id: ChatId,
    pub from_chat_id: ChatId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    pub message_id: i64,
}

#[derive(Debug, Serialize)]
pub struct SendPhoto {
    pub chat_id: ChatId,
    pub photo: InputFile,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendAudio {
    pub chat_id: ChatId,
    pub audio: InputFile,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

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
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendDocument {
    pub chat_id: ChatId,
    pub document: InputFile,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendVideo {
    pub chat_id: ChatId,
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

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendAnimation {
    pub chat_id: ChatId,
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

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendVoice {
    pub chat_id: ChatId,
    pub voice: InputFile,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendVideoNote {
    pub chat_id: ChatId,
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
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendMediaGroup {
    pub chat_id: ChatId,
    pub media: Vec<InputMediaPhoto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct EditMessageLiveLocation {
    pub latitude: f64,
    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct StopMessageLiveLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendVenue {
    pub chat_id: ChatId,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendContact {
    pub chat_id: ChatId,
    pub phone_number: String,
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendPoll {
    pub chat_id: ChatId,
    pub question: String,
    pub options: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendChatAction {
    pub chat_id: ChatId,
    pub action: String, // TODO: enum
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
pub struct GetFile {
    pub file_id: String,
}

#[derive(Debug, Serialize)]
pub struct KickChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct UnbanChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct RestrictChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PromoteChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ExportChatInviteLink {
    pub chat_id: ChatId,
}

#[derive(Debug, Serialize)]
pub struct SetChatPhoto {
    pub chat_id: ChatId,
    pub photo: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct DeleteChatPhoto {
    pub chat_id: ChatId,
}

#[derive(Debug, Serialize)]
pub struct SetChatTitle {
    pub chat_id: ChatId,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct SetChatDescription {
    pub chat_id: ChatId,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct PinChatMessage {
    pub chat_id: ChatId,
    pub message_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UnpinChatMessage {
    pub chat_id: ChatId,
}

#[derive(Debug, Serialize)]
pub struct LeaveChat {
    pub chat_id: ChatId,
}

#[derive(Debug, Serialize)]
pub struct GetChat {
    pub chat_id: ChatId,
}

#[derive(Debug, Serialize)]
pub struct GetChatAdministrators {
    pub chat_id: ChatId,
}

#[derive(Debug, Serialize)]
pub struct GetChatMembersCount {
    pub chat_id: ChatId,
}

#[derive(Debug, Serialize)]
pub struct GetChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct SetChatStickerSet {
    pub chat_id: ChatId,
    pub sticker_set_name: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteChatStickerSet {
    pub chat_id: ChatId,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct EditMessageText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub text: String,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::InlineKeyboardMarkup>,
}

#[derive(Debug, Serialize)]
pub struct EditMessageCaption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::InlineKeyboardMarkup>,
}

#[derive(Debug, Serialize)]
pub struct EditMessageMedia {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    pub media: types::InputMedia,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::InlineKeyboardMarkup>,
}

#[derive(Debug, Serialize)]
pub struct EditMessageReplyMarkup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::InlineKeyboardMarkup>,
}

#[derive(Debug, Serialize)]
pub struct StopPoll {
    pub chat_id: ChatId,
    pub message_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::InlineKeyboardMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendSticker {
    pub chat_id: ChatId,
    pub sticker: InputFile,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct GetStickerSet {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct UploadStickerFile {
    pub user_id: i64,
    pub png_sticker: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct CreateNewStickerSet {
    pub user_id: i64,
    pub name: String,
    pub title: String,
    pub png_sticker: InputFile,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    pub emojis: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_masks: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<types::MaskPosition>,
}

#[derive(Debug, Serialize)]
pub struct AddStickerToSet {
    pub user_id: i64,
    pub name: String,
    pub png_sticker: InputFile,
    pub emojis: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<types::MaskPosition>,
}

#[derive(Debug, Serialize)]
pub struct SetStickerPositionInSet {
    pub sticker: String,
    pub position: i64,
}

#[derive(Debug, Serialize)]
pub struct DeleteStickerFromSet {
    pub sticker: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteMessage {
    pub chat_id: ChatId,
    pub message_id: i64,
}

#[derive(Debug, Serialize)]
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
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }

    pub fn add(&mut self, result: impl Into<types::InlineQueryResult>) {
        self.results.push(result.into());
    }
}

impl TgMethod for AnswerInlineQuery {
    type ResponseType = bool;
    const PATH: &'static str = "answerInlineQuery";
}

#[derive(Debug, Serialize)]
pub struct SendInvoice {
    pub chat_id: i64,
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: String,
    pub start_paramater: String,
    pub currency: String,
    pub prices: Vec<types::LabeledPrice>,

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
    pub is_flexible: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::InlineKeyboardMarkup>,
}

#[derive(Debug, Serialize)]
pub struct AnswerShippingQuery {
    pub shipping_query_id: String,
    pub ok: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<types::ShippingOption>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AnswerPreCheckoutQuery {
    pub pre_checkout_query_id: String,
    pub ok: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SetPassportDataErrors {
    pub user_id: i64,
    pub errors: Vec<types::PassportElementError>,
}


#[derive(Debug, Serialize)]
pub struct SendGame {
    pub chat_id: i64,
    pub game_short_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::InlineKeyboardMarkup>,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct GetGameHighScores {
    pub user_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

