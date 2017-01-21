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

#[derive(Debug, Deserialize)]
pub enum ReplyMarkup {
    InlineKeyboard<'a> {
        inline_keyboard: &'a [&'a InlineKeyboardButton]],
    },

    ReplyKeyboard<'a> {
        keyboard: &'a [&'a KeyboardButton]],
        resize_keyboard: Option<bool>,
        one_time_keyboard: Option<bool>,
        selective: Option<bool>,
    },

    ReplyKeyboardRemove {
        remove_keyboard: bool,
        selective: Option<bool>,
    },

    ForceReply {
        force_reply: bool,
        selective: Option<bool>,
    }
}

impl Serialize for ReplyMarkup {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
     S: Serializer {
        match *self {
            ReplyMarkup::InlineKeyboard {
                ref inline_keyboard,
            } => {
                let mut state = serializer.serialize_struct("InlineKeyboardMarkup",  1)?;
                serializer.serialize_struct_elt(&mut state, "inline_keyboard", inline_keyboard)?;
                serializer.serialize_struct_end(state)
            }

            ReplyMarkup::ReplyKeyboard {
                ref keyboard,
                ref resize_keyboard,
                ref one_time_keyboard,
                ref selective,
            } => {
                let mut state = serializer.serialize_struct("ReplyKeyboardMarkup", 1 +
                option_int!(resize_keyboard, one_time_keyboard, selective))?;

                serializer.serialize_struct_elt(&mut state, "keyboard", keyboard)?;

                option_serialize_struct_elt!(serializer, &mut state, "resize_keyboard", resize_keyboard);
                option_serialize_struct_elt!(serializer, &mut state, "one_time_keyboard", one_time_keyboard);
                option_serialize_struct_elt!(serializer, &mut state, "selective", selective);

                serializer.serialize_struct_end(state)
            }

            ReplyMarkup::ReplyKeyboardRemove {
                ref remove_keyboard,
                ref selective,
            } => {
                let mut state = serializer.serialize_struct("ReplyKeyboardRemove", 1 +
                option_int!(selective))?;

                serializer.serialize_struct_elt(&mut state, "remove_keyboard", remove_keyboard)?;

                option_serialize_struct_elt!(serializer, &mut state, "selective", selective);

                serializer.serialize_struct_end(state)
            }

            ReplyMarkup::ForceReply {
                ref force_reply,
                ref selective,
            } => {
                let mut state = serializer.serialize_struct("ForceReply", 1 +
                option_int!(selective))?;

                serializer.serialize_struct_elt(&mut state, "force_reply", force_reply)?;

                option_serialize_struct_elt!(serializer, &mut state, "selective", selective);

                serializer.serialize_struct_end(state)
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum InlineQueryResult {
    Article<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        title: &'a str,
        input_message_content: &'a InputMessageContent,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        url: Option<&'a str>,
        hide_url: Option<bool>,
        description: Option<&'a str>,
        thumb_url: Option<&'a str>,
        thumb_width: Option<i64>,
        thumb_height: Option<i64>,
    },

    Photo<'a> {
        #[serde(rename="type")]
        type_name: String,
        id: String,
        photo_url: String,
        thumb_url: String,
        photo_width: Option<i64>,
        photo_height: Option<i64>,
        title: Option<&'a str>,
        description: Option<&'a str>,
        caption: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    Gif<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        gif_url: &'a str,
        gif_width: Option<i64>,
        gif_height: Option<i64>,
        thumb_url: &'a str,
        title: Option<&'a str>,
        caption: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    Mpeg4Gif<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        mpeg4_url: &'a str,
        mpeg4_width: Option<i64>,
        mpeg4_height: Option<i64>,
        thumb_url: &'a str,
        title: Option<&'a str>,
        caption: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    Video<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        video_url: &'a str,
        mime_type: &'a str,
        thumb_url: &'a str,
        title: &'a str,
        caption: Option<&'a str>,
        video_width: Option<i64>,
        video_height: Option<i64>,
        video_duration: Option<i64>,
        description: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    Audio<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        audio_url: &'a str,
        title: &'a str,
        caption: Option<&'a str>,
        performer: Option<&'a str>,
        audio_duration: Option<i64>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    Voice<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        voice_url: &'a str,
        title: &'a str,
        caption: Option<&'a str>,
        voice_duration: Option<i64>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    Document<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        title: &'a str,
        caption: Option<&'a str>,
        document_url: &'a str,
        mime_type: &'a str,
        description: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
        thumb_url: Option<&'a str>,
        thumb_width: Option<i64>,
        thumb_height: Option<i64>,
    },

    Location<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        latitude: f64,
        longitude: f64,
        title: &'a str,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
        thumb_url: Option<&'a str>,
        thumb_width: Option<i64>,
        thumb_height: Option<i64>,
    },

    Venue<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        latitude: f64,
        longitude: f64,
        title: &'a str,
        address: &'a str,
        foursquare_id: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
        thumb_url: Option<&'a str>,
        thumb_width: Option<i64>,
        thumb_height: Option<i64>,
    },

