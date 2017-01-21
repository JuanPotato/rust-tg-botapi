use args::*;

use types::InlineKeyboardButton;
use types::KeyboardButton;
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

    buildera!(GetUpdates; offset, i64; limit, i64; timeout, i64; allowed_updates, &'a [&'a str]);
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

    buildera!(SetWebhook; certificate, &'a str; max_connections, i64; allowed_updates, &'a [&'a str]);
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

    buildera!(SendMessage; chat_id, i64; chat_username, &'a str; parse_mode, &'a str; disable_web_page_preview, bool; disable_notification, bool; reply_to_message_id, i64; reply_markup, &'a types::ReplyMarkup);
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

    buildera!(ForwardMessage; chat_id, i64; chat_username, &'a str; from_chat_id, i64; from_chat_username, &'a str; disable_notification, bool);
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

    buildera!(SendPhoto; chat_id, i64; chat_username, &'a str; photo, &'a str; file_id, &'a str; caption, &'a str; disable_notification, bool; reply_to_message_id, i64; reply_markup, &'a types::ReplyMarkup);
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

    buildera!(SendAudio; chat_id, i64; chat_username, &'a str; audio, &'a str; file_id, &'a str; caption, &'a str; duration, i64; performer, &'a str; title, &'a str; disable_notification, bool; reply_to_message_id, i64; reply_markup, &'a types::ReplyMarkup);
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

    buildera!(SendDocument; chat_id, i64; chat_username, &'a str; document, &'a str; file_id, &'a str; caption, &'a str; disable_notification, bool; reply_to_message_id, i64; reply_markup, &'a types::ReplyMarkup);
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

    buildera!(SendSticker; chat_id, i64; chat_username, &'a str; sticker, &'a str; file_id, &'a str; disable_notification, bool; reply_to_message_id, i64; reply_markup, &'a types::ReplyMarkup);
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

    buildera!(SendVideo; chat_id, i64; chat_username, &'a str; video, &'a str; file_id, &'a str; duration, i64; width, i64; height, i64; caption, &'a str; disable_notification, bool; reply_to_message_id, i64; reply_markup, &'a types::ReplyMarkup);
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

    buildera!(SendVoice; chat_id, i64; chat_username, &'a str; voice, &'a str; file_id, &'a str; caption, &'a str; duration, i64; disable_notification, bool; reply_to_message_id, i64; reply_markup, &'a types::ReplyMarkup);
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

    buildera!(SendLocation; chat_id, i64; chat_username, &'a str; disable_notification, bool; reply_to_message_id, i64; reply_markup, &'a types::ReplyMarkup);
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

    buildera!(SendVenue; chat_id, i64; chat_username, &'a str; foursquare_id, &'a str; disable_notification, bool; reply_to_message_id, i64; reply_markup, &'a types::ReplyMarkup);
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

    buildera!(SendContact; chat_id, i64; chat_username, &'a str; last_name, &'a str; disable_notification, bool; reply_to_message_id, i64; reply_markup, &'a types::ReplyMarkup);
}

impl GetUserProfilePhotos {
    pub fn new(user_id: i64) -> GetUserProfilePhotos {
        GetUserProfilePhotos {
            user_id: user_id,
            offset: None,
            limit: None,
        }
    }

    builder!(GetUserProfilePhotos; offset, i64; limit, i64);
}

impl<'a> GetFile<'a> {
    pub fn new(file_id: &'a str) -> GetFile<'a> {
        GetFile {
            file_id: file_id,
        }
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

    buildera!(KickChatMember; chat_id, i64; chat_username, &'a str);
}

impl<'a> LeaveChat<'a> {
    pub fn new() -> LeaveChat<'a> {
        LeaveChat {
            chat_id: None,
            chat_username: None,
        }
    }

    buildera!(LeaveChat; chat_id, i64; chat_username, &'a str);
}

impl<'a> UnbanChatMember<'a> {
    pub fn new(user_id: i64) -> UnbanChatMember<'a> {
        UnbanChatMember {
            chat_id: None,
            chat_username: None,
            user_id: user_id,
        }
    }

    buildera!(UnbanChatMember; chat_id, i64; chat_username, &'a str);
}

impl<'a> GetChat<'a> {
    pub fn new() -> GetChat<'a> {
        GetChat {
            chat_id: None,
            chat_username: None,
        }
    }

    buildera!(GetChat; chat_id, i64; chat_username, &'a str);
}

impl<'a> GetChatAdministrators<'a> {
    pub fn new() -> GetChatAdministrators<'a> {
        GetChatAdministrators {
            chat_id: None,
            chat_username: None,
        }
    }

    buildera!(GetChatAdministrators; chat_id, i64; chat_username, &'a str);
}

impl<'a> GetChatMembersCount<'a> {
    pub fn new() -> GetChatMembersCount<'a> {
        GetChatMembersCount {
            chat_id: None,
            chat_username: None,
        }
    }

    buildera!(GetChatMembersCount; chat_id, i64; chat_username, &'a str);
}

impl<'a> GetChatMember<'a> {
    pub fn new(user_id: i64) -> GetChatMember<'a> {
        GetChatMember {
            chat_id: None,
            chat_username: None,
            user_id: user_id,
        }
    }

    buildera!(GetChatMember; chat_id, i64; chat_username, &'a str);
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

    buildera!(AnswerCallbackQuery; text, &'a str; show_alert, bool; url, &'a str; cache_time, i64);
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

    buildera!(EditMessageText; chat_id, i64; chat_username, &'a str; message_id, i64; inline_message_id, &'a str; parse_mode, &'a str; disable_web_page_preview, bool; reply_markup, &'a types::ReplyMarkup); // InlineKeyboardMarkup
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

    buildera!(EditMessageCaption; chat_id, i64; chat_username, &'a str; message_id, i64; inline_message_id, &'a str; caption, &'a str; reply_markup, &'a types::ReplyMarkup); // InlineKeyboardMarkup
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

    buildera!(EditMessageReplyMarkup; chat_id, i64; chat_username, &'a str; message_id, i64; inline_message_id, &'a str; reply_markup, &'a types::ReplyMarkup); // InlineKeyboardMarkup
}

impl<'a> AnswerInlineQuery<'a> {
    pub fn new(inline_query_id: &'a str, results: &'a [&'a types::InlineQueryResult]) -> AnswerInlineQuery<'a> {
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

    buildera!(AnswerInlineQuery; cache_time, i64; is_personal, bool; next_offset, &'a str; switch_pm_text, &'a str; switch_pm_parameter, &'a str);
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

    buildera!(SendGame; disable_notification, bool; reply_to_message_id, i64; reply_markup, &'a types::ReplyMarkup); // InlineKeyboardMarkup
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

    buildera!(SetGameScore; force, bool; disable_edit_message, bool; chat_id, i64; message_id, i64; inline_message_id, &'a str);
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

    buildera!(GetGameHighScores; chat_id, i64; message_id, i64; inline_message_id, &'a str);
}

impl<'a> KeyboardButton<'a> {
    pub fn new(text: &'a str) -> KeyboardButton<'a> {
        KeyboardButton {
            text: text,
            request_contact: None,
            request_location: None,
        }
    }

    buildera!(KeyboardButton; request_contact, bool; request_location, bool);
}

impl<'a> InlineKeyboardButton<'a> {
    pub fn new(text: &'a str) -> InlineKeyboardButton<'a> {
        InlineKeyboardButton {
            text: text,
            url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
        }
    }

    buildera!(InlineKeyboardButton; url, &'a str; callback_data, &'a str; switch_inline_query, &'a str; switch_inline_query_current_chat, &'a str; callback_game, &'a types::CallbackGame);
}