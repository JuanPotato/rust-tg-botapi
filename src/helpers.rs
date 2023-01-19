use crate::methods::*;
use crate::types::*;
use crate::BotError;
use crate::TgObject;
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
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
                parameters,
                ..
            } => Err(BotError::Api {
                error_code,
                description,
                parameters,
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MessageOrBool {
    Message(Box<Message>),
    Bool(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParseMode {
    Markdown,
    MarkdownV2,
    HTML,
}

impl ToString for ParseMode {
    fn to_string(&self) -> String {
        match self {
            ParseMode::Markdown => "Markdown",
            ParseMode::MarkdownV2 => "MarkdownV2",
            ParseMode::HTML => "HTML",
        }.to_string()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum InputFile {
    FileId(String),
    File(PathBuf),
    Raw { bytes: Vec<u8>, name: String },
}

impl ToString for InputFile {
    fn to_string(&self) -> String {
        match self {
            InputFile::FileId(file_id) => file_id.clone(),
            InputFile::File(path) => {
                let name = path.file_name().unwrap().to_string_lossy();
                format!("attach://{name}")
            }
            InputFile::Raw { bytes: _, name } => {
                format!("attach://{name}")
            }
        }
    }
}

impl Serialize for InputFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            InputFile::FileId(file_id) => serializer.serialize_str(file_id),
            InputFile::File(path) => {
                let name = path.file_name().unwrap().to_string_lossy();
                let input_file = format!("attach://{name}");
                serializer.serialize_str(&input_file)
            }
            InputFile::Raw { bytes: _, name } => {
                let input_file = format!("attach://{name}");
                serializer.serialize_str(&input_file)
            }
        }
    }
}

impl TgObject for InputFile {
    fn add_file(&self, form: Form) -> Form {
        match self {
            InputFile::FileId(_) => form,

            InputFile::File(path) => {
                let mut file = std::fs::File::open(path).unwrap();
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes).unwrap();

                let mut part = reqwest::multipart::Part::bytes(bytes);
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                part = part.file_name(filename.clone());

                form.part(filename, part)
            }

            InputFile::Raw { bytes, name } => {
                let mut part = reqwest::multipart::Part::bytes(bytes.clone());
                part = part.file_name(name.clone());
                form.part(name.clone(), part)
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

impl ToString for ChatId {
    fn to_string(&self) -> String {
        match self {
            ChatId::Id(i) => i.to_string(),
            ChatId::Username(u) => u.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Update {
    pub update_id: i64,
    #[serde(flatten)]
    pub update_type: UpdateType,
}

#[derive(Debug, Clone, Deserialize)]
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

impl Message {
    pub fn reply(&self, text: impl Into<String>) -> SendMessage {
        SendMessage::new(self.chat.id.into(), text.into())
    }

    pub fn reply_photo(&self, photo: InputFile) -> SendPhoto {
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
