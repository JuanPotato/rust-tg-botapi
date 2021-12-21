use crate::better::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatMember {
    Owner(ChatMemberOwner),
    Administrator(ChatMemberAdministrator),
    Member(ChatMemberMember),
    Restricted(ChatMemberRestricted),
    Left(ChatMemberLeft),
    Banned(ChatMemberBanned),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMedia {
    Animation(InputMediaAnimation),
    Document(InputMediaDocument),
    Audio(InputMediaAudio),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineQueryResult {
    CachedAudio(InlineQueryResultCachedAudio),
    CachedDocument(InlineQueryResultCachedDocument),
    CachedGif(InlineQueryResultCachedGif),
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    CachedPhoto(InlineQueryResultCachedPhoto),
    CachedSticker(InlineQueryResultCachedSticker),
    CachedVideo(InlineQueryResultCachedVideo),
    CachedVoice(InlineQueryResultCachedVoice),
    Article(InlineQueryResultArticle),
    Audio(InlineQueryResultAudio),
    Contact(InlineQueryResultContact),
    Game(InlineQueryResultGame),
    Document(InlineQueryResultDocument),
    Gif(InlineQueryResultGif),
    Location(InlineQueryResultLocation),
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    Photo(InlineQueryResultPhoto),
    Venue(InlineQueryResultVenue),
    Video(InlineQueryResultVideo),
    Voice(InlineQueryResultVoice),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
    Invoice(InputInvoiceMessageContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,

    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Chat>,
    pub date: i64,
    pub chat: Chat,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_chat: Option<Chat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_sender_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_date: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<SuccessfulPayment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<PassportData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_chat_scheduled: Option<VoiceChatScheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_chat_started: Option<VoiceChatStarted>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_chat_ended: Option<VoiceChatEnded>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_chat_participants_invited: Option<VoiceChatParticipantsInvited>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageId {
    pub message_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEntity {

    #[serde(rename = "type")]
    pub type_: String,
    pub offset: i64,
    pub length: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: i64,
    pub duration: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
    pub emoji: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollOption {
    pub text: String,
    pub voter_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollAnswer {
    pub poll_id: String,
    pub user: User,
    pub option_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venue {
    pub location: Location,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChatScheduled {
    pub start_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChatStarted {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChatEnded {
    pub duration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChatParticipantsInvited {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    pub total_count: i64,
    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonPollType {

    #[serde(skip_serializing_if = "Option::is_none")]

    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUrl {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub chat_instance: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceReply {
    pub force_reply: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberOwner {
    pub status: String,
    pub user: User,
    pub is_anonymous: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberAdministrator {
    pub status: String,
    pub user: User,
    pub can_be_edited: bool,
    pub is_anonymous: bool,
    pub can_manage_chat: bool,
    pub can_delete_messages: bool,
    pub can_manage_voice_chats: bool,
    pub can_restrict_members: bool,
    pub can_promote_members: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberMember {
    pub status: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberLeft {
    pub status: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberBanned {
    pub status: String,
    pub user: User,
    pub until_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: i64,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatJoinRequest {
    pub chat: Chat,
    pub from: User,
    pub date: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPermissions {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatLocation {
    pub location: Location,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotCommandScopeDefault {

    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotCommandScopeAllPrivateChats {

    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotCommandScopeAllGroupChats {

    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {

    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotCommandScopeChat {

    #[serde(rename = "type")]
    pub type_: String,
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotCommandScopeChatAdministrators {

    #[serde(rename = "type")]
    pub type_: String,
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotCommandScopeChatMember {

    #[serde(rename = "type")]
    pub type_: String,
    pub chat_id: ChatId,
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseParameters {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMediaPhoto {

    #[serde(rename = "type")]
    pub type_: String,
    pub media: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMediaVideo {

    #[serde(rename = "type")]
    pub type_: String,
    pub media: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMediaAnimation {

    #[serde(rename = "type")]
    pub type_: String,
    pub media: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMediaAudio {

    #[serde(rename = "type")]
    pub type_: String,
    pub media: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMediaDocument {

    #[serde(rename = "type")]
    pub type_: String,
    pub media: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub is_animated: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub is_animated: bool,
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub query: String,
    pub offset: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub title: String,
    pub input_message_content: InputMessageContent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_url: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultPhoto {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub photo_url: String,
    pub thumb_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultGif {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub gif_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<i64>,
    pub thumb_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultMpeg4Gif {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub mpeg4_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<i64>,
    pub thumb_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultVideo {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultAudio {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub audio_url: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultVoice {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub voice_url: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultDocument {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub document_url: String,
    pub mime_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultLocation {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultVenue {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
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
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultContact {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub phone_number: String,
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultGame {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub game_short_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedPhoto {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub photo_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedGif {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub gif_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedMpeg4Gif {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub mpeg4_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedSticker {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub sticker_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedDocument {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub title: String,
    pub document_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVideo {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub video_file_id: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVoice {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub voice_file_id: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedAudio {

    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub audio_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTextMessageContent {
    pub message_text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputLocationMessageContent {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputVenueMessageContent {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputInvoiceMessageContent {
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
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderInfo {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: i64,
    pub file_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPassportElement {

    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorDataField {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub field_name: String,
    pub data_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorFrontSide {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorReverseSide {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorSelfie {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorFile {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorFiles {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hashes: Vec<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFile {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub file_hashes: Vec<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorUnspecified {
    pub source: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub element_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackGame {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameHighScore {
    pub position: i64,
    pub user: User,
    pub score: i64,
}

