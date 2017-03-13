use args::*;

use types::InlineKeyboardButton;
use types::KeyboardButton;

use types::ReplyMarkup;

use types::InlineKeyboardMarkup;
use types::ReplyKeyboardMarkup;
use types::ReplyKeyboardRemoveMarkup;
use types::ForceReplyMarkup;

use types::InputMessageContent;

use types::InputTextMessageContent;
use types::InputLocationMessageContent;
use types::InputVenueMessageContent;
use types::InputContactMessageContent;

use types::InlineQueryResult;

use types::InlineQueryResultArticle;
use types::InlineQueryResultMetaPhoto;
use types::InlineQueryResultPhoto;
use types::InlineQueryResultCachedPhoto;
use types::InlineQueryResultMetaGif;
use types::InlineQueryResultGif;
use types::InlineQueryResultCachedGif;
use types::InlineQueryResultMetaMpeg4Gif;
use types::InlineQueryResultMpeg4Gif;
use types::InlineQueryResultCachedMpeg4Gif;
use types::InlineQueryResultMetaVideo;
use types::InlineQueryResultVideo;
use types::InlineQueryResultCachedVideo;
use types::InlineQueryResultMetaAudio;
use types::InlineQueryResultAudio;
use types::InlineQueryResultCachedAudio;
use types::InlineQueryResultMetaVoice;
use types::InlineQueryResultVoice;
use types::InlineQueryResultCachedVoice;
use types::InlineQueryResultMetaDocument;
use types::InlineQueryResultDocument;
use types::InlineQueryResultCachedDocument;
use types::InlineQueryResultLocation;
use types::InlineQueryResultVenue;
use types::InlineQueryResultContact;
use types::InlineQueryResultGame;
use types::InlineQueryResultCachedSticker;

use types;

// Pardon the super long lines this was mostly made by regex and I'm worried
// because it compiled with rather few errors

macro_rules! builder {
    ( $sstruct:ident; $( $name:ident, $ttype:ty );* ) => {
        $(
            pub fn $name(mut self, $name: $ttype) -> $sstruct {
                self.$name = Some($name);
                self
            }
        )*
    }
}

macro_rules! buildera {
    ( $sstruct:ident; $( $name:ident, $ttype:ty );* ) => {
        $(
            pub fn $name(mut self, $name: $ttype) -> $sstruct<'a> {
                self.$name = Some($name);
                self
            }
        )*
    }
}

impl<'a> GetUpdates<'a> {
    buildera!(GetUpdates;
              offset, i64;
              limit, i64;
              timeout, i64;
              allowed_updates, &'a [&'a str]);
}

impl<'a> SetWebhook<'a> {
    buildera!(SetWebhook;
              certificate, &'a str;
              max_connections, i64;
              allowed_updates, &'a [&'a str]);
}

