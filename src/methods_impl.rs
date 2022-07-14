use crate::methods::*;
use crate::types::*;
use crate::form_ser::*;
use crate::helpers::*;
use crate::TgMethod;

impl FormSer for GetUpdates {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.offset.serialize("offset".into(), form);
        form = self.limit.serialize("limit".into(), form);
        form = self.timeout.serialize("timeout".into(), form);
        form = self.allowed_updates.serialize("allowed_updates".into(), form);
        form
    }
}

impl TgMethod for GetUpdates {
    type ResponseType = Vec<Update>;
    const PATH: &'static str = "getUpdates";
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

impl FormSer for SetWebhook {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.url.serialize("url".into(), form);
        form = self.certificate.serialize("certificate".into(), form);
        form = self.ip_address.serialize("ip_address".into(), form);
        form = self.max_connections.serialize("max_connections".into(), form);
        form = self.allowed_updates.serialize("allowed_updates".into(), form);
        form = self.drop_pending_updates.serialize("drop_pending_updates".into(), form);
        form = self.secret_token.serialize("secret_token".into(), form);
        form
    }
}

impl TgMethod for SetWebhook {
    type ResponseType = bool;
    const PATH: &'static str = "setWebhook";
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

impl FormSer for DeleteWebhook {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.drop_pending_updates.serialize("drop_pending_updates".into(), form);
        form
    }
}

impl TgMethod for DeleteWebhook {
    type ResponseType = bool;
    const PATH: &'static str = "deleteWebhook";
}

impl DeleteWebhook {
    pub fn with_drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }

}

impl FormSer for GetWebhookInfo {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form
    }
}

impl TgMethod for GetWebhookInfo {
    type ResponseType = WebhookInfo;
    const PATH: &'static str = "getWebhookInfo";
}

impl FormSer for GetMe {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form
    }
}

impl TgMethod for GetMe {
    type ResponseType = User;
    const PATH: &'static str = "getMe";
}

impl FormSer for LogOut {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form
    }
}

impl TgMethod for LogOut {
    type ResponseType = bool;
    const PATH: &'static str = "logOut";
}

impl FormSer for Close {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form
    }
}

impl TgMethod for Close {
    type ResponseType = bool;
    const PATH: &'static str = "close";
}

