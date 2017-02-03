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
            $serializer.serialize_struct_elt($state, $name, $option)?;
        }
    }};
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult {
    pub ok: bool,
    pub description: Option<String>,
    pub error_code: Option<i64>,
    pub result: Option<Value>,
    pub parameters: Option<ResponseParameters>
}

#[derive(Debug)]
pub enum MessageOrBool {
    M(Message),
    B(bool),
}

#[derive(Debug)]
pub enum ReplyMarkup<'a> {
    InlineKeyboard(InlineKeyboardMarkup<'a>),
    ReplyKeyboard(ReplyKeyboardMarkup<'a>),
    ReplyKeyboardRemove(ReplyKeyboardRemoveMarkup),
    ForceReply(ForceReplyMarkup),
}

#[derive(Debug)]
pub struct InlineKeyboardMarkup<'a> {
    pub inline_keyboard: &'a [&'a [InlineKeyboardButton<'a>]],
}

#[derive(Debug)]
pub struct ReplyKeyboardMarkup<'a> {
    pub keyboard: &'a [&'a [KeyboardButton<'a>]],
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

impl<'a> Serialize for ReplyMarkup<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
     S: Serializer {
        match *self {
            ReplyMarkup::InlineKeyboard(ref markup) => {
                let mut state = serializer.serialize_struct("InlineKeyboardMarkup",  1)?;
                serializer.serialize_struct_elt(&mut state, "inline_keyboard", markup.inline_keyboard)?;
                serializer.serialize_struct_end(state)
            }

            ReplyMarkup::ReplyKeyboard(ref markup) => {
                let mut state = serializer.serialize_struct("ReplyKeyboardMarkup", 1 +
                option_int!(markup.resize_keyboard,
                            markup.one_time_keyboard,
                            markup.selective))?;

                serializer.serialize_struct_elt(&mut state, "keyboard", markup.keyboard)?;

                option_serialize_struct_elt!(serializer, &mut state, "resize_keyboard", markup.resize_keyboard);
                option_serialize_struct_elt!(serializer, &mut state, "one_time_keyboard", markup.one_time_keyboard);
                option_serialize_struct_elt!(serializer, &mut state, "selective", markup.selective);

                serializer.serialize_struct_end(state)
            }

            ReplyMarkup::ReplyKeyboardRemove(ref markup) => {
                let mut state = serializer.serialize_struct("ReplyKeyboardRemove", 1 +
                option_int!(markup.selective))?;

                serializer.serialize_struct_elt(&mut state, "remove_keyboard", markup.remove_keyboard)?;

                option_serialize_struct_elt!(serializer, &mut state, "selective", markup.selective);

                serializer.serialize_struct_end(state)
            }

            ReplyMarkup::ForceReply (ref markup) => {
                let mut state = serializer.serialize_struct("ForceReply", 1 +
                option_int!(markup.selective))?;

                serializer.serialize_struct_elt(&mut state, "force_reply", markup.force_reply)?;

                option_serialize_struct_elt!(serializer, &mut state, "selective", markup.selective);

                serializer.serialize_struct_end(state)
            }
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct InlineQueryResultArticle<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub title: &'a str,
    pub input_message_content: &'a InputMessageContent<'a>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub url: Option<&'a str>,
    pub hide_url: Option<bool>,
    pub description: Option<&'a str>,
    pub thumb_url: Option<&'a str>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug)]
pub struct InlineQueryResultPhoto<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub photo_url: &'a str,
    pub thumb_url: &'a str,
    pub photo_width: Option<i64>,
    pub photo_height: Option<i64>,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultGif<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub gif_url: &'a str,
    pub gif_width: Option<i64>,
    pub gif_height: Option<i64>,
    pub thumb_url: &'a str,
    pub title: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultMpeg4Gif<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub mpeg4_url: &'a str,
    pub mpeg4_width: Option<i64>,
    pub mpeg4_height: Option<i64>,
    pub thumb_url: &'a str,
    pub title: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultVideo<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub video_url: &'a str,
    pub mime_type: &'a str,
    pub thumb_url: &'a str,
    pub title: &'a str,
    pub caption: Option<&'a str>,
    pub video_width: Option<i64>,
    pub video_height: Option<i64>,
    pub video_duration: Option<i64>,
    pub description: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultAudio<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub audio_url: &'a str,
    pub title: &'a str,
    pub caption: Option<&'a str>,
    pub performer: Option<&'a str>,
    pub audio_duration: Option<i64>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultVoice<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub voice_url: &'a str,
    pub title: &'a str,
    pub caption: Option<&'a str>,
    pub voice_duration: Option<i64>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultDocument<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub title: &'a str,
    pub caption: Option<&'a str>,
    pub document_url: &'a str,
    pub mime_type: &'a str,
    pub description: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
    pub thumb_url: Option<&'a str>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug)]
pub struct InlineQueryResultLocation<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub latitude: f64,
    pub longitude: f64,
    pub title: &'a str,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
    pub thumb_url: Option<&'a str>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug)]
pub struct InlineQueryResultVenue<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub latitude: f64,
    pub longitude: f64,
    pub title: &'a str,
    pub address: &'a str,
    pub foursquare_id: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
    pub thumb_url: Option<&'a str>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug)]
pub struct InlineQueryResultContact<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub phone_number: &'a str,
    pub first_name: &'a str,
    pub last_name: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
    pub thumb_url: Option<&'a str>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>,
}

#[derive(Debug)]
pub struct InlineQueryResultGame<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub game_short_name: &'a str,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
}

#[derive(Debug)]
pub struct InlineQueryResultCachedPhoto<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub photo_file_id: &'a str,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedGif<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub gif_file_id: &'a str,
    pub title: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedMpeg4Gif<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub mpeg4_file_id: &'a str,
    pub title: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedSticker<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub sticker_file_id: &'a str,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedDocument<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub title: &'a str,
    pub document_file_id: &'a str,
    pub description: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedVideo<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub video_file_id: &'a str,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedVoice<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub voice_file_id: &'a str,
    pub title: &'a str,
    pub caption: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

#[derive(Debug)]
pub struct InlineQueryResultCachedAudio<'a> {
    pub type_name: &'a str,
    pub id: &'a str,
    pub audio_file_id: &'a str,
    pub caption: Option<&'a str>,
    pub reply_markup: Option<&'a ReplyMarkup<'a>>, // InlineKeyboardMarkup
    pub input_message_content: Option<&'a InputMessageContent<'a>>,
}

impl<'a> Serialize for InlineQueryResult<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
     S: Serializer {
        match *self {
            InlineQueryResult::Article(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultArticle", 4 +
                option_int!(result.reply_markup, result.url, result.hide_url, result.description,
                            result.thumb_url, result.thumb_width, result.thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "title", result.title)?;
                serializer.serialize_struct_elt(&mut state, "input_message_content", result.input_message_content)?;

                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup);
                option_serialize_struct_elt!(serializer, &mut state, "url", result.url);
                option_serialize_struct_elt!(serializer, &mut state, "hide_url", result.hide_url);
                option_serialize_struct_elt!(serializer, &mut state, "description", result.description);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", result.thumb_url);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_width", result.thumb_width);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_height", result.thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Photo(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultPhoto", 4 +
                    option_int!(result.photo_width,
                                result.photo_height,
                                result.title,
                                result.description,
                                result.caption,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "photo_url", result.photo_url)?;
                serializer.serialize_struct_elt(&mut state, "thumb_url", result.thumb_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "photo_width", result.photo_width);
                option_serialize_struct_elt!(serializer, &mut state, "photo_height", result.photo_height);
                option_serialize_struct_elt!(serializer, &mut state, "title", result.title);
                option_serialize_struct_elt!(serializer, &mut state, "description", result.description);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Gif(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultGif", 4 +
                    option_int!(result.gif_width,
                                result.gif_height,
                                result.title,
                                result.caption,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "gif_url", result.gif_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "gif_width", result.gif_width);
                option_serialize_struct_elt!(serializer, &mut state, "gif_height", result.gif_height);
                serializer.serialize_struct_elt(&mut state, "thumb_url", result.thumb_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", result.title);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Mpeg4Gif(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultMpeg4Gif", 4 +
                    option_int!(result.mpeg4_width,
                                result.mpeg4_height,
                                result.title,
                                result.caption,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "mpeg4_url", result.mpeg4_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "mpeg4_width", result.mpeg4_width);
                option_serialize_struct_elt!(serializer, &mut state, "mpeg4_height", result.mpeg4_height);
                serializer.serialize_struct_elt(&mut state, "thumb_url", result.thumb_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", result.title);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Video(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultVideo", 6 +
                    option_int!(result.caption,
                                result.video_width,
                                result.video_height,
                                result.video_duration,
                                result.description,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "video_url", result.video_url)?;
                serializer.serialize_struct_elt(&mut state, "mime_type", result.mime_type)?;
                serializer.serialize_struct_elt(&mut state, "thumb_url", result.thumb_url)?;
                serializer.serialize_struct_elt(&mut state, "title", result.title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "video_width", result.video_width);
                option_serialize_struct_elt!(serializer, &mut state, "video_height", result.video_height);
                option_serialize_struct_elt!(serializer, &mut state, "video_duration", result.video_duration);
                option_serialize_struct_elt!(serializer, &mut state, "description", result.description);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Audio(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultAudio", 4 +
                    option_int!(result.caption,
                                result.performer,
                                result.audio_duration,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "audio_url", result.audio_url)?;
                serializer.serialize_struct_elt(&mut state, "title", result.title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "performer", result.performer);
                option_serialize_struct_elt!(serializer, &mut state, "audio_duration", result.audio_duration);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Voice(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultVoice", 4 +
                    option_int!(result.caption,
                                result.voice_duration,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "voice_url", result.voice_url)?;
                serializer.serialize_struct_elt(&mut state, "title", result.title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "voice_duration", result.voice_duration);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Document(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultDocument", 5 +
                    option_int!(result.caption,
                                result.description,
                                result.reply_markup,
                                result.input_message_content,
                                result.thumb_url,
                                result.thumb_width,
                                result.thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "title", result.title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                serializer.serialize_struct_elt(&mut state, "document_url", result.document_url)?;
                serializer.serialize_struct_elt(&mut state, "mime_type", result.mime_type)?;
                option_serialize_struct_elt!(serializer, &mut state, "description", result.description);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", result.thumb_url);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_width", result.thumb_width);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_height", result.thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Location(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultLocation", 5 +
                    option_int!(result.reply_markup,
                                result.input_message_content,
                                result.thumb_url,
                                result.thumb_width,
                                result.thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "latitude", result.latitude)?;
                serializer.serialize_struct_elt(&mut state, "longitude", result.longitude)?;
                serializer.serialize_struct_elt(&mut state, "title", result.title)?;
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", result.thumb_url);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_width", result.thumb_width);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_height", result.thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Venue(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultVenue", 6 +
                    option_int!(result.foursquare_id,
                                result.reply_markup,
                                result.input_message_content,
                                result.thumb_url,
                                result.thumb_width,
                                result.thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "latitude", result.latitude)?;
                serializer.serialize_struct_elt(&mut state, "longitude", result.longitude)?;
                serializer.serialize_struct_elt(&mut state, "title", result.title)?;
                serializer.serialize_struct_elt(&mut state, "address", result.address)?;
                option_serialize_struct_elt!(serializer, &mut state, "foursquare_id", result.foursquare_id);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", result.thumb_url);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_width", result.thumb_width);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_height", result.thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Contact(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultContact", 4 +
                    option_int!(result.last_name,
                                result.reply_markup,
                                result.input_message_content,
                                result.thumb_url,
                                result.thumb_width,
                                result.thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "phone_number", result.phone_number)?;
                serializer.serialize_struct_elt(&mut state, "first_name", result.first_name)?;
                option_serialize_struct_elt!(serializer, &mut state, "last_name", result.last_name);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", result.thumb_url);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_width", result.thumb_width);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_height", result.thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Game(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultGame", 3 +
                    option_int!(result.reply_markup))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "game_short_name", result.game_short_name)?;
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedPhoto(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedPhoto", 3 +
                    option_int!(result.title,
                                result.description,
                                result.caption,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "photo_file_id", result.photo_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", result.title);
                option_serialize_struct_elt!(serializer, &mut state, "description", result.description);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedGif(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedGif", 3 +
                    option_int!(result.title,
                                result.caption,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "gif_file_id", result.gif_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", result.title);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedMpeg4Gif(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedMpeg4Gif", 3 +
                    option_int!(result.title,
                                result.caption,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "mpeg4_file_id", result.mpeg4_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", result.title);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedSticker(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedSticker", 3 +
                    option_int!(result.reply_markup, result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "sticker_file_id", result.sticker_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedDocument(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedDocument", 4 +
                    option_int!(result.description,
                                result.caption,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "title", result.title)?;
                serializer.serialize_struct_elt(&mut state, "document_file_id", result.document_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "description", result.description);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedVideo(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedVideo", 4 +
                    option_int!(result.description,
                                result.caption,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "video_file_id", result.video_file_id)?;
                serializer.serialize_struct_elt(&mut state, "title", result.title)?;
                option_serialize_struct_elt!(serializer, &mut state, "description", result.description);
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedVoice(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedVoice", 4 +
                    option_int!(result.caption,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "voice_file_id", result.voice_file_id)?;
                serializer.serialize_struct_elt(&mut state, "title", result.title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedAudio(ref result) => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedAudio", 3 +
                    option_int!(result.caption,
                                result.reply_markup,
                                result.input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", result.type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", result.id)?;
                serializer.serialize_struct_elt(&mut state, "audio_file_id", result.audio_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", result.caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", result.reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", result.input_message_content);

                serializer.serialize_struct_end(state)
            }
        }
    }
}

#[derive(Debug)]
pub enum InputMessageContent<'a> {
    Text(InputTextMessageContent<'a>),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent<'a>),
    Contact(InputContactMessageContent<'a>),
}

#[derive(Debug)]
pub struct InputTextMessageContent<'a> {
    pub message_text: &'a str,
    pub parse_mode: Option<&'a str>,
    pub disable_web_page_preview: Option<bool>,
}

#[derive(Debug)]
pub struct InputLocationMessageContent {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug)]
pub struct InputVenueMessageContent<'a> {
    pub latitude: f64,
    pub longitude: f64,
    pub title: &'a str,
    pub address: &'a str,
    pub foursquare_id: Option<&'a str>,
}

#[derive(Debug)]
pub struct InputContactMessageContent<'a> {
    pub phone_number: &'a str,
    pub first_name: &'a str,
    pub last_name: Option<&'a str>,
}

impl<'a> Serialize for InputMessageContent<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
     S: Serializer {
        match *self {
            InputMessageContent::Text(ref markup) => {
                let mut state = serializer.serialize_struct("InputTextMessageContent",  1 +
                    option_int!(markup.parse_mode, markup.disable_web_page_preview))?;

                serializer.serialize_struct_elt(&mut state, "message_text", markup.message_text)?;
                option_serialize_struct_elt!(serializer, &mut state, "parse_mode", markup.parse_mode);
                option_serialize_struct_elt!(serializer, &mut state, "disable_web_page_preview", markup.disable_web_page_preview);

                serializer.serialize_struct_end(state)
            }

            InputMessageContent::Location(ref markup) => {
                let mut state = serializer.serialize_struct("InputLocationMessageContent", 2)?;

                serializer.serialize_struct_elt(&mut state, "latitude", markup.latitude)?;
                serializer.serialize_struct_elt(&mut state, "longitude", markup.longitude)?;

                serializer.serialize_struct_end(state)
            }

            InputMessageContent::Venue(ref markup) => {
                let mut state = serializer.serialize_struct("InputVenueMessageContent", 4 +
                option_int!(markup.foursquare_id))?;

                serializer.serialize_struct_elt(&mut state, "latitude", markup.latitude)?;
                serializer.serialize_struct_elt(&mut state, "longitude", markup.longitude)?;
                serializer.serialize_struct_elt(&mut state, "title", markup.title)?;
                serializer.serialize_struct_elt(&mut state, "address", markup.address)?;

                option_serialize_struct_elt!(serializer, &mut state, "foursquare_id", markup.foursquare_id);

                serializer.serialize_struct_end(state)
            }

            InputMessageContent::Contact(ref markup) => {
                let mut state = serializer.serialize_struct("InputContactMessageContent", 2 +
                option_int!(markup.last_name))?;

                serializer.serialize_struct_elt(&mut state, "phone_number", markup.phone_number)?;
                serializer.serialize_struct_elt(&mut state, "first_name", markup.first_name)?;

                option_serialize_struct_elt!(serializer, &mut state, "last_name", markup.last_name);

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
pub struct KeyboardButton<'a> {
    pub text: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct InlineKeyboardButton<'a> {
    pub text: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<&'a CallbackGame>,
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
pub struct CallbackGame { }

#[derive(Debug, Serialize, Deserialize)]
pub struct GameHighScore {
    pub position: i64,
    pub user: User,
    pub score: i64,
}
