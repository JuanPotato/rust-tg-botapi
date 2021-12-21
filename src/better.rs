use crate::BotError;
use crate::objects::*;
use crate::functions::*;
pub(crate) use reqwest::multipart::Form;
use std::path::PathBuf;
use std::io::Read;

pub trait FormSer {
    fn serialize(&self, key: String, form: Form) -> Form;
}

impl FormSer for bool {
    fn serialize(&self, key: String, form: Form) -> Form {
        form.text(key, self.to_string())
    }
}

impl FormSer for i64 {
    fn serialize(&self, key: String, form: Form) -> Form {
        form.text(key, self.to_string())
    }
}

impl FormSer for f64 {
    fn serialize(&self, key: String, form: Form) -> Form {
        form.text(key, self.to_string())
    }
}

impl FormSer for String {
    fn serialize(&self, key: String, form: Form) -> Form {
        form.text(key, self.to_string())
    }
}

impl<T: FormSer> FormSer for Vec<T> {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        for (i, elem) in self.iter().enumerate() {
            form = elem.serialize(format!("{}[{}]", key, i), form);
        }

        return form;
    }
}

impl<T: FormSer> FormSer for Option<T> {
    fn serialize(&self, key: String, form: Form) -> Form {
        if let Some(s) = self.as_ref() {
            s.serialize(key, form)
        } else {
            form
        }
    }
}

