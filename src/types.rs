use crate::methods;
use crate::BotError;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
    Ok {
        result: T,
    },

    Err {
        error_code: i64,
        description: String,
        parameters: Option<ResponseParameters>,
    },
}

impl<T> Into<Result<T, BotError>> for ApiResult<T> {
    fn into(self) -> Result<T, BotError> {
        match self {
            ApiResult::Ok { result } => Ok(result),

            ApiResult::Err {
                error_code,
                description,
                parameters,
            } => Err(BotError::Api {
                error_code,
                description,
                parameters,
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageOrBool {
    Message(Box<Message>),
    Bool(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParseMode {
    Markdown,
    HTML,
    None,
}

impl Default for ParseMode {
    fn default() -> ParseMode {
        ParseMode::None
    }
}

impl ParseMode {
    pub fn is_none(&self) -> bool {
        *self == ParseMode::None
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputFile {
    File { _path: PathBuf },

    FileId(String),
}

impl InputFile {
    pub fn file(path: PathBuf) -> InputFile {
        InputFile::File {
            _path: path
        }
    }
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

// Actual types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    pub update_id: i64,
    #[serde(flatten)]
    pub update_type: UpdateType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateType {
    Message(Message),

    EditedMessage(Message),

    ChannelPost(Message),

    EditedChannelPost(Message),

    InlineQuery(InlineQuery),

    ChosenInlineResult(ChosenInlineResult),

    CallbackQuery(CallbackQuery),

    ShippingQuery(ShippingQuery),

    PreCheckoutQuery(PreCheckoutQuery),

    Poll(Poll),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: i64,

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: ChatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_members_are_administrators: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub date: i64,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<i64>,
    pub forward_signature: Option<String>,
    pub forward_date: Option<i64>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<i64>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub animation: Option<Animation>,
    pub game: Option<Game>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub voice: Option<Voice>,
    pub video_note: Option<VideoNote>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<Message>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
    pub connected_website: Option<String>,
    pub passport_data: Option<PassportData>,
}

impl Message {
    pub fn reply(&self, text: impl Into<String>) -> methods::SendMessage {
        let mut send_msg = methods::SendMessage::new(self.chat.id, text);
        send_msg.reply_to_message_id = Some(self.message_id);

        send_msg
    }

    pub fn respond(&self, text: impl Into<String>) -> methods::SendMessage {
        methods::SendMessage::new(self.chat.id, text)
    }

    pub fn reply_photo(&self, photo: InputFile) -> methods::SendPhoto {
        let mut send_photo = methods::SendPhoto::new(self.chat.id, photo);
        send_photo.reply_to_message_id = Some(self.message_id);

        send_photo
    }

    pub fn respond_photo(&self, photo: InputFile) -> methods::SendPhoto {
        methods::SendPhoto::new(self.chat.id, photo)
    }

    pub fn get_text(&self) -> Option<&String> {
        if let Some(ref text) = self.text {
            Some(text)
        } else if let Some(ref text) = self.caption {
            Some(text)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub type_name: String, // TODO: Make this an enum
    pub offset: i64,
    pub length: i64,
    pub url: Option<String>,
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    pub file_id: String,
    pub duration: i64,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub thumb: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
    pub file_id: String,
    pub duration: i64,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoNote {
    pub file_id: String,
    pub length: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollOption {
    text: String,
    voter_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    id: String,
    question: String,
    options: Vec<PollOption>,
    is_closed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    pub total_count: i64,
    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub file_id: String,
    pub file_size: Option<i64>,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    ReplyKeyboard(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemoveMarkup),
    ForceReply(ForceReplyMarkup),
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(markup: InlineKeyboardMarkup) -> ReplyMarkup {
        ReplyMarkup::InlineKeyboard(markup)
    }
}

impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(markup: ReplyKeyboardMarkup) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboard(markup)
    }
}

impl From<ReplyKeyboardRemoveMarkup> for ReplyMarkup {
    fn from(markup: ReplyKeyboardRemoveMarkup) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboardRemove(markup)
    }
}

impl From<ForceReplyMarkup> for ReplyMarkup {
    fn from(markup: ForceReplyMarkup) -> ReplyMarkup {
        ReplyMarkup::ForceReply(markup)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardRemoveMarkup {
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
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceReplyMarkup {
    pub force_reply: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub big_file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMember {
    pub user: User,
    pub status: String,
    pub until_date: i64,
    pub can_be_edited: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub is_member: Option<bool>,
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum InputMedia {
    Animation(InputMediaAnimation),
    Document(InputMediaDocument),
    Audio(InputMediaAudio),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

#[derive(Debug, Clone, Serialize)]
pub struct InputMediaAnimation {
    pub media: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputMediaDocument {
    pub media: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputMediaAudio {
    pub media: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMediaPhoto {
    pub media: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputMediaVideo {
    pub media: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

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
pub struct Sticker {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub mask_postition: Option<MaskPosition>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,
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
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum InlineQueryResult {
    #[serde(rename = "article")]
    Article(Box<InlineQueryResultArticle>),
    #[serde(rename = "photo")]
    Photo(Box<InlineQueryResultPhoto>),
    #[serde(rename = "gif")]
    Gif(Box<InlineQueryResultGif>),
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif(Box<InlineQueryResultMpeg4Gif>),
    #[serde(rename = "video")]
    Video(Box<InlineQueryResultVideo>),
    #[serde(rename = "audio")]
    Audio(Box<InlineQueryResultAudio>),
    #[serde(rename = "voice")]
    Voice(Box<InlineQueryResultVoice>),
    #[serde(rename = "document")]
    Document(Box<InlineQueryResultDocument>),
    #[serde(rename = "location")]
    Location(Box<InlineQueryResultLocation>),
    #[serde(rename = "venue")]
    Venue(Box<InlineQueryResultVenue>),
    #[serde(rename = "contact")]
    Contact(Box<InlineQueryResultContact>),
    #[serde(rename = "game")]
    Game(Box<InlineQueryResultGame>),
    #[serde(rename = "photo")]
    CachedPhoto(Box<InlineQueryResultCachedPhoto>),
    #[serde(rename = "gif")]
    CachedGif(Box<InlineQueryResultCachedGif>),
    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif(Box<InlineQueryResultCachedMpeg4Gif>),
    #[serde(rename = "sticker")]
    CachedSticker(Box<InlineQueryResultCachedSticker>),
    #[serde(rename = "document")]
    CachedDocument(Box<InlineQueryResultCachedDocument>),
    #[serde(rename = "video")]
    CachedVideo(Box<InlineQueryResultCachedVideo>),
    #[serde(rename = "voice")]
    CachedVoice(Box<InlineQueryResultCachedVoice>),
    #[serde(rename = "audio")]
    CachedAudio(Box<InlineQueryResultCachedAudio>),
}

impl From<InlineQueryResultArticle> for InlineQueryResult {
    fn from(result: InlineQueryResultArticle) -> InlineQueryResult {
        InlineQueryResult::Article(Box::new(result))
    }
}

impl From<InlineQueryResultPhoto> for InlineQueryResult {
    fn from(result: InlineQueryResultPhoto) -> InlineQueryResult {
        InlineQueryResult::Photo(Box::new(result))
    }
}

impl From<InlineQueryResultGif> for InlineQueryResult {
    fn from(result: InlineQueryResultGif) -> InlineQueryResult {
        InlineQueryResult::Gif(Box::new(result))
    }
}

impl From<InlineQueryResultMpeg4Gif> for InlineQueryResult {
    fn from(result: InlineQueryResultMpeg4Gif) -> InlineQueryResult {
        InlineQueryResult::Mpeg4Gif(Box::new(result))
    }
}

impl From<InlineQueryResultVideo> for InlineQueryResult {
    fn from(result: InlineQueryResultVideo) -> InlineQueryResult {
        InlineQueryResult::Video(Box::new(result))
    }
}

impl From<InlineQueryResultAudio> for InlineQueryResult {
    fn from(result: InlineQueryResultAudio) -> InlineQueryResult {
        InlineQueryResult::Audio(Box::new(result))
    }
}

impl From<InlineQueryResultVoice> for InlineQueryResult {
    fn from(result: InlineQueryResultVoice) -> InlineQueryResult {
        InlineQueryResult::Voice(Box::new(result))
    }
}

impl From<InlineQueryResultDocument> for InlineQueryResult {
    fn from(result: InlineQueryResultDocument) -> InlineQueryResult {
        InlineQueryResult::Document(Box::new(result))
    }
}

impl From<InlineQueryResultLocation> for InlineQueryResult {
    fn from(result: InlineQueryResultLocation) -> InlineQueryResult {
        InlineQueryResult::Location(Box::new(result))
    }
}

impl From<InlineQueryResultVenue> for InlineQueryResult {
    fn from(result: InlineQueryResultVenue) -> InlineQueryResult {
        InlineQueryResult::Venue(Box::new(result))
    }
}

impl From<InlineQueryResultContact> for InlineQueryResult {
    fn from(result: InlineQueryResultContact) -> InlineQueryResult {
        InlineQueryResult::Contact(Box::new(result))
    }
}

impl From<InlineQueryResultGame> for InlineQueryResult {
    fn from(result: InlineQueryResultGame) -> InlineQueryResult {
        InlineQueryResult::Game(Box::new(result))
    }
}

impl From<InlineQueryResultCachedPhoto> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedPhoto) -> InlineQueryResult {
        InlineQueryResult::CachedPhoto(Box::new(result))
    }
}

impl From<InlineQueryResultCachedGif> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedGif) -> InlineQueryResult {
        InlineQueryResult::CachedGif(Box::new(result))
    }
}

impl From<InlineQueryResultCachedMpeg4Gif> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedMpeg4Gif) -> InlineQueryResult {
        InlineQueryResult::CachedMpeg4Gif(Box::new(result))
    }
}

impl From<InlineQueryResultCachedSticker> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedSticker) -> InlineQueryResult {
        InlineQueryResult::CachedSticker(Box::new(result))
    }
}

impl From<InlineQueryResultCachedDocument> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedDocument) -> InlineQueryResult {
        InlineQueryResult::CachedDocument(Box::new(result))
    }
}

impl From<InlineQueryResultCachedVideo> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedVideo) -> InlineQueryResult {
        InlineQueryResult::CachedVideo(Box::new(result))
    }
}

impl From<InlineQueryResultCachedVoice> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedVoice) -> InlineQueryResult {
        InlineQueryResult::CachedVoice(Box::new(result))
    }
}

impl From<InlineQueryResultCachedAudio> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedAudio) -> InlineQueryResult {
        InlineQueryResult::CachedAudio(Box::new(result))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultArticle {
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

impl InlineQueryResultArticle {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<InputMessageContent>,
    ) -> Self {
        InlineQueryResultArticle {
            id: id.into(),
            title: title.into(),
            input_message_content: content.into(),
            reply_markup: None,
            url: None,
            hide_url: None,
            description: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultPhoto {
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

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultGif {
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
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultMpeg4Gif {
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
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultVideo {
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

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

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultAudio {
    pub id: String,
    pub audio_url: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultVoice {
    pub id: String,
    pub voice_url: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultDocument {
    pub id: String,
    pub title: String,
    pub document_url: String,
    pub mime_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

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

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultLocation {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,

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

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultVenue {
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

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultContact {
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

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultGame {
    pub id: String,
    pub game_short_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedPhoto {
    pub id: String,
    pub photo_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedGif {
    pub id: String,
    pub gif_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    pub id: String,
    pub mpeg4_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedSticker {
    pub id: String,
    pub sticker_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedDocument {
    pub id: String,
    pub title: String,
    pub document_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedVideo {
    pub id: String,
    pub video_file_id: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedVoice {
    pub id: String,
    pub voice_file_id: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedAudio {
    pub id: String,
    pub audio_file_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
}

impl From<InputTextMessageContent> for InputMessageContent {
    fn from(text: InputTextMessageContent) -> InputMessageContent {
        InputMessageContent::Text(text)
    }
}

impl From<InputLocationMessageContent> for InputMessageContent {
    fn from(location: InputLocationMessageContent) -> InputMessageContent {
        InputMessageContent::Location(location)
    }
}

impl From<InputVenueMessageContent> for InputMessageContent {
    fn from(venue: InputVenueMessageContent) -> InputMessageContent {
        InputMessageContent::Venue(venue)
    }
}

impl From<InputContactMessageContent> for InputMessageContent {
    fn from(contact: InputContactMessageContent) -> InputMessageContent {
        InputMessageContent::Contact(contact)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InputTextMessageContent {
    pub message_text: String,

    #[serde(skip_serializing_if = "ParseMode::is_none")]
    #[serde(default)]
    pub parse_mode: ParseMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
}

impl InputTextMessageContent {
    pub fn new(message_text: String) -> InputTextMessageContent {
        InputTextMessageContent {
            message_text: message_text,
            parse_mode: ParseMode::None,
            disable_web_page_preview: None,
        }
    }
}

impl From<String> for InputTextMessageContent {
    fn from(s: String) -> InputTextMessageContent {
        InputTextMessageContent {
            message_text: s,
            parse_mode: ParseMode::None,
            disable_web_page_preview: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
}

impl InputLocationMessageContent {
    pub fn new(latitude: f64, longitude: f64) -> InputLocationMessageContent {
        InputLocationMessageContent {
            latitude: latitude,
            longitude: longitude,
            live_period: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InputVenueMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
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
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
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
    pub shipping_option_id: Option<String>,
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
    pub shipping_option_id: Option<String>,
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
    pub file_size: i64,
    pub file_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub element_type: String, // TODO: enum
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "source")]
pub enum PassportElementError {
    #[serde(rename = "data")]
    DataField {
        #[serde(rename = "type")]
        section_type: String, // TODO: Enum
        field_name: String,
        data_hash: String,
        message: String,
    },

    FrontSide {
        #[serde(rename = "type")]
        section_type: String,
        data_hash: String,
        message: String,
    },

    ReverseSide {
        #[serde(rename = "type")]
        section_type: String,
        file_hash: String,
        message: String,
    },

    Selfie {
        #[serde(rename = "type")]
        section_type: String,
        file_hash: String,
        message: String,
    },

    File {
        #[serde(rename = "type")]
        section_type: String,
        file_hash: String,
        message: String,
    },

    Files {
        #[serde(rename = "type")]
        section_type: String,
        file_hashes: Vec<String>,
        message: String,
    },

    TranslationFile {
        #[serde(rename = "type")]
        section_type: String,
        file_hash: String,
        message: String,
    },

    TranslationFiles {
        #[serde(rename = "type")]
        section_type: String,
        file_hashes: Vec<String>,
        message: String,
    },

    Unspecified {
        #[serde(rename = "type")]
        section_type: String,
        element_hash: String,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackGame {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameHighScore {
    pub position: i64,
    pub user: User,
    pub score: i64,
}
