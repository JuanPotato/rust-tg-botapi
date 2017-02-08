use std::borrow::Cow;

use ::serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResult {
    pub ok: bool,
    pub description: Option<String>,
    pub error_code: Option<i64>,
    pub result: Option<serde_json::value::Value>,
    pub parameters: Option<ResponseParameters>,
}

#[derive(Debug)]
pub enum MessageOrBool<'a> {
    M(Message<'a>),
    B(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup<'a> {
    InlineKeyboard(InlineKeyboardMarkup<'a>),
    ReplyKeyboard(ReplyKeyboardMarkup<'a>),
    ReplyKeyboardRemove(ReplyKeyboardRemoveMarkup),
    ForceReply(ForceReplyMarkup),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup<'a> {
    pub inline_keyboard: Cow<'a, [Cow<'a, [InlineKeyboardButton<'a>]>]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup<'a> {
    pub keyboard: Cow<'a, [Cow<'a, [KeyboardButton<'a>]>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardRemoveMarkup {
    pub remove_keyboard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceReplyMarkup {
    pub force_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InlineQueryResult<'a> {
    Article(InlineQueryResultArticle<'a>),
    Photo(InlineQueryResultPhoto<'a>),
    Gif(InlineQueryResultGif<'a>),
    Mpeg4Gif(InlineQueryResultMpeg4Gif<'a>),
    Video(InlineQueryResultVideo<'a>),
    Audio(InlineQueryResultAudio<'a>),
    Voice(InlineQueryResultVoice<'a>),
    Document(InlineQueryResultDocument<'a>),
    Location(InlineQueryResultLocation<'a>),
    Venue(InlineQueryResultVenue<'a>),
    Contact(InlineQueryResultContact<'a>),
    Game(InlineQueryResultGame<'a>),
    CachedPhoto(InlineQueryResultCachedPhoto<'a>),
    CachedGif(InlineQueryResultCachedGif<'a>),
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif<'a>),
    CachedSticker(InlineQueryResultCachedSticker<'a>),
    CachedDocument(InlineQueryResultCachedDocument<'a>),
    CachedVideo(InlineQueryResultCachedVideo<'a>),
    CachedVoice(InlineQueryResultCachedVoice<'a>),
    CachedAudio(InlineQueryResultCachedAudio<'a>),
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultArticle<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub title: &'a str,
    pub input_message_content: &'a InputMessageContent<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultPhoto<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub photo_url: &'a str,
    pub thumb_url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultGif<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub gif_url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<i64>,
    pub thumb_url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultMpeg4Gif<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub mpeg4_url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i64>,
    pub thumb_url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultVideo<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub video_url: &'a str,
    pub mime_type: &'a str,
    pub thumb_url: &'a str,
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultAudio<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub audio_url: &'a str,
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultVoice<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub voice_url: &'a str,
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultDocument<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    pub document_url: &'a str,
    pub mime_type: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultLocation<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub latitude: f64,
    pub longitude: f64,
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultVenue<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub latitude: f64,
    pub longitude: f64,
    pub title: &'a str,
    pub address: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultContact<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub phone_number: &'a str,
    pub first_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultGame<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub game_short_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedPhoto<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub photo_file_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedGif<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub gif_file_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedMpeg4Gif<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub mpeg4_file_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedSticker<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub sticker_file_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedDocument<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub title: &'a str,
    pub document_file_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedVideo<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub video_file_id: &'a str,
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedVoice<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub voice_file_id: &'a str,
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultCachedAudio<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub audio_file_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a ReplyMarkup<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent<'a> {
    Text(InputTextMessageContent<'a>),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent<'a>),
    Contact(InputContactMessageContent<'a>),
}

#[derive(Debug, Clone, Serialize)]
pub struct InputTextMessageContent<'a> {
    pub message_text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputVenueMessageContent<'a> {
    pub latitude: f64,
    pub longitude: f64,
    pub title: &'a str,
    pub address: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputContactMessageContent<'a> {
    pub phone_number: &'a str,
    pub first_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update<'a> {
    pub update_id: i64,
    pub message: Option<Message<'a>>,
    pub edited_message: Option<Message<'a>>,
    pub channel_post: Option<Message<'a>>,
    pub edited_channel_post: Option<Message<'a>>,
    pub inline_query: Option<InlineQuery<'a>>,
    pub chosen_inline_result: Option<ChosenInlineResult<'a>>,
    pub callback_query: Option<CallbackQuery<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookInfo<'a> {
    pub url: Cow<'a, str>,
    pub has_custom_certificate: bool,
    pub pending_update_count: i64,
    pub last_error_date: Option<i64>,
    pub last_error_message: Option<Cow<'a, str>>,
    pub max_connections: Option<i64>,
    pub allowed_updates: Option<Cow<'a, [Cow<'a, str>]>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User<'a> {
    pub id: i64,
    pub first_name: Cow<'a, str>,
    pub last_name: Option<Cow<'a, str>>,
    pub username: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat<'a> {
    pub id: i64,
    #[serde(rename="type")]
    pub type_name: Cow<'a, str>,
    pub title: Option<Cow<'a, str>>,
    pub username: Option<Cow<'a, str>>,
    pub first_name: Option<Cow<'a, str>>,
    pub last_name: Option<Cow<'a, str>>,
    pub all_members_are_administrators: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<'a> {
    pub message_id: i64,
    pub from: Option<User<'a>>,
    pub date: i64,
    pub chat: Chat<'a>,
    pub forward_from: Option<User<'a>>,
    pub forward_from_chat: Option<Chat<'a>>,
    pub forward_from_message_id: Option<i64>,
    pub forward_date: Option<i64>,
    pub reply_to_message: Option<Box<Message<'a>>>,
    pub edit_date: Option<i64>,
    pub text: Option<String>,
    pub entities: Option<Cow<'a, [MessageEntity<'a>]>>,
    pub audio: Option<Audio<'a>>,
    pub document: Option<Document<'a>>,
    pub game: Option<Game<'a>>,
    pub photo: Option<Cow<'a, [PhotoSize<'a>]>>,
    pub sticker: Option<Sticker<'a>>,
    pub video: Option<Video<'a>>,
    pub voice: Option<Voice<'a>>,
    pub caption: Option<String>,
    pub contact: Option<Contact<'a>>,
    pub location: Option<Location>,
    pub venue: Option<Venue<'a>>,
    pub new_chat_member: Option<User<'a>>,
    pub left_chat_member: Option<User<'a>>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Cow<'a, [PhotoSize<'a>]>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<Message<'a>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEntity<'a> {
    #[serde(rename="type")]
    pub type_name: Cow<'a, str>,
    pub offset: i64,
    pub length: i64,
    pub url: Option<Cow<'a, str>>,
    pub user: Option<User<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoSize<'a> {
    pub file_id: Cow<'a, str>,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio<'a> {
    pub file_id: Cow<'a, str>,
    pub duration: i64,
    pub performer: Option<Cow<'a, str>>,
    pub title: Option<Cow<'a, str>>,
    pub mime_type: Option<Cow<'a, str>>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document<'a> {
    pub file_id: Cow<'a, str>,
    pub thumb: Option<PhotoSize<'a>>,
    pub file_name: Option<Cow<'a, str>>,
    pub mime_type: Option<Cow<'a, str>>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sticker<'a> {
    pub file_id: Cow<'a, str>,
    pub width: i64,
    pub height: i64,
    pub thumb: Option<PhotoSize<'a>>,
    pub emoji: Option<Cow<'a, str>>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video<'a> {
    pub file_id: Cow<'a, str>,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize<'a>>,
    pub mime_type: Option<Cow<'a, str>>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice<'a> {
    pub file_id: Cow<'a, str>,
    pub duration: i64,
    pub mime_type: Option<Cow<'a, str>>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact<'a> {
    pub phone_number: Cow<'a, str>,
    pub first_name: Cow<'a, str>,
    pub last_name: Option<Cow<'a, str>>,
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venue<'a> {
    pub location: Location,
    pub title: Cow<'a, str>,
    pub address: Cow<'a, str>,
    pub foursquare_id: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfilePhotos<'a> {
    pub total_count: i64,
    pub photos: Cow<'a, [Cow<'a, [PhotoSize<'a>]>]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File<'a> {
    pub file_id: Cow<'a, str>,
    pub file_size: Option<i64>,
    pub file_path: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButton<'a> {
    pub text: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardButton<'a> {
    pub text: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<Cow<'a, CallbackGame>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackQuery<'a> {
    pub id: Cow<'a, str>,
    pub from: User<'a>,
    pub message: Option<Message<'a>>,
    pub inline_message_id: Option<Cow<'a, str>>,
    pub chat_instance: Cow<'a, str>,
    pub data: Option<Cow<'a, str>>,
    pub game_short_name: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMember<'a> {
    pub user: User<'a>,
    pub status: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQuery<'a> {
    pub id: Cow<'a, str>,
    pub from: User<'a>,
    pub location: Option<Location>,
    pub query: Cow<'a, str>,
    pub offset: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChosenInlineResult<'a> {
    pub result_id: Cow<'a, str>,
    pub from: User<'a>,
    pub location: Option<Location>,
    pub inline_message_id: Option<Cow<'a, str>>,
    pub query: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game<'a> {
    pub title: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub photo: Cow<'a, [PhotoSize<'a>]>,
    pub text: Option<Cow<'a, str>>,
    pub text_entities: Option<Cow<'a, [MessageEntity<'a>]>>,
    pub animation: Option<Animation<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation<'a> {
    pub file_id: Cow<'a, str>,
    pub thumb: Option<PhotoSize<'a>>,
    pub file_name: Option<Cow<'a, str>>,
    pub mime_type: Option<Cow<'a, str>>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackGame { }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameHighScore<'a> {
    pub position: i64,
    pub user: User<'a>,
    pub score: i64,
}
