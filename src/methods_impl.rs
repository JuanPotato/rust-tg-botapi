use crate::methods::*;
use crate::types::*;
use crate::helpers::*;
use crate::{TgMethod, TgObject};

impl TgMethod for GetUpdates {
    type ResponseType = Vec<Update>;
    const PATH: &'static str = "getUpdates";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.offset {
            form = form.text("offset", s.to_string());
        }
        if let Some(s) = &self.limit {
            form = form.text("limit", s.to_string());
        }
        if let Some(s) = &self.timeout {
            form = form.text("timeout", s.to_string());
        }
        if let Some(s) = &self.allowed_updates {
            form = form.text("allowed_updates", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl GetUpdates {
    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_timeout(mut self, timeout: i64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_allowed_updates(mut self, allowed_updates: Vec<String>) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

}

impl TgMethod for SetWebhook {
    type ResponseType = bool;
    const PATH: &'static str = "setWebhook";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.certificate.add_file(form);
        form = form.text("url", self.url.to_string());
        if let Some(s) = &self.certificate {
            form = form.text("certificate", s.to_string());
        }
        if let Some(s) = &self.ip_address {
            form = form.text("ip_address", s.to_string());
        }
        if let Some(s) = &self.max_connections {
            form = form.text("max_connections", s.to_string());
        }
        if let Some(s) = &self.allowed_updates {
            form = form.text("allowed_updates", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.drop_pending_updates {
            form = form.text("drop_pending_updates", s.to_string());
        }
        if let Some(s) = &self.secret_token {
            form = form.text("secret_token", s.to_string());
        }
        form
    }
}

impl SetWebhook {
    pub fn new(url: String, ) -> Self {
        Self {
            url,
            certificate: None,
            ip_address: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: None,
            secret_token: None,
        }
    }
}

impl SetWebhook {
    pub fn with_certificate(mut self, certificate: InputFile) -> Self {
        self.certificate = Some(certificate);
        self
    }

    pub fn with_ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn with_max_connections(mut self, max_connections: i64) -> Self {
        self.max_connections = Some(max_connections);
        self
    }

    pub fn with_allowed_updates(mut self, allowed_updates: Vec<String>) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    pub fn with_drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }

    pub fn with_secret_token(mut self, secret_token: String) -> Self {
        self.secret_token = Some(secret_token);
        self
    }

}

impl crate::TgObject for SetWebhook {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.certificate.add_file(form);
        form
    }
}

impl TgMethod for DeleteWebhook {
    type ResponseType = bool;
    const PATH: &'static str = "deleteWebhook";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.drop_pending_updates {
            form = form.text("drop_pending_updates", s.to_string());
        }
        form
    }
}

impl DeleteWebhook {
    pub fn with_drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }

}

impl TgMethod for GetWebhookInfo {
    type ResponseType = WebhookInfo;
    const PATH: &'static str = "getWebhookInfo";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form
    }
}

impl TgMethod for GetMe {
    type ResponseType = User;
    const PATH: &'static str = "getMe";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form
    }
}

impl TgMethod for LogOut {
    type ResponseType = bool;
    const PATH: &'static str = "logOut";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form
    }
}

impl TgMethod for Close {
    type ResponseType = bool;
    const PATH: &'static str = "close";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form
    }
}

