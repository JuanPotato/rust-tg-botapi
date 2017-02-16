extern crate serde_json;
extern crate serde;

use self::serde::ser::Serialize;
use self::serde::ser::Serializer;

use types::ReplyMarkup;
use types::InlineQueryResult;

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
    pub reply_markup: Option<Box<ReplyMarkup>>,
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

        if self.reply_markup.is_some() {
            serializer.serialize_struct_elt(&mut state, "reply_markup", &self.reply_markup)?;
        }

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct ForwardMessage<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub from_chat_id: Option<i64>,
    pub from_chat_username: Option<&'a str>,
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
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug)]
pub struct SendAudio<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub audio: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub duration: Option<i64>,
    pub performer: Option<&'a str>,
    pub title: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug)]
pub struct SendDocument<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub document: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug)]
pub struct SendSticker<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub sticker: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug)]
pub struct SendVideo<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub video: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub duration: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub caption: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug)]
pub struct SendVoice<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub voice: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub duration: Option<i64>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

#[derive(Debug)]
pub struct SendLocation<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub latitude: f64,
    pub longitude: f64,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

impl <'a> Serialize for SendLocation<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("SendLocation",
            if self.chat_username.is_some() || self.chat_id.is_some() { 3 } else { 2 } +
            option_int!(&self.disable_notification,
                        &self.reply_to_message_id,
                        &self.reply_markup))?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        serializer.serialize_struct_elt(&mut state, "latitude", &self.latitude)?;
        serializer.serialize_struct_elt(&mut state, "longitude", &self.longitude)?;

        option_serialize_struct_elt!(serializer, &mut state, "disable_notification", &self.disable_notification);
        option_serialize_struct_elt!(serializer, &mut state, "reply_to_message_id", &self.reply_to_message_id);
        option_serialize_struct_elt!(serializer, &mut state, "reply_markup", &self.reply_markup);

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct SendVenue<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub latitude: f64,
    pub longitude: f64,
    pub title: &'a str,
    pub address: &'a str,
    pub foursquare_id: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

impl <'a> Serialize for SendVenue<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("SendVenue",
            if self.chat_username.is_some() || self.chat_id.is_some() { 5 } else { 4 } +
            option_int!(&self.foursquare_id,
                        &self.disable_notification,
                        &self.reply_to_message_id,
                        &self.reply_markup))?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        serializer.serialize_struct_elt(&mut state, "latitude", &self.latitude)?;
        serializer.serialize_struct_elt(&mut state, "longitude", &self.longitude)?;
        serializer.serialize_struct_elt(&mut state, "title", &self.title)?;
        serializer.serialize_struct_elt(&mut state, "address", &self.address)?;

        option_serialize_struct_elt!(serializer, &mut state, "foursquare_id", &self.foursquare_id);
        option_serialize_struct_elt!(serializer, &mut state, "disable_notification", &self.disable_notification);
        option_serialize_struct_elt!(serializer, &mut state, "reply_to_message_id", &self.reply_to_message_id);
        option_serialize_struct_elt!(serializer, &mut state, "reply_markup", &self.reply_markup);

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct SendContact<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub phone_number: &'a str,
    pub first_name: &'a str,
    pub last_name: Option<&'a str>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

impl <'a> Serialize for SendContact<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("SendContact",
            if self.chat_username.is_some() || self.chat_id.is_some() { 3 } else { 2 } +
            option_int!(&self.last_name,
                        &self.disable_notification,
                        &self.reply_to_message_id,
                        &self.reply_markup))?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        serializer.serialize_struct_elt(&mut state, "phone_number", &self.phone_number)?;
        serializer.serialize_struct_elt(&mut state, "first_name", &self.first_name)?;

        option_serialize_struct_elt!(serializer, &mut state, "last_name", &self.last_name);
        option_serialize_struct_elt!(serializer, &mut state, "disable_notification", &self.disable_notification);
        option_serialize_struct_elt!(serializer, &mut state, "reply_to_message_id", &self.reply_to_message_id);
        option_serialize_struct_elt!(serializer, &mut state, "reply_markup", &self.reply_markup);

        serializer.serialize_struct_end(state)
    }
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

#[derive(Debug)]
pub struct KickChatMember<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub user_id: i64,
}

impl <'a> Serialize for KickChatMember<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("KickChatMember",
            if self.chat_username.is_some() || self.chat_id.is_some() { 2 } else { 1 })?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        serializer.serialize_struct_elt(&mut state, "user_id", &self.user_id)?;

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct LeaveChat<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
}

impl <'a> Serialize for LeaveChat<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("LeaveChat",
            if self.chat_username.is_some() || self.chat_id.is_some() { 1 } else { 0 })?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct UnbanChatMember<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub user_id: i64,
}

