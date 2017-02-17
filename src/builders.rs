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
use types::InlineQueryResultPhoto;
use types::InlineQueryResultGif;
use types::InlineQueryResultMpeg4Gif;
use types::InlineQueryResultVideo;
use types::InlineQueryResultAudio;
use types::InlineQueryResultVoice;
use types::InlineQueryResultDocument;
use types::InlineQueryResultLocation;
use types::InlineQueryResultVenue;
use types::InlineQueryResultContact;
use types::InlineQueryResultGame;
use types::InlineQueryResultCachedPhoto;
use types::InlineQueryResultCachedGif;
use types::InlineQueryResultCachedMpeg4Gif;
use types::InlineQueryResultCachedSticker;
use types::InlineQueryResultCachedDocument;
use types::InlineQueryResultCachedVideo;
use types::InlineQueryResultCachedVoice;
use types::InlineQueryResultCachedAudio;

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
    pub fn new() -> GetUpdates<'a> {
        GetUpdates {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }

    buildera!(GetUpdates;
              offset, i64;
              limit, i64;
              timeout, i64;
              allowed_updates, &'a [&'a str]);
}

impl<'a> SetWebhook<'a> {
    pub fn new(url: &'a str) -> SetWebhook<'a> {
        SetWebhook {
            url: url,
            certificate: None,
            max_connections: None,
            allowed_updates: None,
        }
    }

    buildera!(SetWebhook;
              certificate, &'a str;
              max_connections, i64;
              allowed_updates, &'a [&'a str]);
}

