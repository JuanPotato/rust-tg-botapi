use crate::objects::*;
use crate::better::*;

impl FormSer for ChatMember {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        match self {
            ChatMember::Owner(e) => e.serialize(key, form),
            ChatMember::Administrator(e) => e.serialize(key, form),
            ChatMember::Member(e) => e.serialize(key, form),
            ChatMember::Restricted(e) => e.serialize(key, form),
            ChatMember::Left(e) => e.serialize(key, form),
            ChatMember::Banned(e) => e.serialize(key, form),
        }
    }
}

impl From<ChatMemberOwner> for ChatMember {
    fn from(o: ChatMemberOwner) -> Self {
        Self::Owner(o)
    }
}

impl From<ChatMemberAdministrator> for ChatMember {
    fn from(o: ChatMemberAdministrator) -> Self {
        Self::Administrator(o)
    }
}

impl From<ChatMemberMember> for ChatMember {
    fn from(o: ChatMemberMember) -> Self {
        Self::Member(o)
    }
}

impl From<ChatMemberRestricted> for ChatMember {
    fn from(o: ChatMemberRestricted) -> Self {
        Self::Restricted(o)
    }
}

impl From<ChatMemberLeft> for ChatMember {
    fn from(o: ChatMemberLeft) -> Self {
        Self::Left(o)
    }
}

impl From<ChatMemberBanned> for ChatMember {
    fn from(o: ChatMemberBanned) -> Self {
        Self::Banned(o)
    }
}

impl FormSer for BotCommandScope {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        match self {
            BotCommandScope::Default(e) => e.serialize(key, form),
            BotCommandScope::AllPrivateChats(e) => e.serialize(key, form),
            BotCommandScope::AllGroupChats(e) => e.serialize(key, form),
            BotCommandScope::AllChatAdministrators(e) => e.serialize(key, form),
            BotCommandScope::Chat(e) => e.serialize(key, form),
            BotCommandScope::ChatAdministrators(e) => e.serialize(key, form),
            BotCommandScope::ChatMember(e) => e.serialize(key, form),
        }
    }
}

impl From<BotCommandScopeDefault> for BotCommandScope {
    fn from(o: BotCommandScopeDefault) -> Self {
        Self::Default(o)
    }
}

impl From<BotCommandScopeAllPrivateChats> for BotCommandScope {
    fn from(o: BotCommandScopeAllPrivateChats) -> Self {
        Self::AllPrivateChats(o)
    }
}

impl From<BotCommandScopeAllGroupChats> for BotCommandScope {
    fn from(o: BotCommandScopeAllGroupChats) -> Self {
        Self::AllGroupChats(o)
    }
}

impl From<BotCommandScopeAllChatAdministrators> for BotCommandScope {
    fn from(o: BotCommandScopeAllChatAdministrators) -> Self {
        Self::AllChatAdministrators(o)
    }
}

impl From<BotCommandScopeChat> for BotCommandScope {
    fn from(o: BotCommandScopeChat) -> Self {
        Self::Chat(o)
    }
}

impl From<BotCommandScopeChatAdministrators> for BotCommandScope {
    fn from(o: BotCommandScopeChatAdministrators) -> Self {
        Self::ChatAdministrators(o)
    }
}

impl From<BotCommandScopeChatMember> for BotCommandScope {
    fn from(o: BotCommandScopeChatMember) -> Self {
        Self::ChatMember(o)
    }
}

impl FormSer for InputMedia {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        match self {
            InputMedia::Animation(e) => e.serialize(key, form),
            InputMedia::Document(e) => e.serialize(key, form),
            InputMedia::Audio(e) => e.serialize(key, form),
            InputMedia::Photo(e) => e.serialize(key, form),
            InputMedia::Video(e) => e.serialize(key, form),
        }
    }
}

impl From<InputMediaAnimation> for InputMedia {
    fn from(o: InputMediaAnimation) -> Self {
        Self::Animation(o)
    }
}

impl From<InputMediaDocument> for InputMedia {
    fn from(o: InputMediaDocument) -> Self {
        Self::Document(o)
    }
}

impl From<InputMediaAudio> for InputMedia {
    fn from(o: InputMediaAudio) -> Self {
        Self::Audio(o)
    }
}

impl From<InputMediaPhoto> for InputMedia {
    fn from(o: InputMediaPhoto) -> Self {
        Self::Photo(o)
    }
}

impl From<InputMediaVideo> for InputMedia {
    fn from(o: InputMediaVideo) -> Self {
        Self::Video(o)
    }
}

impl FormSer for InlineQueryResult {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        match self {
            InlineQueryResult::CachedAudio(e) => e.serialize(key, form),
            InlineQueryResult::CachedDocument(e) => e.serialize(key, form),
            InlineQueryResult::CachedGif(e) => e.serialize(key, form),
            InlineQueryResult::CachedMpeg4Gif(e) => e.serialize(key, form),
            InlineQueryResult::CachedPhoto(e) => e.serialize(key, form),
            InlineQueryResult::CachedSticker(e) => e.serialize(key, form),
            InlineQueryResult::CachedVideo(e) => e.serialize(key, form),
            InlineQueryResult::CachedVoice(e) => e.serialize(key, form),
            InlineQueryResult::Article(e) => e.serialize(key, form),
            InlineQueryResult::Audio(e) => e.serialize(key, form),
            InlineQueryResult::Contact(e) => e.serialize(key, form),
            InlineQueryResult::Game(e) => e.serialize(key, form),
            InlineQueryResult::Document(e) => e.serialize(key, form),
            InlineQueryResult::Gif(e) => e.serialize(key, form),
            InlineQueryResult::Location(e) => e.serialize(key, form),
            InlineQueryResult::Mpeg4Gif(e) => e.serialize(key, form),
            InlineQueryResult::Photo(e) => e.serialize(key, form),
            InlineQueryResult::Venue(e) => e.serialize(key, form),
            InlineQueryResult::Video(e) => e.serialize(key, form),
            InlineQueryResult::Voice(e) => e.serialize(key, form),
        }
    }
}

impl From<InlineQueryResultCachedAudio> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedAudio) -> Self {
        Self::CachedAudio(o)
    }
}

impl From<InlineQueryResultCachedDocument> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedDocument) -> Self {
        Self::CachedDocument(o)
    }
}

impl From<InlineQueryResultCachedGif> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedGif) -> Self {
        Self::CachedGif(o)
    }
}

impl From<InlineQueryResultCachedMpeg4Gif> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedMpeg4Gif) -> Self {
        Self::CachedMpeg4Gif(o)
    }
}

impl From<InlineQueryResultCachedPhoto> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedPhoto) -> Self {
        Self::CachedPhoto(o)
    }
}

impl From<InlineQueryResultCachedSticker> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedSticker) -> Self {
        Self::CachedSticker(o)
    }
}

impl From<InlineQueryResultCachedVideo> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedVideo) -> Self {
        Self::CachedVideo(o)
    }
}

impl From<InlineQueryResultCachedVoice> for InlineQueryResult {
    fn from(o: InlineQueryResultCachedVoice) -> Self {
        Self::CachedVoice(o)
    }
}

impl From<InlineQueryResultArticle> for InlineQueryResult {
    fn from(o: InlineQueryResultArticle) -> Self {
        Self::Article(o)
    }
}

impl From<InlineQueryResultAudio> for InlineQueryResult {
    fn from(o: InlineQueryResultAudio) -> Self {
        Self::Audio(o)
    }
}

impl From<InlineQueryResultContact> for InlineQueryResult {
    fn from(o: InlineQueryResultContact) -> Self {
        Self::Contact(o)
    }
}

impl From<InlineQueryResultGame> for InlineQueryResult {
    fn from(o: InlineQueryResultGame) -> Self {
        Self::Game(o)
    }
}

impl From<InlineQueryResultDocument> for InlineQueryResult {
    fn from(o: InlineQueryResultDocument) -> Self {
        Self::Document(o)
    }
}

impl From<InlineQueryResultGif> for InlineQueryResult {
    fn from(o: InlineQueryResultGif) -> Self {
        Self::Gif(o)
    }
}

impl From<InlineQueryResultLocation> for InlineQueryResult {
    fn from(o: InlineQueryResultLocation) -> Self {
        Self::Location(o)
    }
}

impl From<InlineQueryResultMpeg4Gif> for InlineQueryResult {
    fn from(o: InlineQueryResultMpeg4Gif) -> Self {
        Self::Mpeg4Gif(o)
    }
}

impl From<InlineQueryResultPhoto> for InlineQueryResult {
    fn from(o: InlineQueryResultPhoto) -> Self {
        Self::Photo(o)
    }
}

impl From<InlineQueryResultVenue> for InlineQueryResult {
    fn from(o: InlineQueryResultVenue) -> Self {
        Self::Venue(o)
    }
}

impl From<InlineQueryResultVideo> for InlineQueryResult {
    fn from(o: InlineQueryResultVideo) -> Self {
        Self::Video(o)
    }
}

impl From<InlineQueryResultVoice> for InlineQueryResult {
    fn from(o: InlineQueryResultVoice) -> Self {
        Self::Voice(o)
    }
}

impl FormSer for InputMessageContent {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        match self {
            InputMessageContent::Text(e) => e.serialize(key, form),
            InputMessageContent::Location(e) => e.serialize(key, form),
            InputMessageContent::Venue(e) => e.serialize(key, form),
            InputMessageContent::Contact(e) => e.serialize(key, form),
            InputMessageContent::Invoice(e) => e.serialize(key, form),
        }
    }
}

impl From<InputTextMessageContent> for InputMessageContent {
    fn from(o: InputTextMessageContent) -> Self {
        Self::Text(o)
    }
}

impl From<InputLocationMessageContent> for InputMessageContent {
    fn from(o: InputLocationMessageContent) -> Self {
        Self::Location(o)
    }
}

impl From<InputVenueMessageContent> for InputMessageContent {
    fn from(o: InputVenueMessageContent) -> Self {
        Self::Venue(o)
    }
}

impl From<InputContactMessageContent> for InputMessageContent {
    fn from(o: InputContactMessageContent) -> Self {
        Self::Contact(o)
    }
}

impl From<InputInvoiceMessageContent> for InputMessageContent {
    fn from(o: InputInvoiceMessageContent) -> Self {
        Self::Invoice(o)
    }
}

