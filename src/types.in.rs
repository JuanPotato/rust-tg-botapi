#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: i64,
    pub last_error_date: Option<i64>,
    pub last_error_message: Option<String>,
    pub max_connections: Option<i64>,
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEntity {
    #[serde(rename="type")]
    pub type_name: String,
    pub offset: i64,
    pub length: i64,
    pub url: Option<String>,
    pub user: Option<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audio {
    pub file_id: String,
    pub duration: i64,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sticker {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Voice {
    pub file_id: String,
    pub duration: i64,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    pub total_count: i64,
    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub selective: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyboardButton {
    pub text: String,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,
    pub selective: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub callback_game: Option<CallbackGame>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForceReply {
    pub force_reply: bool,
    pub selective: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMember {
    pub user: User,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub title: String,
    pub input_message_content: InputMessageContent,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub url: Option<String>,
    pub hide_url: Option<bool>,
    pub description: Option<String>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultPhoto {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub photo_url: String,
    pub thumb_url: String,
    pub photo_width: Option<i64>,
    pub photo_height: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultGif {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub gif_url: String,
    pub gif_width: Option<i64>,
    pub gif_height: Option<i64>,
    pub thumb_url: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultMpeg4Gif {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub mpeg4_url: String,
    pub mpeg4_width: Option<i64>,
    pub mpeg4_height: Option<i64>,
    pub thumb_url: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultVideo {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub video_width: Option<i64>,
    pub video_height: Option<i64>,
    pub video_duration: Option<i64>,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultAudio {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub audio_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub performer: Option<String>,
    pub audio_duration: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultVoice {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub voice_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub voice_duration: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultDocument {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub title: String,
    pub caption: Option<String>,
    pub document_url: String,
    pub mime_type: String,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultLocation {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultVenue {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultContact {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub game_short_name: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedPhoto {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub photo_file_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedGif {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub gif_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub mpeg4_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedSticker {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub sticker_file_id: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedDocument {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub title: String,
    pub document_file_id: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVideo {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub video_file_id: String,
    pub title: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVoice {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub voice_file_id: String,
    pub title: String,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedAudio {
    #[serde(rename="type")]
    pub type_name: String,
    pub id: String,
    pub audio_file_id: String,
    pub caption: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InputMessageContent {
    Text {
        message_text: String,
        parse_mode: Option<String>,
        disable_web_page_preview: Option<bool>,
    },

    Location {
        latitude: f64,
        longitude: f64,
    },

    Venue {
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
        foursquare_id: Option<String>,
    },

    Contact {
        phone_number: String,
        first_name: String,
        last_name: Option<String>,
    },
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Animation {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallbackGame { }

#[derive(Debug, Serialize, Deserialize)]
pub struct GameHighScore {
    pub position: i64,
    pub user: User,
    pub score: i64,
}
