extern crate serde_json;
extern crate serde;

use self::serde::ser::Serialize;
use self::serde::ser::Serializer;

use serde_json::value::Value;

macro_rules! option_int {
    ( $( $x:expr ),* ) => {{ (0 $( + if $x.is_some() { 1 } else { 0 } )* ) }};
}

macro_rules! option_serialize_struct_elt {
    ($serializer:expr, $state:expr, $name:expr, $option:expr) => {{
        if $option.is_some() {
            $serializer.serialize_struct_elt($state, $name, &$option)?;
        }
    }};
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult {
    pub ok: bool,
    pub description: Option<String>,
    pub error_code: Option<i64>,
    pub result: Option<Value>,
    pub parameters: Option<ResponseParameters>,
}

#[derive(Debug)]
pub enum MessageOrBool {
    M(Message),
    B(bool),
}

#[derive(Debug)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    ReplyKeyboard(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemoveMarkup),
    ForceReply(ForceReplyMarkup),
}

#[derive(Debug)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub selective: Option<bool>,
}

#[derive(Debug)]
pub struct ReplyKeyboardRemoveMarkup {
    pub remove_keyboard: bool,
    pub selective: Option<bool>,
}

#[derive(Debug)]
pub struct ForceReplyMarkup {
    pub force_reply: bool,
    pub selective: Option<bool>,
}

impl Serialize for ReplyMarkup {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        match *self {
            ReplyMarkup::InlineKeyboard(ref markup) => {
                let mut state = serializer.serialize_struct("InlineKeyboardMarkup", 1)?;
                serializer.serialize_struct_elt(&mut state,
                                                "inline_keyboard",
                                                &markup.inline_keyboard)?;
                serializer.serialize_struct_end(state)
            }

            ReplyMarkup::ReplyKeyboard(ref markup) => {
                let mut state = serializer.serialize_struct("ReplyKeyboardMarkup",
                                      1 +
                                      option_int!(markup.resize_keyboard,
                                                  markup.one_time_keyboard,
                                                  markup.selective))?;

                serializer.serialize_struct_elt(&mut state, "keyboard", &markup.keyboard)?;

                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "resize_keyboard",
                                             markup.resize_keyboard);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "one_time_keyboard",
                                             markup.one_time_keyboard);
                option_serialize_struct_elt!(serializer, &mut state, "selective", markup.selective);

                serializer.serialize_struct_end(state)
            }

            ReplyMarkup::ReplyKeyboardRemove(ref markup) => {
                let mut state = serializer.serialize_struct("ReplyKeyboardRemove", 1 +
                option_int!(markup.selective))?;

                serializer.serialize_struct_elt(&mut state,
                                                "remove_keyboard",
                                                markup.remove_keyboard)?;

                option_serialize_struct_elt!(serializer, &mut state, "selective", markup.selective);

                serializer.serialize_struct_end(state)
            }

            ReplyMarkup::ForceReply(ref markup) => {
                let mut state =
                    serializer.serialize_struct("ForceReply", 1 + option_int!(markup.selective))?;

                serializer.serialize_struct_elt(&mut state, "force_reply", markup.force_reply)?;

                option_serialize_struct_elt!(serializer, &mut state, "selective", markup.selective);

                serializer.serialize_struct_end(state)
            }
        }
    }
}

#[derive(Debug)]
pub enum InlineQueryResult {
    Article(InlineQueryResultArticle),
    Photo(InlineQueryResultPhoto),
    Gif(InlineQueryResultGif),
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    Video(InlineQueryResultVideo),
    Audio(InlineQueryResultAudio),
    Voice(InlineQueryResultVoice),
    Document(InlineQueryResultDocument),
    Location(InlineQueryResultLocation),
    Venue(InlineQueryResultVenue),
    Contact(InlineQueryResultContact),
    Game(InlineQueryResultGame),
    CachedPhoto(InlineQueryResultCachedPhoto),
    CachedGif(InlineQueryResultCachedGif),
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    CachedSticker(InlineQueryResultCachedSticker),
    CachedDocument(InlineQueryResultCachedDocument),
    CachedVideo(InlineQueryResultCachedVideo),
    CachedVoice(InlineQueryResultCachedVoice),
    CachedAudio(InlineQueryResultCachedAudio),
}

#[derive(Debug)]
pub struct InlineQueryResultArticle {
    pub type_name: String,
    pub id: String,
    pub title: String,
    pub input_message_content: InputMessageContent,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub url: Option<String>,
    pub hide_url: Option<bool>,
    pub description: Option<String>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug)]