impl FormSer for PassportElementError {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        match self {
            PassportElementError::DataField(e) => e.serialize(key, form),
            PassportElementError::FrontSide(e) => e.serialize(key, form),
            PassportElementError::ReverseSide(e) => e.serialize(key, form),
            PassportElementError::Selfie(e) => e.serialize(key, form),
            PassportElementError::File(e) => e.serialize(key, form),
            PassportElementError::Files(e) => e.serialize(key, form),
            PassportElementError::TranslationFile(e) => e.serialize(key, form),
            PassportElementError::TranslationFiles(e) => e.serialize(key, form),
            PassportElementError::Unspecified(e) => e.serialize(key, form),
        }
    }
}

impl From<PassportElementErrorDataField> for PassportElementError {
    fn from(o: PassportElementErrorDataField) -> Self {
        Self::DataField(o)
    }
}

impl From<PassportElementErrorFrontSide> for PassportElementError {
    fn from(o: PassportElementErrorFrontSide) -> Self {
        Self::FrontSide(o)
    }
}

impl From<PassportElementErrorReverseSide> for PassportElementError {
    fn from(o: PassportElementErrorReverseSide) -> Self {
        Self::ReverseSide(o)
    }
}

impl From<PassportElementErrorSelfie> for PassportElementError {
    fn from(o: PassportElementErrorSelfie) -> Self {
        Self::Selfie(o)
    }
}

impl From<PassportElementErrorFile> for PassportElementError {
    fn from(o: PassportElementErrorFile) -> Self {
        Self::File(o)
    }
}

impl From<PassportElementErrorFiles> for PassportElementError {
    fn from(o: PassportElementErrorFiles) -> Self {
        Self::Files(o)
    }
}

impl From<PassportElementErrorTranslationFile> for PassportElementError {
    fn from(o: PassportElementErrorTranslationFile) -> Self {
        Self::TranslationFile(o)
    }
}

impl From<PassportElementErrorTranslationFiles> for PassportElementError {
    fn from(o: PassportElementErrorTranslationFiles) -> Self {
        Self::TranslationFiles(o)
    }
}

impl From<PassportElementErrorUnspecified> for PassportElementError {
    fn from(o: PassportElementErrorUnspecified) -> Self {
        Self::Unspecified(o)
    }
}

impl FormSer for WebhookInfo {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.url.serialize(format!("{}[url]", key), form);
        form = self.has_custom_certificate.serialize(format!("{}[has_custom_certificate]", key), form);
        form = self.pending_update_count.serialize(format!("{}[pending_update_count]", key), form);
        form = self.ip_address.serialize(format!("{}[ip_address]", key), form);
        form = self.last_error_date.serialize(format!("{}[last_error_date]", key), form);
        form = self.last_error_message.serialize(format!("{}[last_error_message]", key), form);
        form = self.max_connections.serialize(format!("{}[max_connections]", key), form);
        form = self.allowed_updates.serialize(format!("{}[allowed_updates]", key), form);
        form
    }
}

impl WebhookInfo {
    pub fn new(url: String, has_custom_certificate: bool, pending_update_count: i64, ) -> Self {
        Self {
            url,
            has_custom_certificate,
            pending_update_count,
            ip_address: None,
            last_error_date: None,
            last_error_message: None,
            max_connections: None,
            allowed_updates: None,
        }
    }
}

impl FormSer for User {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.is_bot.serialize(format!("{}[is_bot]", key), form);
        form = self.first_name.serialize(format!("{}[first_name]", key), form);
        form = self.last_name.serialize(format!("{}[last_name]", key), form);
        form = self.username.serialize(format!("{}[username]", key), form);
        form = self.language_code.serialize(format!("{}[language_code]", key), form);
        form = self.can_join_groups.serialize(format!("{}[can_join_groups]", key), form);
        form = self.can_read_all_group_messages.serialize(format!("{}[can_read_all_group_messages]", key), form);
        form = self.supports_inline_queries.serialize(format!("{}[supports_inline_queries]", key), form);
        form
    }
}

impl User {
    pub fn new(id: i64, is_bot: bool, first_name: String, ) -> Self {
        Self {
            id,
            is_bot,
            first_name,
            last_name: None,
            username: None,
            language_code: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_inline_queries: None,
        }
    }
}

impl FormSer for Chat {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.username.serialize(format!("{}[username]", key), form);
        form = self.first_name.serialize(format!("{}[first_name]", key), form);
        form = self.last_name.serialize(format!("{}[last_name]", key), form);
        form = self.photo.serialize(format!("{}[photo]", key), form);
        form = self.bio.serialize(format!("{}[bio]", key), form);
        form = self.has_private_forwards.serialize(format!("{}[has_private_forwards]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.invite_link.serialize(format!("{}[invite_link]", key), form);
        form = self.pinned_message.serialize(format!("{}[pinned_message]", key), form);
        form = self.permissions.serialize(format!("{}[permissions]", key), form);
        form = self.slow_mode_delay.serialize(format!("{}[slow_mode_delay]", key), form);
        form = self.message_auto_delete_time.serialize(format!("{}[message_auto_delete_time]", key), form);
        form = self.has_protected_content.serialize(format!("{}[has_protected_content]", key), form);
        form = self.sticker_set_name.serialize(format!("{}[sticker_set_name]", key), form);
        form = self.can_set_sticker_set.serialize(format!("{}[can_set_sticker_set]", key), form);
        form = self.linked_chat_id.serialize(format!("{}[linked_chat_id]", key), form);
        form = self.location.serialize(format!("{}[location]", key), form);
        form
    }
}

impl Chat {
    pub fn new(id: i64, type_: String, ) -> Self {
        Self {
            id,
            type_,
            title: None,
            username: None,
            first_name: None,
            last_name: None,
            photo: None,
            bio: None,
            has_private_forwards: None,
            description: None,
            invite_link: None,
            pinned_message: None,
            permissions: None,
            slow_mode_delay: None,
            message_auto_delete_time: None,
            has_protected_content: None,
            sticker_set_name: None,
            can_set_sticker_set: None,
            linked_chat_id: None,
            location: None,
        }
    }
}

impl FormSer for Message {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.message_id.serialize(format!("{}[message_id]", key), form);
        form = self.from.serialize(format!("{}[from]", key), form);
        form = self.sender_chat.serialize(format!("{}[sender_chat]", key), form);
        form = self.date.serialize(format!("{}[date]", key), form);
        form = self.chat.serialize(format!("{}[chat]", key), form);
        form = self.forward_from.serialize(format!("{}[forward_from]", key), form);
        form = self.forward_from_chat.serialize(format!("{}[forward_from_chat]", key), form);
        form = self.forward_from_message_id.serialize(format!("{}[forward_from_message_id]", key), form);
        form = self.forward_signature.serialize(format!("{}[forward_signature]", key), form);
        form = self.forward_sender_name.serialize(format!("{}[forward_sender_name]", key), form);
        form = self.forward_date.serialize(format!("{}[forward_date]", key), form);
        form = self.is_automatic_forward.serialize(format!("{}[is_automatic_forward]", key), form);
        form = self.reply_to_message.serialize(format!("{}[reply_to_message]", key), form);
        form = self.via_bot.serialize(format!("{}[via_bot]", key), form);
        form = self.edit_date.serialize(format!("{}[edit_date]", key), form);
        form = self.has_protected_content.serialize(format!("{}[has_protected_content]", key), form);
        form = self.media_group_id.serialize(format!("{}[media_group_id]", key), form);
        form = self.author_signature.serialize(format!("{}[author_signature]", key), form);
        form = self.text.serialize(format!("{}[text]", key), form);
        form = self.entities.serialize(format!("{}[entities]", key), form);
        form = self.animation.serialize(format!("{}[animation]", key), form);
        form = self.audio.serialize(format!("{}[audio]", key), form);
        form = self.document.serialize(format!("{}[document]", key), form);
        form = self.photo.serialize(format!("{}[photo]", key), form);
        form = self.sticker.serialize(format!("{}[sticker]", key), form);
        form = self.video.serialize(format!("{}[video]", key), form);
        form = self.video_note.serialize(format!("{}[video_note]", key), form);
        form = self.voice.serialize(format!("{}[voice]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.contact.serialize(format!("{}[contact]", key), form);
        form = self.dice.serialize(format!("{}[dice]", key), form);
        form = self.game.serialize(format!("{}[game]", key), form);
        form = self.poll.serialize(format!("{}[poll]", key), form);
        form = self.venue.serialize(format!("{}[venue]", key), form);
        form = self.location.serialize(format!("{}[location]", key), form);
        form = self.new_chat_members.serialize(format!("{}[new_chat_members]", key), form);
        form = self.left_chat_member.serialize(format!("{}[left_chat_member]", key), form);
        form = self.new_chat_title.serialize(format!("{}[new_chat_title]", key), form);
        form = self.new_chat_photo.serialize(format!("{}[new_chat_photo]", key), form);
        form = self.delete_chat_photo.serialize(format!("{}[delete_chat_photo]", key), form);
        form = self.group_chat_created.serialize(format!("{}[group_chat_created]", key), form);
        form = self.supergroup_chat_created.serialize(format!("{}[supergroup_chat_created]", key), form);
        form = self.channel_chat_created.serialize(format!("{}[channel_chat_created]", key), form);
        form = self.message_auto_delete_timer_changed.serialize(format!("{}[message_auto_delete_timer_changed]", key), form);
        form = self.migrate_to_chat_id.serialize(format!("{}[migrate_to_chat_id]", key), form);
        form = self.migrate_from_chat_id.serialize(format!("{}[migrate_from_chat_id]", key), form);
        form = self.pinned_message.serialize(format!("{}[pinned_message]", key), form);
        form = self.invoice.serialize(format!("{}[invoice]", key), form);
        form = self.successful_payment.serialize(format!("{}[successful_payment]", key), form);
        form = self.connected_website.serialize(format!("{}[connected_website]", key), form);
        form = self.passport_data.serialize(format!("{}[passport_data]", key), form);
        form = self.proximity_alert_triggered.serialize(format!("{}[proximity_alert_triggered]", key), form);
        form = self.voice_chat_scheduled.serialize(format!("{}[voice_chat_scheduled]", key), form);
        form = self.voice_chat_started.serialize(format!("{}[voice_chat_started]", key), form);
        form = self.voice_chat_ended.serialize(format!("{}[voice_chat_ended]", key), form);
        form = self.voice_chat_participants_invited.serialize(format!("{}[voice_chat_participants_invited]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form
    }
}

impl Message {
    pub fn new(message_id: i64, date: i64, chat: Chat, ) -> Self {
        Self {
            message_id,
            from: None,
            sender_chat: None,
            date,
            chat,
            forward_from: None,
            forward_from_chat: None,
            forward_from_message_id: None,
            forward_signature: None,
            forward_sender_name: None,
            forward_date: None,
            is_automatic_forward: None,
            reply_to_message: None,
            via_bot: None,
            edit_date: None,
            has_protected_content: None,
            media_group_id: None,
            author_signature: None,
            text: None,
            entities: None,
            animation: None,
            audio: None,
            document: None,
            photo: None,
            sticker: None,
            video: None,
            video_note: None,
            voice: None,
            caption: None,
            caption_entities: None,
            contact: None,
            dice: None,
            game: None,
            poll: None,
            venue: None,
            location: None,
            new_chat_members: None,
            left_chat_member: None,
            new_chat_title: None,
            new_chat_photo: None,
            delete_chat_photo: None,
            group_chat_created: None,
            supergroup_chat_created: None,
            channel_chat_created: None,
            message_auto_delete_timer_changed: None,
            migrate_to_chat_id: None,
            migrate_from_chat_id: None,
            pinned_message: None,
            invoice: None,
            successful_payment: None,
            connected_website: None,
            passport_data: None,
            proximity_alert_triggered: None,
            voice_chat_scheduled: None,
            voice_chat_started: None,
            voice_chat_ended: None,
            voice_chat_participants_invited: None,
            reply_markup: None,
        }
    }
}

impl FormSer for MessageId {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.message_id.serialize(format!("{}[message_id]", key), form);
        form
    }
}

impl MessageId {
    pub fn new(message_id: i64, ) -> Self {
        Self {
            message_id,
        }
    }
}

impl FormSer for MessageEntity {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.offset.serialize(format!("{}[offset]", key), form);
        form = self.length.serialize(format!("{}[length]", key), form);
        form = self.url.serialize(format!("{}[url]", key), form);
        form = self.user.serialize(format!("{}[user]", key), form);
        form = self.language.serialize(format!("{}[language]", key), form);
        form
    }
}

impl MessageEntity {
    pub fn new(type_: String, offset: i64, length: i64, ) -> Self {
        Self {
            type_,
            offset,
            length,
            url: None,
            user: None,
            language: None,
        }
    }
}

impl FormSer for PhotoSize {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.file_id.serialize(format!("{}[file_id]", key), form);
        form = self.file_unique_id.serialize(format!("{}[file_unique_id]", key), form);
        form = self.width.serialize(format!("{}[width]", key), form);
        form = self.height.serialize(format!("{}[height]", key), form);
        form = self.file_size.serialize(format!("{}[file_size]", key), form);
        form
    }
}

impl PhotoSize {
    pub fn new(file_id: String, file_unique_id: String, width: i64, height: i64, ) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            file_size: None,
        }
    }
}