impl <'a> Serialize for UnbanChatMember<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("UnbanChatMember",
            if self.chat_username.is_some() || self.chat_id.is_some() { 2 } else { 1 })?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        serializer.serialize_struct_elt(&mut state, "user_id", &self.user_id)?;

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct GetChat<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
}

impl <'a> Serialize for GetChat<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("GetChat",
            if self.chat_username.is_some() || self.chat_id.is_some() { 1 } else { 0 })?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct GetChatAdministrators<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
}

impl <'a> Serialize for GetChatAdministrators<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("GetChatAdministrators",
            if self.chat_username.is_some() || self.chat_id.is_some() { 1 } else { 0 })?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct GetChatMembersCount<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
}

impl <'a> Serialize for GetChatMembersCount<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("GetChatMembersCount",
            if self.chat_username.is_some() || self.chat_id.is_some() { 1 } else { 0 })?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct GetChatMember<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub user_id: i64,
}

impl <'a> Serialize for GetChatMember<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("GetChatMember",
            if self.chat_username.is_some() || self.chat_id.is_some() { 2 } else { 1 })?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        serializer.serialize_struct_elt(&mut state, "user_id", &self.user_id)?;

        serializer.serialize_struct_end(state)
    }
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

#[derive(Debug)]
pub struct EditMessageText<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<&'a str>,
    pub text: &'a str,
    pub parse_mode: Option<&'a str>,
    pub disable_web_page_preview: Option<bool>,
    pub reply_markup: Option<Box<ReplyMarkup>>, // InlineKeyboardMarkup
}

impl <'a> Serialize for EditMessageText<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("EditMessageText",
            if self.chat_username.is_some() || self.chat_id.is_some() { 2 } else { 1 } +
            option_int!(&self.message_id,
                        &self.parse_mode,
                        &self.disable_web_page_preview,
                        &self.reply_markup))?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        serializer.serialize_struct_elt(&mut state, "text", &self.text)?;

        option_serialize_struct_elt!(serializer, &mut state, "message_id", &self.message_id);
        option_serialize_struct_elt!(serializer, &mut state, "parse_mode", &self.parse_mode);
        option_serialize_struct_elt!(serializer, &mut state, "disable_web_page_preview", &self.disable_web_page_preview);
        option_serialize_struct_elt!(serializer, &mut state, "reply_markup", &self.reply_markup);

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct EditMessageCaption<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<&'a str>,
    pub caption: Option<&'a str>,
    pub reply_markup: Option<Box<ReplyMarkup>>, // InlineKeyboardMarkup
}

impl <'a> Serialize for EditMessageCaption<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("EditMessageCaption",
            if self.chat_username.is_some() || self.chat_id.is_some() { 1 } else { 0 } +
            option_int!(&self.message_id,
                        &self.inline_message_id,
                        &self.caption,
                        &self.reply_markup))?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        option_serialize_struct_elt!(serializer, &mut state, "message_id", &self.message_id);
        option_serialize_struct_elt!(serializer, &mut state, "inline_message_id", &self.inline_message_id);
        option_serialize_struct_elt!(serializer, &mut state, "caption", &self.caption);
        option_serialize_struct_elt!(serializer, &mut state, "reply_markup", &self.reply_markup);

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug)]
pub struct EditMessageReplyMarkup<'a> {
    pub chat_id: Option<i64>,
    pub chat_username: Option<&'a str>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<&'a str>,
    pub reply_markup: Option<Box<ReplyMarkup>>, // InlineKeyboardMarkup
}

impl <'a> Serialize for EditMessageReplyMarkup<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("EditMessageReplyMarkup",
            if self.chat_username.is_some() || self.chat_id.is_some() { 1 } else { 0 } +
            option_int!(&self.message_id,
                        &self.inline_message_id,
                        &self.reply_markup))?;

        if self.chat_username.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_username)?;
        } else if self.chat_id.is_some() {
            serializer.serialize_struct_elt(&mut state, "chat_id", &self.chat_id)?;
        }

        option_serialize_struct_elt!(serializer, &mut state, "message_id", &self.message_id);
        option_serialize_struct_elt!(serializer, &mut state, "inline_message_id", &self.inline_message_id);
        option_serialize_struct_elt!(serializer, &mut state, "reply_markup", &self.reply_markup);

        serializer.serialize_struct_end(state)
    }
}

#[derive(Debug, Serialize)]
pub struct AnswerInlineQuery<'a> {
    pub inline_query_id: &'a str,
    pub results: Vec<InlineQueryResult>,

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
    pub reply_markup: Option<Box<ReplyMarkup>>, // InlineKeyboardMarkup
}

#[derive(Debug, Serialize)]
pub struct SetGameScore<'a> {
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