impl TgMethod for SendMessage {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendMessage";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("text", self.text.to_string());
        if let Some(s) = &self.parse_mode {
            form = form.text("parse_mode", s.to_string());
        }
        if let Some(s) = &self.entities {
            form = form.text("entities", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.disable_web_page_preview {
            form = form.text("disable_web_page_preview", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendMessage {
    pub fn new(chat_id: ChatId, text: String, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            text,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendMessage {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn with_entities(mut self, entities: Vec<MessageEntity>) -> Self {
        self.entities = Some(entities);
        self
    }

    pub fn with_disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for ForwardMessage {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "forwardMessage";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("from_chat_id", self.from_chat_id.to_string());
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        form = form.text("message_id", self.message_id.to_string());
        form
    }
}

impl ForwardMessage {
    pub fn new(chat_id: ChatId, from_chat_id: ChatId, message_id: i64, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            from_chat_id,
            disable_notification: None,
            protect_content: None,
            message_id,
        }
    }
}

impl ForwardMessage {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

}

impl TgMethod for CopyMessage {
    type ResponseType = MessageId;
    const PATH: &'static str = "copyMessage";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("from_chat_id", self.from_chat_id.to_string());
        form = form.text("message_id", self.message_id.to_string());
        if let Some(s) = &self.caption {
            form = form.text("caption", s.to_string());
        }
        if let Some(s) = &self.parse_mode {
            form = form.text("parse_mode", s.to_string());
        }
        if let Some(s) = &self.caption_entities {
            form = form.text("caption_entities", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl CopyMessage {
    pub fn new(chat_id: ChatId, from_chat_id: ChatId, message_id: i64, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            from_chat_id,
            message_id,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl CopyMessage {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_caption(mut self, caption: String) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn with_caption_entities(mut self, caption_entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for SendPhoto {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendPhoto";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.photo.add_file(form);
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("photo", self.photo.to_string());
        if let Some(s) = &self.caption {
            form = form.text("caption", s.to_string());
        }
        if let Some(s) = &self.parse_mode {
            form = form.text("parse_mode", s.to_string());
        }
        if let Some(s) = &self.caption_entities {
            form = form.text("caption_entities", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.has_spoiler {
            form = form.text("has_spoiler", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendPhoto {
    pub fn new(chat_id: ChatId, photo: InputFile, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            photo,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            has_spoiler: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendPhoto {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_caption(mut self, caption: String) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn with_caption_entities(mut self, caption_entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }

    pub fn with_has_spoiler(mut self, has_spoiler: bool) -> Self {
        self.has_spoiler = Some(has_spoiler);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl crate::TgObject for SendPhoto {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.photo.add_file(form);
        form
    }
}

impl TgMethod for SendAudio {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendAudio";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.audio.add_file(form);
        form = self.thumb.add_file(form);
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("audio", self.audio.to_string());
        if let Some(s) = &self.caption {
            form = form.text("caption", s.to_string());
        }
        if let Some(s) = &self.parse_mode {
            form = form.text("parse_mode", s.to_string());
        }
        if let Some(s) = &self.caption_entities {
            form = form.text("caption_entities", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.duration {
            form = form.text("duration", s.to_string());
        }
        if let Some(s) = &self.performer {
            form = form.text("performer", s.to_string());
        }
        if let Some(s) = &self.title {
            form = form.text("title", s.to_string());
        }
        if let Some(s) = &self.thumb {
            form = form.text("thumb", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendAudio {
    pub fn new(chat_id: ChatId, audio: InputFile, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            audio,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
            thumb: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendAudio {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_caption(mut self, caption: String) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn with_caption_entities(mut self, caption_entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }

    pub fn with_duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_performer(mut self, performer: String) -> Self {
        self.performer = Some(performer);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_thumb(mut self, thumb: InputFile) -> Self {
        self.thumb = Some(thumb);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl crate::TgObject for SendAudio {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.audio.add_file(form);
        form = self.thumb.add_file(form);
        form
    }
}

impl TgMethod for SendDocument {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendDocument";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.document.add_file(form);
        form = self.thumb.add_file(form);
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("document", self.document.to_string());
        if let Some(s) = &self.thumb {
            form = form.text("thumb", s.to_string());
        }
        if let Some(s) = &self.caption {
            form = form.text("caption", s.to_string());
        }
        if let Some(s) = &self.parse_mode {
            form = form.text("parse_mode", s.to_string());
        }
        if let Some(s) = &self.caption_entities {
            form = form.text("caption_entities", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.disable_content_type_detection {
            form = form.text("disable_content_type_detection", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendDocument {
    pub fn new(chat_id: ChatId, document: InputFile, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            document,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendDocument {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_thumb(mut self, thumb: InputFile) -> Self {
        self.thumb = Some(thumb);
        self
    }

    pub fn with_caption(mut self, caption: String) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn with_caption_entities(mut self, caption_entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }

    pub fn with_disable_content_type_detection(mut self, disable_content_type_detection: bool) -> Self {
        self.disable_content_type_detection = Some(disable_content_type_detection);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl crate::TgObject for SendDocument {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.document.add_file(form);
        form = self.thumb.add_file(form);
        form
    }
}

impl TgMethod for SendVideo {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendVideo";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.video.add_file(form);
        form = self.thumb.add_file(form);
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("video", self.video.to_string());
        if let Some(s) = &self.duration {
            form = form.text("duration", s.to_string());
        }
        if let Some(s) = &self.width {
            form = form.text("width", s.to_string());
        }
        if let Some(s) = &self.height {
            form = form.text("height", s.to_string());
        }
        if let Some(s) = &self.thumb {
            form = form.text("thumb", s.to_string());
        }
        if let Some(s) = &self.caption {
            form = form.text("caption", s.to_string());
        }
        if let Some(s) = &self.parse_mode {
            form = form.text("parse_mode", s.to_string());
        }
        if let Some(s) = &self.caption_entities {
            form = form.text("caption_entities", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.has_spoiler {
            form = form.text("has_spoiler", s.to_string());
        }
        if let Some(s) = &self.supports_streaming {
            form = form.text("supports_streaming", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendVideo {
    pub fn new(chat_id: ChatId, video: InputFile, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            video,
            duration: None,
            width: None,
            height: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            has_spoiler: None,
            supports_streaming: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendVideo {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_width(mut self, width: i64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: i64) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_thumb(mut self, thumb: InputFile) -> Self {
        self.thumb = Some(thumb);
        self
    }

    pub fn with_caption(mut self, caption: String) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn with_caption_entities(mut self, caption_entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }

    pub fn with_has_spoiler(mut self, has_spoiler: bool) -> Self {
        self.has_spoiler = Some(has_spoiler);
        self
    }

    pub fn with_supports_streaming(mut self, supports_streaming: bool) -> Self {
        self.supports_streaming = Some(supports_streaming);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl crate::TgObject for SendVideo {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.video.add_file(form);
        form = self.thumb.add_file(form);
        form
    }
}

impl TgMethod for SendAnimation {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendAnimation";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.animation.add_file(form);
        form = self.thumb.add_file(form);
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("animation", self.animation.to_string());
        if let Some(s) = &self.duration {
            form = form.text("duration", s.to_string());
        }
        if let Some(s) = &self.width {
            form = form.text("width", s.to_string());
        }
        if let Some(s) = &self.height {
            form = form.text("height", s.to_string());
        }
        if let Some(s) = &self.thumb {
            form = form.text("thumb", s.to_string());
        }
        if let Some(s) = &self.caption {
            form = form.text("caption", s.to_string());
        }
        if let Some(s) = &self.parse_mode {
            form = form.text("parse_mode", s.to_string());
        }
        if let Some(s) = &self.caption_entities {
            form = form.text("caption_entities", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.has_spoiler {
            form = form.text("has_spoiler", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendAnimation {
    pub fn new(chat_id: ChatId, animation: InputFile, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            animation,
            duration: None,
            width: None,
            height: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            has_spoiler: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendAnimation {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_width(mut self, width: i64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: i64) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_thumb(mut self, thumb: InputFile) -> Self {
        self.thumb = Some(thumb);
        self
    }

    pub fn with_caption(mut self, caption: String) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn with_caption_entities(mut self, caption_entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }

    pub fn with_has_spoiler(mut self, has_spoiler: bool) -> Self {
        self.has_spoiler = Some(has_spoiler);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl crate::TgObject for SendAnimation {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.animation.add_file(form);
        form = self.thumb.add_file(form);
        form
    }
}

impl TgMethod for SendVoice {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendVoice";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.voice.add_file(form);
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("voice", self.voice.to_string());
        if let Some(s) = &self.caption {
            form = form.text("caption", s.to_string());
        }
        if let Some(s) = &self.parse_mode {
            form = form.text("parse_mode", s.to_string());
        }
        if let Some(s) = &self.caption_entities {
            form = form.text("caption_entities", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.duration {
            form = form.text("duration", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendVoice {
    pub fn new(chat_id: ChatId, voice: InputFile, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            voice,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendVoice {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_caption(mut self, caption: String) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn with_caption_entities(mut self, caption_entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }

    pub fn with_duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl crate::TgObject for SendVoice {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.voice.add_file(form);
        form
    }
}

impl TgMethod for SendVideoNote {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendVideoNote";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.video_note.add_file(form);
        form = self.thumb.add_file(form);
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("video_note", self.video_note.to_string());
        if let Some(s) = &self.duration {
            form = form.text("duration", s.to_string());
        }
        if let Some(s) = &self.length {
            form = form.text("length", s.to_string());
        }
        if let Some(s) = &self.thumb {
            form = form.text("thumb", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendVideoNote {
    pub fn new(chat_id: ChatId, video_note: InputFile, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            video_note,
            duration: None,
            length: None,
            thumb: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendVideoNote {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_length(mut self, length: i64) -> Self {
        self.length = Some(length);
        self
    }

    pub fn with_thumb(mut self, thumb: InputFile) -> Self {
        self.thumb = Some(thumb);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl crate::TgObject for SendVideoNote {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.video_note.add_file(form);
        form = self.thumb.add_file(form);
        form
    }
}

impl TgMethod for SendMediaGroup {
    type ResponseType = Vec<Message>;
    const PATH: &'static str = "sendMediaGroup";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.media.add_file(form);
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("media", serde_json::to_string(&self.media).unwrap());
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        form
    }
}

impl SendMediaGroup {
    pub fn new(chat_id: ChatId, media: Vec<InputMedia>, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            media,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
        }
    }
}

impl SendMediaGroup {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

}

impl crate::TgObject for SendMediaGroup {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.media.add_file(form);
        form
    }
}

impl TgMethod for SendLocation {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendLocation";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("latitude", self.latitude.to_string());
        form = form.text("longitude", self.longitude.to_string());
        if let Some(s) = &self.horizontal_accuracy {
            form = form.text("horizontal_accuracy", s.to_string());
        }
        if let Some(s) = &self.live_period {
            form = form.text("live_period", s.to_string());
        }
        if let Some(s) = &self.heading {
            form = form.text("heading", s.to_string());
        }
        if let Some(s) = &self.proximity_alert_radius {
            form = form.text("proximity_alert_radius", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendLocation {
    pub fn new(chat_id: ChatId, latitude: f64, longitude: f64, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendLocation {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_horizontal_accuracy(mut self, horizontal_accuracy: f64) -> Self {
        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }

    pub fn with_live_period(mut self, live_period: i64) -> Self {
        self.live_period = Some(live_period);
        self
    }

    pub fn with_heading(mut self, heading: i64) -> Self {
        self.heading = Some(heading);
        self
    }

    pub fn with_proximity_alert_radius(mut self, proximity_alert_radius: i64) -> Self {
        self.proximity_alert_radius = Some(proximity_alert_radius);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for EditMessageLiveLocation {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "editMessageLiveLocation";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.chat_id {
            form = form.text("chat_id", s.to_string());
        }
        if let Some(s) = &self.message_id {
            form = form.text("message_id", s.to_string());
        }
        if let Some(s) = &self.inline_message_id {
            form = form.text("inline_message_id", s.to_string());
        }
        form = form.text("latitude", self.latitude.to_string());
        form = form.text("longitude", self.longitude.to_string());
        if let Some(s) = &self.horizontal_accuracy {
            form = form.text("horizontal_accuracy", s.to_string());
        }
        if let Some(s) = &self.heading {
            form = form.text("heading", s.to_string());
        }
        if let Some(s) = &self.proximity_alert_radius {
            form = form.text("proximity_alert_radius", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl EditMessageLiveLocation {
    pub fn new(latitude: f64, longitude: f64, ) -> Self {
        Self {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            latitude,
            longitude,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }
}

impl EditMessageLiveLocation {
    pub fn with_chat_id(mut self, chat_id: ChatId) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn with_message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn with_inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

    pub fn with_horizontal_accuracy(mut self, horizontal_accuracy: f64) -> Self {
        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }

    pub fn with_heading(mut self, heading: i64) -> Self {
        self.heading = Some(heading);
        self
    }

    pub fn with_proximity_alert_radius(mut self, proximity_alert_radius: i64) -> Self {
        self.proximity_alert_radius = Some(proximity_alert_radius);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for StopMessageLiveLocation {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "stopMessageLiveLocation";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.chat_id {
            form = form.text("chat_id", s.to_string());
        }
        if let Some(s) = &self.message_id {
            form = form.text("message_id", s.to_string());
        }
        if let Some(s) = &self.inline_message_id {
            form = form.text("inline_message_id", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl StopMessageLiveLocation {
    pub fn with_chat_id(mut self, chat_id: ChatId) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn with_message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn with_inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for SendVenue {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendVenue";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("latitude", self.latitude.to_string());
        form = form.text("longitude", self.longitude.to_string());
        form = form.text("title", self.title.to_string());
        form = form.text("address", self.address.to_string());
        if let Some(s) = &self.foursquare_id {
            form = form.text("foursquare_id", s.to_string());
        }
        if let Some(s) = &self.foursquare_type {
            form = form.text("foursquare_type", s.to_string());
        }
        if let Some(s) = &self.google_place_id {
            form = form.text("google_place_id", s.to_string());
        }
        if let Some(s) = &self.google_place_type {
            form = form.text("google_place_type", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendVenue {
    pub fn new(chat_id: ChatId, latitude: f64, longitude: f64, title: String, address: String, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendVenue {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_foursquare_id(mut self, foursquare_id: String) -> Self {
        self.foursquare_id = Some(foursquare_id);
        self
    }

    pub fn with_foursquare_type(mut self, foursquare_type: String) -> Self {
        self.foursquare_type = Some(foursquare_type);
        self
    }

    pub fn with_google_place_id(mut self, google_place_id: String) -> Self {
        self.google_place_id = Some(google_place_id);
        self
    }

    pub fn with_google_place_type(mut self, google_place_type: String) -> Self {
        self.google_place_type = Some(google_place_type);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for SendContact {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendContact";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("phone_number", self.phone_number.to_string());
        form = form.text("first_name", self.first_name.to_string());
        if let Some(s) = &self.last_name {
            form = form.text("last_name", s.to_string());
        }
        if let Some(s) = &self.vcard {
            form = form.text("vcard", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendContact {
    pub fn new(chat_id: ChatId, phone_number: String, first_name: String, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendContact {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn with_vcard(mut self, vcard: String) -> Self {
        self.vcard = Some(vcard);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for SendPoll {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendPoll";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("question", self.question.to_string());
        form = form.text("options", serde_json::to_string(&self.options).unwrap());
        if let Some(s) = &self.is_anonymous {
            form = form.text("is_anonymous", s.to_string());
        }
        if let Some(s) = &self.type_ {
            form = form.text("type", s.to_string());
        }
        if let Some(s) = &self.allows_multiple_answers {
            form = form.text("allows_multiple_answers", s.to_string());
        }
        if let Some(s) = &self.correct_option_id {
            form = form.text("correct_option_id", s.to_string());
        }
        if let Some(s) = &self.explanation {
            form = form.text("explanation", s.to_string());
        }
        if let Some(s) = &self.explanation_parse_mode {
            form = form.text("explanation_parse_mode", s.to_string());
        }
        if let Some(s) = &self.explanation_entities {
            form = form.text("explanation_entities", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.open_period {
            form = form.text("open_period", s.to_string());
        }
        if let Some(s) = &self.close_date {
            form = form.text("close_date", s.to_string());
        }
        if let Some(s) = &self.is_closed {
            form = form.text("is_closed", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendPoll {
    pub fn new(chat_id: ChatId, question: String, options: Vec<String>, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            question,
            options,
            is_anonymous: None,
            type_: None,
            allows_multiple_answers: None,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendPoll {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }

    pub fn with_type(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn with_allows_multiple_answers(mut self, allows_multiple_answers: bool) -> Self {
        self.allows_multiple_answers = Some(allows_multiple_answers);
        self
    }

    pub fn with_correct_option_id(mut self, correct_option_id: i64) -> Self {
        self.correct_option_id = Some(correct_option_id);
        self
    }

    pub fn with_explanation(mut self, explanation: String) -> Self {
        self.explanation = Some(explanation);
        self
    }

    pub fn with_explanation_parse_mode(mut self, explanation_parse_mode: String) -> Self {
        self.explanation_parse_mode = Some(explanation_parse_mode);
        self
    }

    pub fn with_explanation_entities(mut self, explanation_entities: Vec<MessageEntity>) -> Self {
        self.explanation_entities = Some(explanation_entities);
        self
    }

    pub fn with_open_period(mut self, open_period: i64) -> Self {
        self.open_period = Some(open_period);
        self
    }

    pub fn with_close_date(mut self, close_date: i64) -> Self {
        self.close_date = Some(close_date);
        self
    }

    pub fn with_is_closed(mut self, is_closed: bool) -> Self {
        self.is_closed = Some(is_closed);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for SendDice {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendDice";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        if let Some(s) = &self.emoji {
            form = form.text("emoji", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendDice {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            emoji: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendDice {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_emoji(mut self, emoji: String) -> Self {
        self.emoji = Some(emoji);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for SendChatAction {
    type ResponseType = bool;
    const PATH: &'static str = "sendChatAction";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("action", self.action.to_string());
        form
    }
}

impl SendChatAction {
    pub fn new(chat_id: ChatId, action: String, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            action,
        }
    }
}

impl SendChatAction {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

}

impl TgMethod for GetUserProfilePhotos {
    type ResponseType = UserProfilePhotos;
    const PATH: &'static str = "getUserProfilePhotos";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("user_id", self.user_id.to_string());
        if let Some(s) = &self.offset {
            form = form.text("offset", s.to_string());
        }
        if let Some(s) = &self.limit {
            form = form.text("limit", s.to_string());
        }
        form
    }
}

impl GetUserProfilePhotos {
    pub fn new(user_id: i64, ) -> Self {
        Self {
            user_id,
            offset: None,
            limit: None,
        }
    }
}

impl GetUserProfilePhotos {
    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

}

impl TgMethod for GetFile {
    type ResponseType = File;
    const PATH: &'static str = "getFile";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("file_id", self.file_id.to_string());
        form
    }
}

impl GetFile {
    pub fn new(file_id: String, ) -> Self {
        Self {
            file_id,
        }
    }
}

impl TgMethod for BanChatMember {
    type ResponseType = bool;
    const PATH: &'static str = "banChatMember";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("user_id", self.user_id.to_string());
        if let Some(s) = &self.until_date {
            form = form.text("until_date", s.to_string());
        }
        if let Some(s) = &self.revoke_messages {
            form = form.text("revoke_messages", s.to_string());
        }
        form
    }
}

impl BanChatMember {
    pub fn new(chat_id: ChatId, user_id: i64, ) -> Self {
        Self {
            chat_id,
            user_id,
            until_date: None,
            revoke_messages: None,
        }
    }
}

impl BanChatMember {
    pub fn with_until_date(mut self, until_date: i64) -> Self {
        self.until_date = Some(until_date);
        self
    }

    pub fn with_revoke_messages(mut self, revoke_messages: bool) -> Self {
        self.revoke_messages = Some(revoke_messages);
        self
    }

}

impl TgMethod for UnbanChatMember {
    type ResponseType = bool;
    const PATH: &'static str = "unbanChatMember";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("user_id", self.user_id.to_string());
        if let Some(s) = &self.only_if_banned {
            form = form.text("only_if_banned", s.to_string());
        }
        form
    }
}

impl UnbanChatMember {
    pub fn new(chat_id: ChatId, user_id: i64, ) -> Self {
        Self {
            chat_id,
            user_id,
            only_if_banned: None,
        }
    }
}

impl UnbanChatMember {
    pub fn with_only_if_banned(mut self, only_if_banned: bool) -> Self {
        self.only_if_banned = Some(only_if_banned);
        self
    }

}

impl TgMethod for RestrictChatMember {
    type ResponseType = bool;
    const PATH: &'static str = "restrictChatMember";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("user_id", self.user_id.to_string());
        form = form.text("permissions", serde_json::to_string(&self.permissions).unwrap());
        if let Some(s) = &self.use_independent_chat_permissions {
            form = form.text("use_independent_chat_permissions", s.to_string());
        }
        if let Some(s) = &self.until_date {
            form = form.text("until_date", s.to_string());
        }
        form
    }
}

impl RestrictChatMember {
    pub fn new(chat_id: ChatId, user_id: i64, permissions: ChatPermissions, ) -> Self {
        Self {
            chat_id,
            user_id,
            permissions,
            use_independent_chat_permissions: None,
            until_date: None,
        }
    }
}

impl RestrictChatMember {
    pub fn with_use_independent_chat_permissions(mut self, use_independent_chat_permissions: bool) -> Self {
        self.use_independent_chat_permissions = Some(use_independent_chat_permissions);
        self
    }

    pub fn with_until_date(mut self, until_date: i64) -> Self {
        self.until_date = Some(until_date);
        self
    }

}

impl TgMethod for PromoteChatMember {
    type ResponseType = bool;
    const PATH: &'static str = "promoteChatMember";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("user_id", self.user_id.to_string());
        if let Some(s) = &self.is_anonymous {
            form = form.text("is_anonymous", s.to_string());
        }
        if let Some(s) = &self.can_manage_chat {
            form = form.text("can_manage_chat", s.to_string());
        }
        if let Some(s) = &self.can_post_messages {
            form = form.text("can_post_messages", s.to_string());
        }
        if let Some(s) = &self.can_edit_messages {
            form = form.text("can_edit_messages", s.to_string());
        }
        if let Some(s) = &self.can_delete_messages {
            form = form.text("can_delete_messages", s.to_string());
        }
        if let Some(s) = &self.can_manage_video_chats {
            form = form.text("can_manage_video_chats", s.to_string());
        }
        if let Some(s) = &self.can_restrict_members {
            form = form.text("can_restrict_members", s.to_string());
        }
        if let Some(s) = &self.can_promote_members {
            form = form.text("can_promote_members", s.to_string());
        }
        if let Some(s) = &self.can_change_info {
            form = form.text("can_change_info", s.to_string());
        }
        if let Some(s) = &self.can_invite_users {
            form = form.text("can_invite_users", s.to_string());
        }
        if let Some(s) = &self.can_pin_messages {
            form = form.text("can_pin_messages", s.to_string());
        }
        if let Some(s) = &self.can_manage_topics {
            form = form.text("can_manage_topics", s.to_string());
        }
        form
    }
}

impl PromoteChatMember {
    pub fn new(chat_id: ChatId, user_id: i64, ) -> Self {
        Self {
            chat_id,
            user_id,
            is_anonymous: None,
            can_manage_chat: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_delete_messages: None,
            can_manage_video_chats: None,
            can_restrict_members: None,
            can_promote_members: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
            can_manage_topics: None,
        }
    }
}

impl PromoteChatMember {
    pub fn with_is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }

    pub fn with_can_manage_chat(mut self, can_manage_chat: bool) -> Self {
        self.can_manage_chat = Some(can_manage_chat);
        self
    }

    pub fn with_can_post_messages(mut self, can_post_messages: bool) -> Self {
        self.can_post_messages = Some(can_post_messages);
        self
    }

    pub fn with_can_edit_messages(mut self, can_edit_messages: bool) -> Self {
        self.can_edit_messages = Some(can_edit_messages);
        self
    }

    pub fn with_can_delete_messages(mut self, can_delete_messages: bool) -> Self {
        self.can_delete_messages = Some(can_delete_messages);
        self
    }

    pub fn with_can_manage_video_chats(mut self, can_manage_video_chats: bool) -> Self {
        self.can_manage_video_chats = Some(can_manage_video_chats);
        self
    }

    pub fn with_can_restrict_members(mut self, can_restrict_members: bool) -> Self {
        self.can_restrict_members = Some(can_restrict_members);
        self
    }

    pub fn with_can_promote_members(mut self, can_promote_members: bool) -> Self {
        self.can_promote_members = Some(can_promote_members);
        self
    }

    pub fn with_can_change_info(mut self, can_change_info: bool) -> Self {
        self.can_change_info = Some(can_change_info);
        self
    }

    pub fn with_can_invite_users(mut self, can_invite_users: bool) -> Self {
        self.can_invite_users = Some(can_invite_users);
        self
    }

    pub fn with_can_pin_messages(mut self, can_pin_messages: bool) -> Self {
        self.can_pin_messages = Some(can_pin_messages);
        self
    }

    pub fn with_can_manage_topics(mut self, can_manage_topics: bool) -> Self {
        self.can_manage_topics = Some(can_manage_topics);
        self
    }

}

impl TgMethod for SetChatAdministratorCustomTitle {
    type ResponseType = bool;
    const PATH: &'static str = "setChatAdministratorCustomTitle";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("user_id", self.user_id.to_string());
        form = form.text("custom_title", self.custom_title.to_string());
        form
    }
}

impl SetChatAdministratorCustomTitle {
    pub fn new(chat_id: ChatId, user_id: i64, custom_title: String, ) -> Self {
        Self {
            chat_id,
            user_id,
            custom_title,
        }
    }
}

impl TgMethod for BanChatSenderChat {
    type ResponseType = bool;
    const PATH: &'static str = "banChatSenderChat";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("sender_chat_id", self.sender_chat_id.to_string());
        form
    }
}

impl BanChatSenderChat {
    pub fn new(chat_id: ChatId, sender_chat_id: i64, ) -> Self {
        Self {
            chat_id,
            sender_chat_id,
        }
    }
}

impl TgMethod for UnbanChatSenderChat {
    type ResponseType = bool;
    const PATH: &'static str = "unbanChatSenderChat";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("sender_chat_id", self.sender_chat_id.to_string());
        form
    }
}

impl UnbanChatSenderChat {
    pub fn new(chat_id: ChatId, sender_chat_id: i64, ) -> Self {
        Self {
            chat_id,
            sender_chat_id,
        }
    }
}

impl TgMethod for SetChatPermissions {
    type ResponseType = bool;
    const PATH: &'static str = "setChatPermissions";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("permissions", serde_json::to_string(&self.permissions).unwrap());
        if let Some(s) = &self.use_independent_chat_permissions {
            form = form.text("use_independent_chat_permissions", s.to_string());
        }
        form
    }
}

impl SetChatPermissions {
    pub fn new(chat_id: ChatId, permissions: ChatPermissions, ) -> Self {
        Self {
            chat_id,
            permissions,
            use_independent_chat_permissions: None,
        }
    }
}

impl SetChatPermissions {
    pub fn with_use_independent_chat_permissions(mut self, use_independent_chat_permissions: bool) -> Self {
        self.use_independent_chat_permissions = Some(use_independent_chat_permissions);
        self
    }

}

impl TgMethod for ExportChatInviteLink {
    type ResponseType = String;
    const PATH: &'static str = "exportChatInviteLink";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}

impl ExportChatInviteLink {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl TgMethod for CreateChatInviteLink {
    type ResponseType = ChatInviteLink;
    const PATH: &'static str = "createChatInviteLink";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.name {
            form = form.text("name", s.to_string());
        }
        if let Some(s) = &self.expire_date {
            form = form.text("expire_date", s.to_string());
        }
        if let Some(s) = &self.member_limit {
            form = form.text("member_limit", s.to_string());
        }
        if let Some(s) = &self.creates_join_request {
            form = form.text("creates_join_request", s.to_string());
        }
        form
    }
}

impl CreateChatInviteLink {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }
}

impl CreateChatInviteLink {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_expire_date(mut self, expire_date: i64) -> Self {
        self.expire_date = Some(expire_date);
        self
    }

    pub fn with_member_limit(mut self, member_limit: i64) -> Self {
        self.member_limit = Some(member_limit);
        self
    }

    pub fn with_creates_join_request(mut self, creates_join_request: bool) -> Self {
        self.creates_join_request = Some(creates_join_request);
        self
    }

}

impl TgMethod for EditChatInviteLink {
    type ResponseType = ChatInviteLink;
    const PATH: &'static str = "editChatInviteLink";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("invite_link", self.invite_link.to_string());
        if let Some(s) = &self.name {
            form = form.text("name", s.to_string());
        }
        if let Some(s) = &self.expire_date {
            form = form.text("expire_date", s.to_string());
        }
        if let Some(s) = &self.member_limit {
            form = form.text("member_limit", s.to_string());
        }
        if let Some(s) = &self.creates_join_request {
            form = form.text("creates_join_request", s.to_string());
        }
        form
    }
}

impl EditChatInviteLink {
    pub fn new(chat_id: ChatId, invite_link: String, ) -> Self {
        Self {
            chat_id,
            invite_link,
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }
}

impl EditChatInviteLink {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_expire_date(mut self, expire_date: i64) -> Self {
        self.expire_date = Some(expire_date);
        self
    }

    pub fn with_member_limit(mut self, member_limit: i64) -> Self {
        self.member_limit = Some(member_limit);
        self
    }

    pub fn with_creates_join_request(mut self, creates_join_request: bool) -> Self {
        self.creates_join_request = Some(creates_join_request);
        self
    }

}

impl TgMethod for RevokeChatInviteLink {
    type ResponseType = ChatInviteLink;
    const PATH: &'static str = "revokeChatInviteLink";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("invite_link", self.invite_link.to_string());
        form
    }
}

impl RevokeChatInviteLink {
    pub fn new(chat_id: ChatId, invite_link: String, ) -> Self {
        Self {
            chat_id,
            invite_link,
        }
    }
}

impl TgMethod for ApproveChatJoinRequest {
    type ResponseType = bool;
    const PATH: &'static str = "approveChatJoinRequest";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("user_id", self.user_id.to_string());
        form
    }
}

impl ApproveChatJoinRequest {
    pub fn new(chat_id: ChatId, user_id: i64, ) -> Self {
        Self {
            chat_id,
            user_id,
        }
    }
}

impl TgMethod for DeclineChatJoinRequest {
    type ResponseType = bool;
    const PATH: &'static str = "declineChatJoinRequest";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("user_id", self.user_id.to_string());
        form
    }
}

impl DeclineChatJoinRequest {
    pub fn new(chat_id: ChatId, user_id: i64, ) -> Self {
        Self {
            chat_id,
            user_id,
        }
    }
}

impl TgMethod for SetChatPhoto {
    type ResponseType = bool;
    const PATH: &'static str = "setChatPhoto";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.photo.add_file(form);
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("photo", self.photo.to_string());
        form
    }
}

impl SetChatPhoto {
    pub fn new(chat_id: ChatId, photo: InputFile, ) -> Self {
        Self {
            chat_id,
            photo,
        }
    }
}

impl crate::TgObject for SetChatPhoto {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.photo.add_file(form);
        form
    }
}

impl TgMethod for DeleteChatPhoto {
    type ResponseType = bool;
    const PATH: &'static str = "deleteChatPhoto";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}

impl DeleteChatPhoto {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl TgMethod for SetChatTitle {
    type ResponseType = bool;
    const PATH: &'static str = "setChatTitle";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("title", self.title.to_string());
        form
    }
}

impl SetChatTitle {
    pub fn new(chat_id: ChatId, title: String, ) -> Self {
        Self {
            chat_id,
            title,
        }
    }
}

impl TgMethod for SetChatDescription {
    type ResponseType = bool;
    const PATH: &'static str = "setChatDescription";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.description {
            form = form.text("description", s.to_string());
        }
        form
    }
}

impl SetChatDescription {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
            description: None,
        }
    }
}

impl SetChatDescription {
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

}

impl TgMethod for PinChatMessage {
    type ResponseType = bool;
    const PATH: &'static str = "pinChatMessage";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("message_id", self.message_id.to_string());
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        form
    }
}

impl PinChatMessage {
    pub fn new(chat_id: ChatId, message_id: i64, ) -> Self {
        Self {
            chat_id,
            message_id,
            disable_notification: None,
        }
    }
}

impl PinChatMessage {
    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

}

impl TgMethod for UnpinChatMessage {
    type ResponseType = bool;
    const PATH: &'static str = "unpinChatMessage";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_id {
            form = form.text("message_id", s.to_string());
        }
        form
    }
}

impl UnpinChatMessage {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
            message_id: None,
        }
    }
}

impl UnpinChatMessage {
    pub fn with_message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

}

impl TgMethod for UnpinAllChatMessages {
    type ResponseType = bool;
    const PATH: &'static str = "unpinAllChatMessages";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}

impl UnpinAllChatMessages {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl TgMethod for LeaveChat {
    type ResponseType = bool;
    const PATH: &'static str = "leaveChat";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}

impl LeaveChat {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl TgMethod for GetChat {
    type ResponseType = Chat;
    const PATH: &'static str = "getChat";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}

impl GetChat {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl TgMethod for GetChatAdministrators {
    type ResponseType = Vec<ChatMember>;
    const PATH: &'static str = "getChatAdministrators";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}

impl GetChatAdministrators {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl TgMethod for GetChatMemberCount {
    type ResponseType = i64;
    const PATH: &'static str = "getChatMemberCount";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}

impl GetChatMemberCount {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl TgMethod for GetChatMember {
    type ResponseType = ChatMember;
    const PATH: &'static str = "getChatMember";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("user_id", self.user_id.to_string());
        form
    }
}

impl GetChatMember {
    pub fn new(chat_id: ChatId, user_id: i64, ) -> Self {
        Self {
            chat_id,
            user_id,
        }
    }
}

impl TgMethod for SetChatStickerSet {
    type ResponseType = bool;
    const PATH: &'static str = "setChatStickerSet";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("sticker_set_name", self.sticker_set_name.to_string());
        form
    }
}

impl SetChatStickerSet {
    pub fn new(chat_id: ChatId, sticker_set_name: String, ) -> Self {
        Self {
            chat_id,
            sticker_set_name,
        }
    }
}

impl TgMethod for DeleteChatStickerSet {
    type ResponseType = bool;
    const PATH: &'static str = "deleteChatStickerSet";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}

impl DeleteChatStickerSet {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl TgMethod for GetForumTopicIconStickers {
    type ResponseType = Vec<Sticker>;
    const PATH: &'static str = "getForumTopicIconStickers";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form
    }
}

impl TgMethod for CreateForumTopic {
    type ResponseType = ForumTopic;
    const PATH: &'static str = "createForumTopic";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("name", self.name.to_string());
        if let Some(s) = &self.icon_color {
            form = form.text("icon_color", s.to_string());
        }
        if let Some(s) = &self.icon_custom_emoji_id {
            form = form.text("icon_custom_emoji_id", s.to_string());
        }
        form
    }
}

impl CreateForumTopic {
    pub fn new(chat_id: ChatId, name: String, ) -> Self {
        Self {
            chat_id,
            name,
            icon_color: None,
            icon_custom_emoji_id: None,
        }
    }
}

impl CreateForumTopic {
    pub fn with_icon_color(mut self, icon_color: i64) -> Self {
        self.icon_color = Some(icon_color);
        self
    }

    pub fn with_icon_custom_emoji_id(mut self, icon_custom_emoji_id: String) -> Self {
        self.icon_custom_emoji_id = Some(icon_custom_emoji_id);
        self
    }

}

impl TgMethod for EditForumTopic {
    type ResponseType = bool;
    const PATH: &'static str = "editForumTopic";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("message_thread_id", self.message_thread_id.to_string());
        if let Some(s) = &self.name {
            form = form.text("name", s.to_string());
        }
        if let Some(s) = &self.icon_custom_emoji_id {
            form = form.text("icon_custom_emoji_id", s.to_string());
        }
        form
    }
}

impl EditForumTopic {
    pub fn new(chat_id: ChatId, message_thread_id: i64, ) -> Self {
        Self {
            chat_id,
            message_thread_id,
            name: None,
            icon_custom_emoji_id: None,
        }
    }
}

impl EditForumTopic {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_icon_custom_emoji_id(mut self, icon_custom_emoji_id: String) -> Self {
        self.icon_custom_emoji_id = Some(icon_custom_emoji_id);
        self
    }

}

impl TgMethod for CloseForumTopic {
    type ResponseType = bool;
    const PATH: &'static str = "closeForumTopic";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("message_thread_id", self.message_thread_id.to_string());
        form
    }
}

impl CloseForumTopic {
    pub fn new(chat_id: ChatId, message_thread_id: i64, ) -> Self {
        Self {
            chat_id,
            message_thread_id,
        }
    }
}

impl TgMethod for ReopenForumTopic {
    type ResponseType = bool;
    const PATH: &'static str = "reopenForumTopic";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("message_thread_id", self.message_thread_id.to_string());
        form
    }
}

impl ReopenForumTopic {
    pub fn new(chat_id: ChatId, message_thread_id: i64, ) -> Self {
        Self {
            chat_id,
            message_thread_id,
        }
    }
}

impl TgMethod for DeleteForumTopic {
    type ResponseType = bool;
    const PATH: &'static str = "deleteForumTopic";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("message_thread_id", self.message_thread_id.to_string());
        form
    }
}

impl DeleteForumTopic {
    pub fn new(chat_id: ChatId, message_thread_id: i64, ) -> Self {
        Self {
            chat_id,
            message_thread_id,
        }
    }
}

impl TgMethod for UnpinAllForumTopicMessages {
    type ResponseType = bool;
    const PATH: &'static str = "unpinAllForumTopicMessages";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("message_thread_id", self.message_thread_id.to_string());
        form
    }
}

impl UnpinAllForumTopicMessages {
    pub fn new(chat_id: ChatId, message_thread_id: i64, ) -> Self {
        Self {
            chat_id,
            message_thread_id,
        }
    }
}

impl TgMethod for EditGeneralForumTopic {
    type ResponseType = bool;
    const PATH: &'static str = "editGeneralForumTopic";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("name", self.name.to_string());
        form
    }
}

impl EditGeneralForumTopic {
    pub fn new(chat_id: ChatId, name: String, ) -> Self {
        Self {
            chat_id,
            name,
        }
    }
}

impl TgMethod for CloseGeneralForumTopic {
    type ResponseType = bool;
    const PATH: &'static str = "closeGeneralForumTopic";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}

impl CloseGeneralForumTopic {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl TgMethod for ReopenGeneralForumTopic {
    type ResponseType = bool;
    const PATH: &'static str = "reopenGeneralForumTopic";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}

impl ReopenGeneralForumTopic {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl TgMethod for HideGeneralForumTopic {
    type ResponseType = bool;
    const PATH: &'static str = "hideGeneralForumTopic";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}

impl HideGeneralForumTopic {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl TgMethod for UnhideGeneralForumTopic {
    type ResponseType = bool;
    const PATH: &'static str = "unhideGeneralForumTopic";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form
    }
}

impl UnhideGeneralForumTopic {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl TgMethod for AnswerCallbackQuery {
    type ResponseType = bool;
    const PATH: &'static str = "answerCallbackQuery";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("callback_query_id", self.callback_query_id.to_string());
        if let Some(s) = &self.text {
            form = form.text("text", s.to_string());
        }
        if let Some(s) = &self.show_alert {
            form = form.text("show_alert", s.to_string());
        }
        if let Some(s) = &self.url {
            form = form.text("url", s.to_string());
        }
        if let Some(s) = &self.cache_time {
            form = form.text("cache_time", s.to_string());
        }
        form
    }
}

impl AnswerCallbackQuery {
    pub fn new(callback_query_id: String, ) -> Self {
        Self {
            callback_query_id,
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }
}

impl AnswerCallbackQuery {
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn with_show_alert(mut self, show_alert: bool) -> Self {
        self.show_alert = Some(show_alert);
        self
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn with_cache_time(mut self, cache_time: i64) -> Self {
        self.cache_time = Some(cache_time);
        self
    }

}

impl TgMethod for SetMyCommands {
    type ResponseType = bool;
    const PATH: &'static str = "setMyCommands";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("commands", serde_json::to_string(&self.commands).unwrap());
        if let Some(s) = &self.scope {
            form = form.text("scope", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.language_code {
            form = form.text("language_code", s.to_string());
        }
        form
    }
}

impl SetMyCommands {
    pub fn new(commands: Vec<BotCommand>, ) -> Self {
        Self {
            commands,
            scope: None,
            language_code: None,
        }
    }
}

impl SetMyCommands {
    pub fn with_scope(mut self, scope: BotCommandScope) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn with_language_code(mut self, language_code: String) -> Self {
        self.language_code = Some(language_code);
        self
    }

}

impl TgMethod for DeleteMyCommands {
    type ResponseType = bool;
    const PATH: &'static str = "deleteMyCommands";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.scope {
            form = form.text("scope", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.language_code {
            form = form.text("language_code", s.to_string());
        }
        form
    }
}

impl DeleteMyCommands {
    pub fn with_scope(mut self, scope: BotCommandScope) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn with_language_code(mut self, language_code: String) -> Self {
        self.language_code = Some(language_code);
        self
    }

}

impl TgMethod for GetMyCommands {
    type ResponseType = Vec<BotCommand>;
    const PATH: &'static str = "getMyCommands";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.scope {
            form = form.text("scope", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.language_code {
            form = form.text("language_code", s.to_string());
        }
        form
    }
}

impl GetMyCommands {
    pub fn with_scope(mut self, scope: BotCommandScope) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn with_language_code(mut self, language_code: String) -> Self {
        self.language_code = Some(language_code);
        self
    }

}

impl TgMethod for SetChatMenuButton {
    type ResponseType = bool;
    const PATH: &'static str = "setChatMenuButton";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.chat_id {
            form = form.text("chat_id", s.to_string());
        }
        if let Some(s) = &self.menu_button {
            form = form.text("menu_button", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SetChatMenuButton {
    pub fn with_chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn with_menu_button(mut self, menu_button: MenuButton) -> Self {
        self.menu_button = Some(menu_button);
        self
    }

}

impl TgMethod for GetChatMenuButton {
    type ResponseType = MenuButton;
    const PATH: &'static str = "getChatMenuButton";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.chat_id {
            form = form.text("chat_id", s.to_string());
        }
        form
    }
}

impl GetChatMenuButton {
    pub fn with_chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

}

impl TgMethod for SetMyDefaultAdministratorRights {
    type ResponseType = bool;
    const PATH: &'static str = "setMyDefaultAdministratorRights";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.rights {
            form = form.text("rights", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.for_channels {
            form = form.text("for_channels", s.to_string());
        }
        form
    }
}

impl SetMyDefaultAdministratorRights {
    pub fn with_rights(mut self, rights: ChatAdministratorRights) -> Self {
        self.rights = Some(rights);
        self
    }

    pub fn with_for_channels(mut self, for_channels: bool) -> Self {
        self.for_channels = Some(for_channels);
        self
    }

}

impl TgMethod for GetMyDefaultAdministratorRights {
    type ResponseType = ChatAdministratorRights;
    const PATH: &'static str = "getMyDefaultAdministratorRights";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.for_channels {
            form = form.text("for_channels", s.to_string());
        }
        form
    }
}

impl GetMyDefaultAdministratorRights {
    pub fn with_for_channels(mut self, for_channels: bool) -> Self {
        self.for_channels = Some(for_channels);
        self
    }

}

impl TgMethod for EditMessageText {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "editMessageText";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.chat_id {
            form = form.text("chat_id", s.to_string());
        }
        if let Some(s) = &self.message_id {
            form = form.text("message_id", s.to_string());
        }
        if let Some(s) = &self.inline_message_id {
            form = form.text("inline_message_id", s.to_string());
        }
        form = form.text("text", self.text.to_string());
        if let Some(s) = &self.parse_mode {
            form = form.text("parse_mode", s.to_string());
        }
        if let Some(s) = &self.entities {
            form = form.text("entities", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.disable_web_page_preview {
            form = form.text("disable_web_page_preview", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl EditMessageText {
    pub fn new(text: String, ) -> Self {
        Self {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            text,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }
}

impl EditMessageText {
    pub fn with_chat_id(mut self, chat_id: ChatId) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn with_message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn with_inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn with_entities(mut self, entities: Vec<MessageEntity>) -> Self {
        self.entities = Some(entities);
        self
    }

    pub fn with_disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for EditMessageCaption {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "editMessageCaption";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.chat_id {
            form = form.text("chat_id", s.to_string());
        }
        if let Some(s) = &self.message_id {
            form = form.text("message_id", s.to_string());
        }
        if let Some(s) = &self.inline_message_id {
            form = form.text("inline_message_id", s.to_string());
        }
        if let Some(s) = &self.caption {
            form = form.text("caption", s.to_string());
        }
        if let Some(s) = &self.parse_mode {
            form = form.text("parse_mode", s.to_string());
        }
        if let Some(s) = &self.caption_entities {
            form = form.text("caption_entities", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl EditMessageCaption {
    pub fn with_chat_id(mut self, chat_id: ChatId) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn with_message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn with_inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

    pub fn with_caption(mut self, caption: String) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn with_caption_entities(mut self, caption_entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for EditMessageMedia {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "editMessageMedia";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.media.add_file(form);
        if let Some(s) = &self.chat_id {
            form = form.text("chat_id", s.to_string());
        }
        if let Some(s) = &self.message_id {
            form = form.text("message_id", s.to_string());
        }
        if let Some(s) = &self.inline_message_id {
            form = form.text("inline_message_id", s.to_string());
        }
        form = form.text("media", serde_json::to_string(&self.media).unwrap());
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl EditMessageMedia {
    pub fn new(media: InputMedia, ) -> Self {
        Self {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            media,
            reply_markup: None,
        }
    }
}

impl EditMessageMedia {
    pub fn with_chat_id(mut self, chat_id: ChatId) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn with_message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn with_inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl crate::TgObject for EditMessageMedia {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.media.add_file(form);
        form
    }
}

impl TgMethod for EditMessageReplyMarkup {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "editMessageReplyMarkup";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = &self.chat_id {
            form = form.text("chat_id", s.to_string());
        }
        if let Some(s) = &self.message_id {
            form = form.text("message_id", s.to_string());
        }
        if let Some(s) = &self.inline_message_id {
            form = form.text("inline_message_id", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl EditMessageReplyMarkup {
    pub fn with_chat_id(mut self, chat_id: ChatId) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn with_message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn with_inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for StopPoll {
    type ResponseType = Poll;
    const PATH: &'static str = "stopPoll";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("message_id", self.message_id.to_string());
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl StopPoll {
    pub fn new(chat_id: ChatId, message_id: i64, ) -> Self {
        Self {
            chat_id,
            message_id,
            reply_markup: None,
        }
    }
}

impl StopPoll {
    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for DeleteMessage {
    type ResponseType = bool;
    const PATH: &'static str = "deleteMessage";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        form = form.text("message_id", self.message_id.to_string());
        form
    }
}

impl DeleteMessage {
    pub fn new(chat_id: ChatId, message_id: i64, ) -> Self {
        Self {
            chat_id,
            message_id,
        }
    }
}

impl TgMethod for SendSticker {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendSticker";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.sticker.add_file(form);
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("sticker", self.sticker.to_string());
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendSticker {
    pub fn new(chat_id: ChatId, sticker: InputFile, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            sticker,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendSticker {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl crate::TgObject for SendSticker {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.sticker.add_file(form);
        form
    }
}

impl TgMethod for GetStickerSet {
    type ResponseType = StickerSet;
    const PATH: &'static str = "getStickerSet";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("name", self.name.to_string());
        form
    }
}

impl GetStickerSet {
    pub fn new(name: String, ) -> Self {
        Self {
            name,
        }
    }
}

impl TgMethod for GetCustomEmojiStickers {
    type ResponseType = Vec<Sticker>;
    const PATH: &'static str = "getCustomEmojiStickers";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("custom_emoji_ids", serde_json::to_string(&self.custom_emoji_ids).unwrap());
        form
    }
}

impl GetCustomEmojiStickers {
    pub fn new(custom_emoji_ids: Vec<String>, ) -> Self {
        Self {
            custom_emoji_ids,
        }
    }
}

impl TgMethod for UploadStickerFile {
    type ResponseType = File;
    const PATH: &'static str = "uploadStickerFile";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.png_sticker.add_file(form);
        form = form.text("user_id", self.user_id.to_string());
        form = form.text("png_sticker", self.png_sticker.to_string());
        form
    }
}

impl UploadStickerFile {
    pub fn new(user_id: i64, png_sticker: InputFile, ) -> Self {
        Self {
            user_id,
            png_sticker,
        }
    }
}

impl crate::TgObject for UploadStickerFile {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.png_sticker.add_file(form);
        form
    }
}

impl TgMethod for CreateNewStickerSet {
    type ResponseType = bool;
    const PATH: &'static str = "createNewStickerSet";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.png_sticker.add_file(form);
        form = self.tgs_sticker.add_file(form);
        form = self.webm_sticker.add_file(form);
        form = form.text("user_id", self.user_id.to_string());
        form = form.text("name", self.name.to_string());
        form = form.text("title", self.title.to_string());
        if let Some(s) = &self.png_sticker {
            form = form.text("png_sticker", s.to_string());
        }
        if let Some(s) = &self.tgs_sticker {
            form = form.text("tgs_sticker", s.to_string());
        }
        if let Some(s) = &self.webm_sticker {
            form = form.text("webm_sticker", s.to_string());
        }
        if let Some(s) = &self.sticker_type {
            form = form.text("sticker_type", s.to_string());
        }
        form = form.text("emojis", self.emojis.to_string());
        if let Some(s) = &self.mask_position {
            form = form.text("mask_position", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl CreateNewStickerSet {
    pub fn new(user_id: i64, name: String, title: String, emojis: String, ) -> Self {
        Self {
            user_id,
            name,
            title,
            png_sticker: None,
            tgs_sticker: None,
            webm_sticker: None,
            sticker_type: None,
            emojis,
            mask_position: None,
        }
    }
}

impl CreateNewStickerSet {
    pub fn with_png_sticker(mut self, png_sticker: InputFile) -> Self {
        self.png_sticker = Some(png_sticker);
        self
    }

    pub fn with_tgs_sticker(mut self, tgs_sticker: InputFile) -> Self {
        self.tgs_sticker = Some(tgs_sticker);
        self
    }

    pub fn with_webm_sticker(mut self, webm_sticker: InputFile) -> Self {
        self.webm_sticker = Some(webm_sticker);
        self
    }

    pub fn with_sticker_type(mut self, sticker_type: String) -> Self {
        self.sticker_type = Some(sticker_type);
        self
    }

    pub fn with_mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }

}

impl crate::TgObject for CreateNewStickerSet {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.png_sticker.add_file(form);
        form = self.tgs_sticker.add_file(form);
        form = self.webm_sticker.add_file(form);
        form
    }
}

impl TgMethod for AddStickerToSet {
    type ResponseType = bool;
    const PATH: &'static str = "addStickerToSet";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.png_sticker.add_file(form);
        form = self.tgs_sticker.add_file(form);
        form = self.webm_sticker.add_file(form);
        form = form.text("user_id", self.user_id.to_string());
        form = form.text("name", self.name.to_string());
        if let Some(s) = &self.png_sticker {
            form = form.text("png_sticker", s.to_string());
        }
        if let Some(s) = &self.tgs_sticker {
            form = form.text("tgs_sticker", s.to_string());
        }
        if let Some(s) = &self.webm_sticker {
            form = form.text("webm_sticker", s.to_string());
        }
        form = form.text("emojis", self.emojis.to_string());
        if let Some(s) = &self.mask_position {
            form = form.text("mask_position", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl AddStickerToSet {
    pub fn new(user_id: i64, name: String, emojis: String, ) -> Self {
        Self {
            user_id,
            name,
            png_sticker: None,
            tgs_sticker: None,
            webm_sticker: None,
            emojis,
            mask_position: None,
        }
    }
}

impl AddStickerToSet {
    pub fn with_png_sticker(mut self, png_sticker: InputFile) -> Self {
        self.png_sticker = Some(png_sticker);
        self
    }

    pub fn with_tgs_sticker(mut self, tgs_sticker: InputFile) -> Self {
        self.tgs_sticker = Some(tgs_sticker);
        self
    }

    pub fn with_webm_sticker(mut self, webm_sticker: InputFile) -> Self {
        self.webm_sticker = Some(webm_sticker);
        self
    }

    pub fn with_mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }

}

impl crate::TgObject for AddStickerToSet {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.png_sticker.add_file(form);
        form = self.tgs_sticker.add_file(form);
        form = self.webm_sticker.add_file(form);
        form
    }
}

impl TgMethod for SetStickerPositionInSet {
    type ResponseType = bool;
    const PATH: &'static str = "setStickerPositionInSet";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("sticker", self.sticker.to_string());
        form = form.text("position", self.position.to_string());
        form
    }
}

impl SetStickerPositionInSet {
    pub fn new(sticker: String, position: i64, ) -> Self {
        Self {
            sticker,
            position,
        }
    }
}

impl TgMethod for DeleteStickerFromSet {
    type ResponseType = bool;
    const PATH: &'static str = "deleteStickerFromSet";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("sticker", self.sticker.to_string());
        form
    }
}

impl DeleteStickerFromSet {
    pub fn new(sticker: String, ) -> Self {
        Self {
            sticker,
        }
    }
}

impl TgMethod for SetStickerSetThumb {
    type ResponseType = bool;
    const PATH: &'static str = "setStickerSetThumb";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = self.thumb.add_file(form);
        form = form.text("name", self.name.to_string());
        form = form.text("user_id", self.user_id.to_string());
        if let Some(s) = &self.thumb {
            form = form.text("thumb", s.to_string());
        }
        form
    }
}

impl SetStickerSetThumb {
    pub fn new(name: String, user_id: i64, ) -> Self {
        Self {
            name,
            user_id,
            thumb: None,
        }
    }
}

impl SetStickerSetThumb {
    pub fn with_thumb(mut self, thumb: InputFile) -> Self {
        self.thumb = Some(thumb);
        self
    }

}

impl crate::TgObject for SetStickerSetThumb {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.thumb.add_file(form);
        form
    }
}

impl TgMethod for AnswerInlineQuery {
    type ResponseType = bool;
    const PATH: &'static str = "answerInlineQuery";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("inline_query_id", self.inline_query_id.to_string());
        form = form.text("results", serde_json::to_string(&self.results).unwrap());
        if let Some(s) = &self.cache_time {
            form = form.text("cache_time", s.to_string());
        }
        if let Some(s) = &self.is_personal {
            form = form.text("is_personal", s.to_string());
        }
        if let Some(s) = &self.next_offset {
            form = form.text("next_offset", s.to_string());
        }
        if let Some(s) = &self.switch_pm_text {
            form = form.text("switch_pm_text", s.to_string());
        }
        if let Some(s) = &self.switch_pm_parameter {
            form = form.text("switch_pm_parameter", s.to_string());
        }
        form
    }
}

impl AnswerInlineQuery {
    pub fn new(inline_query_id: String, results: Vec<InlineQueryResult>, ) -> Self {
        Self {
            inline_query_id,
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }
}

impl AnswerInlineQuery {
    pub fn with_cache_time(mut self, cache_time: i64) -> Self {
        self.cache_time = Some(cache_time);
        self
    }

    pub fn with_is_personal(mut self, is_personal: bool) -> Self {
        self.is_personal = Some(is_personal);
        self
    }

    pub fn with_next_offset(mut self, next_offset: String) -> Self {
        self.next_offset = Some(next_offset);
        self
    }

    pub fn with_switch_pm_text(mut self, switch_pm_text: String) -> Self {
        self.switch_pm_text = Some(switch_pm_text);
        self
    }

    pub fn with_switch_pm_parameter(mut self, switch_pm_parameter: String) -> Self {
        self.switch_pm_parameter = Some(switch_pm_parameter);
        self
    }

}

impl TgMethod for AnswerWebAppQuery {
    type ResponseType = SentWebAppMessage;
    const PATH: &'static str = "answerWebAppQuery";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("web_app_query_id", self.web_app_query_id.to_string());
        form = form.text("result", serde_json::to_string(&self.result).unwrap());
        form
    }
}

impl AnswerWebAppQuery {
    pub fn new(web_app_query_id: String, result: InlineQueryResult, ) -> Self {
        Self {
            web_app_query_id,
            result,
        }
    }
}

impl TgMethod for SendInvoice {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendInvoice";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("title", self.title.to_string());
        form = form.text("description", self.description.to_string());
        form = form.text("payload", self.payload.to_string());
        form = form.text("provider_token", self.provider_token.to_string());
        form = form.text("currency", self.currency.to_string());
        form = form.text("prices", serde_json::to_string(&self.prices).unwrap());
        if let Some(s) = &self.max_tip_amount {
            form = form.text("max_tip_amount", s.to_string());
        }
        if let Some(s) = &self.suggested_tip_amounts {
            form = form.text("suggested_tip_amounts", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.start_parameter {
            form = form.text("start_parameter", s.to_string());
        }
        if let Some(s) = &self.provider_data {
            form = form.text("provider_data", s.to_string());
        }
        if let Some(s) = &self.photo_url {
            form = form.text("photo_url", s.to_string());
        }
        if let Some(s) = &self.photo_size {
            form = form.text("photo_size", s.to_string());
        }
        if let Some(s) = &self.photo_width {
            form = form.text("photo_width", s.to_string());
        }
        if let Some(s) = &self.photo_height {
            form = form.text("photo_height", s.to_string());
        }
        if let Some(s) = &self.need_name {
            form = form.text("need_name", s.to_string());
        }
        if let Some(s) = &self.need_phone_number {
            form = form.text("need_phone_number", s.to_string());
        }
        if let Some(s) = &self.need_email {
            form = form.text("need_email", s.to_string());
        }
        if let Some(s) = &self.need_shipping_address {
            form = form.text("need_shipping_address", s.to_string());
        }
        if let Some(s) = &self.send_phone_number_to_provider {
            form = form.text("send_phone_number_to_provider", s.to_string());
        }
        if let Some(s) = &self.send_email_to_provider {
            form = form.text("send_email_to_provider", s.to_string());
        }
        if let Some(s) = &self.is_flexible {
            form = form.text("is_flexible", s.to_string());
        }
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendInvoice {
    pub fn new(chat_id: ChatId, title: String, description: String, payload: String, provider_token: String, currency: String, prices: Vec<LabeledPrice>, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            start_parameter: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendInvoice {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_max_tip_amount(mut self, max_tip_amount: i64) -> Self {
        self.max_tip_amount = Some(max_tip_amount);
        self
    }

    pub fn with_suggested_tip_amounts(mut self, suggested_tip_amounts: Vec<i64>) -> Self {
        self.suggested_tip_amounts = Some(suggested_tip_amounts);
        self
    }

    pub fn with_start_parameter(mut self, start_parameter: String) -> Self {
        self.start_parameter = Some(start_parameter);
        self
    }

    pub fn with_provider_data(mut self, provider_data: String) -> Self {
        self.provider_data = Some(provider_data);
        self
    }

    pub fn with_photo_url(mut self, photo_url: String) -> Self {
        self.photo_url = Some(photo_url);
        self
    }

    pub fn with_photo_size(mut self, photo_size: i64) -> Self {
        self.photo_size = Some(photo_size);
        self
    }

    pub fn with_photo_width(mut self, photo_width: i64) -> Self {
        self.photo_width = Some(photo_width);
        self
    }

    pub fn with_photo_height(mut self, photo_height: i64) -> Self {
        self.photo_height = Some(photo_height);
        self
    }

    pub fn with_need_name(mut self, need_name: bool) -> Self {
        self.need_name = Some(need_name);
        self
    }

    pub fn with_need_phone_number(mut self, need_phone_number: bool) -> Self {
        self.need_phone_number = Some(need_phone_number);
        self
    }

    pub fn with_need_email(mut self, need_email: bool) -> Self {
        self.need_email = Some(need_email);
        self
    }

    pub fn with_need_shipping_address(mut self, need_shipping_address: bool) -> Self {
        self.need_shipping_address = Some(need_shipping_address);
        self
    }

    pub fn with_send_phone_number_to_provider(mut self, send_phone_number_to_provider: bool) -> Self {
        self.send_phone_number_to_provider = Some(send_phone_number_to_provider);
        self
    }

    pub fn with_send_email_to_provider(mut self, send_email_to_provider: bool) -> Self {
        self.send_email_to_provider = Some(send_email_to_provider);
        self
    }

    pub fn with_is_flexible(mut self, is_flexible: bool) -> Self {
        self.is_flexible = Some(is_flexible);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for CreateInvoiceLink {
    type ResponseType = String;
    const PATH: &'static str = "createInvoiceLink";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("title", self.title.to_string());
        form = form.text("description", self.description.to_string());
        form = form.text("payload", self.payload.to_string());
        form = form.text("provider_token", self.provider_token.to_string());
        form = form.text("currency", self.currency.to_string());
        form = form.text("prices", serde_json::to_string(&self.prices).unwrap());
        if let Some(s) = &self.max_tip_amount {
            form = form.text("max_tip_amount", s.to_string());
        }
        if let Some(s) = &self.suggested_tip_amounts {
            form = form.text("suggested_tip_amounts", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.provider_data {
            form = form.text("provider_data", s.to_string());
        }
        if let Some(s) = &self.photo_url {
            form = form.text("photo_url", s.to_string());
        }
        if let Some(s) = &self.photo_size {
            form = form.text("photo_size", s.to_string());
        }
        if let Some(s) = &self.photo_width {
            form = form.text("photo_width", s.to_string());
        }
        if let Some(s) = &self.photo_height {
            form = form.text("photo_height", s.to_string());
        }
        if let Some(s) = &self.need_name {
            form = form.text("need_name", s.to_string());
        }
        if let Some(s) = &self.need_phone_number {
            form = form.text("need_phone_number", s.to_string());
        }
        if let Some(s) = &self.need_email {
            form = form.text("need_email", s.to_string());
        }
        if let Some(s) = &self.need_shipping_address {
            form = form.text("need_shipping_address", s.to_string());
        }
        if let Some(s) = &self.send_phone_number_to_provider {
            form = form.text("send_phone_number_to_provider", s.to_string());
        }
        if let Some(s) = &self.send_email_to_provider {
            form = form.text("send_email_to_provider", s.to_string());
        }
        if let Some(s) = &self.is_flexible {
            form = form.text("is_flexible", s.to_string());
        }
        form
    }
}

impl CreateInvoiceLink {
    pub fn new(title: String, description: String, payload: String, provider_token: String, currency: String, prices: Vec<LabeledPrice>, ) -> Self {
        Self {
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
        }
    }
}

impl CreateInvoiceLink {
    pub fn with_max_tip_amount(mut self, max_tip_amount: i64) -> Self {
        self.max_tip_amount = Some(max_tip_amount);
        self
    }

    pub fn with_suggested_tip_amounts(mut self, suggested_tip_amounts: Vec<i64>) -> Self {
        self.suggested_tip_amounts = Some(suggested_tip_amounts);
        self
    }

    pub fn with_provider_data(mut self, provider_data: String) -> Self {
        self.provider_data = Some(provider_data);
        self
    }

    pub fn with_photo_url(mut self, photo_url: String) -> Self {
        self.photo_url = Some(photo_url);
        self
    }

    pub fn with_photo_size(mut self, photo_size: i64) -> Self {
        self.photo_size = Some(photo_size);
        self
    }

    pub fn with_photo_width(mut self, photo_width: i64) -> Self {
        self.photo_width = Some(photo_width);
        self
    }

    pub fn with_photo_height(mut self, photo_height: i64) -> Self {
        self.photo_height = Some(photo_height);
        self
    }

    pub fn with_need_name(mut self, need_name: bool) -> Self {
        self.need_name = Some(need_name);
        self
    }

    pub fn with_need_phone_number(mut self, need_phone_number: bool) -> Self {
        self.need_phone_number = Some(need_phone_number);
        self
    }

    pub fn with_need_email(mut self, need_email: bool) -> Self {
        self.need_email = Some(need_email);
        self
    }

    pub fn with_need_shipping_address(mut self, need_shipping_address: bool) -> Self {
        self.need_shipping_address = Some(need_shipping_address);
        self
    }

    pub fn with_send_phone_number_to_provider(mut self, send_phone_number_to_provider: bool) -> Self {
        self.send_phone_number_to_provider = Some(send_phone_number_to_provider);
        self
    }

    pub fn with_send_email_to_provider(mut self, send_email_to_provider: bool) -> Self {
        self.send_email_to_provider = Some(send_email_to_provider);
        self
    }

    pub fn with_is_flexible(mut self, is_flexible: bool) -> Self {
        self.is_flexible = Some(is_flexible);
        self
    }

}

impl TgMethod for AnswerShippingQuery {
    type ResponseType = bool;
    const PATH: &'static str = "answerShippingQuery";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("shipping_query_id", self.shipping_query_id.to_string());
        form = form.text("ok", self.ok.to_string());
        if let Some(s) = &self.shipping_options {
            form = form.text("shipping_options", serde_json::to_string(&s).unwrap());
        }
        if let Some(s) = &self.error_message {
            form = form.text("error_message", s.to_string());
        }
        form
    }
}

impl AnswerShippingQuery {
    pub fn new(shipping_query_id: String, ok: bool, ) -> Self {
        Self {
            shipping_query_id,
            ok,
            shipping_options: None,
            error_message: None,
        }
    }
}

impl AnswerShippingQuery {
    pub fn with_shipping_options(mut self, shipping_options: Vec<ShippingOption>) -> Self {
        self.shipping_options = Some(shipping_options);
        self
    }

    pub fn with_error_message(mut self, error_message: String) -> Self {
        self.error_message = Some(error_message);
        self
    }

}

impl TgMethod for AnswerPreCheckoutQuery {
    type ResponseType = bool;
    const PATH: &'static str = "answerPreCheckoutQuery";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("pre_checkout_query_id", self.pre_checkout_query_id.to_string());
        form = form.text("ok", self.ok.to_string());
        if let Some(s) = &self.error_message {
            form = form.text("error_message", s.to_string());
        }
        form
    }
}

impl AnswerPreCheckoutQuery {
    pub fn new(pre_checkout_query_id: String, ok: bool, ) -> Self {
        Self {
            pre_checkout_query_id,
            ok,
            error_message: None,
        }
    }
}

impl AnswerPreCheckoutQuery {
    pub fn with_error_message(mut self, error_message: String) -> Self {
        self.error_message = Some(error_message);
        self
    }

}

impl TgMethod for SetPassportDataErrors {
    type ResponseType = bool;
    const PATH: &'static str = "setPassportDataErrors";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("user_id", self.user_id.to_string());
        form = form.text("errors", serde_json::to_string(&self.errors).unwrap());
        form
    }
}

impl SetPassportDataErrors {
    pub fn new(user_id: i64, errors: Vec<PassportElementError>, ) -> Self {
        Self {
            user_id,
            errors,
        }
    }
}

impl TgMethod for SendGame {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendGame";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("chat_id", self.chat_id.to_string());
        if let Some(s) = &self.message_thread_id {
            form = form.text("message_thread_id", s.to_string());
        }
        form = form.text("game_short_name", self.game_short_name.to_string());
        if let Some(s) = &self.disable_notification {
            form = form.text("disable_notification", s.to_string());
        }
        if let Some(s) = &self.protect_content {
            form = form.text("protect_content", s.to_string());
        }
        if let Some(s) = &self.reply_to_message_id {
            form = form.text("reply_to_message_id", s.to_string());
        }
        if let Some(s) = &self.allow_sending_without_reply {
            form = form.text("allow_sending_without_reply", s.to_string());
        }
        if let Some(s) = &self.reply_markup {
            form = form.text("reply_markup", serde_json::to_string(&s).unwrap());
        }
        form
    }
}

impl SendGame {
    pub fn new(chat_id: i64, game_short_name: String, ) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            game_short_name,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl SendGame {
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn with_reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    pub fn with_allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl TgMethod for SetGameScore {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "setGameScore";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("user_id", self.user_id.to_string());
        form = form.text("score", self.score.to_string());
        if let Some(s) = &self.force {
            form = form.text("force", s.to_string());
        }
        if let Some(s) = &self.disable_edit_message {
            form = form.text("disable_edit_message", s.to_string());
        }
        if let Some(s) = &self.chat_id {
            form = form.text("chat_id", s.to_string());
        }
        if let Some(s) = &self.message_id {
            form = form.text("message_id", s.to_string());
        }
        if let Some(s) = &self.inline_message_id {
            form = form.text("inline_message_id", s.to_string());
        }
        form
    }
}

impl SetGameScore {
    pub fn new(user_id: i64, score: i64, ) -> Self {
        Self {
            user_id,
            score,
            force: None,
            disable_edit_message: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }
}

impl SetGameScore {
    pub fn with_force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }

    pub fn with_disable_edit_message(mut self, disable_edit_message: bool) -> Self {
        self.disable_edit_message = Some(disable_edit_message);
        self
    }

    pub fn with_chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn with_message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn with_inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

}

impl TgMethod for GetGameHighScores {
    type ResponseType = Vec<GameHighScore>;
    const PATH: &'static str = "getGameHighScores";

    fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("user_id", self.user_id.to_string());
        if let Some(s) = &self.chat_id {
            form = form.text("chat_id", s.to_string());
        }
        if let Some(s) = &self.message_id {
            form = form.text("message_id", s.to_string());
        }
        if let Some(s) = &self.inline_message_id {
            form = form.text("inline_message_id", s.to_string());
        }
        form
    }
}

impl GetGameHighScores {
    pub fn new(user_id: i64, ) -> Self {
        Self {
            user_id,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }
}

impl GetGameHighScores {
    pub fn with_chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn with_message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn with_inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

}