impl FormSer for Animation {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.file_id.serialize(format!("{}[file_id]", key), form);
        form = self.file_unique_id.serialize(format!("{}[file_unique_id]", key), form);
        form = self.width.serialize(format!("{}[width]", key), form);
        form = self.height.serialize(format!("{}[height]", key), form);
        form = self.duration.serialize(format!("{}[duration]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form = self.file_name.serialize(format!("{}[file_name]", key), form);
        form = self.mime_type.serialize(format!("{}[mime_type]", key), form);
        form = self.file_size.serialize(format!("{}[file_size]", key), form);
        form
    }
}

impl Animation {
    pub fn new(file_id: String, file_unique_id: String, width: i64, height: i64, duration: i64, ) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            duration,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }
}

impl FormSer for Audio {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.file_id.serialize(format!("{}[file_id]", key), form);
        form = self.file_unique_id.serialize(format!("{}[file_unique_id]", key), form);
        form = self.duration.serialize(format!("{}[duration]", key), form);
        form = self.performer.serialize(format!("{}[performer]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.file_name.serialize(format!("{}[file_name]", key), form);
        form = self.mime_type.serialize(format!("{}[mime_type]", key), form);
        form = self.file_size.serialize(format!("{}[file_size]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form
    }
}

impl Audio {
    pub fn new(file_id: String, file_unique_id: String, duration: i64, ) -> Self {
        Self {
            file_id,
            file_unique_id,
            duration,
            performer: None,
            title: None,
            file_name: None,
            mime_type: None,
            file_size: None,
            thumb: None,
        }
    }
}

impl FormSer for Document {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.file_id.serialize(format!("{}[file_id]", key), form);
        form = self.file_unique_id.serialize(format!("{}[file_unique_id]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form = self.file_name.serialize(format!("{}[file_name]", key), form);
        form = self.mime_type.serialize(format!("{}[mime_type]", key), form);
        form = self.file_size.serialize(format!("{}[file_size]", key), form);
        form
    }
}

impl Document {
    pub fn new(file_id: String, file_unique_id: String, ) -> Self {
        Self {
            file_id,
            file_unique_id,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }
}

impl FormSer for Video {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.file_id.serialize(format!("{}[file_id]", key), form);
        form = self.file_unique_id.serialize(format!("{}[file_unique_id]", key), form);
        form = self.width.serialize(format!("{}[width]", key), form);
        form = self.height.serialize(format!("{}[height]", key), form);
        form = self.duration.serialize(format!("{}[duration]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form = self.file_name.serialize(format!("{}[file_name]", key), form);
        form = self.mime_type.serialize(format!("{}[mime_type]", key), form);
        form = self.file_size.serialize(format!("{}[file_size]", key), form);
        form
    }
}

impl Video {
    pub fn new(file_id: String, file_unique_id: String, width: i64, height: i64, duration: i64, ) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            duration,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }
}

impl FormSer for VideoNote {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.file_id.serialize(format!("{}[file_id]", key), form);
        form = self.file_unique_id.serialize(format!("{}[file_unique_id]", key), form);
        form = self.length.serialize(format!("{}[length]", key), form);
        form = self.duration.serialize(format!("{}[duration]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form = self.file_size.serialize(format!("{}[file_size]", key), form);
        form
    }
}

impl VideoNote {
    pub fn new(file_id: String, file_unique_id: String, length: i64, duration: i64, ) -> Self {
        Self {
            file_id,
            file_unique_id,
            length,
            duration,
            thumb: None,
            file_size: None,
        }
    }
}

impl FormSer for Voice {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.file_id.serialize(format!("{}[file_id]", key), form);
        form = self.file_unique_id.serialize(format!("{}[file_unique_id]", key), form);
        form = self.duration.serialize(format!("{}[duration]", key), form);
        form = self.mime_type.serialize(format!("{}[mime_type]", key), form);
        form = self.file_size.serialize(format!("{}[file_size]", key), form);
        form
    }
}

impl Voice {
    pub fn new(file_id: String, file_unique_id: String, duration: i64, ) -> Self {
        Self {
            file_id,
            file_unique_id,
            duration,
            mime_type: None,
            file_size: None,
        }
    }
}

impl FormSer for Contact {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.phone_number.serialize(format!("{}[phone_number]", key), form);
        form = self.first_name.serialize(format!("{}[first_name]", key), form);
        form = self.last_name.serialize(format!("{}[last_name]", key), form);
        form = self.user_id.serialize(format!("{}[user_id]", key), form);
        form = self.vcard.serialize(format!("{}[vcard]", key), form);
        form
    }
}

impl Contact {
    pub fn new(phone_number: String, first_name: String, ) -> Self {
        Self {
            phone_number,
            first_name,
            last_name: None,
            user_id: None,
            vcard: None,
        }
    }
}

impl FormSer for Dice {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.emoji.serialize(format!("{}[emoji]", key), form);
        form = self.value.serialize(format!("{}[value]", key), form);
        form
    }
}

impl Dice {
    pub fn new(emoji: String, value: i64, ) -> Self {
        Self {
            emoji,
            value,
        }
    }
}

impl FormSer for PollOption {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.text.serialize(format!("{}[text]", key), form);
        form = self.voter_count.serialize(format!("{}[voter_count]", key), form);
        form
    }
}

impl PollOption {
    pub fn new(text: String, voter_count: i64, ) -> Self {
        Self {
            text,
            voter_count,
        }
    }
}

impl FormSer for PollAnswer {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.poll_id.serialize(format!("{}[poll_id]", key), form);
        form = self.user.serialize(format!("{}[user]", key), form);
        form = self.option_ids.serialize(format!("{}[option_ids]", key), form);
        form
    }
}

impl PollAnswer {
    pub fn new(poll_id: String, user: User, option_ids: Vec<i64>, ) -> Self {
        Self {
            poll_id,
            user,
            option_ids,
        }
    }
}

impl FormSer for Poll {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.question.serialize(format!("{}[question]", key), form);
        form = self.options.serialize(format!("{}[options]", key), form);
        form = self.total_voter_count.serialize(format!("{}[total_voter_count]", key), form);
        form = self.is_closed.serialize(format!("{}[is_closed]", key), form);
        form = self.is_anonymous.serialize(format!("{}[is_anonymous]", key), form);
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.allows_multiple_answers.serialize(format!("{}[allows_multiple_answers]", key), form);
        form = self.correct_option_id.serialize(format!("{}[correct_option_id]", key), form);
        form = self.explanation.serialize(format!("{}[explanation]", key), form);
        form = self.explanation_entities.serialize(format!("{}[explanation_entities]", key), form);
        form = self.open_period.serialize(format!("{}[open_period]", key), form);
        form = self.close_date.serialize(format!("{}[close_date]", key), form);
        form
    }
}

impl Poll {
    pub fn new(id: String, question: String, options: Vec<PollOption>, total_voter_count: i64, is_closed: bool, is_anonymous: bool, type_: String, allows_multiple_answers: bool, ) -> Self {
        Self {
            id,
            question,
            options,
            total_voter_count,
            is_closed,
            is_anonymous,
            type_,
            allows_multiple_answers,
            correct_option_id: None,
            explanation: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
        }
    }
}

