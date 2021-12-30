use crate::objects::*;
use crate::better::*;

#[derive(Debug, Clone, Deserialize)]
pub struct GetUpdates {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub timeout: Option<i64>,
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetWebhook {
    pub url: String,
    pub certificate: Option<InputFile>,
    pub ip_address: Option<String>,
    pub max_connections: Option<i64>,
    pub allowed_updates: Option<Vec<String>>,
    pub drop_pending_updates: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteWebhook {
    pub drop_pending_updates: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetWebhookInfo {
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetMe {
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogOut {
}

#[derive(Debug, Clone, Deserialize)]
pub struct Close {
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendMessage {
    pub chat_id: ChatId,
    pub text: String,
    pub parse_mode: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForwardMessage {
    pub chat_id: ChatId,
    pub from_chat_id: ChatId,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CopyMessage {
    pub chat_id: ChatId,
    pub from_chat_id: ChatId,
    pub message_id: i64,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendPhoto {
    pub chat_id: ChatId,
    pub photo: InputFile,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendAudio {
    pub chat_id: ChatId,
    pub audio: InputFile,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<i64>,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub thumb: Option<InputFile>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendDocument {
    pub chat_id: ChatId,
    pub document: InputFile,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_content_type_detection: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendVideo {
    pub chat_id: ChatId,
    pub video: InputFile,
    pub duration: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub supports_streaming: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendAnimation {
    pub chat_id: ChatId,
    pub animation: InputFile,
    pub duration: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendVoice {
    pub chat_id: ChatId,
    pub voice: InputFile,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendVideoNote {
    pub chat_id: ChatId,
    pub video_note: InputFile,
    pub duration: Option<i64>,
    pub length: Option<i64>,
    pub thumb: Option<InputFile>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendMediaGroup {
    pub chat_id: ChatId,
    pub media: Vec<InputMedia>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendLocation {
    pub chat_id: ChatId,
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditMessageLiveLocation {
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StopMessageLiveLocation {
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendVenue {
    pub chat_id: ChatId,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendContact {
    pub chat_id: ChatId,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendPoll {
    pub chat_id: ChatId,
    pub question: String,
    pub options: Vec<String>,
    pub is_anonymous: Option<bool>,

    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub allows_multiple_answers: Option<bool>,
    pub correct_option_id: Option<i64>,
    pub explanation: Option<String>,
    pub explanation_parse_mode: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<i64>,
    pub close_date: Option<i64>,
    pub is_closed: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendDice {
    pub chat_id: ChatId,
    pub emoji: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendChatAction {
    pub chat_id: ChatId,
    pub action: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserProfilePhotos {
    pub user_id: i64,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetFile {
    pub file_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BanChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub until_date: Option<i64>,
    pub revoke_messages: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnbanChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub only_if_banned: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RestrictChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub permissions: ChatPermissions,
    pub until_date: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PromoteChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub is_anonymous: Option<bool>,
    pub can_manage_chat: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_manage_voice_chats: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetChatAdministratorCustomTitle {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub custom_title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BanChatSenderChat {
    pub chat_id: ChatId,
    pub sender_chat_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnbanChatSenderChat {
    pub chat_id: ChatId,
    pub sender_chat_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetChatPermissions {
    pub chat_id: ChatId,
    pub permissions: ChatPermissions,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExportChatInviteLink {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateChatInviteLink {
    pub chat_id: ChatId,
    pub name: Option<String>,
    pub expire_date: Option<i64>,
    pub member_limit: Option<i64>,
    pub creates_join_request: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditChatInviteLink {
    pub chat_id: ChatId,
    pub invite_link: String,
    pub name: Option<String>,
    pub expire_date: Option<i64>,
    pub member_limit: Option<i64>,
    pub creates_join_request: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RevokeChatInviteLink {
    pub chat_id: ChatId,
    pub invite_link: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApproveChatJoinRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclineChatJoinRequest {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetChatPhoto {
    pub chat_id: ChatId,
    pub photo: InputFile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteChatPhoto {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetChatTitle {
    pub chat_id: ChatId,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetChatDescription {
    pub chat_id: ChatId,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinChatMessage {
    pub chat_id: ChatId,
    pub message_id: i64,
    pub disable_notification: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnpinChatMessage {
    pub chat_id: ChatId,
    pub message_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnpinAllChatMessages {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LeaveChat {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetChat {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetChatAdministrators {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetChatMemberCount {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetChatMember {
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetChatStickerSet {
    pub chat_id: ChatId,
    pub sticker_set_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteChatStickerSet {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnswerCallbackQuery {
    pub callback_query_id: String,
    pub text: Option<String>,
    pub show_alert: Option<bool>,
    pub url: Option<String>,
    pub cache_time: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetMyCommands {
    pub commands: Vec<BotCommand>,
    pub scope: Option<BotCommandScope>,
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMyCommands {
    pub scope: Option<BotCommandScope>,
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetMyCommands {
    pub scope: Option<BotCommandScope>,
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditMessageText {
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub text: String,
    pub parse_mode: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub disable_web_page_preview: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditMessageCaption {
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditMessageMedia {
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub media: InputMedia,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditMessageReplyMarkup {
    pub chat_id: Option<ChatId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StopPoll {
    pub chat_id: ChatId,
    pub message_id: i64,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMessage {
    pub chat_id: ChatId,
    pub message_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendSticker {
    pub chat_id: ChatId,
    pub sticker: InputFile,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetStickerSet {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UploadStickerFile {
    pub user_id: i64,
    pub png_sticker: InputFile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNewStickerSet {
    pub user_id: i64,
    pub name: String,
    pub title: String,
    pub png_sticker: Option<InputFile>,
    pub tgs_sticker: Option<InputFile>,
    pub emojis: String,
    pub contains_masks: Option<bool>,
    pub mask_position: Option<MaskPosition>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddStickerToSet {
    pub user_id: i64,
    pub name: String,
    pub png_sticker: Option<InputFile>,
    pub tgs_sticker: Option<InputFile>,
    pub emojis: String,
    pub mask_position: Option<MaskPosition>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetStickerPositionInSet {
    pub sticker: String,
    pub position: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteStickerFromSet {
    pub sticker: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetStickerSetThumb {
    pub name: String,
    pub user_id: i64,
    pub thumb: Option<InputFile>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnswerInlineQuery {
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,
    pub cache_time: Option<i64>,
    pub is_personal: Option<bool>,
    pub next_offset: Option<String>,
    pub switch_pm_text: Option<String>,
    pub switch_pm_parameter: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendInvoice {
    pub chat_id: ChatId,
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: String,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    pub max_tip_amount: Option<i64>,
    pub suggested_tip_amounts: Option<Vec<i64>>,
    pub start_parameter: Option<String>,
    pub provider_data: Option<String>,
    pub photo_url: Option<String>,
    pub photo_size: Option<i64>,
    pub photo_width: Option<i64>,
    pub photo_height: Option<i64>,
    pub need_name: Option<bool>,
    pub need_phone_number: Option<bool>,
    pub need_email: Option<bool>,
    pub need_shipping_address: Option<bool>,
    pub send_phone_number_to_provider: Option<bool>,
    pub send_email_to_provider: Option<bool>,
    pub is_flexible: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnswerShippingQuery {
    pub shipping_query_id: String,
    pub ok: bool,
    pub shipping_options: Option<Vec<ShippingOption>>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnswerPreCheckoutQuery {
    pub pre_checkout_query_id: String,
    pub ok: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetPassportDataErrors {
    pub user_id: i64,
    pub errors: Vec<PassportElementError>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendGame {
    pub chat_id: i64,
    pub game_short_name: String,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetGameScore {
    pub user_id: i64,
    pub score: i64,
    pub force: Option<bool>,
    pub disable_edit_message: Option<bool>,
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetGameHighScores {
    pub user_id: i64,
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
}