impl FormSer for SendMessage {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.text.serialize("text".into(), form);
        form = self.parse_mode.serialize("parse_mode".into(), form);
        form = self.entities.serialize("entities".into(), form);
        form = self.disable_web_page_preview.serialize("disable_web_page_preview".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendMessage {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendMessage";
}

impl SendMessage {
    pub fn new(chat_id: ChatId, text: String, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for ForwardMessage {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.from_chat_id.serialize("from_chat_id".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form
    }
}

impl TgMethod for ForwardMessage {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "forwardMessage";
}

impl ForwardMessage {
    pub fn new(chat_id: ChatId, from_chat_id: ChatId, message_id: i64, ) -> Self {
        Self {
            chat_id,
            from_chat_id,
            disable_notification: None,
            protect_content: None,
            message_id,
        }
    }
}

impl ForwardMessage {
    pub fn with_disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn with_protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

}

impl FormSer for CopyMessage {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.from_chat_id.serialize("from_chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form = self.caption.serialize("caption".into(), form);
        form = self.parse_mode.serialize("parse_mode".into(), form);
        form = self.caption_entities.serialize("caption_entities".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for CopyMessage {
    type ResponseType = MessageId;
    const PATH: &'static str = "copyMessage";
}

impl CopyMessage {
    pub fn new(chat_id: ChatId, from_chat_id: ChatId, message_id: i64, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for SendPhoto {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.photo.serialize("photo".into(), form);
        form = self.caption.serialize("caption".into(), form);
        form = self.parse_mode.serialize("parse_mode".into(), form);
        form = self.caption_entities.serialize("caption_entities".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendPhoto {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendPhoto";
}

impl SendPhoto {
    pub fn new(chat_id: ChatId, photo: InputFile, ) -> Self {
        Self {
            chat_id,
            photo,
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

impl SendPhoto {
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

impl FormSer for SendAudio {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.audio.serialize("audio".into(), form);
        form = self.caption.serialize("caption".into(), form);
        form = self.parse_mode.serialize("parse_mode".into(), form);
        form = self.caption_entities.serialize("caption_entities".into(), form);
        form = self.duration.serialize("duration".into(), form);
        form = self.performer.serialize("performer".into(), form);
        form = self.title.serialize("title".into(), form);
        form = self.thumb.serialize("thumb".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendAudio {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendAudio";
}

impl SendAudio {
    pub fn new(chat_id: ChatId, audio: InputFile, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for SendDocument {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.document.serialize("document".into(), form);
        form = self.thumb.serialize("thumb".into(), form);
        form = self.caption.serialize("caption".into(), form);
        form = self.parse_mode.serialize("parse_mode".into(), form);
        form = self.caption_entities.serialize("caption_entities".into(), form);
        form = self.disable_content_type_detection.serialize("disable_content_type_detection".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendDocument {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendDocument";
}

impl SendDocument {
    pub fn new(chat_id: ChatId, document: InputFile, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for SendVideo {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.video.serialize("video".into(), form);
        form = self.duration.serialize("duration".into(), form);
        form = self.width.serialize("width".into(), form);
        form = self.height.serialize("height".into(), form);
        form = self.thumb.serialize("thumb".into(), form);
        form = self.caption.serialize("caption".into(), form);
        form = self.parse_mode.serialize("parse_mode".into(), form);
        form = self.caption_entities.serialize("caption_entities".into(), form);
        form = self.supports_streaming.serialize("supports_streaming".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendVideo {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendVideo";
}

impl SendVideo {
    pub fn new(chat_id: ChatId, video: InputFile, ) -> Self {
        Self {
            chat_id,
            video,
            duration: None,
            width: None,
            height: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
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

impl FormSer for SendAnimation {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.animation.serialize("animation".into(), form);
        form = self.duration.serialize("duration".into(), form);
        form = self.width.serialize("width".into(), form);
        form = self.height.serialize("height".into(), form);
        form = self.thumb.serialize("thumb".into(), form);
        form = self.caption.serialize("caption".into(), form);
        form = self.parse_mode.serialize("parse_mode".into(), form);
        form = self.caption_entities.serialize("caption_entities".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendAnimation {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendAnimation";
}

impl SendAnimation {
    pub fn new(chat_id: ChatId, animation: InputFile, ) -> Self {
        Self {
            chat_id,
            animation,
            duration: None,
            width: None,
            height: None,
            thumb: None,
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

impl SendAnimation {
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

impl FormSer for SendVoice {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.voice.serialize("voice".into(), form);
        form = self.caption.serialize("caption".into(), form);
        form = self.parse_mode.serialize("parse_mode".into(), form);
        form = self.caption_entities.serialize("caption_entities".into(), form);
        form = self.duration.serialize("duration".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendVoice {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendVoice";
}

impl SendVoice {
    pub fn new(chat_id: ChatId, voice: InputFile, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for SendVideoNote {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.video_note.serialize("video_note".into(), form);
        form = self.duration.serialize("duration".into(), form);
        form = self.length.serialize("length".into(), form);
        form = self.thumb.serialize("thumb".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendVideoNote {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendVideoNote";
}

impl SendVideoNote {
    pub fn new(chat_id: ChatId, video_note: InputFile, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for SendMediaGroup {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.media.serialize("media".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form
    }
}

impl TgMethod for SendMediaGroup {
    type ResponseType = Vec<Message>;
    const PATH: &'static str = "sendMediaGroup";
}

impl SendMediaGroup {
    pub fn new(chat_id: ChatId, media: Vec<InputMedia>, ) -> Self {
        Self {
            chat_id,
            media,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
        }
    }
}

impl SendMediaGroup {
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

impl FormSer for SendLocation {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.latitude.serialize("latitude".into(), form);
        form = self.longitude.serialize("longitude".into(), form);
        form = self.horizontal_accuracy.serialize("horizontal_accuracy".into(), form);
        form = self.live_period.serialize("live_period".into(), form);
        form = self.heading.serialize("heading".into(), form);
        form = self.proximity_alert_radius.serialize("proximity_alert_radius".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendLocation {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendLocation";
}

impl SendLocation {
    pub fn new(chat_id: ChatId, latitude: f64, longitude: f64, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for EditMessageLiveLocation {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form = self.inline_message_id.serialize("inline_message_id".into(), form);
        form = self.latitude.serialize("latitude".into(), form);
        form = self.longitude.serialize("longitude".into(), form);
        form = self.horizontal_accuracy.serialize("horizontal_accuracy".into(), form);
        form = self.heading.serialize("heading".into(), form);
        form = self.proximity_alert_radius.serialize("proximity_alert_radius".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for EditMessageLiveLocation {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "editMessageLiveLocation";
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

impl FormSer for StopMessageLiveLocation {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form = self.inline_message_id.serialize("inline_message_id".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for StopMessageLiveLocation {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "stopMessageLiveLocation";
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

impl FormSer for SendVenue {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.latitude.serialize("latitude".into(), form);
        form = self.longitude.serialize("longitude".into(), form);
        form = self.title.serialize("title".into(), form);
        form = self.address.serialize("address".into(), form);
        form = self.foursquare_id.serialize("foursquare_id".into(), form);
        form = self.foursquare_type.serialize("foursquare_type".into(), form);
        form = self.google_place_id.serialize("google_place_id".into(), form);
        form = self.google_place_type.serialize("google_place_type".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendVenue {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendVenue";
}

impl SendVenue {
    pub fn new(chat_id: ChatId, latitude: f64, longitude: f64, title: String, address: String, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for SendContact {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.phone_number.serialize("phone_number".into(), form);
        form = self.first_name.serialize("first_name".into(), form);
        form = self.last_name.serialize("last_name".into(), form);
        form = self.vcard.serialize("vcard".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendContact {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendContact";
}

impl SendContact {
    pub fn new(chat_id: ChatId, phone_number: String, first_name: String, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for SendPoll {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.question.serialize("question".into(), form);
        form = self.options.serialize("options".into(), form);
        form = self.is_anonymous.serialize("is_anonymous".into(), form);
        form = self.type_.serialize("type".into(), form);
        form = self.allows_multiple_answers.serialize("allows_multiple_answers".into(), form);
        form = self.correct_option_id.serialize("correct_option_id".into(), form);
        form = self.explanation.serialize("explanation".into(), form);
        form = self.explanation_parse_mode.serialize("explanation_parse_mode".into(), form);
        form = self.explanation_entities.serialize("explanation_entities".into(), form);
        form = self.open_period.serialize("open_period".into(), form);
        form = self.close_date.serialize("close_date".into(), form);
        form = self.is_closed.serialize("is_closed".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendPoll {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendPoll";
}

impl SendPoll {
    pub fn new(chat_id: ChatId, question: String, options: Vec<String>, ) -> Self {
        Self {
            chat_id,
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
    pub fn with_is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }

    pub fn with_type_(mut self, type_: String) -> Self {
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

impl FormSer for SendDice {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.emoji.serialize("emoji".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendDice {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendDice";
}

impl SendDice {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for SendChatAction {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.action.serialize("action".into(), form);
        form
    }
}

impl TgMethod for SendChatAction {
    type ResponseType = bool;
    const PATH: &'static str = "sendChatAction";
}

impl SendChatAction {
    pub fn new(chat_id: ChatId, action: String, ) -> Self {
        Self {
            chat_id,
            action,
        }
    }
}

impl FormSer for GetUserProfilePhotos {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.user_id.serialize("user_id".into(), form);
        form = self.offset.serialize("offset".into(), form);
        form = self.limit.serialize("limit".into(), form);
        form
    }
}

impl TgMethod for GetUserProfilePhotos {
    type ResponseType = UserProfilePhotos;
    const PATH: &'static str = "getUserProfilePhotos";
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

impl FormSer for GetFile {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.file_id.serialize("file_id".into(), form);
        form
    }
}

impl TgMethod for GetFile {
    type ResponseType = File;
    const PATH: &'static str = "getFile";
}

impl GetFile {
    pub fn new(file_id: String, ) -> Self {
        Self {
            file_id,
        }
    }
}

impl FormSer for BanChatMember {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.user_id.serialize("user_id".into(), form);
        form = self.until_date.serialize("until_date".into(), form);
        form = self.revoke_messages.serialize("revoke_messages".into(), form);
        form
    }
}

impl TgMethod for BanChatMember {
    type ResponseType = bool;
    const PATH: &'static str = "banChatMember";
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

impl FormSer for UnbanChatMember {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.user_id.serialize("user_id".into(), form);
        form = self.only_if_banned.serialize("only_if_banned".into(), form);
        form
    }
}

impl TgMethod for UnbanChatMember {
    type ResponseType = bool;
    const PATH: &'static str = "unbanChatMember";
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

impl FormSer for RestrictChatMember {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.user_id.serialize("user_id".into(), form);
        form = self.permissions.serialize("permissions".into(), form);
        form = self.until_date.serialize("until_date".into(), form);
        form
    }
}

impl TgMethod for RestrictChatMember {
    type ResponseType = bool;
    const PATH: &'static str = "restrictChatMember";
}

impl RestrictChatMember {
    pub fn new(chat_id: ChatId, user_id: i64, permissions: ChatPermissions, ) -> Self {
        Self {
            chat_id,
            user_id,
            permissions,
            until_date: None,
        }
    }
}

impl RestrictChatMember {
    pub fn with_until_date(mut self, until_date: i64) -> Self {
        self.until_date = Some(until_date);
        self
    }

}

impl FormSer for PromoteChatMember {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.user_id.serialize("user_id".into(), form);
        form = self.is_anonymous.serialize("is_anonymous".into(), form);
        form = self.can_manage_chat.serialize("can_manage_chat".into(), form);
        form = self.can_post_messages.serialize("can_post_messages".into(), form);
        form = self.can_edit_messages.serialize("can_edit_messages".into(), form);
        form = self.can_delete_messages.serialize("can_delete_messages".into(), form);
        form = self.can_manage_video_chats.serialize("can_manage_video_chats".into(), form);
        form = self.can_restrict_members.serialize("can_restrict_members".into(), form);
        form = self.can_promote_members.serialize("can_promote_members".into(), form);
        form = self.can_change_info.serialize("can_change_info".into(), form);
        form = self.can_invite_users.serialize("can_invite_users".into(), form);
        form = self.can_pin_messages.serialize("can_pin_messages".into(), form);
        form
    }
}

impl TgMethod for PromoteChatMember {
    type ResponseType = bool;
    const PATH: &'static str = "promoteChatMember";
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

}

impl FormSer for SetChatAdministratorCustomTitle {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.user_id.serialize("user_id".into(), form);
        form = self.custom_title.serialize("custom_title".into(), form);
        form
    }
}

impl TgMethod for SetChatAdministratorCustomTitle {
    type ResponseType = bool;
    const PATH: &'static str = "setChatAdministratorCustomTitle";
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

impl FormSer for BanChatSenderChat {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.sender_chat_id.serialize("sender_chat_id".into(), form);
        form
    }
}

impl TgMethod for BanChatSenderChat {
    type ResponseType = bool;
    const PATH: &'static str = "banChatSenderChat";
}

impl BanChatSenderChat {
    pub fn new(chat_id: ChatId, sender_chat_id: i64, ) -> Self {
        Self {
            chat_id,
            sender_chat_id,
        }
    }
}

impl FormSer for UnbanChatSenderChat {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.sender_chat_id.serialize("sender_chat_id".into(), form);
        form
    }
}

impl TgMethod for UnbanChatSenderChat {
    type ResponseType = bool;
    const PATH: &'static str = "unbanChatSenderChat";
}

impl UnbanChatSenderChat {
    pub fn new(chat_id: ChatId, sender_chat_id: i64, ) -> Self {
        Self {
            chat_id,
            sender_chat_id,
        }
    }
}

impl FormSer for SetChatPermissions {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.permissions.serialize("permissions".into(), form);
        form
    }
}

impl TgMethod for SetChatPermissions {
    type ResponseType = bool;
    const PATH: &'static str = "setChatPermissions";
}

impl SetChatPermissions {
    pub fn new(chat_id: ChatId, permissions: ChatPermissions, ) -> Self {
        Self {
            chat_id,
            permissions,
        }
    }
}

impl FormSer for ExportChatInviteLink {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form
    }
}

impl TgMethod for ExportChatInviteLink {
    type ResponseType = String;
    const PATH: &'static str = "exportChatInviteLink";
}

impl ExportChatInviteLink {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl FormSer for CreateChatInviteLink {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.name.serialize("name".into(), form);
        form = self.expire_date.serialize("expire_date".into(), form);
        form = self.member_limit.serialize("member_limit".into(), form);
        form = self.creates_join_request.serialize("creates_join_request".into(), form);
        form
    }
}

impl TgMethod for CreateChatInviteLink {
    type ResponseType = ChatInviteLink;
    const PATH: &'static str = "createChatInviteLink";
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

impl FormSer for EditChatInviteLink {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.invite_link.serialize("invite_link".into(), form);
        form = self.name.serialize("name".into(), form);
        form = self.expire_date.serialize("expire_date".into(), form);
        form = self.member_limit.serialize("member_limit".into(), form);
        form = self.creates_join_request.serialize("creates_join_request".into(), form);
        form
    }
}

impl TgMethod for EditChatInviteLink {
    type ResponseType = ChatInviteLink;
    const PATH: &'static str = "editChatInviteLink";
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

impl FormSer for RevokeChatInviteLink {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.invite_link.serialize("invite_link".into(), form);
        form
    }
}

impl TgMethod for RevokeChatInviteLink {
    type ResponseType = ChatInviteLink;
    const PATH: &'static str = "revokeChatInviteLink";
}

impl RevokeChatInviteLink {
    pub fn new(chat_id: ChatId, invite_link: String, ) -> Self {
        Self {
            chat_id,
            invite_link,
        }
    }
}

impl FormSer for ApproveChatJoinRequest {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.user_id.serialize("user_id".into(), form);
        form
    }
}

impl TgMethod for ApproveChatJoinRequest {
    type ResponseType = bool;
    const PATH: &'static str = "approveChatJoinRequest";
}

impl ApproveChatJoinRequest {
    pub fn new(chat_id: ChatId, user_id: i64, ) -> Self {
        Self {
            chat_id,
            user_id,
        }
    }
}

impl FormSer for DeclineChatJoinRequest {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.user_id.serialize("user_id".into(), form);
        form
    }
}

impl TgMethod for DeclineChatJoinRequest {
    type ResponseType = bool;
    const PATH: &'static str = "declineChatJoinRequest";
}

impl DeclineChatJoinRequest {
    pub fn new(chat_id: ChatId, user_id: i64, ) -> Self {
        Self {
            chat_id,
            user_id,
        }
    }
}

impl FormSer for SetChatPhoto {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.photo.serialize("photo".into(), form);
        form
    }
}

impl TgMethod for SetChatPhoto {
    type ResponseType = bool;
    const PATH: &'static str = "setChatPhoto";
}

impl SetChatPhoto {
    pub fn new(chat_id: ChatId, photo: InputFile, ) -> Self {
        Self {
            chat_id,
            photo,
        }
    }
}

impl FormSer for DeleteChatPhoto {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form
    }
}

impl TgMethod for DeleteChatPhoto {
    type ResponseType = bool;
    const PATH: &'static str = "deleteChatPhoto";
}

impl DeleteChatPhoto {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl FormSer for SetChatTitle {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.title.serialize("title".into(), form);
        form
    }
}

impl TgMethod for SetChatTitle {
    type ResponseType = bool;
    const PATH: &'static str = "setChatTitle";
}

impl SetChatTitle {
    pub fn new(chat_id: ChatId, title: String, ) -> Self {
        Self {
            chat_id,
            title,
        }
    }
}

impl FormSer for SetChatDescription {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.description.serialize("description".into(), form);
        form
    }
}

impl TgMethod for SetChatDescription {
    type ResponseType = bool;
    const PATH: &'static str = "setChatDescription";
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

impl FormSer for PinChatMessage {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form
    }
}

impl TgMethod for PinChatMessage {
    type ResponseType = bool;
    const PATH: &'static str = "pinChatMessage";
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

impl FormSer for UnpinChatMessage {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form
    }
}

impl TgMethod for UnpinChatMessage {
    type ResponseType = bool;
    const PATH: &'static str = "unpinChatMessage";
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

impl FormSer for UnpinAllChatMessages {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form
    }
}

impl TgMethod for UnpinAllChatMessages {
    type ResponseType = bool;
    const PATH: &'static str = "unpinAllChatMessages";
}

impl UnpinAllChatMessages {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl FormSer for LeaveChat {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form
    }
}

impl TgMethod for LeaveChat {
    type ResponseType = bool;
    const PATH: &'static str = "leaveChat";
}

impl LeaveChat {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl FormSer for GetChat {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form
    }
}

impl TgMethod for GetChat {
    type ResponseType = Chat;
    const PATH: &'static str = "getChat";
}

impl GetChat {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl FormSer for GetChatAdministrators {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form
    }
}

impl TgMethod for GetChatAdministrators {
    type ResponseType = Vec<ChatMember>;
    const PATH: &'static str = "getChatAdministrators";
}

impl GetChatAdministrators {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl FormSer for GetChatMemberCount {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form
    }
}

impl TgMethod for GetChatMemberCount {
    type ResponseType = i64;
    const PATH: &'static str = "getChatMemberCount";
}

impl GetChatMemberCount {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl FormSer for GetChatMember {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.user_id.serialize("user_id".into(), form);
        form
    }
}

impl TgMethod for GetChatMember {
    type ResponseType = ChatMember;
    const PATH: &'static str = "getChatMember";
}

impl GetChatMember {
    pub fn new(chat_id: ChatId, user_id: i64, ) -> Self {
        Self {
            chat_id,
            user_id,
        }
    }
}

impl FormSer for SetChatStickerSet {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.sticker_set_name.serialize("sticker_set_name".into(), form);
        form
    }
}

impl TgMethod for SetChatStickerSet {
    type ResponseType = bool;
    const PATH: &'static str = "setChatStickerSet";
}

impl SetChatStickerSet {
    pub fn new(chat_id: ChatId, sticker_set_name: String, ) -> Self {
        Self {
            chat_id,
            sticker_set_name,
        }
    }
}

impl FormSer for DeleteChatStickerSet {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form
    }
}

impl TgMethod for DeleteChatStickerSet {
    type ResponseType = bool;
    const PATH: &'static str = "deleteChatStickerSet";
}

impl DeleteChatStickerSet {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl FormSer for AnswerCallbackQuery {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.callback_query_id.serialize("callback_query_id".into(), form);
        form = self.text.serialize("text".into(), form);
        form = self.show_alert.serialize("show_alert".into(), form);
        form = self.url.serialize("url".into(), form);
        form = self.cache_time.serialize("cache_time".into(), form);
        form
    }
}

impl TgMethod for AnswerCallbackQuery {
    type ResponseType = bool;
    const PATH: &'static str = "answerCallbackQuery";
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

impl FormSer for SetMyCommands {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.commands.serialize("commands".into(), form);
        form = self.scope.serialize("scope".into(), form);
        form = self.language_code.serialize("language_code".into(), form);
        form
    }
}

impl TgMethod for SetMyCommands {
    type ResponseType = bool;
    const PATH: &'static str = "setMyCommands";
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

impl FormSer for DeleteMyCommands {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.scope.serialize("scope".into(), form);
        form = self.language_code.serialize("language_code".into(), form);
        form
    }
}

impl TgMethod for DeleteMyCommands {
    type ResponseType = bool;
    const PATH: &'static str = "deleteMyCommands";
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

impl FormSer for GetMyCommands {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.scope.serialize("scope".into(), form);
        form = self.language_code.serialize("language_code".into(), form);
        form
    }
}

impl TgMethod for GetMyCommands {
    type ResponseType = Vec<BotCommand>;
    const PATH: &'static str = "getMyCommands";
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

impl FormSer for SetChatMenuButton {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.menu_button.serialize("menu_button".into(), form);
        form
    }
}

impl TgMethod for SetChatMenuButton {
    type ResponseType = bool;
    const PATH: &'static str = "setChatMenuButton";
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

impl FormSer for GetChatMenuButton {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form
    }
}

impl TgMethod for GetChatMenuButton {
    type ResponseType = MenuButton;
    const PATH: &'static str = "getChatMenuButton";
}

impl GetChatMenuButton {
    pub fn with_chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

}

impl FormSer for SetMyDefaultAdministratorRights {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.rights.serialize("rights".into(), form);
        form = self.for_channels.serialize("for_channels".into(), form);
        form
    }
}

impl TgMethod for SetMyDefaultAdministratorRights {
    type ResponseType = bool;
    const PATH: &'static str = "setMyDefaultAdministratorRights";
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

impl FormSer for GetMyDefaultAdministratorRights {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.for_channels.serialize("for_channels".into(), form);
        form
    }
}

impl TgMethod for GetMyDefaultAdministratorRights {
    type ResponseType = ChatAdministratorRights;
    const PATH: &'static str = "getMyDefaultAdministratorRights";
}

impl GetMyDefaultAdministratorRights {
    pub fn with_for_channels(mut self, for_channels: bool) -> Self {
        self.for_channels = Some(for_channels);
        self
    }

}

impl FormSer for EditMessageText {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form = self.inline_message_id.serialize("inline_message_id".into(), form);
        form = self.text.serialize("text".into(), form);
        form = self.parse_mode.serialize("parse_mode".into(), form);
        form = self.entities.serialize("entities".into(), form);
        form = self.disable_web_page_preview.serialize("disable_web_page_preview".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for EditMessageText {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "editMessageText";
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

impl FormSer for EditMessageCaption {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form = self.inline_message_id.serialize("inline_message_id".into(), form);
        form = self.caption.serialize("caption".into(), form);
        form = self.parse_mode.serialize("parse_mode".into(), form);
        form = self.caption_entities.serialize("caption_entities".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for EditMessageCaption {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "editMessageCaption";
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

impl FormSer for EditMessageMedia {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form = self.inline_message_id.serialize("inline_message_id".into(), form);
        form = self.media.serialize("media".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for EditMessageMedia {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "editMessageMedia";
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

impl FormSer for EditMessageReplyMarkup {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form = self.inline_message_id.serialize("inline_message_id".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for EditMessageReplyMarkup {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "editMessageReplyMarkup";
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

impl FormSer for StopPoll {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for StopPoll {
    type ResponseType = Poll;
    const PATH: &'static str = "stopPoll";
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

impl FormSer for DeleteMessage {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form
    }
}

impl TgMethod for DeleteMessage {
    type ResponseType = bool;
    const PATH: &'static str = "deleteMessage";
}

impl DeleteMessage {
    pub fn new(chat_id: ChatId, message_id: i64, ) -> Self {
        Self {
            chat_id,
            message_id,
        }
    }
}

impl FormSer for SendSticker {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.sticker.serialize("sticker".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendSticker {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendSticker";
}

impl SendSticker {
    pub fn new(chat_id: ChatId, sticker: InputFile, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for GetStickerSet {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.name.serialize("name".into(), form);
        form
    }
}

impl TgMethod for GetStickerSet {
    type ResponseType = StickerSet;
    const PATH: &'static str = "getStickerSet";
}

impl GetStickerSet {
    pub fn new(name: String, ) -> Self {
        Self {
            name,
        }
    }
}

impl FormSer for UploadStickerFile {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.user_id.serialize("user_id".into(), form);
        form = self.png_sticker.serialize("png_sticker".into(), form);
        form
    }
}

impl TgMethod for UploadStickerFile {
    type ResponseType = File;
    const PATH: &'static str = "uploadStickerFile";
}

impl UploadStickerFile {
    pub fn new(user_id: i64, png_sticker: InputFile, ) -> Self {
        Self {
            user_id,
            png_sticker,
        }
    }
}

impl FormSer for CreateNewStickerSet {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.user_id.serialize("user_id".into(), form);
        form = self.name.serialize("name".into(), form);
        form = self.title.serialize("title".into(), form);
        form = self.png_sticker.serialize("png_sticker".into(), form);
        form = self.tgs_sticker.serialize("tgs_sticker".into(), form);
        form = self.webm_sticker.serialize("webm_sticker".into(), form);
        form = self.emojis.serialize("emojis".into(), form);
        form = self.contains_masks.serialize("contains_masks".into(), form);
        form = self.mask_position.serialize("mask_position".into(), form);
        form
    }
}

impl TgMethod for CreateNewStickerSet {
    type ResponseType = bool;
    const PATH: &'static str = "createNewStickerSet";
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
            emojis,
            contains_masks: None,
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

    pub fn with_contains_masks(mut self, contains_masks: bool) -> Self {
        self.contains_masks = Some(contains_masks);
        self
    }

    pub fn with_mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }

}

impl FormSer for AddStickerToSet {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.user_id.serialize("user_id".into(), form);
        form = self.name.serialize("name".into(), form);
        form = self.png_sticker.serialize("png_sticker".into(), form);
        form = self.tgs_sticker.serialize("tgs_sticker".into(), form);
        form = self.webm_sticker.serialize("webm_sticker".into(), form);
        form = self.emojis.serialize("emojis".into(), form);
        form = self.mask_position.serialize("mask_position".into(), form);
        form
    }
}

impl TgMethod for AddStickerToSet {
    type ResponseType = bool;
    const PATH: &'static str = "addStickerToSet";
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

impl FormSer for SetStickerPositionInSet {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.sticker.serialize("sticker".into(), form);
        form = self.position.serialize("position".into(), form);
        form
    }
}

impl TgMethod for SetStickerPositionInSet {
    type ResponseType = bool;
    const PATH: &'static str = "setStickerPositionInSet";
}

impl SetStickerPositionInSet {
    pub fn new(sticker: String, position: i64, ) -> Self {
        Self {
            sticker,
            position,
        }
    }
}

impl FormSer for DeleteStickerFromSet {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.sticker.serialize("sticker".into(), form);
        form
    }
}

impl TgMethod for DeleteStickerFromSet {
    type ResponseType = bool;
    const PATH: &'static str = "deleteStickerFromSet";
}

impl DeleteStickerFromSet {
    pub fn new(sticker: String, ) -> Self {
        Self {
            sticker,
        }
    }
}

impl FormSer for SetStickerSetThumb {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.name.serialize("name".into(), form);
        form = self.user_id.serialize("user_id".into(), form);
        form = self.thumb.serialize("thumb".into(), form);
        form
    }
}

impl TgMethod for SetStickerSetThumb {
    type ResponseType = bool;
    const PATH: &'static str = "setStickerSetThumb";
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

impl FormSer for AnswerInlineQuery {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.inline_query_id.serialize("inline_query_id".into(), form);
        form = self.results.serialize("results".into(), form);
        form = self.cache_time.serialize("cache_time".into(), form);
        form = self.is_personal.serialize("is_personal".into(), form);
        form = self.next_offset.serialize("next_offset".into(), form);
        form = self.switch_pm_text.serialize("switch_pm_text".into(), form);
        form = self.switch_pm_parameter.serialize("switch_pm_parameter".into(), form);
        form
    }
}

impl TgMethod for AnswerInlineQuery {
    type ResponseType = bool;
    const PATH: &'static str = "answerInlineQuery";
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

impl FormSer for AnswerWebAppQuery {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.web_app_query_id.serialize("web_app_query_id".into(), form);
        form = self.result.serialize("result".into(), form);
        form
    }
}

impl TgMethod for AnswerWebAppQuery {
    type ResponseType = SentWebAppMessage;
    const PATH: &'static str = "answerWebAppQuery";
}

impl AnswerWebAppQuery {
    pub fn new(web_app_query_id: String, result: InlineQueryResult, ) -> Self {
        Self {
            web_app_query_id,
            result,
        }
    }
}

impl FormSer for SendInvoice {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.title.serialize("title".into(), form);
        form = self.description.serialize("description".into(), form);
        form = self.payload.serialize("payload".into(), form);
        form = self.provider_token.serialize("provider_token".into(), form);
        form = self.currency.serialize("currency".into(), form);
        form = self.prices.serialize("prices".into(), form);
        form = self.max_tip_amount.serialize("max_tip_amount".into(), form);
        form = self.suggested_tip_amounts.serialize("suggested_tip_amounts".into(), form);
        form = self.start_parameter.serialize("start_parameter".into(), form);
        form = self.provider_data.serialize("provider_data".into(), form);
        form = self.photo_url.serialize("photo_url".into(), form);
        form = self.photo_size.serialize("photo_size".into(), form);
        form = self.photo_width.serialize("photo_width".into(), form);
        form = self.photo_height.serialize("photo_height".into(), form);
        form = self.need_name.serialize("need_name".into(), form);
        form = self.need_phone_number.serialize("need_phone_number".into(), form);
        form = self.need_email.serialize("need_email".into(), form);
        form = self.need_shipping_address.serialize("need_shipping_address".into(), form);
        form = self.send_phone_number_to_provider.serialize("send_phone_number_to_provider".into(), form);
        form = self.send_email_to_provider.serialize("send_email_to_provider".into(), form);
        form = self.is_flexible.serialize("is_flexible".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendInvoice {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendInvoice";
}

impl SendInvoice {
    pub fn new(chat_id: ChatId, title: String, description: String, payload: String, provider_token: String, currency: String, prices: Vec<LabeledPrice>, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for CreateInvoiceLink {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.title.serialize("title".into(), form);
        form = self.description.serialize("description".into(), form);
        form = self.payload.serialize("payload".into(), form);
        form = self.provider_token.serialize("provider_token".into(), form);
        form = self.currency.serialize("currency".into(), form);
        form = self.prices.serialize("prices".into(), form);
        form = self.max_tip_amount.serialize("max_tip_amount".into(), form);
        form = self.suggested_tip_amounts.serialize("suggested_tip_amounts".into(), form);
        form = self.provider_data.serialize("provider_data".into(), form);
        form = self.photo_url.serialize("photo_url".into(), form);
        form = self.photo_size.serialize("photo_size".into(), form);
        form = self.photo_width.serialize("photo_width".into(), form);
        form = self.photo_height.serialize("photo_height".into(), form);
        form = self.need_name.serialize("need_name".into(), form);
        form = self.need_phone_number.serialize("need_phone_number".into(), form);
        form = self.need_email.serialize("need_email".into(), form);
        form = self.need_shipping_address.serialize("need_shipping_address".into(), form);
        form = self.send_phone_number_to_provider.serialize("send_phone_number_to_provider".into(), form);
        form = self.send_email_to_provider.serialize("send_email_to_provider".into(), form);
        form = self.is_flexible.serialize("is_flexible".into(), form);
        form
    }
}

impl TgMethod for CreateInvoiceLink {
    type ResponseType = String;
    const PATH: &'static str = "createInvoiceLink";
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

impl FormSer for AnswerShippingQuery {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.shipping_query_id.serialize("shipping_query_id".into(), form);
        form = self.ok.serialize("ok".into(), form);
        form = self.shipping_options.serialize("shipping_options".into(), form);
        form = self.error_message.serialize("error_message".into(), form);
        form
    }
}

impl TgMethod for AnswerShippingQuery {
    type ResponseType = bool;
    const PATH: &'static str = "answerShippingQuery";
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

impl FormSer for AnswerPreCheckoutQuery {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.pre_checkout_query_id.serialize("pre_checkout_query_id".into(), form);
        form = self.ok.serialize("ok".into(), form);
        form = self.error_message.serialize("error_message".into(), form);
        form
    }
}

impl TgMethod for AnswerPreCheckoutQuery {
    type ResponseType = bool;
    const PATH: &'static str = "answerPreCheckoutQuery";
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

impl FormSer for SetPassportDataErrors {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.user_id.serialize("user_id".into(), form);
        form = self.errors.serialize("errors".into(), form);
        form
    }
}

impl TgMethod for SetPassportDataErrors {
    type ResponseType = bool;
    const PATH: &'static str = "setPassportDataErrors";
}

impl SetPassportDataErrors {
    pub fn new(user_id: i64, errors: Vec<PassportElementError>, ) -> Self {
        Self {
            user_id,
            errors,
        }
    }
}

impl FormSer for SendGame {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.game_short_name.serialize("game_short_name".into(), form);
        form = self.disable_notification.serialize("disable_notification".into(), form);
        form = self.protect_content.serialize("protect_content".into(), form);
        form = self.reply_to_message_id.serialize("reply_to_message_id".into(), form);
        form = self.allow_sending_without_reply.serialize("allow_sending_without_reply".into(), form);
        form = self.reply_markup.serialize("reply_markup".into(), form);
        form
    }
}

impl TgMethod for SendGame {
    type ResponseType = Box<Message>;
    const PATH: &'static str = "sendGame";
}

impl SendGame {
    pub fn new(chat_id: i64, game_short_name: String, ) -> Self {
        Self {
            chat_id,
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

impl FormSer for SetGameScore {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.user_id.serialize("user_id".into(), form);
        form = self.score.serialize("score".into(), form);
        form = self.force.serialize("force".into(), form);
        form = self.disable_edit_message.serialize("disable_edit_message".into(), form);
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form = self.inline_message_id.serialize("inline_message_id".into(), form);
        form
    }
}

impl TgMethod for SetGameScore {
    type ResponseType = MessageOrBool;
    const PATH: &'static str = "setGameScore";
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

impl FormSer for GetGameHighScores {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.user_id.serialize("user_id".into(), form);
        form = self.chat_id.serialize("chat_id".into(), form);
        form = self.message_id.serialize("message_id".into(), form);
        form = self.inline_message_id.serialize("inline_message_id".into(), form);
        form
    }
}

impl TgMethod for GetGameHighScores {
    type ResponseType = Vec<GameHighScore>;
    const PATH: &'static str = "getGameHighScores";
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