impl FormSer for Location {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.longitude.serialize(format!("{}[longitude]", key), form);
        form = self.latitude.serialize(format!("{}[latitude]", key), form);
        form = self.horizontal_accuracy.serialize(format!("{}[horizontal_accuracy]", key), form);
        form = self.live_period.serialize(format!("{}[live_period]", key), form);
        form = self.heading.serialize(format!("{}[heading]", key), form);
        form = self.proximity_alert_radius.serialize(format!("{}[proximity_alert_radius]", key), form);
        form
    }
}

impl Location {
    pub fn new(longitude: f64, latitude: f64, ) -> Self {
        Self {
            longitude,
            latitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        }
    }
}

impl FormSer for Venue {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.location.serialize(format!("{}[location]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.address.serialize(format!("{}[address]", key), form);
        form = self.foursquare_id.serialize(format!("{}[foursquare_id]", key), form);
        form = self.foursquare_type.serialize(format!("{}[foursquare_type]", key), form);
        form = self.google_place_id.serialize(format!("{}[google_place_id]", key), form);
        form = self.google_place_type.serialize(format!("{}[google_place_type]", key), form);
        form
    }
}

impl Venue {
    pub fn new(location: Location, title: String, address: String, ) -> Self {
        Self {
            location,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }
}

impl FormSer for ProximityAlertTriggered {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.traveler.serialize(format!("{}[traveler]", key), form);
        form = self.watcher.serialize(format!("{}[watcher]", key), form);
        form = self.distance.serialize(format!("{}[distance]", key), form);
        form
    }
}

impl ProximityAlertTriggered {
    pub fn new(traveler: User, watcher: User, distance: i64, ) -> Self {
        Self {
            traveler,
            watcher,
            distance,
        }
    }
}

impl FormSer for MessageAutoDeleteTimerChanged {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.message_auto_delete_time.serialize(format!("{}[message_auto_delete_time]", key), form);
        form
    }
}

impl MessageAutoDeleteTimerChanged {
    pub fn new(message_auto_delete_time: i64, ) -> Self {
        Self {
            message_auto_delete_time,
        }
    }
}

impl FormSer for VoiceChatScheduled {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.start_date.serialize(format!("{}[start_date]", key), form);
        form
    }
}

impl VoiceChatScheduled {
    pub fn new(start_date: i64, ) -> Self {
        Self {
            start_date,
        }
    }
}

impl FormSer for VoiceChatStarted {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form
    }
}

impl FormSer for VoiceChatEnded {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.duration.serialize(format!("{}[duration]", key), form);
        form
    }
}

impl VoiceChatEnded {
    pub fn new(duration: i64, ) -> Self {
        Self {
            duration,
        }
    }
}

impl FormSer for VoiceChatParticipantsInvited {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.users.serialize(format!("{}[users]", key), form);
        form
    }
}

impl FormSer for UserProfilePhotos {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.total_count.serialize(format!("{}[total_count]", key), form);
        form = self.photos.serialize(format!("{}[photos]", key), form);
        form
    }
}

impl UserProfilePhotos {
    pub fn new(total_count: i64, photos: Vec<Vec<PhotoSize>>, ) -> Self {
        Self {
            total_count,
            photos,
        }
    }
}

impl FormSer for File {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.file_id.serialize(format!("{}[file_id]", key), form);
        form = self.file_unique_id.serialize(format!("{}[file_unique_id]", key), form);
        form = self.file_size.serialize(format!("{}[file_size]", key), form);
        form = self.file_path.serialize(format!("{}[file_path]", key), form);
        form
    }
}

impl File {
    pub fn new(file_id: String, file_unique_id: String, ) -> Self {
        Self {
            file_id,
            file_unique_id,
            file_size: None,
            file_path: None,
        }
    }
}

impl FormSer for ReplyKeyboardMarkup {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.keyboard.serialize(format!("{}[keyboard]", key), form);
        form = self.resize_keyboard.serialize(format!("{}[resize_keyboard]", key), form);
        form = self.one_time_keyboard.serialize(format!("{}[one_time_keyboard]", key), form);
        form = self.input_field_placeholder.serialize(format!("{}[input_field_placeholder]", key), form);
        form = self.selective.serialize(format!("{}[selective]", key), form);
        form
    }
}

impl ReplyKeyboardMarkup {
    pub fn new(keyboard: Vec<Vec<KeyboardButton>>, ) -> Self {
        Self {
            keyboard,
            resize_keyboard: None,
            one_time_keyboard: None,
            input_field_placeholder: None,
            selective: None,
        }
    }
}

impl FormSer for KeyboardButton {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.text.serialize(format!("{}[text]", key), form);
        form = self.request_contact.serialize(format!("{}[request_contact]", key), form);
        form = self.request_location.serialize(format!("{}[request_location]", key), form);
        form = self.request_poll.serialize(format!("{}[request_poll]", key), form);
        form
    }
}

impl KeyboardButton {
    pub fn new(text: String, ) -> Self {
        Self {
            text,
            request_contact: None,
            request_location: None,
            request_poll: None,
        }
    }
}

impl FormSer for KeyboardButtonPollType {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form
    }
}

impl FormSer for ReplyKeyboardRemove {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.remove_keyboard.serialize(format!("{}[remove_keyboard]", key), form);
        form = self.selective.serialize(format!("{}[selective]", key), form);
        form
    }
}

impl ReplyKeyboardRemove {
    pub fn new(remove_keyboard: bool, ) -> Self {
        Self {
            remove_keyboard,
            selective: None,
        }
    }
}

impl FormSer for InlineKeyboardMarkup {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.inline_keyboard.serialize(format!("{}[inline_keyboard]", key), form);
        form
    }
}

impl InlineKeyboardMarkup {
    pub fn new(inline_keyboard: Vec<Vec<InlineKeyboardButton>>, ) -> Self {
        Self {
            inline_keyboard,
        }
    }
}

impl FormSer for InlineKeyboardButton {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.text.serialize(format!("{}[text]", key), form);
        form = self.url.serialize(format!("{}[url]", key), form);
        form = self.login_url.serialize(format!("{}[login_url]", key), form);
        form = self.callback_data.serialize(format!("{}[callback_data]", key), form);
        form = self.switch_inline_query.serialize(format!("{}[switch_inline_query]", key), form);
        form = self.switch_inline_query_current_chat.serialize(format!("{}[switch_inline_query_current_chat]", key), form);
        form = self.callback_game.serialize(format!("{}[callback_game]", key), form);
        form = self.pay.serialize(format!("{}[pay]", key), form);
        form
    }
}

impl InlineKeyboardButton {
    pub fn new(text: String, ) -> Self {
        Self {
            text,
            url: None,
            login_url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }
    }
}

impl FormSer for LoginUrl {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.url.serialize(format!("{}[url]", key), form);
        form = self.forward_text.serialize(format!("{}[forward_text]", key), form);
        form = self.bot_username.serialize(format!("{}[bot_username]", key), form);
        form = self.request_write_access.serialize(format!("{}[request_write_access]", key), form);
        form
    }
}

impl LoginUrl {
    pub fn new(url: String, ) -> Self {
        Self {
            url,
            forward_text: None,
            bot_username: None,
            request_write_access: None,
        }
    }
}

impl FormSer for CallbackQuery {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.from.serialize(format!("{}[from]", key), form);
        form = self.message.serialize(format!("{}[message]", key), form);
        form = self.inline_message_id.serialize(format!("{}[inline_message_id]", key), form);
        form = self.chat_instance.serialize(format!("{}[chat_instance]", key), form);
        form = self.data.serialize(format!("{}[data]", key), form);
        form = self.game_short_name.serialize(format!("{}[game_short_name]", key), form);
        form
    }
}

impl CallbackQuery {
    pub fn new(id: String, from: User, chat_instance: String, ) -> Self {
        Self {
            id,
            from,
            message: None,
            inline_message_id: None,
            chat_instance,
            data: None,
            game_short_name: None,
        }
    }
}

impl FormSer for ForceReply {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.force_reply.serialize(format!("{}[force_reply]", key), form);
        form = self.input_field_placeholder.serialize(format!("{}[input_field_placeholder]", key), form);
        form = self.selective.serialize(format!("{}[selective]", key), form);
        form
    }
}

impl ForceReply {
    pub fn new(force_reply: bool, ) -> Self {
        Self {
            force_reply,
            input_field_placeholder: None,
            selective: None,
        }
    }
}

impl FormSer for ChatPhoto {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.small_file_id.serialize(format!("{}[small_file_id]", key), form);
        form = self.small_file_unique_id.serialize(format!("{}[small_file_unique_id]", key), form);
        form = self.big_file_id.serialize(format!("{}[big_file_id]", key), form);
        form = self.big_file_unique_id.serialize(format!("{}[big_file_unique_id]", key), form);
        form
    }
}

impl ChatPhoto {
    pub fn new(small_file_id: String, small_file_unique_id: String, big_file_id: String, big_file_unique_id: String, ) -> Self {
        Self {
            small_file_id,
            small_file_unique_id,
            big_file_id,
            big_file_unique_id,
        }
    }
}

impl FormSer for ChatInviteLink {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.invite_link.serialize(format!("{}[invite_link]", key), form);
        form = self.creator.serialize(format!("{}[creator]", key), form);
        form = self.creates_join_request.serialize(format!("{}[creates_join_request]", key), form);
        form = self.is_primary.serialize(format!("{}[is_primary]", key), form);
        form = self.is_revoked.serialize(format!("{}[is_revoked]", key), form);
        form = self.name.serialize(format!("{}[name]", key), form);
        form = self.expire_date.serialize(format!("{}[expire_date]", key), form);
        form = self.member_limit.serialize(format!("{}[member_limit]", key), form);
        form = self.pending_join_request_count.serialize(format!("{}[pending_join_request_count]", key), form);
        form
    }
}

impl ChatInviteLink {
    pub fn new(invite_link: String, creator: User, creates_join_request: bool, is_primary: bool, is_revoked: bool, ) -> Self {
        Self {
            invite_link,
            creator,
            creates_join_request,
            is_primary,
            is_revoked,
            name: None,
            expire_date: None,
            member_limit: None,
            pending_join_request_count: None,
        }
    }
}

