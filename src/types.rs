use crate::helpers::*;
pub use crate::inline_query::*;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ChatMember {
    Owner(ChatMemberOwner),
    Administrator(ChatMemberAdministrator),
    Member(ChatMemberMember),
    Restricted(ChatMemberRestricted),
    Left(ChatMemberLeft),
    Banned(ChatMemberBanned),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BotCommandScope {
    Default(BotCommandScopeDefault),
    AllPrivateChats(BotCommandScopeAllPrivateChats),
    AllGroupChats(BotCommandScopeAllGroupChats),
    AllChatAdministrators(BotCommandScopeAllChatAdministrators),
    Chat(BotCommandScopeChat),
    ChatAdministrators(BotCommandScopeChatAdministrators),
    ChatMember(BotCommandScopeChatMember),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MenuButton {
    Commands(MenuButtonCommands),
    WebApp(MenuButtonWebApp),
    Default(MenuButtonDefault),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum InputMedia {
    Animation(InputMediaAnimation),
    Document(InputMediaDocument),
    Audio(InputMediaAudio),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
    Invoice(InputInvoiceMessageContent),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PassportElementError {
    DataField(PassportElementErrorDataField),
    FrontSide(PassportElementErrorFrontSide),
    ReverseSide(PassportElementErrorReverseSide),
    Selfie(PassportElementErrorSelfie),
    File(PassportElementErrorFile),
    Files(PassportElementErrorFiles),
    TranslationFile(PassportElementErrorTranslationFile),
    TranslationFiles(PassportElementErrorTranslationFiles),
    Unspecified(PassportElementErrorUnspecified),
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: i64,
    pub ip_address: Option<String>,
    pub last_error_date: Option<i64>,
    pub last_error_message: Option<String>,
    pub last_synchronization_error_date: Option<i64>,
    pub max_connections: Option<i64>,
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
    pub added_to_attachment_menu: Option<bool>,
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_inline_queries: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Chat {
    pub id: i64,

    #[serde(rename = "type")]
    pub type_: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo: Option<ChatPhoto>,
    pub bio: Option<String>,
    pub has_private_forwards: Option<bool>,
    pub join_to_send_messages: Option<bool>,
    pub join_by_request: Option<bool>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<Message>>,
    pub permissions: Option<ChatPermissions>,
    pub slow_mode_delay: Option<i64>,
    pub message_auto_delete_time: Option<i64>,
    pub has_protected_content: Option<bool>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
    pub linked_chat_id: Option<i64>,
    pub location: Option<ChatLocation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub sender_chat: Option<Chat>,
    pub date: i64,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<i64>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    pub forward_date: Option<i64>,
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<Message>>,
    pub via_bot: Option<User>,
    pub edit_date: Option<i64>,
    pub has_protected_content: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub contact: Option<Contact>,
    pub dice: Option<Dice>,
    pub game: Option<Game>,
    pub poll: Option<Poll>,
    pub venue: Option<Venue>,
    pub location: Option<Location>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<Message>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
    pub connected_website: Option<String>,
    pub passport_data: Option<PassportData>,
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    pub video_chat_started: Option<VideoChatStarted>,
    pub video_chat_ended: Option<VideoChatEnded>,
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    pub web_app_data: Option<WebAppData>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageId {
    pub message_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageEntity {

    #[serde(rename = "type")]
    pub type_: String,
    pub offset: i64,
    pub length: i64,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i64,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub thumb: Option<PhotoSize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i64,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dice {
    pub emoji: String,
    pub value: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PollOption {
    pub text: String,
    pub voter_count: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PollAnswer {
    pub poll_id: String,
    pub user: User,
    pub option_ids: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: i64,
    pub is_closed: bool,
    pub is_anonymous: bool,

    #[serde(rename = "type")]
    pub type_: String,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<i64>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<i64>,
    pub close_date: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoChatScheduled {
    pub start_date: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoChatStarted {
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoChatEnded {
    pub duration: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoChatParticipantsInvited {
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserProfilePhotos {
    pub total_count: i64,
    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<i64>,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebAppInfo {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeyboardButton {
    pub text: String,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
    pub request_poll: Option<KeyboardButtonPollType>,
    pub web_app: Option<WebAppInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeyboardButtonPollType {

    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    pub web_app: Option<WebAppInfo>,
    pub login_url: Option<LoginUrl>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginUrl {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Box<Message>>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForceReply {
    pub force_reply: bool,
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub name: Option<String>,
    pub expire_date: Option<i64>,
    pub member_limit: Option<i64>,
    pub pending_join_request_count: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatAdministratorRights {
    pub is_anonymous: bool,
    pub can_manage_chat: bool,
    pub can_delete_messages: bool,
    pub can_manage_video_chats: bool,
    pub can_restrict_members: bool,
    pub can_promote_members: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_pin_messages: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMemberOwner {
    pub status: String,
    pub user: User,
    pub is_anonymous: bool,
    pub custom_title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMemberAdministrator {
    pub status: String,
    pub user: User,
    pub can_be_edited: bool,
    pub is_anonymous: bool,
    pub can_manage_chat: bool,
    pub can_delete_messages: bool,
    pub can_manage_video_chats: bool,
    pub can_restrict_members: bool,
    pub can_promote_members: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub custom_title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMemberMember {
    pub status: String,
    pub user: User,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMemberRestricted {
    pub status: String,
    pub user: User,
    pub is_member: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_pin_messages: bool,
    pub can_send_messages: bool,
    pub can_send_media_messages: bool,
    pub can_send_polls: bool,
    pub can_send_other_messages: bool,
    pub can_add_web_page_previews: bool,
    pub until_date: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMemberLeft {
    pub status: String,
    pub user: User,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMemberBanned {
    pub status: String,
    pub user: User,
    pub until_date: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: i64,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatJoinRequest {
    pub chat: Chat,
    pub from: User,
    pub date: i64,
    pub bio: Option<String>,
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatPermissions {
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatLocation {
    pub location: Location,
    pub address: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotCommandScopeDefault {

    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotCommandScopeAllPrivateChats {

    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotCommandScopeAllGroupChats {

    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {

    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotCommandScopeChat {

    #[serde(rename = "type")]
    pub type_: String,
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotCommandScopeChatAdministrators {

    #[serde(rename = "type")]
    pub type_: String,
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotCommandScopeChatMember {

    #[serde(rename = "type")]
    pub type_: String,
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MenuButtonCommands {

    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MenuButtonWebApp {

    #[serde(rename = "type")]
    pub type_: String,
    pub text: String,
    pub web_app: WebAppInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MenuButtonDefault {

    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputMediaPhoto {

    #[serde(rename = "type")]
    pub type_: String,
    pub media: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputMediaVideo {

    #[serde(rename = "type")]
    pub type_: String,
    pub media: String,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration: Option<i64>,
    pub supports_streaming: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputMediaAnimation {

    #[serde(rename = "type")]
    pub type_: String,
    pub media: String,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputMediaAudio {

    #[serde(rename = "type")]
    pub type_: String,
    pub media: String,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<i64>,
    pub performer: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputMediaDocument {

    #[serde(rename = "type")]
    pub type_: String,
    pub media: String,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_content_type_detection: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub is_animated: bool,
    pub is_video: bool,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub premium_animation: Option<File>,
    pub mask_position: Option<MaskPosition>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub is_animated: bool,
    pub is_video: bool,
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,
    pub thumb: Option<PhotoSize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub query: String,
    pub offset: String,
    pub chat_type: Option<String>,
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputTextMessageContent {
    pub message_text: String,
    pub parse_mode: Option<ParseMode>,
    pub entities: Option<Vec<MessageEntity>>,
    pub disable_web_page_preview: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputVenueMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputInvoiceMessageContent {
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: String,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    pub max_tip_amount: Option<i64>,
    pub suggested_tip_amounts: Option<Vec<i64>>,
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
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SentWebAppMessage {
    pub inline_message_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: i64,
    pub file_date: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EncryptedPassportElement {

    #[serde(rename = "type")]
    pub type_: String,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<PassportFile>>,
    pub front_side: Option<PassportFile>,
    pub reverse_side: Option<PassportFile>,
    pub selfie: Option<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PassportElementErrorDataField {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub field_name: String,
    pub data_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PassportElementErrorFrontSide {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PassportElementErrorReverseSide {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PassportElementErrorSelfie {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PassportElementErrorFile {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PassportElementErrorFiles {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hashes: Vec<String>,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PassportElementErrorTranslationFile {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hashes: Vec<String>,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PassportElementErrorUnspecified {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub element_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CallbackGame {
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameHighScore {
    pub position: i64,
    pub user: User,
    pub score: i64,
}