    Contact<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        phone_number: &'a str,
        first_name: &'a str,
        last_name: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
        thumb_url: Option<&'a str>,
        thumb_width: Option<i64>,
        thumb_height: Option<i64>,
    },

    Game<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        game_short_name: &'a str,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
    },

    CachedPhoto<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        photo_file_id: &'a str,
        title: Option<&'a str>,
        description: Option<&'a str>,
        caption: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    CachedGif<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        gif_file_id: &'a str,
        title: Option<&'a str>,
        caption: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    CachedMpeg4Gif<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        mpeg4_file_id: &'a str,
        title: Option<&'a str>,
        caption: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    CachedSticker<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        sticker_file_id: &'a str,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    CachedDocument<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        title: &'a str,
        document_file_id: &'a str,
        description: Option<&'a str>,
        caption: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    CachedVideo<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        video_file_id: &'a str,
        title: &'a str,
        description: Option<&'a str>,
        caption: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    CachedVoice<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        voice_file_id: &'a str,
        title: &'a str,
        caption: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },

    CachedAudio<'a> {
        #[serde(rename="type")]
        type_name: &'a str,
        id: &'a str,
        audio_file_id: &'a str,
        caption: Option<&'a str>,
        reply_markup: Option<&'a ReplyMarkup>, // InlineKeyboardMarkup
        input_message_content: Option<&'a InputMessageContent>,
    },
}