impl FormSer for ChatMemberOwner {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.status.serialize(format!("{}[status]", key), form);
        form = self.user.serialize(format!("{}[user]", key), form);
        form = self.is_anonymous.serialize(format!("{}[is_anonymous]", key), form);
        form = self.custom_title.serialize(format!("{}[custom_title]", key), form);
        form
    }
}

impl ChatMemberOwner {
    pub fn new(status: String, user: User, is_anonymous: bool, ) -> Self {
        Self {
            status,
            user,
            is_anonymous,
            custom_title: None,
        }
    }
}

impl FormSer for ChatMemberAdministrator {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.status.serialize(format!("{}[status]", key), form);
        form = self.user.serialize(format!("{}[user]", key), form);
        form = self.can_be_edited.serialize(format!("{}[can_be_edited]", key), form);
        form = self.is_anonymous.serialize(format!("{}[is_anonymous]", key), form);
        form = self.can_manage_chat.serialize(format!("{}[can_manage_chat]", key), form);
        form = self.can_delete_messages.serialize(format!("{}[can_delete_messages]", key), form);
        form = self.can_manage_voice_chats.serialize(format!("{}[can_manage_voice_chats]", key), form);
        form = self.can_restrict_members.serialize(format!("{}[can_restrict_members]", key), form);
        form = self.can_promote_members.serialize(format!("{}[can_promote_members]", key), form);
        form = self.can_change_info.serialize(format!("{}[can_change_info]", key), form);
        form = self.can_invite_users.serialize(format!("{}[can_invite_users]", key), form);
        form = self.can_post_messages.serialize(format!("{}[can_post_messages]", key), form);
        form = self.can_edit_messages.serialize(format!("{}[can_edit_messages]", key), form);
        form = self.can_pin_messages.serialize(format!("{}[can_pin_messages]", key), form);
        form = self.custom_title.serialize(format!("{}[custom_title]", key), form);
        form
    }
}

impl ChatMemberAdministrator {
    pub fn new(status: String, user: User, can_be_edited: bool, is_anonymous: bool, can_manage_chat: bool, can_delete_messages: bool, can_manage_voice_chats: bool, can_restrict_members: bool, can_promote_members: bool, can_change_info: bool, can_invite_users: bool, ) -> Self {
        Self {
            status,
            user,
            can_be_edited,
            is_anonymous,
            can_manage_chat,
            can_delete_messages,
            can_manage_voice_chats,
            can_restrict_members,
            can_promote_members,
            can_change_info,
            can_invite_users,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
            custom_title: None,
        }
    }
}

impl FormSer for ChatMemberMember {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.status.serialize(format!("{}[status]", key), form);
        form = self.user.serialize(format!("{}[user]", key), form);
        form
    }
}

impl ChatMemberMember {
    pub fn new(status: String, user: User, ) -> Self {
        Self {
            status,
            user,
        }
    }
}

impl FormSer for ChatMemberRestricted {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.status.serialize(format!("{}[status]", key), form);
        form = self.user.serialize(format!("{}[user]", key), form);
        form = self.is_member.serialize(format!("{}[is_member]", key), form);
        form = self.can_change_info.serialize(format!("{}[can_change_info]", key), form);
        form = self.can_invite_users.serialize(format!("{}[can_invite_users]", key), form);
        form = self.can_pin_messages.serialize(format!("{}[can_pin_messages]", key), form);
        form = self.can_send_messages.serialize(format!("{}[can_send_messages]", key), form);
        form = self.can_send_media_messages.serialize(format!("{}[can_send_media_messages]", key), form);
        form = self.can_send_polls.serialize(format!("{}[can_send_polls]", key), form);
        form = self.can_send_other_messages.serialize(format!("{}[can_send_other_messages]", key), form);
        form = self.can_add_web_page_previews.serialize(format!("{}[can_add_web_page_previews]", key), form);
        form = self.until_date.serialize(format!("{}[until_date]", key), form);
        form
    }
}

impl ChatMemberRestricted {
    pub fn new(status: String, user: User, is_member: bool, can_change_info: bool, can_invite_users: bool, can_pin_messages: bool, can_send_messages: bool, can_send_media_messages: bool, can_send_polls: bool, can_send_other_messages: bool, can_add_web_page_previews: bool, until_date: i64, ) -> Self {
        Self {
            status,
            user,
            is_member,
            can_change_info,
            can_invite_users,
            can_pin_messages,
            can_send_messages,
            can_send_media_messages,
            can_send_polls,
            can_send_other_messages,
            can_add_web_page_previews,
            until_date,
        }
    }
}

impl FormSer for ChatMemberLeft {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.status.serialize(format!("{}[status]", key), form);
        form = self.user.serialize(format!("{}[user]", key), form);
        form
    }
}

impl ChatMemberLeft {
    pub fn new(status: String, user: User, ) -> Self {
        Self {
            status,
            user,
        }
    }
}

impl FormSer for ChatMemberBanned {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.status.serialize(format!("{}[status]", key), form);
        form = self.user.serialize(format!("{}[user]", key), form);
        form = self.until_date.serialize(format!("{}[until_date]", key), form);
        form
    }
}

impl ChatMemberBanned {
    pub fn new(status: String, user: User, until_date: i64, ) -> Self {
        Self {
            status,
            user,
            until_date,
        }
    }
}

impl FormSer for ChatMemberUpdated {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat.serialize(format!("{}[chat]", key), form);
        form = self.from.serialize(format!("{}[from]", key), form);
        form = self.date.serialize(format!("{}[date]", key), form);
        form = self.old_chat_member.serialize(format!("{}[old_chat_member]", key), form);
        form = self.new_chat_member.serialize(format!("{}[new_chat_member]", key), form);
        form = self.invite_link.serialize(format!("{}[invite_link]", key), form);
        form
    }
}

impl ChatMemberUpdated {
    pub fn new(chat: Chat, from: User, date: i64, old_chat_member: ChatMember, new_chat_member: ChatMember, ) -> Self {
        Self {
            chat,
            from,
            date,
            old_chat_member,
            new_chat_member,
            invite_link: None,
        }
    }
}

impl FormSer for ChatJoinRequest {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.chat.serialize(format!("{}[chat]", key), form);
        form = self.from.serialize(format!("{}[from]", key), form);
        form = self.date.serialize(format!("{}[date]", key), form);
        form = self.bio.serialize(format!("{}[bio]", key), form);
        form = self.invite_link.serialize(format!("{}[invite_link]", key), form);
        form
    }
}

impl ChatJoinRequest {
    pub fn new(chat: Chat, from: User, date: i64, ) -> Self {
        Self {
            chat,
            from,
            date,
            bio: None,
            invite_link: None,
        }
    }
}

impl FormSer for ChatPermissions {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.can_send_messages.serialize(format!("{}[can_send_messages]", key), form);
        form = self.can_send_media_messages.serialize(format!("{}[can_send_media_messages]", key), form);
        form = self.can_send_polls.serialize(format!("{}[can_send_polls]", key), form);
        form = self.can_send_other_messages.serialize(format!("{}[can_send_other_messages]", key), form);
        form = self.can_add_web_page_previews.serialize(format!("{}[can_add_web_page_previews]", key), form);
        form = self.can_change_info.serialize(format!("{}[can_change_info]", key), form);
        form = self.can_invite_users.serialize(format!("{}[can_invite_users]", key), form);
        form = self.can_pin_messages.serialize(format!("{}[can_pin_messages]", key), form);
        form
    }
}

impl FormSer for ChatLocation {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.location.serialize(format!("{}[location]", key), form);
        form = self.address.serialize(format!("{}[address]", key), form);
        form
    }
}

impl ChatLocation {
    pub fn new(location: Location, address: String, ) -> Self {
        Self {
            location,
            address,
        }
    }
}

impl FormSer for BotCommand {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.command.serialize(format!("{}[command]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form
    }
}

impl BotCommand {
    pub fn new(command: String, description: String, ) -> Self {
        Self {
            command,
            description,
        }
    }
}

impl FormSer for BotCommandScopeDefault {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form
    }
}

impl BotCommandScopeDefault {
    pub fn new(type_: String, ) -> Self {
        Self {
            type_,
        }
    }
}

impl FormSer for BotCommandScopeAllPrivateChats {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form
    }
}

impl BotCommandScopeAllPrivateChats {
    pub fn new(type_: String, ) -> Self {
        Self {
            type_,
        }
    }
}

impl FormSer for BotCommandScopeAllGroupChats {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form
    }
}

impl BotCommandScopeAllGroupChats {
    pub fn new(type_: String, ) -> Self {
        Self {
            type_,
        }
    }
}

impl FormSer for BotCommandScopeAllChatAdministrators {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form
    }
}

impl BotCommandScopeAllChatAdministrators {
    pub fn new(type_: String, ) -> Self {
        Self {
            type_,
        }
    }
}

impl FormSer for BotCommandScopeChat {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.chat_id.serialize(format!("{}[chat_id]", key), form);
        form
    }
}

impl BotCommandScopeChat {
    pub fn new(type_: String, chat_id: ChatId, ) -> Self {
        Self {
            type_,
            chat_id,
        }
    }
}

impl FormSer for BotCommandScopeChatAdministrators {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.chat_id.serialize(format!("{}[chat_id]", key), form);
        form
    }
}

impl BotCommandScopeChatAdministrators {
    pub fn new(type_: String, chat_id: ChatId, ) -> Self {
        Self {
            type_,
            chat_id,
        }
    }
}

impl FormSer for BotCommandScopeChatMember {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.chat_id.serialize(format!("{}[chat_id]", key), form);
        form = self.user_id.serialize(format!("{}[user_id]", key), form);
        form
    }
}

impl BotCommandScopeChatMember {
    pub fn new(type_: String, chat_id: ChatId, user_id: i64, ) -> Self {
        Self {
            type_,
            chat_id,
            user_id,
        }
    }
}

impl FormSer for ResponseParameters {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.migrate_to_chat_id.serialize(format!("{}[migrate_to_chat_id]", key), form);
        form = self.retry_after.serialize(format!("{}[retry_after]", key), form);
        form
    }
}

