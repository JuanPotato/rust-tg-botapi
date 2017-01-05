extern crate serde_json;

use serde_json::value::Value;

use super::types;

pub struct GetUpdates<'a> {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub timeout: Option<i64>,
    pub allowed_updates: Option<Vec<&'a str>>,
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

    pub fn offset(&'a mut self, offset: i64) -> &'a mut GetUpdates {
        self.offset = Some(offset);
        self
    }

    pub fn limit(&'a mut self, limit: i64) -> &'a mut GetUpdates {
        self.limit = Some(limit);
        self
    }

    pub fn timeout(&'a mut self, timeout: i64) -> &'a mut GetUpdates {
        self.timeout = Some(timeout);
        self
    }

    pub fn allowed_updates(&'a mut self, allowed_updates: &[&'a str]) -> &'a mut GetUpdates {
        self.allowed_updates = Some(allowed_updates.to_vec());
        self
    }

    pub fn send(&self, bot: &super::BotApi) -> Result<Vec<types::Update>, super::BotError> {
        let url = bot.base_url + "getUpdates";
        let res = super::request(bot.client.get(&url));
        match res {
            Ok(val) => Ok(serde_json::value::from_value(val).unwrap()),
            Err(e) => Err(e),
        }
    } // Find a better way to send. Maybe to intialize, pass bot token. 
}


struct SetWebhook<'a> {
    url: &'a str,
    certificate: Option<&'a str>,
    max_connections: Option<i64>,
    allowed_updates: Option<Vec<&'a str>>,
}

struct SendMessage<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    text: &'a str,
    parse_mode: Option<&'a str>,
    disable_web_page_preview: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<types::ReplyMarkup>,
}

struct ForwardMessage<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    from_chat_id: Option<i64>,
    from_chat_username: Option<&'a str>,
    disable_notification: Option<bool>,
    message_id: i64,
}

struct SendPhoto<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    photo: Option<&'a str>,
    file_id: Option<&'a str>,
    caption: Option<&'a str>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<types::ReplyMarkup>,
}

struct SendAudio<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    audio: Option<&'a str>,
    file_id: Option<&'a str>,
    caption: Option<&'a str>,
    duration: Option<i64>,
    performer: Option<&'a str>,
    title: Option<&'a str>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<types::ReplyMarkup>,
}

struct SendDocument<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    document: Option<&'a str>,
    file_id: Option<&'a str>,
    caption: Option<&'a str>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<types::ReplyMarkup>,
}

struct SendSticker<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    sticker: Option<&'a str>,
    file_id: Option<&'a str>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<types::ReplyMarkup>,
}

struct SendVideo<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    video: Option<&'a str>,
    file_id: Option<&'a str>,
    duration: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    caption: Option<&'a str>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<types::ReplyMarkup>,
}

struct SendVoice<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    voice: Option<&'a str>,
    file_id: Option<&'a str>,
    caption: Option<&'a str>,
    duration: Option<i64>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<types::ReplyMarkup>,
}

struct SendLocation<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    latitude: f64,
    longitude: f64,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<types::ReplyMarkup>,
}

struct SendVenue<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    latitude: f64,
    longitude: f64,
    title: &'a str,
    address: &'a str,
    foursquare_id: Option<&'a str>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<types::ReplyMarkup>,
}

struct SendContact<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    phone_number: &'a str,
    first_name: &'a str,
    last_name: Option<&'a str>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<types::ReplyMarkup>,
}

struct GetUserProfilePhotos {
    user_id: i64,
    offset: Option<i64>,
    limit: Option<i64>,
}

struct GetFile<'a> {
    file_id: &'a str,
}

struct KickChatMember<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    user_id: i64,
}

struct LeaveChat<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
}

struct UnbanChatMember<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    user_id: i64,
}

struct GetChat<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
}

struct GetChatAdministrators<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
}

struct GetChatMembersCount<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
}

struct GetChatMember<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    user_id: i64,
}

struct AnswerCallbackQuery<'a> {
    callback_query_id: &'a str,
    text: Option<&'a str>,
    show_alert: Option<bool>,
    url: Option<&'a str>,
    cache_time: Option<i64>,
}

struct EditMessageText<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
    text: &'a str,
    parse_mode: Option<&'a str>,
    disable_web_page_preview: Option<bool>,
    reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

struct EditMessageCaption<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
    caption: Option<&'a str>,
    reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

struct EditMessageReplyMarkup<'a> {
    chat_id: Option<i64>,
    chat_username: Option<&'a str>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
    reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

struct AnswerInlineQuery<'a> {
    inline_query_id: &'a str,
    results: Vec<types::InlineQueryResult>,
    cache_time: Option<i64>,
    is_personal: Option<bool>,
    next_offset: Option<&'a str>,
    switch_pm_text: Option<&'a str>,
    switch_pm_parameter: Option<&'a str>,
}

struct SendGame<'a> {
    chat_id: i64,
    game_short_name: &'a str,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<types::ReplyMarkup>, // InlineKeyboardMarkup
}

struct GetGameScore<'a> {
    user_id: i64,
    score: i64,
    force: Option<bool>,
    disable_edit_message: Option<bool>,
    chat_id: Option<i64>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
}

struct GetGameHighScores<'a> {
    user_id: i64,
    chat_id: Option<i64>,
    message_id: Option<i64>,
    inline_message_id: Option<&'a str>,
}