pub struct InlineQueryResultPhoto {
    pub type_name: String,
    pub id: String,
    pub photo_url: String,
    pub thumb_url: String,
    pub photo_width: Option<i64>,
    pub photo_height: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultGif {
    pub type_name: String,
    pub id: String,
    pub gif_url: String,
    pub gif_width: Option<i64>,
    pub gif_height: Option<i64>,
    pub thumb_url: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultMpeg4Gif {
    pub type_name: String,
    pub id: String,
    pub mpeg4_url: String,
    pub mpeg4_width: Option<i64>,
    pub mpeg4_height: Option<i64>,
    pub thumb_url: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultVideo {
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
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultAudio {
    pub type_name: String,
    pub id: String,
    pub audio_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub performer: Option<String>,
    pub audio_duration: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultVoice {
    pub type_name: String,
    pub id: String,
    pub voice_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub voice_duration: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultDocument {
    pub type_name: String,
    pub id: String,
    pub title: String,
    pub caption: Option<String>,
    pub document_url: String,
    pub mime_type: String,
    pub description: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug)]
pub struct InlineQueryResultLocation {
    pub type_name: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug)]
pub struct InlineQueryResultVenue {
    pub type_name: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug)]
pub struct InlineQueryResultContact {
    pub type_name: String,
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug)]
pub struct InlineQueryResultGame {
    pub type_name: String,
    pub id: String,
    pub game_short_name: String,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug)]
pub struct InlineQueryResultCachedPhoto {
    pub type_name: String,
    pub id: String,
    pub photo_file_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedGif {
    pub type_name: String,
    pub id: String,
    pub gif_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedMpeg4Gif {
    pub type_name: String,
    pub id: String,
    pub mpeg4_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedSticker {
    pub type_name: String,
    pub id: String,
    pub sticker_file_id: String,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedDocument {
    pub type_name: String,
    pub id: String,
    pub title: String,
    pub document_file_id: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedVideo {
    pub type_name: String,
    pub id: String,
    pub video_file_id: String,
    pub title: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedVoice {
    pub type_name: String,
    pub id: String,
    pub voice_file_id: String,
    pub title: String,
    pub caption: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedAudio {
    pub type_name: String,
    pub id: String,
    pub audio_file_id: String,
    pub caption: Option<String>,
    pub reply_markup: Option<ReplyMarkup>, // InlineKeyboardMarkup
    pub input_message_content: Option<InputMessageContent>,
}

impl Serialize for InlineQueryResult {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        match *self {
            InlineQueryResult::Article(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultArticle",
                                      4 +
                                      option_int!(result.reply_markup,
                                                  result.url,
                                                  result.hide_url,
                                                  result.description,
                                                  result.thumb_url,
                                                  result.thumb_width,
                                                  result.thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "title", &result.title)?;
                serializer.serialize_struct_elt(&mut state,
                                          "input_message_content",
                                          &result.input_message_content)?;

                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup);
                option_serialize_struct_elt!(serializer, &mut state, "url", result.url);
                option_serialize_struct_elt!(serializer, &mut state, "hide_url", result.hide_url);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "description",
                                             result.description);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", result.thumb_url);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "thumb_width",
                                             result.thumb_width);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "thumb_height",
                                             result.thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Photo(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultPhoto",
                                      4 +
                                      option_int!(result.photo_width,
                                                  result.photo_height,
                                                  result.title,
                                                  result.description,
                                                  result.caption,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "photo_url", &result.photo_url)?;
                serializer.serialize_struct_elt(&mut state, "thumb_url", &result.thumb_url)?;
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "photo_width",
                                             result.photo_width);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "photo_height",
                                             result.photo_height);
                option_serialize_struct_elt!(serializer, &mut state, "title", result.title);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "description",
                                             result.description);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Gif(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultGif",
                                      4 +
                                      option_int!(result.gif_width,
                                                  result.gif_height,
                                                  result.title,
                                                  result.caption,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "gif_url", &result.gif_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "gif_width", result.gif_width);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "gif_height",
                                             result.gif_height);
                serializer.serialize_struct_elt(&mut state, "thumb_url", &result.thumb_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", result.title);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Mpeg4Gif(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultMpeg4Gif",
                                      4 +
                                      option_int!(result.mpeg4_width,
                                                  result.mpeg4_height,
                                                  result.title,
                                                  result.caption,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "mpeg4_url", &result.mpeg4_url)?;
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "mpeg4_width",
                                             result.mpeg4_width);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "mpeg4_height",
                                             result.mpeg4_height);
                serializer.serialize_struct_elt(&mut state, "thumb_url", &result.thumb_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", result.title);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Video(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultVideo",
                                      6 +
                                      option_int!(result.caption,
                                                  result.video_width,
                                                  result.video_height,
                                                  result.video_duration,
                                                  result.description,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "video_url", &result.video_url)?;
                serializer.serialize_struct_elt(&mut state, "mime_type", &result.mime_type)?;
                serializer.serialize_struct_elt(&mut state, "thumb_url", &result.thumb_url)?;
                serializer.serialize_struct_elt(&mut state, "title", &result.title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "video_width",
                                             result.video_width);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "video_height",
                                             result.video_height);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "video_duration",
                                             result.video_duration);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "description",
                                             result.description);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Audio(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultAudio",
                                      4 +
                                      option_int!(result.caption,
                                                  result.performer,
                                                  result.audio_duration,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "audio_url", &result.audio_url)?;
                serializer.serialize_struct_elt(&mut state, "title", &result.title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "performer", result.performer);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "audio_duration",
                                             result.audio_duration);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Voice(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultVoice",
                                      4 +
                                      option_int!(result.caption,
                                                  result.voice_duration,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "voice_url", &result.voice_url)?;
                serializer.serialize_struct_elt(&mut state, "title", &result.title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "voice_duration",
                                             result.voice_duration);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Document(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultDocument",
                                      5 +
                                      option_int!(result.caption,
                                                  result.description,
                                                  result.reply_markup,
                                                  result.input_message_content,
                                                  result.thumb_url,
                                                  result.thumb_width,
                                                  result.thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "title", &result.title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                serializer.serialize_struct_elt(&mut state, "document_url", &result.document_url)?;
                serializer.serialize_struct_elt(&mut state, "mime_type", &result.mime_type)?;
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "description",
                                             result.description);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", result.thumb_url);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "thumb_width",
                                             result.thumb_width);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "thumb_height",
                                             result.thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Location(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultLocation",
                                      5 +
                                      option_int!(result.reply_markup,
                                                  result.input_message_content,
                                                  result.thumb_url,
                                                  result.thumb_width,
                                                  result.thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "latitude", &result.latitude)?;
                serializer.serialize_struct_elt(&mut state, "longitude", &result.longitude)?;
                serializer.serialize_struct_elt(&mut state, "title", &result.title)?;
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", result.thumb_url);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "thumb_width",
                                             result.thumb_width);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "thumb_height",
                                             result.thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Venue(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultVenue",
                                      6 +
                                      option_int!(result.foursquare_id,
                                                  result.reply_markup,
                                                  result.input_message_content,
                                                  result.thumb_url,
                                                  result.thumb_width,
                                                  result.thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "latitude", &result.latitude)?;
                serializer.serialize_struct_elt(&mut state, "longitude", &result.longitude)?;
                serializer.serialize_struct_elt(&mut state, "title", &result.title)?;
                serializer.serialize_struct_elt(&mut state, "address", &result.address)?;
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "foursquare_id",
                                             result.foursquare_id);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", result.thumb_url);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "thumb_width",
                                             result.thumb_width);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "thumb_height",
                                             result.thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Contact(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultContact",
                                      4 +
                                      option_int!(result.last_name,
                                                  result.reply_markup,
                                                  result.input_message_content,
                                                  result.thumb_url,
                                                  result.thumb_width,
                                                  result.thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "phone_number", &result.phone_number)?;
                serializer.serialize_struct_elt(&mut state, "first_name", &result.first_name)?;
                option_serialize_struct_elt!(serializer, &mut state, "last_name", result.last_name);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", result.thumb_url);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "thumb_width",
                                             result.thumb_width);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "thumb_height",
                                             result.thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Game(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultGame",
                                      3 + option_int!(result.reply_markup))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state,
                                                "game_short_name",
                                                &result.game_short_name)?;
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedPhoto(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedPhoto",
                                      3 +
                                      option_int!(result.title,
                                                  result.description,
                                                  result.caption,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state,
                                                "photo_file_id",
                                                &result.photo_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", result.title);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "description",
                                             result.description);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedGif(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedGif",
                                      3 +
                                      option_int!(result.title,
                                                  result.caption,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "gif_file_id", &result.gif_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", result.title);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedMpeg4Gif(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedMpeg4Gif",
                                      3 +
                                      option_int!(result.title,
                                                  result.caption,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state,
                    "mpeg4_file_id",
                    &result.mpeg4_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", result.title);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedSticker(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedSticker",
                                      3 +
                                      option_int!(result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state,
                 "sticker_file_id",
                  &result.sticker_file_id)?;
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedDocument(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedDocument",
                                      4 +
                                      option_int!(result.description,
                                                  result.caption,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state, "title", &result.title)?;
                serializer.serialize_struct_elt(&mut state,
                 "document_file_id",
                 &result.document_file_id)?;
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "description",
                                             result.description);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedVideo(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedVideo",
                                      4 +
                                      option_int!(result.description,
                                                  result.caption,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state,
                    "video_file_id",
                    &result.video_file_id)?;
                serializer.serialize_struct_elt(&mut state, "title", &result.title)?;
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "description",
                                             result.description);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedVoice(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedVoice",
                                      4 +
                                      option_int!(result.caption,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state,
                 "voice_file_id",
                  &result.voice_file_id)?;
                serializer.serialize_struct_elt(&mut state, "title", &result.title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedAudio(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedAudio",
                                      3 +
                                      option_int!(result.caption,
                                                  result.reply_markup,
                                                  result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", &result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", &result.id)?;
                serializer.serialize_struct_elt(&mut state,
                    "audio_file_id",
                     &result.audio_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "reply_markup",
                                             result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "input_message_content",
                                             result.input_message_content);

                serializer.serialize_struct_end(state)
            }
        }
    }
}

#[derive(Debug)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
}

#[derive(Debug)]
pub struct InputTextMessageContent {
    pub message_text: String,
    pub parse_mode: Option<String>,
    pub disable_web_page_preview: Option<bool>,
}

#[derive(Debug)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug)]
pub struct InputVenueMessageContent {
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
}

#[derive(Debug)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

impl Serialize for InputMessageContent {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        match *self {
            InputMessageContent::Text(ref content) => {
                let mut state = serializer.serialize_struct("InputTextMessageContent",
                                      1 +
                                      option_int!(content.parse_mode,
                                                  content.disable_web_page_preview))?;

                serializer.serialize_struct_elt(&mut state, "message_text", &content.message_text)?;
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "parse_mode",
                                             content.parse_mode);
                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "disable_web_page_preview",
                                             content.disable_web_page_preview);

                serializer.serialize_struct_end(state)
            }

            InputMessageContent::Location(ref content) => {
                let mut state = serializer.serialize_struct("InputLocationMessageContent", 2)?;

                serializer.serialize_struct_elt(&mut state, "latitude", &content.latitude)?;
                serializer.serialize_struct_elt(&mut state, "longitude", &content.longitude)?;

                serializer.serialize_struct_end(state)
            }

            InputMessageContent::Venue(ref content) => {
                let mut state = serializer.serialize_struct("InputVenueMessageContent",
                                      4 + option_int!(content.foursquare_id))?;

                serializer.serialize_struct_elt(&mut state, "latitude", &content.latitude)?;
                serializer.serialize_struct_elt(&mut state, "longitude", &content.longitude)?;
                serializer.serialize_struct_elt(&mut state, "title", &content.title)?;
                serializer.serialize_struct_elt(&mut state, "address", &content.address)?;

                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "foursquare_id",
                                             content.foursquare_id);

                serializer.serialize_struct_end(state)
            }

            InputMessageContent::Contact(ref content) => {
                let mut state = serializer.serialize_struct("InputContactMessageContent",
                                      2 + option_int!(content.last_name))?;

                serializer.serialize_struct_elt(&mut state, "phone_number", &content.phone_number)?;
                serializer.serialize_struct_elt(&mut state, "first_name", &content.first_name)?;

                option_serialize_struct_elt!(serializer,
                                             &mut state,
                                             "last_name",
                                             content.last_name);

                serializer.serialize_struct_end(state)
            }
        }
    }
}

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
pub struct File {
    pub file_id: String,
    pub file_size: Option<i64>,
    pub file_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct KeyboardButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
}

#[derive(Debug, Serialize)]
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
pub struct CallbackGame {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameHighScore {
    pub position: i64,
    pub user: User,
    pub score: i64,
}
