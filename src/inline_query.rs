use crate::types::*;
use crate::form_ser::*;
use crate::helpers::*;

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultArticle {
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

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultPhoto {
    pub id: String,
    pub photo_url: String,
    pub thumb_url: String,
    pub photo_width: Option<i64>,
    pub photo_height: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultGif {
    pub id: String,
    pub gif_url: String,
    pub gif_width: Option<i64>,
    pub gif_height: Option<i64>,
    pub gif_duration: Option<i64>,
    pub thumb_url: String,
    pub thumb_mime_type: Option<String>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultMpeg4Gif {
    pub id: String,
    pub mpeg4_url: String,
    pub mpeg4_width: Option<i64>,
    pub mpeg4_height: Option<i64>,
    pub mpeg4_duration: Option<i64>,
    pub thumb_url: String,
    pub thumb_mime_type: Option<String>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultVideo {
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub video_width: Option<i64>,
    pub video_height: Option<i64>,
    pub video_duration: Option<i64>,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultAudio {
    pub id: String,
    pub audio_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub performer: Option<String>,
    pub audio_duration: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultVoice {
    pub id: String,
    pub voice_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub voice_duration: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultDocument {
    pub id: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub document_url: String,
    pub mime_type: String,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultLocation {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultVenue {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultContact {
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultGame {
    pub id: String,
    pub game_short_name: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultCachedPhoto {
    pub id: String,
    pub photo_file_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultCachedGif {
    pub id: String,
    pub gif_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    pub id: String,
    pub mpeg4_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultCachedSticker {
    pub id: String,
    pub sticker_file_id: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultCachedDocument {
    pub id: String,
    pub title: String,
    pub document_file_id: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultCachedVideo {
    pub id: String,
    pub video_file_id: String,
    pub title: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultCachedVoice {
    pub id: String,
    pub voice_file_id: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InlineQueryResultCachedAudio {
    pub id: String,
    pub audio_file_id: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}

impl FormSer for InlineQueryResult {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        match self {
            InlineQueryResult::CachedAudio(e) => {
                form = form.text(format!("{}[type]", &key), "audio");
                e.serialize(key, form)
            },
            InlineQueryResult::CachedDocument(e) => {
                form = form.text(format!("{}[type]", &key), "document");
                e.serialize(key, form)
            },
            InlineQueryResult::CachedGif(e) => {
                form = form.text(format!("{}[type]", &key), "gif");
                e.serialize(key, form)
            },
            InlineQueryResult::CachedMpeg4Gif(e) => {
                form = form.text(format!("{}[type]", &key), "mpeg4_gif");
                e.serialize(key, form)
            },
            InlineQueryResult::CachedPhoto(e) => {
                form = form.text(format!("{}[type]", &key), "photo");
                e.serialize(key, form)
            },
            InlineQueryResult::CachedSticker(e) => {
                form = form.text(format!("{}[type]", &key), "sticker");
                e.serialize(key, form)
            },
            InlineQueryResult::CachedVideo(e) => {
                form = form.text(format!("{}[type]", &key), "video");
                e.serialize(key, form)
            },
            InlineQueryResult::CachedVoice(e) => {
                form = form.text(format!("{}[type]", &key), "voice");
                e.serialize(key, form)
            },
            InlineQueryResult::Article(e) => {
                form = form.text(format!("{}[type]", &key), "article");
                e.serialize(key, form)
            },
            InlineQueryResult::Audio(e) => {
                form = form.text(format!("{}[type]", &key), "audio");
                e.serialize(key, form)
            },
            InlineQueryResult::Contact(e) => {
                form = form.text(format!("{}[type]", &key), "contact");
                e.serialize(key, form)
            },
            InlineQueryResult::Game(e) => {
                form = form.text(format!("{}[type]", &key), "game");
                e.serialize(key, form)
            },
            InlineQueryResult::Document(e) => {
                form = form.text(format!("{}[type]", &key), "document");
                e.serialize(key, form)
            },
            InlineQueryResult::Gif(e) => {
                form = form.text(format!("{}[type]", &key), "gif");
                e.serialize(key, form)
            },
            InlineQueryResult::Location(e) => {
                form = form.text(format!("{}[type]", &key), "location");
                e.serialize(key, form)
            },
            InlineQueryResult::Mpeg4Gif(e) => {
                form = form.text(format!("{}[type]", &key), "mpeg4_gif");
                e.serialize(key, form)
            },
            InlineQueryResult::Photo(e) => {
                form = form.text(format!("{}[type]", &key), "photo");
                e.serialize(key, form)
            },
            InlineQueryResult::Venue(e) => {
                form = form.text(format!("{}[type]", &key), "venue");
                e.serialize(key, form)
            },
            InlineQueryResult::Video(e) => {
                form = form.text(format!("{}[type]", &key), "video");
                e.serialize(key, form)
            },
            InlineQueryResult::Voice(e) => {
                form = form.text(format!("{}[type]", &key), "voice");
                e.serialize(key, form)
            },
        }
    }
}

impl From<InlineQueryResultCachedAudio> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedAudio) -> Self {
        Self::CachedAudio(o)
    }
}

impl From<InlineQueryResultCachedDocument> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedDocument) -> Self {
        Self::CachedDocument(o)
    }
}

impl From<InlineQueryResultCachedGif> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedGif) -> Self {
        Self::CachedGif(o)
    }
}

impl From<InlineQueryResultCachedMpeg4Gif> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedMpeg4Gif) -> Self {
        Self::CachedMpeg4Gif(o)
    }
}

impl From<InlineQueryResultCachedPhoto> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedPhoto) -> Self {
        Self::CachedPhoto(o)
    }
}

impl From<InlineQueryResultCachedSticker> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedSticker) -> Self {
        Self::CachedSticker(o)
    }
}

impl From<InlineQueryResultCachedVideo> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedVideo) -> Self {
        Self::CachedVideo(o)
    }
}

impl From<InlineQueryResultCachedVoice> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedVoice) -> Self {
        Self::CachedVoice(o)
    }
}

impl From<InlineQueryResultArticle> for InlineQueryResult {
    fn from(o: InlineQueryResultArticle) -> Self {
        Self::Article(o)
    }
}

impl From<InlineQueryResultAudio> for InlineQueryResult {
    fn from(o: InlineQueryResultAudio) -> Self {
        Self::Audio(o)
    }
}

impl From<InlineQueryResultContact> for InlineQueryResult {
    fn from(o: InlineQueryResultContact) -> Self {
        Self::Contact(o)
    }
}

impl From<InlineQueryResultGame> for InlineQueryResult {
    fn from(o: InlineQueryResultGame) -> Self {
        Self::Game(o)
    }
}

impl From<InlineQueryResultDocument> for InlineQueryResult {
    fn from(o: InlineQueryResultDocument) -> Self {
        Self::Document(o)
    }
}

impl From<InlineQueryResultGif> for InlineQueryResult {
    fn from(o: InlineQueryResultGif) -> Self {
        Self::Gif(o)
    }
}

impl From<InlineQueryResultLocation> for InlineQueryResult {
    fn from(o: InlineQueryResultLocation) -> Self {
        Self::Location(o)
    }
}

impl From<InlineQueryResultMpeg4Gif> for InlineQueryResult {
    fn from(o: InlineQueryResultMpeg4Gif) -> Self {
        Self::Mpeg4Gif(o)
    }
}

impl From<InlineQueryResultPhoto> for InlineQueryResult {
    fn from(o: InlineQueryResultPhoto) -> Self {
        Self::Photo(o)
    }
}

impl From<InlineQueryResultVenue> for InlineQueryResult {
    fn from(o: InlineQueryResultVenue) -> Self {
        Self::Venue(o)
    }
}

impl From<InlineQueryResultVideo> for InlineQueryResult {
    fn from(o: InlineQueryResultVideo) -> Self {
        Self::Video(o)
    }
}

impl From<InlineQueryResultVoice> for InlineQueryResult {
    fn from(o: InlineQueryResultVoice) -> Self {
        Self::Voice(o)
    }
}

impl FormSer for InlineQueryResultArticle {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.url.serialize(format!("{}[url]", key), form);
        form = self.hide_url.serialize(format!("{}[hide_url]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_width.serialize(format!("{}[thumb_width]", key), form);
        form = self.thumb_height.serialize(format!("{}[thumb_height]", key), form);
        form
    }
}

impl InlineQueryResultArticle {
    pub fn new(id: String, title: String, input_message_content: InputMessageContent, ) -> Self {
        Self {
            id,
            title,
            input_message_content,
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

impl FormSer for InlineQueryResultPhoto {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.photo_url.serialize(format!("{}[photo_url]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.photo_width.serialize(format!("{}[photo_width]", key), form);
        form = self.photo_height.serialize(format!("{}[photo_height]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultPhoto {
    pub fn new(id: String, photo_url: String, thumb_url: String, ) -> Self {
        Self {
            id,
            photo_url,
            thumb_url,
            photo_width: None,
            photo_height: None,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultGif {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.gif_url.serialize(format!("{}[gif_url]", key), form);
        form = self.gif_width.serialize(format!("{}[gif_width]", key), form);
        form = self.gif_height.serialize(format!("{}[gif_height]", key), form);
        form = self.gif_duration.serialize(format!("{}[gif_duration]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_mime_type.serialize(format!("{}[thumb_mime_type]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultGif {
    pub fn new(id: String, gif_url: String, thumb_url: String, ) -> Self {
        Self {
            id,
            gif_url,
            gif_width: None,
            gif_height: None,
            gif_duration: None,
            thumb_url,
            thumb_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultMpeg4Gif {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.mpeg4_url.serialize(format!("{}[mpeg4_url]", key), form);
        form = self.mpeg4_width.serialize(format!("{}[mpeg4_width]", key), form);
        form = self.mpeg4_height.serialize(format!("{}[mpeg4_height]", key), form);
        form = self.mpeg4_duration.serialize(format!("{}[mpeg4_duration]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_mime_type.serialize(format!("{}[thumb_mime_type]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultMpeg4Gif {
    pub fn new(id: String, mpeg4_url: String, thumb_url: String, ) -> Self {
        Self {
            id,
            mpeg4_url,
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            thumb_url,
            thumb_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultVideo {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.video_url.serialize(format!("{}[video_url]", key), form);
        form = self.mime_type.serialize(format!("{}[mime_type]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.video_width.serialize(format!("{}[video_width]", key), form);
        form = self.video_height.serialize(format!("{}[video_height]", key), form);
        form = self.video_duration.serialize(format!("{}[video_duration]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultVideo {
    pub fn new(id: String, video_url: String, mime_type: String, thumb_url: String, title: String, ) -> Self {
        Self {
            id,
            video_url,
            mime_type,
            thumb_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultAudio {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.audio_url.serialize(format!("{}[audio_url]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.performer.serialize(format!("{}[performer]", key), form);
        form = self.audio_duration.serialize(format!("{}[audio_duration]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultAudio {
    pub fn new(id: String, audio_url: String, title: String, ) -> Self {
        Self {
            id,
            audio_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            performer: None,
            audio_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultVoice {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.voice_url.serialize(format!("{}[voice_url]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.voice_duration.serialize(format!("{}[voice_duration]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultVoice {
    pub fn new(id: String, voice_url: String, title: String, ) -> Self {
        Self {
            id,
            voice_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            voice_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultDocument {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.document_url.serialize(format!("{}[document_url]", key), form);
        form = self.mime_type.serialize(format!("{}[mime_type]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_width.serialize(format!("{}[thumb_width]", key), form);
        form = self.thumb_height.serialize(format!("{}[thumb_height]", key), form);
        form
    }
}

impl InlineQueryResultDocument {
    pub fn new(id: String, title: String, document_url: String, mime_type: String, ) -> Self {
        Self {
            id,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            document_url,
            mime_type,
            description: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

impl FormSer for InlineQueryResultLocation {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.latitude.serialize(format!("{}[latitude]", key), form);
        form = self.longitude.serialize(format!("{}[longitude]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.horizontal_accuracy.serialize(format!("{}[horizontal_accuracy]", key), form);
        form = self.live_period.serialize(format!("{}[live_period]", key), form);
        form = self.heading.serialize(format!("{}[heading]", key), form);
        form = self.proximity_alert_radius.serialize(format!("{}[proximity_alert_radius]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_width.serialize(format!("{}[thumb_width]", key), form);
        form = self.thumb_height.serialize(format!("{}[thumb_height]", key), form);
        form
    }
}

impl InlineQueryResultLocation {
    pub fn new(id: String, latitude: f64, longitude: f64, title: String, ) -> Self {
        Self {
            id,
            latitude,
            longitude,
            title,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

impl FormSer for InlineQueryResultVenue {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.latitude.serialize(format!("{}[latitude]", key), form);
        form = self.longitude.serialize(format!("{}[longitude]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.address.serialize(format!("{}[address]", key), form);
        form = self.foursquare_id.serialize(format!("{}[foursquare_id]", key), form);
        form = self.foursquare_type.serialize(format!("{}[foursquare_type]", key), form);
        form = self.google_place_id.serialize(format!("{}[google_place_id]", key), form);
        form = self.google_place_type.serialize(format!("{}[google_place_type]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_width.serialize(format!("{}[thumb_width]", key), form);
        form = self.thumb_height.serialize(format!("{}[thumb_height]", key), form);
        form
    }
}

impl InlineQueryResultVenue {
    pub fn new(id: String, latitude: f64, longitude: f64, title: String, address: String, ) -> Self {
        Self {
            id,
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

impl FormSer for InlineQueryResultContact {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.phone_number.serialize(format!("{}[phone_number]", key), form);
        form = self.first_name.serialize(format!("{}[first_name]", key), form);
        form = self.last_name.serialize(format!("{}[last_name]", key), form);
        form = self.vcard.serialize(format!("{}[vcard]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_width.serialize(format!("{}[thumb_width]", key), form);
        form = self.thumb_height.serialize(format!("{}[thumb_height]", key), form);
        form
    }
}

impl InlineQueryResultContact {
    pub fn new(id: String, phone_number: String, first_name: String, ) -> Self {
        Self {
            id,
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

impl FormSer for InlineQueryResultGame {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.game_short_name.serialize(format!("{}[game_short_name]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form
    }
}

impl InlineQueryResultGame {
    pub fn new(id: String, game_short_name: String, ) -> Self {
        Self {
            id,
            game_short_name,
            reply_markup: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedPhoto {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.photo_file_id.serialize(format!("{}[photo_file_id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedPhoto {
    pub fn new(id: String, photo_file_id: String, ) -> Self {
        Self {
            id,
            photo_file_id,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedGif {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.gif_file_id.serialize(format!("{}[gif_file_id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedGif {
    pub fn new(id: String, gif_file_id: String, ) -> Self {
        Self {
            id,
            gif_file_id,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedMpeg4Gif {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.mpeg4_file_id.serialize(format!("{}[mpeg4_file_id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedMpeg4Gif {
    pub fn new(id: String, mpeg4_file_id: String, ) -> Self {
        Self {
            id,
            mpeg4_file_id,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedSticker {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.sticker_file_id.serialize(format!("{}[sticker_file_id]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedSticker {
    pub fn new(id: String, sticker_file_id: String, ) -> Self {
        Self {
            id,
            sticker_file_id,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedDocument {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.document_file_id.serialize(format!("{}[document_file_id]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedDocument {
    pub fn new(id: String, title: String, document_file_id: String, ) -> Self {
        Self {
            id,
            title,
            document_file_id,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedVideo {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.video_file_id.serialize(format!("{}[video_file_id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedVideo {
    pub fn new(id: String, video_file_id: String, title: String, ) -> Self {
        Self {
            id,
            video_file_id,
            title,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedVoice {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.voice_file_id.serialize(format!("{}[voice_file_id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedVoice {
    pub fn new(id: String, voice_file_id: String, title: String, ) -> Self {
        Self {
            id,
            voice_file_id,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedAudio {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.audio_file_id.serialize(format!("{}[audio_file_id]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedAudio {
    pub fn new(id: String, audio_file_id: String, ) -> Self {
        Self {
            id,
            audio_file_id,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

