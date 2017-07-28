use serde_json;

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct ApiResult {
    pub ok: bool,
    pub description: Option<String>,
    pub error_code: Option<i64>,
    pub result: Option<serde_json::value::Value>,
    pub parameters: Option<ResponseParameters>,
}

#[derive(Debug, Clone, From, Serialize, Deserialize)]
pub enum MessageOrBool {
    M(Box<Message>),
    B(bool),
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

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    pub fn new(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup { inline_keyboard: inline_keyboard }
    }
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

impl ReplyKeyboardMarkup {
    pub fn new(keyboard: Vec<Vec<KeyboardButton>>) -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup {
            keyboard: keyboard,
            resize_keyboard: None,
            one_time_keyboard: None,
            selective: None,
        }
    }
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct ReplyKeyboardRemoveMarkup {
    pub remove_keyboard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

impl ReplyKeyboardRemoveMarkup {
    pub fn new(remove_keyboard: bool) -> ReplyKeyboardRemoveMarkup {
        ReplyKeyboardRemoveMarkup {
            remove_keyboard: remove_keyboard,
            selective: None,
        }
    }
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct ForceReplyMarkup {
    pub force_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

impl ForceReplyMarkup {
    pub fn new(force_reply: bool) -> ForceReplyMarkup {
        ForceReplyMarkup {
            force_reply: force_reply,
            selective: None,
        }
    }
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

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultArticle {
    pub id: String,
    pub title: String,
    pub input_message_content: InputMessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
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

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultPhoto {
    pub type_name: String,
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
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultGif {
    pub type_name: String,
    pub id: String,
    pub gif_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<i64>,
    pub thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultMpeg4Gif {
    pub type_name: String,
    pub id: String,
    pub mpeg4_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i64>,
    pub thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultVideo {
    pub type_name: String,
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultAudio {
    pub type_name: String,
    pub id: String,
    pub audio_url: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultVoice {
    pub type_name: String,
    pub id: String,
    pub voice_url: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultDocument {
    pub type_name: String,
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    pub document_url: String,
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultLocation {
    pub type_name: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultVenue {
    pub type_name: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultContact {
    pub type_name: String,
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultGame {
    pub type_name: String,
    pub id: String,
    pub game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultCachedPhoto {
    pub type_name: String,
    pub id: String,
    pub photo_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultCachedGif {
    pub type_name: String,
    pub id: String,
    pub gif_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    pub type_name: String,
    pub id: String,
    pub mpeg4_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultCachedSticker {
    pub type_name: String,
    pub id: String,
    pub sticker_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultCachedDocument {
    pub type_name: String,
    pub id: String,
    pub title: String,
    pub document_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultCachedVideo {
    pub type_name: String,
    pub id: String,
    pub video_file_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultCachedVoice {
    pub type_name: String,
    pub id: String,
    pub voice_file_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InlineQueryResultCachedAudio {
    pub type_name: String,
    pub id: String,
    pub audio_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
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

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InputTextMessageContent {
    pub message_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
}

impl InputTextMessageContent {
    pub fn new(message_text: String) -> InputTextMessageContent {
        InputTextMessageContent {
            message_text: message_text,
            parse_mode: None,
            disable_web_page_preview: None,
        }
    }
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,
}

impl InputLocationMessageContent {
    pub fn new(latitude: f64, longitude: f64) -> InputLocationMessageContent {
        InputLocationMessageContent {
            latitude: latitude,
            longitude: longitude,
        }
    }
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InputVenueMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    pub inline_query: Option<InlineQuery>,
    pub chosen_inline_result: Option<ChosenInlineResult>,
    pub callback_query: Option<CallbackQuery>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: i64,
    pub last_error_date: Option<i64>,
    pub last_error_message: Option<String>,
    pub max_connections: Option<i64>,
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
    #[serde(rename="type")]
    pub type_name: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub all_members_are_administrators: Option<bool>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub date: i64,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<i64>,
    pub forward_date: Option<i64>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<i64>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub game: Option<Game>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
    pub new_chat_member: Option<User>,
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
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct MessageEntity {
    #[serde(rename="type")]
    pub type_name: String,
    pub offset: i64,
    pub length: i64,
    pub url: Option<String>,
    pub user: Option<User>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Audio {
    pub file_id: String,
    pub duration: i64,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Document {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Sticker {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Video {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Voice {
    pub file_id: String,
    pub duration: i64,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    pub total_count: i64,
    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct File {
    pub file_id: String,
    pub file_size: Option<i64>,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct KeyboardButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub request_contact: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub request_location: Option<bool>,
}

impl KeyboardButton {
    pub fn new(text: String) -> KeyboardButton {
        KeyboardButton {
            text: text,
            request_contact: None,
            request_location: None,
        }
    }
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct ChatMember {
    pub user: User,
    pub status: String,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Animation {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct CallbackGame {}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct GameHighScore {
    pub position: i64,
    pub user: User,
    pub score: i64,
}