impl<'a> SendMessage<'a> {
    buildera!(SendMessage;
              parse_mode, &'a str;
              disable_web_page_preview, bool;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> ForwardMessage<'a> {
    buildera!(ForwardMessage;
              disable_notification, bool);
}

impl<'a> SendPhoto<'a> {
    buildera!(SendPhoto;
              photo, &'a str;
              file_id, &'a str;
              caption, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendAudio<'a> {
    buildera!(SendAudio;
              audio, &'a str;
              file_id, &'a str;
              caption, &'a str;
              duration, i64;
              performer, &'a str;
              title, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendDocument<'a> {
    buildera!(SendDocument;
              document, &'a str;
              file_id, &'a str;
              caption, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendSticker<'a> {
    buildera!(SendSticker;
              sticker, &'a str;
              file_id, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendVideo<'a> {
    buildera!(SendVideo;
              video, &'a str;
              file_id, &'a str;
              duration, i64;
              width, i64;
              height, i64;
              caption, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendVoice<'a> {
    buildera!(SendVoice;
              voice, &'a str;
              file_id, &'a str;
              caption, &'a str;
              duration, i64;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendLocation<'a> {
    buildera!(SendLocation;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendVenue<'a> {
    buildera!(SendVenue;
              foursquare_id, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendContact<'a> {
    buildera!(SendContact;
              last_name, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl GetUserProfilePhotos {
    builder!(GetUserProfilePhotos;
              offset, i64;
              limit, i64);
}

impl<'a> AnswerCallbackQuery<'a> {
    buildera!(AnswerCallbackQuery;
              text, &'a str;
              show_alert, bool;
              url, &'a str;
              cache_time, i64);
}

impl<'a> EditMessageText<'a> {
    buildera!(EditMessageText;
              message_id, i64;
              inline_message_id, &'a str;
              parse_mode, &'a str;
              disable_web_page_preview, bool;
              reply_markup, Box<ReplyMarkup>); // InlineKeyboardMarkup
}

impl<'a> EditMessageCaption<'a> {
    buildera!(EditMessageCaption;
              message_id, i64;
              inline_message_id, &'a str;
              caption, &'a str;
              reply_markup, Box<ReplyMarkup>); // InlineKeyboardMarkup
}

impl<'a> EditMessageReplyMarkup<'a> {
    buildera!(EditMessageReplyMarkup;
              message_id, i64;
              inline_message_id, &'a str;
              reply_markup, Box<ReplyMarkup>); // InlineKeyboardMarkup
}

impl<'a> AnswerInlineQuery<'a> {
    buildera!(AnswerInlineQuery;
              cache_time, i64;
              is_personal, bool;
              next_offset, &'a str;
              switch_pm_text, &'a str;
              switch_pm_parameter, &'a str);
}

impl<'a> SendGame<'a> {
    buildera!(SendGame;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>); // InlineKeyboardMarkup
}

impl<'a> SetGameScore<'a> {
    buildera!(SetGameScore;
              force, bool;
              disable_edit_message, bool;
              chat_id, i64;
              message_id, i64;
              inline_message_id, &'a str);
}

impl<'a> GetGameHighScores<'a> {
    buildera!(GetGameHighScores;
              chat_id, i64;
              message_id, i64;
              inline_message_id, &'a str);
}

impl KeyboardButton {
    builder!(KeyboardButton;
              request_contact, bool;
              request_location, bool);
}

impl InlineKeyboardButton {
    builder!(InlineKeyboardButton;
              url, String;
              callback_data, String;
              switch_inline_query, String;
              switch_inline_query_current_chat, String;
              callback_game, types::CallbackGame);
}

impl ReplyMarkup {
    pub fn new_inline_keyboard(keyboard: Vec<Vec<InlineKeyboardButton>>) -> ReplyMarkup {
        InlineKeyboardMarkup { inline_keyboard: keyboard }.into()
    }

    pub fn new_reply_keyboard(keyboard: Vec<Vec<KeyboardButton>>) -> ReplyMarkup {
        ReplyKeyboardMarkup {
                keyboard: keyboard,
                resize_keyboard: None,
                one_time_keyboard: None,
                selective: None,
            }
            .into()
    }

    pub fn new_reply_keyboard_remove(remove_keyboard: bool) -> ReplyMarkup {
        ReplyKeyboardRemoveMarkup {
                remove_keyboard: remove_keyboard,
                selective: None,
            }
            .into()
    }

    pub fn new_force_reply(force_reply: bool) -> ReplyMarkup {
        ForceReplyMarkup {
                force_reply: force_reply,
                selective: None,
            }
            .into()
    }

    pub fn selective(mut self, selective: bool) -> ReplyMarkup {
        match self {
            ReplyMarkup::ReplyKeyboard(ref mut markup) => {
                markup.selective = Some(selective);
            }

            ReplyMarkup::ReplyKeyboardRemove(ref mut markup) => {
                markup.selective = Some(selective);
            }

            ReplyMarkup::ForceReply(ref mut markup) => {
                markup.selective = Some(selective);
            }

            _ => {}
        }
        self
    }

    pub fn resize_keyboard(mut self, resize_keyboard: bool) -> ReplyMarkup {
        match self {
            ReplyMarkup::ReplyKeyboard(ref mut markup) => {
                markup.resize_keyboard = Some(resize_keyboard);
            }

            _ => {}
        }
        self
    }

    pub fn one_time_keyboard(mut self, one_time_keyboard: bool) -> ReplyMarkup {
        match self {
            ReplyMarkup::ReplyKeyboard(ref mut markup) => {
                markup.one_time_keyboard = Some(one_time_keyboard);
            }

            _ => {}
        }
        self
    }
}

impl InputMessageContent {
    pub fn new_text(message_text: String) -> InputMessageContent {
        InputTextMessageContent {
                message_text: message_text,
                parse_mode: None,
                disable_web_page_preview: None,
            }
            .into()
    }

    pub fn new_location(latitude: f64, longitude: f64) -> InputMessageContent {
        InputLocationMessageContent {
                latitude: latitude,
                longitude: longitude,
            }
            .into()
    }

    pub fn new_venue(latitude: f64,
                     longitude: f64,
                     title: String,
                     address: String)
                     -> InputMessageContent {
        InputVenueMessageContent {
                latitude: latitude,
                longitude: longitude,
                title: title,
                address: address,
                foursquare_id: None,
            }
            .into()
    }

    pub fn new_contact(phone_number: String, first_name: String) -> InputMessageContent {
        InputContactMessageContent {
                phone_number: phone_number,
                first_name: first_name,
                last_name: None,
            }
            .into()
    }

    pub fn parse_mode(mut self, parse_mode: String) -> InputMessageContent {
        match self {
            InputMessageContent::Text(ref mut markup) => {
                markup.parse_mode = Some(parse_mode);
            }

            _ => {}
        }
        self
    }

    pub fn disable_web_page_preview(mut self,
                                    disable_web_page_preview: bool)
                                    -> InputMessageContent {
        match self {
            InputMessageContent::Text(ref mut markup) => {
                markup.disable_web_page_preview = Some(disable_web_page_preview);
            }

            _ => {}
        }
        self
    }

    pub fn foursquare_id(mut self, foursquare_id: String) -> InputMessageContent {
        match self {
            InputMessageContent::Venue(ref mut markup) => {
                markup.foursquare_id = Some(foursquare_id);
            }

            _ => {}
        }
        self
    }

    pub fn last_name(mut self, last_name: String) -> InputMessageContent {
        match self {
            InputMessageContent::Contact(ref mut markup) => {
                markup.last_name = Some(last_name);
            }

            _ => {}
        }
        self
    }
}

impl InlineQueryResult {
    pub fn new_article(id: String,
                       title: String,
                       input_message_content: InputMessageContent)
                       -> InlineQueryResult {
        InlineQueryResultArticle {
                type_name: "article".into(),
                id: id,
                title: title,
                input_message_content: input_message_content,
                // InlineKeyboardMarkup
                reply_markup: None,
                url: None,
                hide_url: None,
                description: None,
                thumb_url: None,
                thumb_width: None,
                thumb_height: None,
            }
            .into()
    }

    pub fn new_photo(id: String, photo_url: String, thumb_url: String) -> InlineQueryResult {
        InlineQueryResultMetaPhoto::from(InlineQueryResultPhoto {
                                             type_name: "photo".into(),
                                             id: id,
                                             photo_url: photo_url,
                                             thumb_url: thumb_url,
                                             photo_width: None,
                                             photo_height: None,
                                             title: None,
                                             description: None,
                                             caption: None,
                                             // InlineKeyboardMarkup
                                             reply_markup: None,
                                             input_message_content: None,
                                         })
                .into()
    }

    pub fn new_gif(id: String, gif_url: String, thumb_url: String) -> InlineQueryResult {
        InlineQueryResultMetaGif::Fresh(InlineQueryResultGif {
                                            type_name: "gif".into(),
                                            id: id,
                                            gif_url: gif_url,
                                            gif_width: None,
                                            gif_height: None,
                                            thumb_url: thumb_url,
                                            title: None,
                                            caption: None,
                                            // InlineKeyboardMarkup
                                            reply_markup: None,
                                            input_message_content: None,
                                        })
                .into()
    }

    pub fn new_mpeg4_gif(id: String, mpeg4_url: String, thumb_url: String) -> InlineQueryResult {
        InlineQueryResultMetaMpeg4Gif::Fresh(InlineQueryResultMpeg4Gif {
                                                 type_name: "mpeg4_gif".into(),
                                                 id: id,
                                                 mpeg4_url: mpeg4_url,
                                                 mpeg4_width: None,
                                                 mpeg4_height: None,
                                                 thumb_url: thumb_url,
                                                 title: None,
                                                 caption: None,
                                                 // InlineKeyboardMarkup
                                                 reply_markup: None,
                                                 input_message_content: None,
                                             })
                .into()
    }

    pub fn new_video(id: String,
                     video_url: String,
                     mime_type: String,
                     thumb_url: String,
                     title: String)
                     -> InlineQueryResult {
        InlineQueryResultMetaVideo::Fresh(InlineQueryResultVideo {
                                              type_name: "video".into(),
                                              id: id,
                                              video_url: video_url,
                                              mime_type: mime_type,
                                              thumb_url: thumb_url,
                                              title: title,
                                              caption: None,
                                              video_width: None,
                                              video_height: None,
                                              video_duration: None,
                                              description: None,
                                              // InlineKeyboardMarkup
                                              reply_markup: None,
                                              input_message_content: None,
                                          })
                .into()
    }

    pub fn new_audio(id: String, audio_url: String, title: String) -> InlineQueryResult {
        InlineQueryResultMetaAudio::Fresh(InlineQueryResultAudio {
                                              type_name: "audio".into(),
                                              id: id,
                                              audio_url: audio_url,
                                              title: title,
                                              caption: None,
                                              performer: None,
                                              audio_duration: None,
                                              // InlineKeyboardMarkup
                                              reply_markup: None,
                                              input_message_content: None,
                                          })
                .into()
    }

    pub fn new_voice(id: String, voice_url: String, title: String) -> InlineQueryResult {
        InlineQueryResultMetaVoice::Fresh(InlineQueryResultVoice {
                                              type_name: "voice".into(),
                                              id: id,
                                              voice_url: voice_url,
                                              title: title,
                                              caption: None,
                                              voice_duration: None,
                                              // InlineKeyboardMarkup
                                              reply_markup: None,
                                              input_message_content: None,
                                          })
                .into()
    }

    pub fn new_document(id: String,
                        title: String,
                        document_url: String,
                        mime_type: String)
                        -> InlineQueryResult {
        InlineQueryResultMetaDocument::Fresh(InlineQueryResultDocument {
                                                 type_name: "document".into(),
                                                 id: id,
                                                 title: title,
                                                 caption: None,
                                                 document_url: document_url,
                                                 mime_type: mime_type,
                                                 description: None,
                                                 // InlineKeyboardMarkup
                                                 reply_markup: None,
                                                 input_message_content: None,
                                                 thumb_url: None,
                                                 thumb_width: None,
                                                 thumb_height: None,
                                             })
                .into()
    }

    pub fn new_location(id: String,
                        latitude: f64,
                        longitude: f64,
                        title: String)
                        -> InlineQueryResult {
        InlineQueryResultLocation {
                type_name: "location".into(),
                id: id,
                latitude: latitude,
                longitude: longitude,
                title: title,
                // InlineKeyboardMarkup
                reply_markup: None,
                input_message_content: None,
                thumb_url: None,
                thumb_width: None,
                thumb_height: None,
            }
            .into()
    }

    pub fn new_venue(id: String,
                     latitude: f64,
                     longitude: f64,
                     title: String,
                     address: String)
                     -> InlineQueryResult {
        InlineQueryResultVenue {
                type_name: "venue".into(),
                id: id,
                latitude: latitude,
                longitude: longitude,
                title: title,
                address: address,
                foursquare_id: None,
                // InlineKeyboardMarkup
                reply_markup: None,
                input_message_content: None,
                thumb_url: None,
                thumb_width: None,
                thumb_height: None,
            }
            .into()
    }

    pub fn new_contact(id: String, phone_number: String, first_name: String) -> InlineQueryResult {
        InlineQueryResultContact {
                type_name: "contact".into(),
                id: id,
                phone_number: phone_number,
                first_name: first_name,
                last_name: None,
                // InlineKeyboardMarkup
                reply_markup: None,
                input_message_content: None,
                thumb_url: None,
                thumb_width: None,
                thumb_height: None,
            }
            .into()
    }

    pub fn new_game(id: String, game_short_name: String) -> InlineQueryResult {
        InlineQueryResultGame {
                type_name: "game".into(),
                id: id,
                game_short_name: game_short_name,
                // InlineKeyboardMarkup
                reply_markup: None,
            }
            .into()
    }

    pub fn new_cached_photo(id: String, photo_file_id: String) -> InlineQueryResult {
        InlineQueryResultMetaPhoto::Cached(InlineQueryResultCachedPhoto {
                                               type_name: "photo".into(),
                                               id: id,
                                               photo_file_id: photo_file_id,
                                               title: None,
                                               description: None,
                                               caption: None,
                                               // InlineKeyboardMarkup
                                               reply_markup: None,
                                               input_message_content: None,
                                           })
                .into()
    }

    pub fn new_cached_gif(id: String, gif_file_id: String) -> InlineQueryResult {
        InlineQueryResultMetaGif::Cached(InlineQueryResultCachedGif {
                                             type_name: "gif".into(),
                                             id: id,
                                             gif_file_id: gif_file_id,
                                             title: None,
                                             caption: None,
                                             // InlineKeyboardMarkup
                                             reply_markup: None,
                                             input_message_content: None,
                                         })
                .into()
    }

    pub fn new_cached_mpeg4_gif(id: String, mpeg4_file_id: String) -> InlineQueryResult {
        InlineQueryResultMetaMpeg4Gif::Cached(InlineQueryResultCachedMpeg4Gif {
                                                  type_name: "mpeg4_gif".into(),
                                                  id: id,
                                                  mpeg4_file_id: mpeg4_file_id,
                                                  title: None,
                                                  caption: None,
                                                  // InlineKeyboardMarkup
                                                  reply_markup: None,
                                                  input_message_content: None,
                                              })
                .into()
    }

    pub fn new_cached_sticker(id: String, sticker_file_id: String) -> InlineQueryResult {
        InlineQueryResultCachedSticker {
                type_name: "sticker".into(),
                id: id,
                sticker_file_id: sticker_file_id,
                // InlineKeyboardMarkup
                reply_markup: None,
                input_message_content: None,
            }
            .into()
    }

    pub fn new_cached_document(id: String,
                               title: String,
                               document_file_id: String)
                               -> InlineQueryResult {
        InlineQueryResultMetaDocument::Cached(InlineQueryResultCachedDocument {
                                                  type_name: "document".into(),
                                                  id: id,
                                                  title: title,
                                                  document_file_id: document_file_id,
                                                  description: None,
                                                  caption: None,
                                                  // InlineKeyboardMarkup
                                                  reply_markup: None,
                                                  input_message_content: None,
                                              })
                .into()
    }

    pub fn new_cached_video(id: String, video_file_id: String, title: String) -> InlineQueryResult {
        InlineQueryResultMetaVideo::Cached(InlineQueryResultCachedVideo {
                                               type_name: "video".into(),
                                               id: id,
                                               video_file_id: video_file_id,
                                               title: title,
                                               description: None,
                                               caption: None,
                                               // InlineKeyboardMarkup
                                               reply_markup: None,
                                               input_message_content: None,
                                           })
                .into()
    }

    pub fn new_cached_voice(id: String, voice_file_id: String, title: String) -> InlineQueryResult {
        InlineQueryResultMetaVoice::Cached(InlineQueryResultCachedVoice {
                                               type_name: "voice".into(),
                                               id: id,
                                               voice_file_id: voice_file_id,
                                               title: title,
                                               caption: None,
                                               // InlineKeyboardMarkup
                                               reply_markup: None,
                                               input_message_content: None,
                                           })
                .into()
    }

    pub fn new_cached_audio(id: String, audio_file_id: String) -> InlineQueryResult {
        InlineQueryResultMetaAudio::Cached(InlineQueryResultCachedAudio {
                                               type_name: "audio".into(),
                                               id: id,
                                               audio_file_id: audio_file_id,
                                               caption: None,
                                               // InlineKeyboardMarkup
                                               reply_markup: None,
                                               input_message_content: None,
                                           })
                .into()
    }

    pub fn reply_markup(mut self, reply_markup: ReplyMarkup) -> InlineQueryResult {
        match self {
            InlineQueryResult::Article(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::Photo(ref mut result) => {
                match *result {
                    InlineQueryResultMetaPhoto::Fresh(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                    InlineQueryResultMetaPhoto::Cached(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                }
            }

            InlineQueryResult::Gif(ref mut result) => {
                match *result {
                    InlineQueryResultMetaGif::Fresh(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                    InlineQueryResultMetaGif::Cached(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                }
            }

            InlineQueryResult::Mpeg4Gif(ref mut result) => {
                match *result {
                    InlineQueryResultMetaMpeg4Gif::Fresh(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                    InlineQueryResultMetaMpeg4Gif::Cached(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                }
            }

            InlineQueryResult::Video(ref mut result) => {
                match *result {
                    InlineQueryResultMetaVideo::Fresh(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                    InlineQueryResultMetaVideo::Cached(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                }
            }

            InlineQueryResult::Audio(ref mut result) => {
                match *result {
                    InlineQueryResultMetaAudio::Fresh(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                    InlineQueryResultMetaAudio::Cached(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                }
            }

            InlineQueryResult::Voice(ref mut result) => {
                match *result {
                    InlineQueryResultMetaVoice::Fresh(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                    InlineQueryResultMetaVoice::Cached(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                }
            }

            InlineQueryResult::Document(ref mut result) => {
                match *result {
                    InlineQueryResultMetaDocument::Fresh(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                    InlineQueryResultMetaDocument::Cached(ref mut result) => {
                        result.reply_markup = Some(reply_markup);
                    }
                }
            }

            InlineQueryResult::Location(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::Venue(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::Contact(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::Game(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::CachedSticker(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }
        }
        self
    }

    pub fn url(mut self, url: String) -> InlineQueryResult {
        match self {
            InlineQueryResult::Article(ref mut result) => {
                result.url = Some(url);
            }

            _ => {}
        }
        self
    }

    pub fn hide_url(mut self, hide_url: bool) -> InlineQueryResult {
        match self {
            InlineQueryResult::Article(ref mut result) => {
                result.hide_url = Some(hide_url);
            }

            _ => {}
        }
        self
    }

    pub fn description(mut self, description: String) -> InlineQueryResult {
        match self {
            InlineQueryResult::Article(ref mut result) => {
                result.description = Some(description);
            }

            InlineQueryResult::Photo(ref mut result) => {
                match *result {
                    InlineQueryResultMetaPhoto::Fresh(ref mut result) => {
                        result.description = Some(description);
                    }
                    InlineQueryResultMetaPhoto::Cached(ref mut result) => {
                        result.description = Some(description);
                    }
                }
            }

            InlineQueryResult::Video(ref mut result) => {
                match *result {
                    InlineQueryResultMetaVideo::Fresh(ref mut result) => {
                        result.description = Some(description);
                    }
                    InlineQueryResultMetaVideo::Cached(ref mut result) => {
                        result.description = Some(description);
                    }
                }
            }

            InlineQueryResult::Document(ref mut result) => {
                match *result {
                    InlineQueryResultMetaDocument::Fresh(ref mut result) => {
                        result.description = Some(description);
                    }
                    InlineQueryResultMetaDocument::Cached(ref mut result) => {
                        result.description = Some(description);
                    }
                }
            }

            _ => {}
        }
        self
    }

    pub fn thumb_url(mut self, thumb_url: String) -> InlineQueryResult {
        match self {
            InlineQueryResult::Article(ref mut result) => result.thumb_url = Some(thumb_url),

            InlineQueryResult::Document(InlineQueryResultMetaDocument::Fresh(ref mut result)) => {
                result.thumb_url = Some(thumb_url);
            }

            InlineQueryResult::Location(ref mut result) => result.thumb_url = Some(thumb_url),

            InlineQueryResult::Venue(ref mut result) => result.thumb_url = Some(thumb_url),

            InlineQueryResult::Contact(ref mut result) => result.thumb_url = Some(thumb_url),

            _ => {}
        }
        self
    }

    pub fn thumb_width(mut self, thumb_width: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Article(ref mut result) => result.thumb_width = Some(thumb_width),

            InlineQueryResult::Document(InlineQueryResultMetaDocument::Fresh(ref mut result)) => {
                result.thumb_width = Some(thumb_width);
            }

            InlineQueryResult::Location(ref mut result) => result.thumb_width = Some(thumb_width),

            InlineQueryResult::Venue(ref mut result) => result.thumb_width = Some(thumb_width),

            InlineQueryResult::Contact(ref mut result) => result.thumb_width = Some(thumb_width),

            _ => {}
        }
        self
    }

    pub fn thumb_height(mut self, thumb_height: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Article(ref mut result) => result.thumb_height = Some(thumb_height),

            InlineQueryResult::Document(InlineQueryResultMetaDocument::Fresh(ref mut result)) => {
                result.thumb_height = Some(thumb_height);
            }

            InlineQueryResult::Location(ref mut result) => result.thumb_height = Some(thumb_height),

            InlineQueryResult::Venue(ref mut result) => result.thumb_height = Some(thumb_height),

            InlineQueryResult::Contact(ref mut result) => result.thumb_height = Some(thumb_height),

            _ => {}
        }
        self
    }

    pub fn photo_width(mut self, photo_width: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Photo(InlineQueryResultMetaPhoto::Fresh(ref mut result)) => {
                result.photo_width = Some(photo_width);
            }

            _ => {}
        }
        self
    }

    pub fn photo_height(mut self, photo_height: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Photo(InlineQueryResultMetaPhoto::Fresh(ref mut result)) => {
                result.photo_height = Some(photo_height);
            }

            _ => {}
        }
        self
    }

    pub fn gif_width(mut self, gif_width: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Gif(InlineQueryResultMetaGif::Fresh(ref mut result)) => {
                result.gif_width = Some(gif_width);
            }

            _ => {}
        }
        self
    }

    pub fn gif_height(mut self, gif_height: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Gif(InlineQueryResultMetaGif::Fresh(ref mut result)) => {
                result.gif_height = Some(gif_height);
            }

            _ => {}
        }
        self
    }

    pub fn mpeg4_width(mut self, mpeg4_width: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Mpeg4Gif(InlineQueryResultMetaMpeg4Gif::Fresh(ref mut result)) => {
                result.mpeg4_width = Some(mpeg4_width);
            }

            _ => {}
        }
        self
    }

    pub fn mpeg4_height(mut self, mpeg4_height: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Mpeg4Gif(InlineQueryResultMetaMpeg4Gif::Fresh(ref mut result)) => {
                result.mpeg4_height = Some(mpeg4_height);
            }

            _ => {}
        }
        self
    }

    pub fn video_width(mut self, video_width: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Video(InlineQueryResultMetaVideo::Fresh(ref mut result)) => {
                result.video_width = Some(video_width);
            }

            _ => {}
        }
        self
    }

    pub fn video_height(mut self, video_height: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Video(InlineQueryResultMetaVideo::Fresh(ref mut result)) => {
                result.video_height = Some(video_height);
            }

            _ => {}
        }
        self
    }

    pub fn video_duration(mut self, video_duration: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Video(InlineQueryResultMetaVideo::Fresh(ref mut result)) => {
                result.video_duration = Some(video_duration);
            }

            _ => {}
        }
        self
    }

    pub fn audio_duration(mut self, audio_duration: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Audio(InlineQueryResultMetaAudio::Fresh(ref mut result)) => {
                result.audio_duration = Some(audio_duration);
            }

            _ => {}
        }
        self
    }

    pub fn performer(mut self, performer: String) -> InlineQueryResult {
        match self {
            InlineQueryResult::Audio(InlineQueryResultMetaAudio::Fresh(ref mut result)) => {
                result.performer = Some(performer);
            }

            _ => {}
        }
        self
    }

    pub fn voice_duration(mut self, voice_duration: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Voice(InlineQueryResultMetaVoice::Fresh(ref mut result)) => {
                result.voice_duration = Some(voice_duration);
            }

            _ => {}
        }
        self
    }

    pub fn title(mut self, title: String) -> InlineQueryResult {
        match self {
            InlineQueryResult::Photo(ref mut result) => {
                match *result {
                    InlineQueryResultMetaPhoto::Fresh(ref mut result) => {
                        result.title = Some(title);
                    }
                    InlineQueryResultMetaPhoto::Cached(ref mut result) => {
                        result.title = Some(title);
                    }
                }
            }

            InlineQueryResult::Gif(ref mut result) => {
                match *result {
                    InlineQueryResultMetaGif::Fresh(ref mut result) => {
                        result.title = Some(title);
                    }
                    InlineQueryResultMetaGif::Cached(ref mut result) => {
                        result.title = Some(title);
                    }
                }
            }

            InlineQueryResult::Mpeg4Gif(ref mut result) => {
                match *result {
                    InlineQueryResultMetaMpeg4Gif::Fresh(ref mut result) => {
                        result.title = Some(title);
                    }
                    InlineQueryResultMetaMpeg4Gif::Cached(ref mut result) => {
                        result.title = Some(title);
                    }
                }
            }

            _ => {}
        }
        self
    }

    pub fn foursquare_id(mut self, foursquare_id: String) -> InlineQueryResult {
        match self {
            InlineQueryResult::Venue(ref mut result) => {
                result.foursquare_id = Some(foursquare_id);
            }

            _ => {}
        }
        self
    }

    pub fn last_name(mut self, last_name: String) -> InlineQueryResult {
        match self {
            InlineQueryResult::Contact(ref mut result) => {
                result.last_name = Some(last_name);
            }

            _ => {}
        }
        self
    }

    pub fn caption(mut self, caption: String) -> InlineQueryResult {
        match self {
            InlineQueryResult::Photo(ref mut result) => {
                match *result {
                    InlineQueryResultMetaPhoto::Fresh(ref mut result) => {
                        result.caption = Some(caption);
                    }
                    InlineQueryResultMetaPhoto::Cached(ref mut result) => {
                        result.caption = Some(caption);
                    }
                }
            }

            InlineQueryResult::Gif(ref mut result) => {
                match *result {
                    InlineQueryResultMetaGif::Fresh(ref mut result) => {
                        result.caption = Some(caption);
                    }
                    InlineQueryResultMetaGif::Cached(ref mut result) => {
                        result.caption = Some(caption);
                    }
                }
            }

            InlineQueryResult::Mpeg4Gif(ref mut result) => {
                match *result {
                    InlineQueryResultMetaMpeg4Gif::Fresh(ref mut result) => {
                        result.caption = Some(caption);
                    }
                    InlineQueryResultMetaMpeg4Gif::Cached(ref mut result) => {
                        result.caption = Some(caption);
                    }
                }
            }

            InlineQueryResult::Video(ref mut result) => {
                match *result {
                    InlineQueryResultMetaVideo::Fresh(ref mut result) => {
                        result.caption = Some(caption);
                    }
                    InlineQueryResultMetaVideo::Cached(ref mut result) => {
                        result.caption = Some(caption);
                    }
                }
            }

            InlineQueryResult::Audio(ref mut result) => {
                match *result {
                    InlineQueryResultMetaAudio::Fresh(ref mut result) => {
                        result.caption = Some(caption);
                    }
                    InlineQueryResultMetaAudio::Cached(ref mut result) => {
                        result.caption = Some(caption);
                    }
                }
            }

            InlineQueryResult::Voice(ref mut result) => {
                match *result {
                    InlineQueryResultMetaVoice::Fresh(ref mut result) => {
                        result.caption = Some(caption);
                    }
                    InlineQueryResultMetaVoice::Cached(ref mut result) => {
                        result.caption = Some(caption);
                    }
                }
            }

            InlineQueryResult::Document(ref mut result) => {
                match *result {
                    InlineQueryResultMetaDocument::Fresh(ref mut result) => {
                        result.caption = Some(caption);
                    }
                    InlineQueryResultMetaDocument::Cached(ref mut result) => {
                        result.caption = Some(caption);
                    }
                }
            }

            _ => {}
        }
        self
    }

    pub fn input_message_content(mut self,
                                 input_message_content: types::InputMessageContent)
                                 -> InlineQueryResult {
        match self {
            InlineQueryResult::Photo(ref mut result) => {
                match *result {
                    InlineQueryResultMetaPhoto::Fresh(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                    InlineQueryResultMetaPhoto::Cached(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                }
            }

            InlineQueryResult::Gif(ref mut result) => {
                match *result {
                    InlineQueryResultMetaGif::Fresh(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                    InlineQueryResultMetaGif::Cached(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                }
            }

            InlineQueryResult::Mpeg4Gif(ref mut result) => {
                match *result {
                    InlineQueryResultMetaMpeg4Gif::Fresh(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                    InlineQueryResultMetaMpeg4Gif::Cached(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                }
            }

            InlineQueryResult::Video(ref mut result) => {
                match *result {
                    InlineQueryResultMetaVideo::Fresh(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                    InlineQueryResultMetaVideo::Cached(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                }
            }

            InlineQueryResult::Audio(ref mut result) => {
                match *result {
                    InlineQueryResultMetaAudio::Fresh(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                    InlineQueryResultMetaAudio::Cached(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                }
            }

            InlineQueryResult::Voice(ref mut result) => {
                match *result {
                    InlineQueryResultMetaVoice::Fresh(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                    InlineQueryResultMetaVoice::Cached(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                }
            }

            InlineQueryResult::Document(ref mut result) => {
                match *result {
                    InlineQueryResultMetaDocument::Fresh(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                    InlineQueryResultMetaDocument::Cached(ref mut result) => {
                        result.input_message_content = Some(input_message_content);
                    }
                }
            }

            InlineQueryResult::Location(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::Venue(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::Contact(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::CachedSticker(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            _ => {}
        }
        self
    }
}