impl<'a> SendMessage<'a> {
    pub fn new(text: &'a str) -> SendMessage<'a> {
        SendMessage {
            chat_id: None,
            chat_username: None,
            text: text,
            parse_mode: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    buildera!(SendMessage;
              chat_id, i64;
              chat_username, &'a str;
              parse_mode, &'a str;
              disable_web_page_preview, bool;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> ForwardMessage<'a> {
    pub fn new(message_id: i64) -> ForwardMessage<'a> {
        ForwardMessage {
            chat_id: None,
            chat_username: None,
            from_chat_id: None,
            from_chat_username: None,
            disable_notification: None,
            message_id: message_id,
        }
    }

    buildera!(ForwardMessage;
              chat_id, i64;
              chat_username, &'a str;
              from_chat_id, i64;
              from_chat_username, &'a str;
              disable_notification, bool);
}

impl<'a> SendPhoto<'a> {
    pub fn new() -> SendPhoto<'a> {
        SendPhoto {
            chat_id: None,
            chat_username: None,
            photo: None,
            file_id: None,
            caption: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    buildera!(SendPhoto;
              chat_id, i64;
              chat_username, &'a str;
              photo, &'a str;
              file_id, &'a str;
              caption, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendAudio<'a> {
    pub fn new() -> SendAudio<'a> {
        SendAudio {
            chat_id: None,
            chat_username: None,
            audio: None,
            file_id: None,
            caption: None,
            duration: None,
            performer: None,
            title: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    buildera!(SendAudio;
              chat_id, i64;
              chat_username, &'a str;
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
    pub fn new() -> SendDocument<'a> {
        SendDocument {
            chat_id: None,
            chat_username: None,
            document: None,
            file_id: None,
            caption: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    buildera!(SendDocument;
              chat_id, i64;
              chat_username, &'a str;
              document, &'a str;
              file_id, &'a str;
              caption, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendSticker<'a> {
    pub fn new() -> SendSticker<'a> {
        SendSticker {
            chat_id: None,
            chat_username: None,
            sticker: None,
            file_id: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    buildera!(SendSticker;
              chat_id, i64;
              chat_username, &'a str;
              sticker, &'a str;
              file_id, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendVideo<'a> {
    pub fn new() -> SendVideo<'a> {
        SendVideo {
            chat_id: None,
            chat_username: None,
            video: None,
            file_id: None,
            duration: None,
            width: None,
            height: None,
            caption: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    buildera!(SendVideo;
              chat_id, i64;
              chat_username, &'a str;
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
    pub fn new() -> SendVoice<'a> {
        SendVoice {
            chat_id: None,
            chat_username: None,
            voice: None,
            file_id: None,
            caption: None,
            duration: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    buildera!(SendVoice;
              chat_id, i64;
              chat_username, &'a str;
              voice, &'a str;
              file_id, &'a str;
              caption, &'a str;
              duration, i64;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendLocation<'a> {
    pub fn new(latitude: f64, longitude: f64) -> SendLocation<'a> {
        SendLocation {
            chat_id: None,
            chat_username: None,
            latitude: latitude,
            longitude: longitude,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    buildera!(SendLocation;
              chat_id, i64;
              chat_username, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendVenue<'a> {
    pub fn new(latitude: f64, longitude: f64, title: &'a str, address: &'a str) -> SendVenue<'a> {
        SendVenue {
            chat_id: None,
            chat_username: None,
            latitude: latitude,
            longitude: longitude,
            title: title,
            address: address,
            foursquare_id: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    buildera!(SendVenue;
              chat_id, i64;
              chat_username, &'a str;
              foursquare_id, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl<'a> SendContact<'a> {
    pub fn new(phone_number: &'a str, first_name: &'a str) -> SendContact<'a> {
        SendContact {
            chat_id: None,
            chat_username: None,
            phone_number: phone_number,
            first_name: first_name,
            last_name: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    buildera!(SendContact;
              chat_id, i64;
              chat_username, &'a str;
              last_name, &'a str;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>);
}

impl GetUserProfilePhotos {
    pub fn new(user_id: i64) -> GetUserProfilePhotos {
        GetUserProfilePhotos {
            user_id: user_id,
            offset: None,
            limit: None,
        }
    }

    builder!(GetUserProfilePhotos;
              offset, i64;
              limit, i64);
}

impl<'a> GetFile<'a> {
    pub fn new(file_id: &'a str) -> GetFile<'a> {
        GetFile { file_id: file_id }
    }
}

impl<'a> KickChatMember<'a> {
    pub fn new(user_id: i64) -> KickChatMember<'a> {
        KickChatMember {
            chat_id: None,
            chat_username: None,
            user_id: user_id,
        }
    }

    buildera!(KickChatMember;
              chat_id, i64;
              chat_username, &'a str);
}

impl<'a> LeaveChat<'a> {
    pub fn new() -> LeaveChat<'a> {
        LeaveChat {
            chat_id: None,
            chat_username: None,
        }
    }

    buildera!(LeaveChat;
              chat_id, i64;
              chat_username, &'a str);
}

impl<'a> UnbanChatMember<'a> {
    pub fn new(user_id: i64) -> UnbanChatMember<'a> {
        UnbanChatMember {
            chat_id: None,
            chat_username: None,
            user_id: user_id,
        }
    }

    buildera!(UnbanChatMember;
              chat_id, i64;
              chat_username, &'a str);
}

impl<'a> GetChat<'a> {
    pub fn new() -> GetChat<'a> {
        GetChat {
            chat_id: None,
            chat_username: None,
        }
    }

    buildera!(GetChat;
              chat_id, i64;
              chat_username, &'a str);
}

impl<'a> GetChatAdministrators<'a> {
    pub fn new() -> GetChatAdministrators<'a> {
        GetChatAdministrators {
            chat_id: None,
            chat_username: None,
        }
    }

    buildera!(GetChatAdministrators;
              chat_id, i64;
              chat_username, &'a str);
}

impl<'a> GetChatMembersCount<'a> {
    pub fn new() -> GetChatMembersCount<'a> {
        GetChatMembersCount {
            chat_id: None,
            chat_username: None,
        }
    }

    buildera!(GetChatMembersCount;
              chat_id, i64;
              chat_username, &'a str);
}

impl<'a> GetChatMember<'a> {
    pub fn new(user_id: i64) -> GetChatMember<'a> {
        GetChatMember {
            chat_id: None,
            chat_username: None,
            user_id: user_id,
        }
    }

    buildera!(GetChatMember;
              chat_id, i64;
              chat_username, &'a str);
}

impl<'a> AnswerCallbackQuery<'a> {
    pub fn new(callback_query_id: &'a str) -> AnswerCallbackQuery<'a> {
        AnswerCallbackQuery {
            callback_query_id: callback_query_id,
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }

    buildera!(AnswerCallbackQuery;
              text, &'a str;
              show_alert, bool;
              url, &'a str;
              cache_time, i64);
}

impl<'a> EditMessageText<'a> {
    pub fn new(text: &'a str) -> EditMessageText<'a> {
        EditMessageText {
            chat_id: None,
            chat_username: None,
            message_id: None,
            inline_message_id: None,
            text: text,
            parse_mode: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    buildera!(EditMessageText;
              chat_id, i64;
              chat_username, &'a str;
              message_id, i64;
              inline_message_id, &'a str;
              parse_mode, &'a str;
              disable_web_page_preview, bool;
              reply_markup, Box<ReplyMarkup>); // InlineKeyboardMarkup
}

impl<'a> EditMessageCaption<'a> {
    pub fn new() -> EditMessageCaption<'a> {
        EditMessageCaption {
            chat_id: None,
            chat_username: None,
            message_id: None,
            inline_message_id: None,
            caption: None,
            reply_markup: None,
        }
    }

    buildera!(EditMessageCaption;
              chat_id, i64;
              chat_username, &'a str;
              message_id, i64;
              inline_message_id, &'a str;
              caption, &'a str;
              reply_markup, Box<ReplyMarkup>); // InlineKeyboardMarkup
}

impl<'a> EditMessageReplyMarkup<'a> {
    pub fn new() -> EditMessageReplyMarkup<'a> {
        EditMessageReplyMarkup {
            chat_id: None,
            chat_username: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
        }
    }

    buildera!(EditMessageReplyMarkup;
              chat_id, i64;
              chat_username, &'a str;
              message_id, i64;
              inline_message_id, &'a str;
              reply_markup, Box<ReplyMarkup>); // InlineKeyboardMarkup
}

impl<'a> AnswerInlineQuery<'a> {
    pub fn new(inline_query_id: &'a str,
               results: Vec<types::InlineQueryResult>)
               -> AnswerInlineQuery<'a> {
        AnswerInlineQuery {
            inline_query_id: inline_query_id,
            results: results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }

    buildera!(AnswerInlineQuery;
              cache_time, i64;
              is_personal, bool;
              next_offset, &'a str;
              switch_pm_text, &'a str;
              switch_pm_parameter, &'a str);
}

impl<'a> SendGame<'a> {
    pub fn new(chat_id: i64, game_short_name: &'a str) -> SendGame<'a> {
        SendGame {
            chat_id: chat_id,
            game_short_name: game_short_name,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    buildera!(SendGame;
              disable_notification, bool;
              reply_to_message_id, i64;
              reply_markup, Box<ReplyMarkup>); // InlineKeyboardMarkup
}

impl<'a> SetGameScore<'a> {
    pub fn new(user_id: i64, score: i64) -> SetGameScore<'a> {
        SetGameScore {
            user_id: user_id,
            score: score,
            force: None,
            disable_edit_message: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }

    buildera!(SetGameScore;
              force, bool;
              disable_edit_message, bool;
              chat_id, i64;
              message_id, i64;
              inline_message_id, &'a str);
}

impl<'a> GetGameHighScores<'a> {
    pub fn new(user_id: i64) -> GetGameHighScores<'a> {
        GetGameHighScores {
            user_id: user_id,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }

    buildera!(GetGameHighScores;
              chat_id, i64;
              message_id, i64;
              inline_message_id, &'a str);
}

impl KeyboardButton {
    pub fn new(text: String) -> KeyboardButton {
        KeyboardButton {
            text: text,
            request_contact: None,
            request_location: None,
        }
    }

    builder!(KeyboardButton;
              request_contact, bool;
              request_location, bool);
}

impl InlineKeyboardButton {
    pub fn new(text: String) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text: text,
            url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
        }
    }

    builder!(InlineKeyboardButton;
              url, String;
              callback_data, String;
              switch_inline_query, String;
              switch_inline_query_current_chat, String;
              callback_game, types::CallbackGame);
}

impl ReplyMarkup {
    pub fn new_inline_keyboard(keyboard: Vec<Vec<InlineKeyboardButton>>) -> ReplyMarkup {
        ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup { inline_keyboard: keyboard })
    }

    pub fn new_reply_keyboard(keyboard: Vec<Vec<KeyboardButton>>) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboard(ReplyKeyboardMarkup {
            keyboard: keyboard,
            resize_keyboard: None,
            one_time_keyboard: None,
            selective: None,
        })
    }

    pub fn new_reply_keyboard_remove(remove_keyboard: bool) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboardRemove(ReplyKeyboardRemoveMarkup {
            remove_keyboard: remove_keyboard,
            selective: None,
        })
    }

    pub fn new_force_reply(force_reply: bool) -> ReplyMarkup {
        ReplyMarkup::ForceReply(ForceReplyMarkup {
            force_reply: force_reply,
            selective: None,
        })
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
        InputMessageContent::Text(InputTextMessageContent {
            message_text: message_text,
            parse_mode: None,
            disable_web_page_preview: None,
        })
    }

    pub fn new_location(latitude: f64, longitude: f64) -> InputMessageContent {
        InputMessageContent::Location(InputLocationMessageContent {
            latitude: latitude,
            longitude: longitude,
        })
    }

    pub fn new_venue(latitude: f64,
                     longitude: f64,
                     title: String,
                     address: String)
                     -> InputMessageContent {
        InputMessageContent::Venue(InputVenueMessageContent {
            latitude: latitude,
            longitude: longitude,
            title: title,
            address: address,
            foursquare_id: None,
        })
    }

    pub fn new_contact(phone_number: String, first_name: String) -> InputMessageContent {
        InputMessageContent::Contact(InputContactMessageContent {
            phone_number: phone_number,
            first_name: first_name,
            last_name: None,
        })
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
        InlineQueryResult::Article(InlineQueryResultArticle {
            type_name: "article".into(),
            id: id,
            title: title,
            input_message_content: input_message_content,
            reply_markup: None, // InlineKeyboardMarkup
            url: None,
            hide_url: None,
            description: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        })
    }

    pub fn new_photo(id: String, photo_url: String, thumb_url: String) -> InlineQueryResult {
        InlineQueryResult::Photo(InlineQueryResultPhoto {
            type_name: "photo".into(),
            id: id,
            photo_url: photo_url,
            thumb_url: thumb_url,
            photo_width: None,
            photo_height: None,
            title: None,
            description: None,
            caption: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_gif(id: String, gif_url: String, thumb_url: String) -> InlineQueryResult {
        InlineQueryResult::Gif(InlineQueryResultGif {
            type_name: "gif".into(),
            id: id,
            gif_url: gif_url,
            gif_width: None,
            gif_height: None,
            thumb_url: thumb_url,
            title: None,
            caption: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_mpeg4_gif(id: String, mpeg4_url: String, thumb_url: String) -> InlineQueryResult {
        InlineQueryResult::Mpeg4Gif(InlineQueryResultMpeg4Gif {
            type_name: "mpeg4_gif".into(),
            id: id,
            mpeg4_url: mpeg4_url,
            mpeg4_width: None,
            mpeg4_height: None,
            thumb_url: thumb_url,
            title: None,
            caption: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_video(id: String,
                     video_url: String,
                     mime_type: String,
                     thumb_url: String,
                     title: String)
                     -> InlineQueryResult {
        InlineQueryResult::Video(InlineQueryResultVideo {
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
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_audio(id: String, audio_url: String, title: String) -> InlineQueryResult {
        InlineQueryResult::Audio(InlineQueryResultAudio {
            type_name: "audio".into(),
            id: id,
            audio_url: audio_url,
            title: title,
            caption: None,
            performer: None,
            audio_duration: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_voice(id: String, voice_url: String, title: String) -> InlineQueryResult {
        InlineQueryResult::Voice(InlineQueryResultVoice {
            type_name: "voice".into(),
            id: id,
            voice_url: voice_url,
            title: title,
            caption: None,
            voice_duration: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_document(id: String,
                        title: String,
                        document_url: String,
                        mime_type: String)
                        -> InlineQueryResult {
        InlineQueryResult::Document(InlineQueryResultDocument {
            type_name: "document".into(),
            id: id,
            title: title,
            caption: None,
            document_url: document_url,
            mime_type: mime_type,
            description: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        })
    }

    pub fn new_location(id: String,
                        latitude: f64,
                        longitude: f64,
                        title: String)
                        -> InlineQueryResult {
        InlineQueryResult::Location(InlineQueryResultLocation {
            type_name: "location".into(),
            id: id,
            latitude: latitude,
            longitude: longitude,
            title: title,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        })
    }

    pub fn new_venue(id: String,
                     latitude: f64,
                     longitude: f64,
                     title: String,
                     address: String)
                     -> InlineQueryResult {
        InlineQueryResult::Venue(InlineQueryResultVenue {
            type_name: "venue".into(),
            id: id,
            latitude: latitude,
            longitude: longitude,
            title: title,
            address: address,
            foursquare_id: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        })
    }

    pub fn new_contact(id: String, phone_number: String, first_name: String) -> InlineQueryResult {
        InlineQueryResult::Contact(InlineQueryResultContact {
            type_name: "contact".into(),
            id: id,
            phone_number: phone_number,
            first_name: first_name,
            last_name: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        })
    }

    pub fn new_game(id: String, game_short_name: String) -> InlineQueryResult {
        InlineQueryResult::Game(InlineQueryResultGame {
            type_name: "game".into(),
            id: id,
            game_short_name: game_short_name,
            reply_markup: None, // InlineKeyboardMarkup
        })
    }

    pub fn new_cached_photo(id: String, photo_file_id: String) -> InlineQueryResult {
        InlineQueryResult::CachedPhoto(InlineQueryResultCachedPhoto {
            type_name: "photo".into(),
            id: id,
            photo_file_id: photo_file_id,
            title: None,
            description: None,
            caption: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_cached_gif(id: String, gif_file_id: String) -> InlineQueryResult {
        InlineQueryResult::CachedGif(InlineQueryResultCachedGif {
            type_name: "gif".into(),
            id: id,
            gif_file_id: gif_file_id,
            title: None,
            caption: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_cached_mpeg4_gif(id: String, mpeg4_file_id: String) -> InlineQueryResult {
        InlineQueryResult::CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif {
            type_name: "mpeg4_gif".into(),
            id: id,
            mpeg4_file_id: mpeg4_file_id,
            title: None,
            caption: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_cached_sticker(id: String, sticker_file_id: String) -> InlineQueryResult {
        InlineQueryResult::CachedSticker(InlineQueryResultCachedSticker {
            type_name: "sticker".into(),
            id: id,
            sticker_file_id: sticker_file_id,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_cached_document(id: String,
                               title: String,
                               document_file_id: String)
                               -> InlineQueryResult {
        InlineQueryResult::CachedDocument(InlineQueryResultCachedDocument {
            type_name: "document".into(),
            id: id,
            title: title,
            document_file_id: document_file_id,
            description: None,
            caption: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_cached_video(id: String, video_file_id: String, title: String) -> InlineQueryResult {
        InlineQueryResult::CachedVideo(InlineQueryResultCachedVideo {
            type_name: "video".into(),
            id: id,
            video_file_id: video_file_id,
            title: title,
            description: None,
            caption: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_cached_voice(id: String, voice_file_id: String, title: String) -> InlineQueryResult {
        InlineQueryResult::CachedVoice(InlineQueryResultCachedVoice {
            type_name: "voice".into(),
            id: id,
            voice_file_id: voice_file_id,
            title: title,
            caption: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn new_cached_audio(id: String, audio_file_id: String) -> InlineQueryResult {
        InlineQueryResult::CachedAudio(InlineQueryResultCachedAudio {
            type_name: "audio".into(),
            id: id,
            audio_file_id: audio_file_id,
            caption: None,
            reply_markup: None, // InlineKeyboardMarkup
            input_message_content: None,
        })
    }

    pub fn reply_markup(mut self, reply_markup: ReplyMarkup) -> InlineQueryResult {
        match self {
            InlineQueryResult::Article(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::Photo(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::Gif(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::Mpeg4Gif(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::Video(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::Audio(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::Voice(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::Document(ref mut result) => {
                result.reply_markup = Some(reply_markup);
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

            InlineQueryResult::CachedPhoto(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::CachedGif(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::CachedMpeg4Gif(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::CachedSticker(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::CachedDocument(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::CachedVideo(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::CachedVoice(ref mut result) => {
                result.reply_markup = Some(reply_markup);
            }

            InlineQueryResult::CachedAudio(ref mut result) => {
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
                result.description = Some(description);
            }

            InlineQueryResult::Video(ref mut result) => {
                result.description = Some(description);
            }

            InlineQueryResult::Document(ref mut result) => {
                result.description = Some(description);
            }

            InlineQueryResult::CachedPhoto(ref mut result) => {
                result.description = Some(description);
            }

            InlineQueryResult::CachedDocument(ref mut result) => {
                result.description = Some(description);
            }

            InlineQueryResult::CachedVideo(ref mut result) => {
                result.description = Some(description);
            }

            _ => {}
        }
        self
    }

    pub fn thumb_url(mut self, thumb_url: String) -> InlineQueryResult {
        match self {
            InlineQueryResult::Article(ref mut result) => result.thumb_url = Some(thumb_url),

            InlineQueryResult::Document(ref mut result) => result.thumb_url = Some(thumb_url),

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

            InlineQueryResult::Document(ref mut result) => result.thumb_width = Some(thumb_width),

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

            InlineQueryResult::Document(ref mut result) => result.thumb_height = Some(thumb_height),

            InlineQueryResult::Location(ref mut result) => result.thumb_height = Some(thumb_height),

            InlineQueryResult::Venue(ref mut result) => result.thumb_height = Some(thumb_height),

            InlineQueryResult::Contact(ref mut result) => result.thumb_height = Some(thumb_height),

            _ => {}
        }
        self
    }

    pub fn photo_width(mut self, photo_width: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Photo(ref mut result) => {
                result.photo_width = Some(photo_width);
            }

            _ => {}
        }
        self
    }

    pub fn photo_height(mut self, photo_height: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Photo(ref mut result) => {
                result.photo_height = Some(photo_height);
            }

            _ => {}
        }
        self
    }

    pub fn gif_width(mut self, gif_width: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Gif(ref mut result) => {
                result.gif_width = Some(gif_width);
            }

            _ => {}
        }
        self
    }

    pub fn gif_height(mut self, gif_height: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Gif(ref mut result) => {
                result.gif_height = Some(gif_height);
            }

            _ => {}
        }
        self
    }

    pub fn mpeg4_width(mut self, mpeg4_width: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Mpeg4Gif(ref mut result) => {
                result.mpeg4_width = Some(mpeg4_width);
            }

            _ => {}
        }
        self
    }

    pub fn mpeg4_height(mut self, mpeg4_height: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Mpeg4Gif(ref mut result) => {
                result.mpeg4_height = Some(mpeg4_height);
            }

            _ => {}
        }
        self
    }

    pub fn video_width(mut self, video_width: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Video(ref mut result) => {
                result.video_width = Some(video_width);
            }

            _ => {}
        }
        self
    }

    pub fn video_height(mut self, video_height: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Video(ref mut result) => {
                result.video_height = Some(video_height);
            }

            _ => {}
        }
        self
    }

    pub fn video_duration(mut self, video_duration: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Video(ref mut result) => {
                result.video_duration = Some(video_duration);
            }

            _ => {}
        }
        self
    }

    pub fn audio_duration(mut self, audio_duration: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Audio(ref mut result) => {
                result.audio_duration = Some(audio_duration);
            }

            _ => {}
        }
        self
    }

    pub fn performer(mut self, performer: String) -> InlineQueryResult {
        match self {
            InlineQueryResult::Audio(ref mut result) => {
                result.performer = Some(performer);
            }

            _ => {}
        }
        self
    }

    pub fn voice_duration(mut self, voice_duration: i64) -> InlineQueryResult {
        match self {
            InlineQueryResult::Voice(ref mut result) => {
                result.voice_duration = Some(voice_duration);
            }

            _ => {}
        }
        self
    }

    pub fn title(mut self, title: String) -> InlineQueryResult {
        match self {
            InlineQueryResult::Photo(ref mut result) => {
                result.title = Some(title);
            }

            InlineQueryResult::Gif(ref mut result) => {
                result.title = Some(title);
            }

            InlineQueryResult::Mpeg4Gif(ref mut result) => {
                result.title = Some(title);
            }

            InlineQueryResult::CachedPhoto(ref mut result) => {
                result.title = Some(title);
            }

            InlineQueryResult::CachedGif(ref mut result) => {
                result.title = Some(title);
            }

            InlineQueryResult::CachedMpeg4Gif(ref mut result) => {
                result.title = Some(title);
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
                result.caption = Some(caption);
            }

            InlineQueryResult::Gif(ref mut result) => {
                result.caption = Some(caption);
            }

            InlineQueryResult::Mpeg4Gif(ref mut result) => {
                result.caption = Some(caption);
            }

            InlineQueryResult::Video(ref mut result) => {
                result.caption = Some(caption);
            }

            InlineQueryResult::Audio(ref mut result) => {
                result.caption = Some(caption);
            }

            InlineQueryResult::Voice(ref mut result) => {
                result.caption = Some(caption);
            }

            InlineQueryResult::Document(ref mut result) => {
                result.caption = Some(caption);
            }

            InlineQueryResult::CachedPhoto(ref mut result) => {
                result.caption = Some(caption);
            }

            InlineQueryResult::CachedGif(ref mut result) => {
                result.caption = Some(caption);
            }

            InlineQueryResult::CachedMpeg4Gif(ref mut result) => {
                result.caption = Some(caption);
            }

            InlineQueryResult::CachedDocument(ref mut result) => {
                result.caption = Some(caption);
            }

            InlineQueryResult::CachedVideo(ref mut result) => {
                result.caption = Some(caption);
            }

            InlineQueryResult::CachedVoice(ref mut result) => {
                result.caption = Some(caption);
            }

            InlineQueryResult::CachedAudio(ref mut result) => {
                result.caption = Some(caption);
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
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::Gif(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::Mpeg4Gif(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::Video(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::Audio(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::Voice(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::Document(ref mut result) => {
                result.input_message_content = Some(input_message_content);
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

            InlineQueryResult::CachedPhoto(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::CachedGif(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::CachedMpeg4Gif(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::CachedSticker(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::CachedDocument(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::CachedVideo(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::CachedVoice(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            InlineQueryResult::CachedAudio(ref mut result) => {
                result.input_message_content = Some(input_message_content);
            }

            _ => {}
        }
        self
    }
}