impl Serialize for InlineQueryResult {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
     S: Serializer {
        match *self {
            InlineQueryResult::Article {
                ref type_name,
                ref id,
                ref title,
                ref input_message_content,

                ref reply_markup,
                ref url,
                ref hide_url,
                ref description,
                ref thumb_url,
                ref thumb_width,
                ref thumb_height,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultArticle", 4 +
                option_int!(reply_markup, url, hide_url, description,
                            thumb_url, thumb_width, thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "title", title)?;
                serializer.serialize_struct_elt(&mut state, "input_message_content", input_message_content)?;

                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup);
                option_serialize_struct_elt!(serializer, &mut state, "url", url);
                option_serialize_struct_elt!(serializer, &mut state, "hide_url", hide_url);
                option_serialize_struct_elt!(serializer, &mut state, "description", description);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", thumb_url);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_width", thumb_width);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_height", thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Photo {
                ref type_name,
                ref id,
                ref photo_url,
                ref thumb_url,

                ref photo_width,
                ref photo_height,
                ref title,
                ref description,
                ref caption,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultPhoto", 4 +
                    option_int!(photo_width,
                                photo_height,
                                title,
                                description,
                                caption,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "photo_url", photo_url)?;
                serializer.serialize_struct_elt(&mut state, "thumb_url", thumb_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "photo_width", photo_width);
                option_serialize_struct_elt!(serializer, &mut state, "photo_height", photo_height);
                option_serialize_struct_elt!(serializer, &mut state, "title", title);
                option_serialize_struct_elt!(serializer, &mut state, "description", description);
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Gif {
                ref type_name,
                ref id,
                ref gif_url,

                ref gif_width,
                ref gif_height,

                ref thumb_url,

                ref title,
                ref caption,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultGif", 4 +
                    option_int!(gif_width,
                                gif_height,
                                title,
                                caption,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "gif_url", gif_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "gif_width", gif_width);
                option_serialize_struct_elt!(serializer, &mut state, "gif_height", gif_height);
                serializer.serialize_struct_elt(&mut state, "thumb_url", thumb_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", title);
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Mpeg4Gif {
                ref type_name,
                ref id,
                ref mpeg4_url,

                ref mpeg4_width,
                ref mpeg4_height,

                ref thumb_url,

                ref title,
                ref caption,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultMpeg4Gif", 4 +
                    option_int!(mpeg4_width,
                                mpeg4_height,
                                title,
                                caption,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "mpeg4_url", mpeg4_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "mpeg4_width", mpeg4_width);
                option_serialize_struct_elt!(serializer, &mut state, "mpeg4_height", mpeg4_height);
                serializer.serialize_struct_elt(&mut state, "thumb_url", thumb_url)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", title);
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Video {
                ref type_name,
                ref id,
                ref video_url,
                ref mime_type,
                ref thumb_url,
                ref title,

                ref caption,
                ref video_width,
                ref video_height,
                ref video_duration,
                ref description,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultVideo", 6 +
                    option_int!(caption,
                                video_width,
                                video_height,
                                video_duration,
                                description,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "video_url", video_url)?;
                serializer.serialize_struct_elt(&mut state, "mime_type", mime_type)?;
                serializer.serialize_struct_elt(&mut state, "thumb_url", thumb_url)?;
                serializer.serialize_struct_elt(&mut state, "title", title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "video_width", video_width);
                option_serialize_struct_elt!(serializer, &mut state, "video_height", video_height);
                option_serialize_struct_elt!(serializer, &mut state, "video_duration", video_duration);
                option_serialize_struct_elt!(serializer, &mut state, "description", description);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Audio {
                ref type_name,
                ref id,
                ref audio_url,
                ref title,

                ref caption,
                ref performer,
                ref audio_duration,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultAudio", 4 +
                    option_int!(caption,
                                performer,
                                audio_duration,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "audio_url", audio_url)?;
                serializer.serialize_struct_elt(&mut state, "title", title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "performer", performer);
                option_serialize_struct_elt!(serializer, &mut state, "audio_duration", audio_duration);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Voice {
                ref type_name,
                ref id,
                ref voice_url,
                ref title,

                ref caption,
                ref voice_duration,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultVoice", 4 +
                    option_int!(caption,
                                voice_duration,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "voice_url", voice_url)?;
                serializer.serialize_struct_elt(&mut state, "title", title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "voice_duration", voice_duration);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Document {
                ref type_name,
                ref id,
                ref title,

                ref caption,

                ref document_url,
                ref mime_type,

                ref description,
                ref reply_markup,
                ref input_message_content,
                ref thumb_url,
                ref thumb_width,
                ref thumb_height,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultDocument", 5 +
                    option_int!(caption,
                                description,
                                reply_markup,
                                input_message_content,
                                thumb_url,
                                thumb_width,
                                thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "title", title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                serializer.serialize_struct_elt(&mut state, "document_url", document_url)?;
                serializer.serialize_struct_elt(&mut state, "mime_type", mime_type)?;
                option_serialize_struct_elt!(serializer, &mut state, "description", description);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", thumb_url);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_width", thumb_width);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_height", thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Location {
                ref type_name,
                ref id,
                ref latitude,
                ref longitude,
                ref title,

                ref reply_markup,
                ref input_message_content,
                ref thumb_url,
                ref thumb_width,
                ref thumb_height,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultLocation", 5 +
                    option_int!(reply_markup,
                                input_message_content,
                                thumb_url,
                                thumb_width,
                                thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "latitude", latitude)?;
                serializer.serialize_struct_elt(&mut state, "longitude", longitude)?;
                serializer.serialize_struct_elt(&mut state, "title", title)?;
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", thumb_url);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_width", thumb_width);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_height", thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Venue {
                ref type_name,
                ref id,
                ref latitude,
                ref longitude,
                ref title,
                ref address,

                ref foursquare_id,
                ref reply_markup,
                ref input_message_content,
                ref thumb_url,
                ref thumb_width,
                ref thumb_height,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultVenue", 6 +
                    option_int!(foursquare_id,
                                reply_markup,
                                input_message_content,
                                thumb_url,
                                thumb_width,
                                thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "latitude", latitude)?;
                serializer.serialize_struct_elt(&mut state, "longitude", longitude)?;
                serializer.serialize_struct_elt(&mut state, "title", title)?;
                serializer.serialize_struct_elt(&mut state, "address", address)?;
                option_serialize_struct_elt!(serializer, &mut state, "foursquare_id", foursquare_id);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", thumb_url);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_width", thumb_width);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_height", thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Contact {
                ref type_name,
                ref id,
                ref phone_number,
                ref first_name,

                ref last_name,
                ref reply_markup,
                ref input_message_content,
                ref thumb_url,
                ref thumb_width,
                ref thumb_height,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultContact", 4 +
                    option_int!(last_name,
                                reply_markup,
                                input_message_content,
                                thumb_url,
                                thumb_width,
                                thumb_height))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "phone_number", phone_number)?;
                serializer.serialize_struct_elt(&mut state, "first_name", first_name)?;
                option_serialize_struct_elt!(serializer, &mut state, "last_name", last_name);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_url", thumb_url);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_width", thumb_width);
                option_serialize_struct_elt!(serializer, &mut state, "thumb_height", thumb_height);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::Game {
                ref type_name,
                ref id,
                ref game_short_name,

                ref reply_markup,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultGame", 3 +
                    option_int!(reply_markup))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "game_short_name", game_short_name)?;
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedPhoto {
                ref type_name,
                ref id,
                ref photo_file_id,

                ref title,
                ref description,
                ref caption,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedPhoto", 3 +
                    option_int!(title,
                                description,
                                caption,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "photo_file_id", photo_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", title);
                option_serialize_struct_elt!(serializer, &mut state, "description", description);
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedGif {
                ref type_name,
                ref id,
                ref gif_file_id,

                ref title,
                ref caption,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedGif", 3 +
                    option_int!(title,
                                caption,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "gif_file_id", gif_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", title);
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedMpeg4Gif {
                ref type_name,
                ref id,
                ref mpeg4_file_id,

                ref title,
                ref caption,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedMpeg4Gif", 3 +
                    option_int!(title,
                                caption,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "mpeg4_file_id", mpeg4_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "title", title);
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedSticker {
                ref type_name,
                ref id,
                ref sticker_file_id,

                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedSticker", 3 +
                    option_int!(reply_markup, input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "sticker_file_id", sticker_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedDocument {
                ref type_name,
                ref id,
                ref title,
                ref document_file_id,

                ref description,
                ref caption,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedDocument", 4 +
                    option_int!(description,
                                caption,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "title", title)?;
                serializer.serialize_struct_elt(&mut state, "document_file_id", document_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "description", description);
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedVideo {
                ref type_name,
                ref id,
                ref video_file_id,
                ref title,

                ref description,
                ref caption,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedVideo", 4 +
                    option_int!(description,
                                caption,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "video_file_id", video_file_id)?;
                serializer.serialize_struct_elt(&mut state, "title", title)?;
                option_serialize_struct_elt!(serializer, &mut state, "description", description);
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedVoice {
                ref type_name,
                ref id,
                ref voice_file_id,
                ref title,

                ref caption,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedVoice", 4 +
                    option_int!(caption,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "voice_file_id", voice_file_id)?;
                serializer.serialize_struct_elt(&mut state, "title", title)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }

            InlineQueryResult::CachedAudio {
                ref type_name,
                ref id,
                ref audio_file_id,
                
                ref caption,
                ref reply_markup,
                ref input_message_content,
            } => {
                let mut state = serializer.serialize_struct("InlineQueryResultCachedAudio", 3 +
                    option_int!(caption,
                                reply_markup,
                                input_message_content))?;

                serializer.serialize_struct_elt(&mut state, "type", type_name)?;
                serializer.serialize_struct_elt(&mut state, "id", id)?;
                serializer.serialize_struct_elt(&mut state, "audio_file_id", audio_file_id)?;
                option_serialize_struct_elt!(serializer, &mut state, "caption", caption);
                option_serialize_struct_elt!(serializer, &mut state, "reply_markup", reply_markup); // InlineKeyboardMarkup
                option_serialize_struct_elt!(serializer, &mut state, "input_message_content", input_message_content);

                serializer.serialize_struct_end(state)
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum InputMessageContent {
    Text {
        message_text: &'a str,
        parse_mode: Option<&'a str>,
        disable_web_page_preview: Option<bool>,
    },

    Location {
        latitude: f64,
        longitude: f64,
    },

    Venue {
        latitude: f64,
        longitude: f64,
        title: &'a str,
        address: &'a str,
        foursquare_id: Option<&'a str>,
    },

    Contact {
        phone_number: &'a str,
        first_name: &'a str,
        last_name: Option<&'a str>,
    },
}

impl Serialize for InputMessageContent {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
     S: Serializer {
        match *self {
            InputMessageContent::Text {
                ref message_text,

                ref parse_mode,
                ref disable_web_page_preview,
            } => {
                let mut state = serializer.serialize_struct("InputTextMessageContent",  1 +
                    option_int!(parse_mode, disable_web_page_preview))?;

                serializer.serialize_struct_elt(&mut state, "message_text", message_text)?;
                option_serialize_struct_elt!(serializer, &mut state, "parse_mode", parse_mode);
                option_serialize_struct_elt!(serializer, &mut state, "disable_web_page_preview", disable_web_page_preview);

                serializer.serialize_struct_end(state)
            }

            InputMessageContent::Location {
                ref latitude,
                ref longitude,
            } => {
                let mut state = serializer.serialize_struct("InputLocationMessageContent", 2)?;

                serializer.serialize_struct_elt(&mut state, "latitude", latitude)?;
                serializer.serialize_struct_elt(&mut state, "longitude", longitude)?;

                serializer.serialize_struct_end(state)
            }

            InputMessageContent::Venue {
                ref latitude,
                ref longitude,
                ref title,
                ref address,

                ref foursquare_id,
            } => {
                let mut state = serializer.serialize_struct("InputVenueMessageContent", 4 +
                option_int!(foursquare_id))?;

                serializer.serialize_struct_elt(&mut state, "latitude", latitude)?;
                serializer.serialize_struct_elt(&mut state, "longitude", longitude)?;
                serializer.serialize_struct_elt(&mut state, "title", title)?;
                serializer.serialize_struct_elt(&mut state, "address", address)?;

                option_serialize_struct_elt!(serializer, &mut state, "foursquare_id", foursquare_id);

                serializer.serialize_struct_end(state)
            }

            InputMessageContent::Contact {
                ref phone_number,
                ref first_name,

                ref last_name,
            } => {
                let mut state = serializer.serialize_struct("InputContactMessageContent", 2 +
                option_int!(last_name))?;

                serializer.serialize_struct_elt(&mut state, "phone_number", phone_number)?;
                serializer.serialize_struct_elt(&mut state, "first_name", first_name)?;

                option_serialize_struct_elt!(serializer, &mut state, "last_name", last_name);

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

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyboardButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
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
pub struct CallbackGame { }

#[derive(Debug, Serialize, Deserialize)]
pub struct GameHighScore {
    pub position: i64,
    pub user: User,
    pub score: i64,
}