impl FormSer for InputMediaPhoto {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.media.serialize(format!("{}[media]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form
    }
}

impl InputMediaPhoto {
    pub fn new(type_: String, media: String, ) -> Self {
        Self {
            type_,
            media,
            caption: None,
            parse_mode: None,
            caption_entities: None,
        }
    }
}

impl FormSer for InputMediaVideo {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.media.serialize(format!("{}[media]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.width.serialize(format!("{}[width]", key), form);
        form = self.height.serialize(format!("{}[height]", key), form);
        form = self.duration.serialize(format!("{}[duration]", key), form);
        form = self.supports_streaming.serialize(format!("{}[supports_streaming]", key), form);
        form
    }
}

impl InputMediaVideo {
    pub fn new(type_: String, media: String, ) -> Self {
        Self {
            type_,
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        }
    }
}

impl FormSer for InputMediaAnimation {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.media.serialize(format!("{}[media]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.width.serialize(format!("{}[width]", key), form);
        form = self.height.serialize(format!("{}[height]", key), form);
        form = self.duration.serialize(format!("{}[duration]", key), form);
        form
    }
}

impl InputMediaAnimation {
    pub fn new(type_: String, media: String, ) -> Self {
        Self {
            type_,
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
        }
    }
}

impl FormSer for InputMediaAudio {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.media.serialize(format!("{}[media]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.duration.serialize(format!("{}[duration]", key), form);
        form = self.performer.serialize(format!("{}[performer]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form
    }
}

impl InputMediaAudio {
    pub fn new(type_: String, media: String, ) -> Self {
        Self {
            type_,
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
        }
    }
}

impl FormSer for InputMediaDocument {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.media.serialize(format!("{}[media]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.disable_content_type_detection.serialize(format!("{}[disable_content_type_detection]", key), form);
        form
    }
}

impl InputMediaDocument {
    pub fn new(type_: String, media: String, ) -> Self {
        Self {
            type_,
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
        }
    }
}

impl FormSer for Sticker {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.file_id.serialize(format!("{}[file_id]", key), form);
        form = self.file_unique_id.serialize(format!("{}[file_unique_id]", key), form);
        form = self.width.serialize(format!("{}[width]", key), form);
        form = self.height.serialize(format!("{}[height]", key), form);
        form = self.is_animated.serialize(format!("{}[is_animated]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form = self.emoji.serialize(format!("{}[emoji]", key), form);
        form = self.set_name.serialize(format!("{}[set_name]", key), form);
        form = self.mask_position.serialize(format!("{}[mask_position]", key), form);
        form = self.file_size.serialize(format!("{}[file_size]", key), form);
        form
    }
}

impl Sticker {
    pub fn new(file_id: String, file_unique_id: String, width: i64, height: i64, is_animated: bool, ) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            is_animated,
            thumb: None,
            emoji: None,
            set_name: None,
            mask_position: None,
            file_size: None,
        }
    }
}

impl FormSer for StickerSet {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.name.serialize(format!("{}[name]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.is_animated.serialize(format!("{}[is_animated]", key), form);
        form = self.contains_masks.serialize(format!("{}[contains_masks]", key), form);
        form = self.stickers.serialize(format!("{}[stickers]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form
    }
}

impl StickerSet {
    pub fn new(name: String, title: String, is_animated: bool, contains_masks: bool, stickers: Vec<Sticker>, ) -> Self {
        Self {
            name,
            title,
            is_animated,
            contains_masks,
            stickers,
            thumb: None,
        }
    }
}

impl FormSer for MaskPosition {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.point.serialize(format!("{}[point]", key), form);
        form = self.x_shift.serialize(format!("{}[x_shift]", key), form);
        form = self.y_shift.serialize(format!("{}[y_shift]", key), form);
        form = self.scale.serialize(format!("{}[scale]", key), form);
        form
    }
}

impl MaskPosition {
    pub fn new(point: String, x_shift: f64, y_shift: f64, scale: f64, ) -> Self {
        Self {
            point,
            x_shift,
            y_shift,
            scale,
        }
    }
}

impl FormSer for InlineQuery {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.from.serialize(format!("{}[from]", key), form);
        form = self.query.serialize(format!("{}[query]", key), form);
        form = self.offset.serialize(format!("{}[offset]", key), form);
        form = self.chat_type.serialize(format!("{}[chat_type]", key), form);
        form = self.location.serialize(format!("{}[location]", key), form);
        form
    }
}

impl InlineQuery {
    pub fn new(id: String, from: User, query: String, offset: String, ) -> Self {
        Self {
            id,
            from,
            query,
            offset,
            chat_type: None,
            location: None,
        }
    }
}

impl FormSer for InlineQueryResultArticle {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.url.serialize(format!("{}[url]", key), form);
        form = self.hide_url.serialize(format!("{}[hide_url]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_width.serialize(format!("{}[thumb_width]", key), form);
        form = self.thumb_height.serialize(format!("{}[thumb_height]", key), form);
        form
    }
}

impl InlineQueryResultArticle {
    pub fn new(type_: String, id: String, title: String, input_message_content: InputMessageContent, ) -> Self {
        Self {
            type_,
            id,
            title,
            input_message_content,
            reply_markup: None,
            url: None,
            hide_url: None,
            description: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

impl FormSer for InlineQueryResultPhoto {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.photo_url.serialize(format!("{}[photo_url]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.photo_width.serialize(format!("{}[photo_width]", key), form);
        form = self.photo_height.serialize(format!("{}[photo_height]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultPhoto {
    pub fn new(type_: String, id: String, photo_url: String, thumb_url: String, ) -> Self {
        Self {
            type_,
            id,
            photo_url,
            thumb_url,
            photo_width: None,
            photo_height: None,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultGif {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.gif_url.serialize(format!("{}[gif_url]", key), form);
        form = self.gif_width.serialize(format!("{}[gif_width]", key), form);
        form = self.gif_height.serialize(format!("{}[gif_height]", key), form);
        form = self.gif_duration.serialize(format!("{}[gif_duration]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_mime_type.serialize(format!("{}[thumb_mime_type]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultGif {
    pub fn new(type_: String, id: String, gif_url: String, thumb_url: String, ) -> Self {
        Self {
            type_,
            id,
            gif_url,
            gif_width: None,
            gif_height: None,
            gif_duration: None,
            thumb_url,
            thumb_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultMpeg4Gif {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.mpeg4_url.serialize(format!("{}[mpeg4_url]", key), form);
        form = self.mpeg4_width.serialize(format!("{}[mpeg4_width]", key), form);
        form = self.mpeg4_height.serialize(format!("{}[mpeg4_height]", key), form);
        form = self.mpeg4_duration.serialize(format!("{}[mpeg4_duration]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_mime_type.serialize(format!("{}[thumb_mime_type]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultMpeg4Gif {
    pub fn new(type_: String, id: String, mpeg4_url: String, thumb_url: String, ) -> Self {
        Self {
            type_,
            id,
            mpeg4_url,
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            thumb_url,
            thumb_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultVideo {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.video_url.serialize(format!("{}[video_url]", key), form);
        form = self.mime_type.serialize(format!("{}[mime_type]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.video_width.serialize(format!("{}[video_width]", key), form);
        form = self.video_height.serialize(format!("{}[video_height]", key), form);
        form = self.video_duration.serialize(format!("{}[video_duration]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultVideo {
    pub fn new(type_: String, id: String, video_url: String, mime_type: String, thumb_url: String, title: String, ) -> Self {
        Self {
            type_,
            id,
            video_url,
            mime_type,
            thumb_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultAudio {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.audio_url.serialize(format!("{}[audio_url]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.performer.serialize(format!("{}[performer]", key), form);
        form = self.audio_duration.serialize(format!("{}[audio_duration]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultAudio {
    pub fn new(type_: String, id: String, audio_url: String, title: String, ) -> Self {
        Self {
            type_,
            id,
            audio_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            performer: None,
            audio_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultVoice {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.voice_url.serialize(format!("{}[voice_url]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.voice_duration.serialize(format!("{}[voice_duration]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultVoice {
    pub fn new(type_: String, id: String, voice_url: String, title: String, ) -> Self {
        Self {
            type_,
            id,
            voice_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            voice_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultDocument {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.document_url.serialize(format!("{}[document_url]", key), form);
        form = self.mime_type.serialize(format!("{}[mime_type]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_width.serialize(format!("{}[thumb_width]", key), form);
        form = self.thumb_height.serialize(format!("{}[thumb_height]", key), form);
        form
    }
}

impl InlineQueryResultDocument {
    pub fn new(type_: String, id: String, title: String, document_url: String, mime_type: String, ) -> Self {
        Self {
            type_,
            id,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            document_url,
            mime_type,
            description: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

impl FormSer for InlineQueryResultLocation {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.latitude.serialize(format!("{}[latitude]", key), form);
        form = self.longitude.serialize(format!("{}[longitude]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.horizontal_accuracy.serialize(format!("{}[horizontal_accuracy]", key), form);
        form = self.live_period.serialize(format!("{}[live_period]", key), form);
        form = self.heading.serialize(format!("{}[heading]", key), form);
        form = self.proximity_alert_radius.serialize(format!("{}[proximity_alert_radius]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_width.serialize(format!("{}[thumb_width]", key), form);
        form = self.thumb_height.serialize(format!("{}[thumb_height]", key), form);
        form
    }
}

impl InlineQueryResultLocation {
    pub fn new(type_: String, id: String, latitude: f64, longitude: f64, title: String, ) -> Self {
        Self {
            type_,
            id,
            latitude,
            longitude,
            title,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

impl FormSer for InlineQueryResultVenue {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.latitude.serialize(format!("{}[latitude]", key), form);
        form = self.longitude.serialize(format!("{}[longitude]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.address.serialize(format!("{}[address]", key), form);
        form = self.foursquare_id.serialize(format!("{}[foursquare_id]", key), form);
        form = self.foursquare_type.serialize(format!("{}[foursquare_type]", key), form);
        form = self.google_place_id.serialize(format!("{}[google_place_id]", key), form);
        form = self.google_place_type.serialize(format!("{}[google_place_type]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_width.serialize(format!("{}[thumb_width]", key), form);
        form = self.thumb_height.serialize(format!("{}[thumb_height]", key), form);
        form
    }
}

impl InlineQueryResultVenue {
    pub fn new(type_: String, id: String, latitude: f64, longitude: f64, title: String, address: String, ) -> Self {
        Self {
            type_,
            id,
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

impl FormSer for InlineQueryResultContact {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.phone_number.serialize(format!("{}[phone_number]", key), form);
        form = self.first_name.serialize(format!("{}[first_name]", key), form);
        form = self.last_name.serialize(format!("{}[last_name]", key), form);
        form = self.vcard.serialize(format!("{}[vcard]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form = self.thumb_url.serialize(format!("{}[thumb_url]", key), form);
        form = self.thumb_width.serialize(format!("{}[thumb_width]", key), form);
        form = self.thumb_height.serialize(format!("{}[thumb_height]", key), form);
        form
    }
}

impl InlineQueryResultContact {
    pub fn new(type_: String, id: String, phone_number: String, first_name: String, ) -> Self {
        Self {
            type_,
            id,
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

impl FormSer for InlineQueryResultGame {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.game_short_name.serialize(format!("{}[game_short_name]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form
    }
}

impl InlineQueryResultGame {
    pub fn new(type_: String, id: String, game_short_name: String, ) -> Self {
        Self {
            type_,
            id,
            game_short_name,
            reply_markup: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedPhoto {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.photo_file_id.serialize(format!("{}[photo_file_id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedPhoto {
    pub fn new(type_: String, id: String, photo_file_id: String, ) -> Self {
        Self {
            type_,
            id,
            photo_file_id,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedGif {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.gif_file_id.serialize(format!("{}[gif_file_id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedGif {
    pub fn new(type_: String, id: String, gif_file_id: String, ) -> Self {
        Self {
            type_,
            id,
            gif_file_id,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedMpeg4Gif {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.mpeg4_file_id.serialize(format!("{}[mpeg4_file_id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedMpeg4Gif {
    pub fn new(type_: String, id: String, mpeg4_file_id: String, ) -> Self {
        Self {
            type_,
            id,
            mpeg4_file_id,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedSticker {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.sticker_file_id.serialize(format!("{}[sticker_file_id]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedSticker {
    pub fn new(type_: String, id: String, sticker_file_id: String, ) -> Self {
        Self {
            type_,
            id,
            sticker_file_id,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedDocument {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.document_file_id.serialize(format!("{}[document_file_id]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedDocument {
    pub fn new(type_: String, id: String, title: String, document_file_id: String, ) -> Self {
        Self {
            type_,
            id,
            title,
            document_file_id,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedVideo {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.video_file_id.serialize(format!("{}[video_file_id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedVideo {
    pub fn new(type_: String, id: String, video_file_id: String, title: String, ) -> Self {
        Self {
            type_,
            id,
            video_file_id,
            title,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedVoice {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.voice_file_id.serialize(format!("{}[voice_file_id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedVoice {
    pub fn new(type_: String, id: String, voice_file_id: String, title: String, ) -> Self {
        Self {
            type_,
            id,
            voice_file_id,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InlineQueryResultCachedAudio {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.audio_file_id.serialize(format!("{}[audio_file_id]", key), form);
        form = self.caption.serialize(format!("{}[caption]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.caption_entities.serialize(format!("{}[caption_entities]", key), form);
        form = self.reply_markup.serialize(format!("{}[reply_markup]", key), form);
        form = self.input_message_content.serialize(format!("{}[input_message_content]", key), form);
        form
    }
}

impl InlineQueryResultCachedAudio {
    pub fn new(type_: String, id: String, audio_file_id: String, ) -> Self {
        Self {
            type_,
            id,
            audio_file_id,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl FormSer for InputTextMessageContent {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.message_text.serialize(format!("{}[message_text]", key), form);
        form = self.parse_mode.serialize(format!("{}[parse_mode]", key), form);
        form = self.entities.serialize(format!("{}[entities]", key), form);
        form = self.disable_web_page_preview.serialize(format!("{}[disable_web_page_preview]", key), form);
        form
    }
}

impl InputTextMessageContent {
    pub fn new(message_text: String, ) -> Self {
        Self {
            message_text,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
        }
    }
}

impl FormSer for InputLocationMessageContent {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.latitude.serialize(format!("{}[latitude]", key), form);
        form = self.longitude.serialize(format!("{}[longitude]", key), form);
        form = self.horizontal_accuracy.serialize(format!("{}[horizontal_accuracy]", key), form);
        form = self.live_period.serialize(format!("{}[live_period]", key), form);
        form = self.heading.serialize(format!("{}[heading]", key), form);
        form = self.proximity_alert_radius.serialize(format!("{}[proximity_alert_radius]", key), form);
        form
    }
}

impl InputLocationMessageContent {
    pub fn new(latitude: f64, longitude: f64, ) -> Self {
        Self {
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        }
    }
}

impl FormSer for InputVenueMessageContent {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.latitude.serialize(format!("{}[latitude]", key), form);
        form = self.longitude.serialize(format!("{}[longitude]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.address.serialize(format!("{}[address]", key), form);
        form = self.foursquare_id.serialize(format!("{}[foursquare_id]", key), form);
        form = self.foursquare_type.serialize(format!("{}[foursquare_type]", key), form);
        form = self.google_place_id.serialize(format!("{}[google_place_id]", key), form);
        form = self.google_place_type.serialize(format!("{}[google_place_type]", key), form);
        form
    }
}

impl InputVenueMessageContent {
    pub fn new(latitude: f64, longitude: f64, title: String, address: String, ) -> Self {
        Self {
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }
}

impl FormSer for InputContactMessageContent {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.phone_number.serialize(format!("{}[phone_number]", key), form);
        form = self.first_name.serialize(format!("{}[first_name]", key), form);
        form = self.last_name.serialize(format!("{}[last_name]", key), form);
        form = self.vcard.serialize(format!("{}[vcard]", key), form);
        form
    }
}

impl InputContactMessageContent {
    pub fn new(phone_number: String, first_name: String, ) -> Self {
        Self {
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
        }
    }
}

impl FormSer for InputInvoiceMessageContent {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.payload.serialize(format!("{}[payload]", key), form);
        form = self.provider_token.serialize(format!("{}[provider_token]", key), form);
        form = self.currency.serialize(format!("{}[currency]", key), form);
        form = self.prices.serialize(format!("{}[prices]", key), form);
        form = self.max_tip_amount.serialize(format!("{}[max_tip_amount]", key), form);
        form = self.suggested_tip_amounts.serialize(format!("{}[suggested_tip_amounts]", key), form);
        form = self.provider_data.serialize(format!("{}[provider_data]", key), form);
        form = self.photo_url.serialize(format!("{}[photo_url]", key), form);
        form = self.photo_size.serialize(format!("{}[photo_size]", key), form);
        form = self.photo_width.serialize(format!("{}[photo_width]", key), form);
        form = self.photo_height.serialize(format!("{}[photo_height]", key), form);
        form = self.need_name.serialize(format!("{}[need_name]", key), form);
        form = self.need_phone_number.serialize(format!("{}[need_phone_number]", key), form);
        form = self.need_email.serialize(format!("{}[need_email]", key), form);
        form = self.need_shipping_address.serialize(format!("{}[need_shipping_address]", key), form);
        form = self.send_phone_number_to_provider.serialize(format!("{}[send_phone_number_to_provider]", key), form);
        form = self.send_email_to_provider.serialize(format!("{}[send_email_to_provider]", key), form);
        form = self.is_flexible.serialize(format!("{}[is_flexible]", key), form);
        form
    }
}

impl InputInvoiceMessageContent {
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

impl FormSer for ChosenInlineResult {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.result_id.serialize(format!("{}[result_id]", key), form);
        form = self.from.serialize(format!("{}[from]", key), form);
        form = self.location.serialize(format!("{}[location]", key), form);
        form = self.inline_message_id.serialize(format!("{}[inline_message_id]", key), form);
        form = self.query.serialize(format!("{}[query]", key), form);
        form
    }
}

impl ChosenInlineResult {
    pub fn new(result_id: String, from: User, query: String, ) -> Self {
        Self {
            result_id,
            from,
            location: None,
            inline_message_id: None,
            query,
        }
    }
}

impl FormSer for LabeledPrice {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.label.serialize(format!("{}[label]", key), form);
        form = self.amount.serialize(format!("{}[amount]", key), form);
        form
    }
}

impl LabeledPrice {
    pub fn new(label: String, amount: i64, ) -> Self {
        Self {
            label,
            amount,
        }
    }
}

impl FormSer for Invoice {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.start_parameter.serialize(format!("{}[start_parameter]", key), form);
        form = self.currency.serialize(format!("{}[currency]", key), form);
        form = self.total_amount.serialize(format!("{}[total_amount]", key), form);
        form
    }
}

impl Invoice {
    pub fn new(title: String, description: String, start_parameter: String, currency: String, total_amount: i64, ) -> Self {
        Self {
            title,
            description,
            start_parameter,
            currency,
            total_amount,
        }
    }
}

impl FormSer for ShippingAddress {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.country_code.serialize(format!("{}[country_code]", key), form);
        form = self.state.serialize(format!("{}[state]", key), form);
        form = self.city.serialize(format!("{}[city]", key), form);
        form = self.street_line1.serialize(format!("{}[street_line1]", key), form);
        form = self.street_line2.serialize(format!("{}[street_line2]", key), form);
        form = self.post_code.serialize(format!("{}[post_code]", key), form);
        form
    }
}

impl ShippingAddress {
    pub fn new(country_code: String, state: String, city: String, street_line1: String, street_line2: String, post_code: String, ) -> Self {
        Self {
            country_code,
            state,
            city,
            street_line1,
            street_line2,
            post_code,
        }
    }
}

impl FormSer for OrderInfo {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.name.serialize(format!("{}[name]", key), form);
        form = self.phone_number.serialize(format!("{}[phone_number]", key), form);
        form = self.email.serialize(format!("{}[email]", key), form);
        form = self.shipping_address.serialize(format!("{}[shipping_address]", key), form);
        form
    }
}

impl FormSer for ShippingOption {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.prices.serialize(format!("{}[prices]", key), form);
        form
    }
}

impl ShippingOption {
    pub fn new(id: String, title: String, prices: Vec<LabeledPrice>, ) -> Self {
        Self {
            id,
            title,
            prices,
        }
    }
}

impl FormSer for SuccessfulPayment {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.currency.serialize(format!("{}[currency]", key), form);
        form = self.total_amount.serialize(format!("{}[total_amount]", key), form);
        form = self.invoice_payload.serialize(format!("{}[invoice_payload]", key), form);
        form = self.shipping_option_id.serialize(format!("{}[shipping_option_id]", key), form);
        form = self.order_info.serialize(format!("{}[order_info]", key), form);
        form = self.telegram_payment_charge_id.serialize(format!("{}[telegram_payment_charge_id]", key), form);
        form = self.provider_payment_charge_id.serialize(format!("{}[provider_payment_charge_id]", key), form);
        form
    }
}

impl SuccessfulPayment {
    pub fn new(currency: String, total_amount: i64, invoice_payload: String, telegram_payment_charge_id: String, provider_payment_charge_id: String, ) -> Self {
        Self {
            currency,
            total_amount,
            invoice_payload,
            shipping_option_id: None,
            order_info: None,
            telegram_payment_charge_id,
            provider_payment_charge_id,
        }
    }
}

impl FormSer for ShippingQuery {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.from.serialize(format!("{}[from]", key), form);
        form = self.invoice_payload.serialize(format!("{}[invoice_payload]", key), form);
        form = self.shipping_address.serialize(format!("{}[shipping_address]", key), form);
        form
    }
}

impl ShippingQuery {
    pub fn new(id: String, from: User, invoice_payload: String, shipping_address: ShippingAddress, ) -> Self {
        Self {
            id,
            from,
            invoice_payload,
            shipping_address,
        }
    }
}

impl FormSer for PreCheckoutQuery {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.from.serialize(format!("{}[from]", key), form);
        form = self.currency.serialize(format!("{}[currency]", key), form);
        form = self.total_amount.serialize(format!("{}[total_amount]", key), form);
        form = self.invoice_payload.serialize(format!("{}[invoice_payload]", key), form);
        form = self.shipping_option_id.serialize(format!("{}[shipping_option_id]", key), form);
        form = self.order_info.serialize(format!("{}[order_info]", key), form);
        form
    }
}

impl PreCheckoutQuery {
    pub fn new(id: String, from: User, currency: String, total_amount: i64, invoice_payload: String, ) -> Self {
        Self {
            id,
            from,
            currency,
            total_amount,
            invoice_payload,
            shipping_option_id: None,
            order_info: None,
        }
    }
}

impl FormSer for PassportData {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.data.serialize(format!("{}[data]", key), form);
        form = self.credentials.serialize(format!("{}[credentials]", key), form);
        form
    }
}

impl PassportData {
    pub fn new(data: Vec<EncryptedPassportElement>, credentials: EncryptedCredentials, ) -> Self {
        Self {
            data,
            credentials,
        }
    }
}

impl FormSer for PassportFile {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.file_id.serialize(format!("{}[file_id]", key), form);
        form = self.file_unique_id.serialize(format!("{}[file_unique_id]", key), form);
        form = self.file_size.serialize(format!("{}[file_size]", key), form);
        form = self.file_date.serialize(format!("{}[file_date]", key), form);
        form
    }
}

impl PassportFile {
    pub fn new(file_id: String, file_unique_id: String, file_size: i64, file_date: i64, ) -> Self {
        Self {
            file_id,
            file_unique_id,
            file_size,
            file_date,
        }
    }
}

impl FormSer for EncryptedPassportElement {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.data.serialize(format!("{}[data]", key), form);
        form = self.phone_number.serialize(format!("{}[phone_number]", key), form);
        form = self.email.serialize(format!("{}[email]", key), form);
        form = self.files.serialize(format!("{}[files]", key), form);
        form = self.front_side.serialize(format!("{}[front_side]", key), form);
        form = self.reverse_side.serialize(format!("{}[reverse_side]", key), form);
        form = self.selfie.serialize(format!("{}[selfie]", key), form);
        form = self.translation.serialize(format!("{}[translation]", key), form);
        form = self.hash.serialize(format!("{}[hash]", key), form);
        form
    }
}

impl EncryptedPassportElement {
    pub fn new(type_: String, hash: String, ) -> Self {
        Self {
            type_,
            data: None,
            phone_number: None,
            email: None,
            files: None,
            front_side: None,
            reverse_side: None,
            selfie: None,
            translation: None,
            hash,
        }
    }
}

impl FormSer for EncryptedCredentials {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.data.serialize(format!("{}[data]", key), form);
        form = self.hash.serialize(format!("{}[hash]", key), form);
        form = self.secret.serialize(format!("{}[secret]", key), form);
        form
    }
}

impl EncryptedCredentials {
    pub fn new(data: String, hash: String, secret: String, ) -> Self {
        Self {
            data,
            hash,
            secret,
        }
    }
}

impl FormSer for PassportElementErrorDataField {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.source.serialize(format!("{}[source]", key), form);
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.field_name.serialize(format!("{}[field_name]", key), form);
        form = self.data_hash.serialize(format!("{}[data_hash]", key), form);
        form = self.message.serialize(format!("{}[message]", key), form);
        form
    }
}

impl PassportElementErrorDataField {
    pub fn new(source: String, type_: String, field_name: String, data_hash: String, message: String, ) -> Self {
        Self {
            source,
            type_,
            field_name,
            data_hash,
            message,
        }
    }
}

impl FormSer for PassportElementErrorFrontSide {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.source.serialize(format!("{}[source]", key), form);
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.file_hash.serialize(format!("{}[file_hash]", key), form);
        form = self.message.serialize(format!("{}[message]", key), form);
        form
    }
}

impl PassportElementErrorFrontSide {
    pub fn new(source: String, type_: String, file_hash: String, message: String, ) -> Self {
        Self {
            source,
            type_,
            file_hash,
            message,
        }
    }
}

impl FormSer for PassportElementErrorReverseSide {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.source.serialize(format!("{}[source]", key), form);
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.file_hash.serialize(format!("{}[file_hash]", key), form);
        form = self.message.serialize(format!("{}[message]", key), form);
        form
    }
}

impl PassportElementErrorReverseSide {
    pub fn new(source: String, type_: String, file_hash: String, message: String, ) -> Self {
        Self {
            source,
            type_,
            file_hash,
            message,
        }
    }
}

impl FormSer for PassportElementErrorSelfie {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.source.serialize(format!("{}[source]", key), form);
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.file_hash.serialize(format!("{}[file_hash]", key), form);
        form = self.message.serialize(format!("{}[message]", key), form);
        form
    }
}

impl PassportElementErrorSelfie {
    pub fn new(source: String, type_: String, file_hash: String, message: String, ) -> Self {
        Self {
            source,
            type_,
            file_hash,
            message,
        }
    }
}

impl FormSer for PassportElementErrorFile {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.source.serialize(format!("{}[source]", key), form);
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.file_hash.serialize(format!("{}[file_hash]", key), form);
        form = self.message.serialize(format!("{}[message]", key), form);
        form
    }
}

impl PassportElementErrorFile {
    pub fn new(source: String, type_: String, file_hash: String, message: String, ) -> Self {
        Self {
            source,
            type_,
            file_hash,
            message,
        }
    }
}

impl FormSer for PassportElementErrorFiles {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.source.serialize(format!("{}[source]", key), form);
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.file_hashes.serialize(format!("{}[file_hashes]", key), form);
        form = self.message.serialize(format!("{}[message]", key), form);
        form
    }
}

impl PassportElementErrorFiles {
    pub fn new(source: String, type_: String, file_hashes: Vec<String>, message: String, ) -> Self {
        Self {
            source,
            type_,
            file_hashes,
            message,
        }
    }
}

impl FormSer for PassportElementErrorTranslationFile {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.source.serialize(format!("{}[source]", key), form);
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.file_hash.serialize(format!("{}[file_hash]", key), form);
        form = self.message.serialize(format!("{}[message]", key), form);
        form
    }
}

impl PassportElementErrorTranslationFile {
    pub fn new(source: String, type_: String, file_hash: String, message: String, ) -> Self {
        Self {
            source,
            type_,
            file_hash,
            message,
        }
    }
}

impl FormSer for PassportElementErrorTranslationFiles {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.source.serialize(format!("{}[source]", key), form);
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.file_hashes.serialize(format!("{}[file_hashes]", key), form);
        form = self.message.serialize(format!("{}[message]", key), form);
        form
    }
}

impl PassportElementErrorTranslationFiles {
    pub fn new(source: String, type_: String, file_hashes: Vec<String>, message: String, ) -> Self {
        Self {
            source,
            type_,
            file_hashes,
            message,
        }
    }
}

impl FormSer for PassportElementErrorUnspecified {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.source.serialize(format!("{}[source]", key), form);
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.element_hash.serialize(format!("{}[element_hash]", key), form);
        form = self.message.serialize(format!("{}[message]", key), form);
        form
    }
}

impl PassportElementErrorUnspecified {
    pub fn new(source: String, type_: String, element_hash: String, message: String, ) -> Self {
        Self {
            source,
            type_,
            element_hash,
            message,
        }
    }
}

impl FormSer for Game {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.description.serialize(format!("{}[description]", key), form);
        form = self.photo.serialize(format!("{}[photo]", key), form);
        form = self.text.serialize(format!("{}[text]", key), form);
        form = self.text_entities.serialize(format!("{}[text_entities]", key), form);
        form = self.animation.serialize(format!("{}[animation]", key), form);
        form
    }
}

impl Game {
    pub fn new(title: String, description: String, photo: Vec<PhotoSize>, ) -> Self {
        Self {
            title,
            description,
            photo,
            text: None,
            text_entities: None,
            animation: None,
        }
    }
}

impl FormSer for CallbackGame {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form
    }
}

impl FormSer for GameHighScore {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.position.serialize(format!("{}[position]", key), form);
        form = self.user.serialize(format!("{}[user]", key), form);
        form = self.score.serialize(format!("{}[score]", key), form);
        form
    }
}

impl GameHighScore {
    pub fn new(position: i64, user: User, score: i64, ) -> Self {
        Self {
            position,
            user,
            score,
        }
    }
}