impl<T: FormSer> FormSer for Box<T> {
    fn serialize(&self, key: String, form: Form) -> Form {
        self.as_ref().serialize(key, form)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
    Ok {
        result: T,
    },

    Err {
        error_code: i64,
        description: String,
        parameters: Option<ResponseParameters>,
    },
}

impl<T> Into<Result<T, BotError>> for ApiResult<T> {
    fn into(self) -> Result<T, BotError> {
        match self {
            ApiResult::Ok { result, .. } => Ok(result),

            ApiResult::Err {
                error_code,
                description,
                parameters, ..
            } => Err(BotError::Api {
                error_code,
                description,
                parameters,
            }),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageOrBool {
    Message(Box<Message>),
    Bool(bool),
}

impl FormSer for MessageOrBool {
    fn serialize(&self, key: String, form: Form) -> Form {
        match self {
            MessageOrBool::Message(m) => m.serialize(key, form),
            MessageOrBool::Bool(b) => b.serialize(key, form),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParseMode {
    Markdown,
    HTML,
    None,
}

impl Default for ParseMode {
    fn default() -> ParseMode {
        ParseMode::None
    }
}

impl ParseMode {
    pub fn is_none(&self) -> bool {
        *self == ParseMode::None
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputFile {
    FileId(String),
    File(PathBuf),
    Raw {
        bytes: Vec<u8>,
        name: String
    },
}

impl FormSer for InputFile {
    fn serialize(&self, key: String, form: Form) -> Form {
        match self {
            InputFile::FileId(f) => f.serialize(key, form),

            InputFile::File(path) => {
                let mut file = std::fs::File::open(path).unwrap();
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes).unwrap();

                let mut part = reqwest::multipart::Part::bytes(bytes);
                let filename: String = path.file_name().unwrap().to_str().unwrap().to_string();
                part = part.file_name(filename);

                form.part(key, part)
            }

            InputFile::Raw {bytes, name} => {
                let mut part = reqwest::multipart::Part::bytes(bytes.clone());
                part = part.file_name(name.clone());
                form.part(key, part)
            }
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatId {
    Id(i64),
    Username(String),
}

impl From<i64> for ChatId {
    fn from(id: i64) -> ChatId {
        ChatId::Id(id)
    }
}

impl From<String> for ChatId {
    fn from(id: String) -> ChatId {
        ChatId::Username(id)
    }
}

impl FormSer for ChatId {
    fn serialize(&self, key: String, form: Form) -> Form {
        match self {
            ChatId::Id(id) => id.serialize(key, form),
            ChatId::Username(name) => name.serialize(key, form),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    ReplyKeyboard(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl FormSer for ReplyMarkup {
    fn serialize(&self, key: String, form: Form) -> Form {
        match self {
            ReplyMarkup::InlineKeyboard(i) => i.serialize(key, form),
            ReplyMarkup::ReplyKeyboard(r) => r.serialize(key, form),
            ReplyMarkup::ReplyKeyboardRemove(r) => r.serialize(key, form),
            ReplyMarkup::ForceReply(f) => f.serialize(key, form),
        }
    }
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(markup: InlineKeyboardMarkup) -> ReplyMarkup {
        ReplyMarkup::InlineKeyboard(markup)
    }
}

impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(markup: ReplyKeyboardMarkup) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboard(markup)
    }
}

impl From<ReplyKeyboardRemove> for ReplyMarkup {
    fn from(markup: ReplyKeyboardRemove) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboardRemove(markup)
    }
}

impl From<ForceReply> for ReplyMarkup {
    fn from(markup: ForceReply) -> ReplyMarkup {
        ReplyMarkup::ForceReply(markup)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    pub update_id: i64,
    #[serde(flatten)]
    pub update_type: UpdateType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateType {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    InlineQuery(InlineQuery),
    ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(CallbackQuery),
    ShippingQuery(ShippingQuery),
    PreCheckoutQuery(PreCheckoutQuery),
    Poll(Poll),
    PollAnswer(PollAnswer),
    MyChatMember(ChatMemberUpdated),
    ChatMember(ChatMemberUpdated),
    ChatJoinRequest(ChatJoinRequest),
}

impl FormSer for Update {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.update_id.serialize(format!("{}[update_id]", key), form);
        form = self.update_type.serialize(key, form);
        return form;
    }
}

impl FormSer for UpdateType {
    fn serialize(&self, key: String, form: Form) -> Form {
        match self {
            UpdateType::Message(v) => v.serialize(format!("{}[message]", key), form),
            UpdateType::EditedMessage(v) => v.serialize(format!("{}[edited_message]", key), form),
            UpdateType::ChannelPost(v) => v.serialize(format!("{}[channel_post]", key), form),
            UpdateType::EditedChannelPost(v) => v.serialize(format!("{}[edited_channel_post]", key), form),
            UpdateType::InlineQuery(v) => v.serialize(format!("{}[inline_query]", key), form),
            UpdateType::ChosenInlineResult(v) => v.serialize(format!("{}[chosen_inline_result]", key), form),
            UpdateType::CallbackQuery(v) => v.serialize(format!("{}[callback_query]", key), form),
            UpdateType::ShippingQuery(v) => v.serialize(format!("{}[shipping_query]", key), form),
            UpdateType::PreCheckoutQuery(v) => v.serialize(format!("{}[pre_checkout_query]", key), form),
            UpdateType::Poll(v) => v.serialize(format!("{}[poll]", key), form),
            UpdateType::PollAnswer(v) => v.serialize(format!("{}[poll_answer]", key), form),
            UpdateType::MyChatMember(v) => v.serialize(format!("{}[my_chat_member]", key), form),
            UpdateType::ChatMember(v) => v.serialize(format!("{}[chat_member]", key), form),
            UpdateType::ChatJoinRequest(v) => v.serialize(format!("{}[chat_join_request]", key), form),
        }
    }
}

impl Message {
    pub fn reply(&self, text: impl Into<String>) -> SendMessage {
        let mut send_msg = SendMessage::new(self.chat.id.into(), text.into());
        send_msg.reply_to_message_id = Some(self.message_id);

        send_msg
    }

    pub fn respond(&self, text: impl Into<String>) -> SendMessage {
        SendMessage::new(self.chat.id.into(), text.into())
    }

    pub fn reply_photo(&self, photo: InputFile) -> SendPhoto {
        let mut send_photo = SendPhoto::new(self.chat.id.into(), photo);
        send_photo.reply_to_message_id = Some(self.message_id);

        send_photo
    }

    pub fn respond_photo(&self, photo: InputFile) -> SendPhoto {
        SendPhoto::new(self.chat.id.into(), photo)
    }

    pub fn get_text(&self) -> Option<&String> {
        if let Some(ref text) = self.text {
            Some(text)
        } else if let Some(ref text) = self.caption {
            Some(text)
        } else {
            None
        }
    }
}

impl SendPhoto {
    pub fn new_fileid(chat: impl Into<ChatId>, fileid: impl Into<String>) -> SendPhoto {
        SendPhoto::new(chat.into(), InputFile::FileId(fileid.into()))
    }

    pub fn new_file(chat: impl Into<ChatId>, filepath: impl Into<PathBuf>) -> SendPhoto {
        SendPhoto::new(chat.into(), InputFile::File(filepath.into()))
    }
}
