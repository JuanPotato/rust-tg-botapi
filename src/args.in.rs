extern crate serde_json;
extern crate serde;

use self::serde::ser::Serialize;
use self::serde::ser::Serializer;

use super::types;

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

    
#[derive(Debug, Serialize)]
pub struct GetUpdates<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<&'a [&'a str]>,
}

#[derive(Debug, Serialize)]
pub struct SetWebhook<'a> {
    pub url: &'a str,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<&'a [&'a str]>,
}

#[derive(Debug)]
pub struct SendMessage<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub text: &'a str,
    pub parse_mode: Option<&'a str>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<&'a types::ReplyMarkup>,
}

impl <'a> Serialize for SendMessage<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("SendMessage",
            if self.chat_username.is_some() || self.chat_id.is_some() { 2 } else { 1 } +
            option_int!(&self.parse_mode,
                        &self.disable_web_page_preview,
                        &self.disable_notification,
                        &self.reply_to_message_id,
                        &self.reply_markup))?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        serializer.serialize_struct_elt(&mut state, "text", &self.text)?;

        option_serialize_struct_elt!(serializer, &mut state, "parse_mode", &self.parse_mode);
        option_serialize_struct_elt!(serializer, &mut state, "disable_web_page_preview", &self.disable_web_page_preview);
        option_serialize_struct_elt!(serializer, &mut state, "disable_notification", &self.disable_notification);
        option_serialize_struct_elt!(serializer, &mut state, "reply_to_message_id", &self.reply_to_message_id);
        option_serialize_struct_elt!(serializer, &mut state, "reply_markup", &self.reply_markup);

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct ForwardMessage<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_chat_username: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    pub message_id: i64,
}

impl <'a> Serialize for ForwardMessage<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("ForwardMessage",
            if self.chat_username.is_some() || self.chat_id.is_some() { 2 } else { 1 } +
            if self.from_chat_username.is_some() || self.from_chat_id.is_some() { 1 } else { 0 } +
            option_int!(&self.disable_notification))?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        if self.from_chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "from_chat_id", &self.from_chat_username)?;
        } else if self.from_chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "from_chat_id", &self.from_chat_id)?;
        }

        option_serialize_struct_elt!(serializer, &mut state, "disable_notification", &self.disable_notification);

        serializer.serialize_struct_elt(&mut state, "message_id", &self.message_id)?;

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct SendPhoto<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub photo: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<&'a types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendAudio<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendDocument<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendSticker<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendVideo<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendVoice<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendLocation<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    pub latitude: f64,
    pub longitude: f64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendVenue<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    pub latitude: f64,
    pub longitude: f64,
    pub title: &'a str,
    pub address: &'a str,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct SendContact<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    pub phone_number: &'a str,
    pub first_name: &'a str,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup>,
}

#[derive(Debug, Serialize)]
pub struct GetUserProfilePhotos {
    pub user_id: i64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct GetFile<'a> {
    pub file_id: &'a str,
}

#[derive(Debug, Serialize)]
pub struct KickChatMember<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct LeaveChat<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct UnbanChatMember<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct GetChat<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct GetChatAdministrators<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct GetChatMembersCount<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct GetChatMember<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct AnswerCallbackQuery<'a> {
    pub callback_query_id: &'a str,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct EditMessageText<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<&'a str>,
    pub text: &'a str,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug, Serialize)]
pub struct EditMessageCaption<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug, Serialize)]
pub struct EditMessageReplyMarkup<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_username: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug, Serialize)]
pub struct AnswerInlineQuery<'a> {
    pub inline_query_id: &'a str,
    pub results: Vec<types::InlineQueryResult>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<&'a str>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct SendGame<'a> {
    pub chat_id: i64,
    pub game_short_name: &'a str,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<&'a types::ReplyMarkup>, // InlineKeyboardMarkup
}

#[derive(Debug, Serialize)]
pub struct GetGameScore<'a> {
    pub user_id: i64,
    pub score: i64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct GetGameHighScores<'a> {
    pub user_id: i64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<&'a str>,
}
