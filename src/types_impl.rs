use crate::types::*;
use crate::form_ser::*;
use crate::helpers::*;

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

impl FormSer for MenuButton {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        match self {
            MenuButton::Commands(e) => e.serialize(key, form),
            MenuButton::WebApp(e) => e.serialize(key, form),
            MenuButton::Default(e) => e.serialize(key, form),
        }
    }
}

impl From<MenuButtonCommands> for MenuButton {
    fn from(o: MenuButtonCommands) -> Self {
        Self::Commands(o)
    }
}

impl From<MenuButtonWebApp> for MenuButton {
    fn from(o: MenuButtonWebApp) -> Self {
        Self::WebApp(o)
    }
}

impl From<MenuButtonDefault> for MenuButton {
    fn from(o: MenuButtonDefault) -> Self {
        Self::Default(o)
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
        form = self.last_synchronization_error_date.serialize(format!("{}[last_synchronization_error_date]", key), form);
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
            last_synchronization_error_date: None,
            max_connections: None,
            allowed_updates: None,
        }
    }
}

impl WebhookInfo {
    pub fn with_ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn with_last_error_date(mut self, last_error_date: i64) -> Self {
        self.last_error_date = Some(last_error_date);
        self
    }

    pub fn with_last_error_message(mut self, last_error_message: String) -> Self {
        self.last_error_message = Some(last_error_message);
        self
    }

    pub fn with_last_synchronization_error_date(mut self, last_synchronization_error_date: i64) -> Self {
        self.last_synchronization_error_date = Some(last_synchronization_error_date);
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

}

impl FormSer for User {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.id.serialize(format!("{}[id]", key), form);
        form = self.is_bot.serialize(format!("{}[is_bot]", key), form);
        form = self.first_name.serialize(format!("{}[first_name]", key), form);
        form = self.last_name.serialize(format!("{}[last_name]", key), form);
        form = self.username.serialize(format!("{}[username]", key), form);
        form = self.language_code.serialize(format!("{}[language_code]", key), form);
        form = self.is_premium.serialize(format!("{}[is_premium]", key), form);
        form = self.added_to_attachment_menu.serialize(format!("{}[added_to_attachment_menu]", key), form);
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
            is_premium: None,
            added_to_attachment_menu: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_inline_queries: None,
        }
    }
}

impl User {
    pub fn with_last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn with_username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn with_language_code(mut self, language_code: String) -> Self {
        self.language_code = Some(language_code);
        self
    }

    pub fn with_is_premium(mut self, is_premium: bool) -> Self {
        self.is_premium = Some(is_premium);
        self
    }

    pub fn with_added_to_attachment_menu(mut self, added_to_attachment_menu: bool) -> Self {
        self.added_to_attachment_menu = Some(added_to_attachment_menu);
        self
    }

    pub fn with_can_join_groups(mut self, can_join_groups: bool) -> Self {
        self.can_join_groups = Some(can_join_groups);
        self
    }

    pub fn with_can_read_all_group_messages(mut self, can_read_all_group_messages: bool) -> Self {
        self.can_read_all_group_messages = Some(can_read_all_group_messages);
        self
    }

    pub fn with_supports_inline_queries(mut self, supports_inline_queries: bool) -> Self {
        self.supports_inline_queries = Some(supports_inline_queries);
        self
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
        form = self.join_to_send_messages.serialize(format!("{}[join_to_send_messages]", key), form);
        form = self.join_by_request.serialize(format!("{}[join_by_request]", key), form);
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
            join_to_send_messages: None,
            join_by_request: None,
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

impl Chat {
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn with_first_name(mut self, first_name: String) -> Self {
        self.first_name = Some(first_name);
        self
    }

    pub fn with_last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn with_photo(mut self, photo: ChatPhoto) -> Self {
        self.photo = Some(photo);
        self
    }

    pub fn with_bio(mut self, bio: String) -> Self {
        self.bio = Some(bio);
        self
    }

    pub fn with_has_private_forwards(mut self, has_private_forwards: bool) -> Self {
        self.has_private_forwards = Some(has_private_forwards);
        self
    }

    pub fn with_join_to_send_messages(mut self, join_to_send_messages: bool) -> Self {
        self.join_to_send_messages = Some(join_to_send_messages);
        self
    }

    pub fn with_join_by_request(mut self, join_by_request: bool) -> Self {
        self.join_by_request = Some(join_by_request);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_invite_link(mut self, invite_link: String) -> Self {
        self.invite_link = Some(invite_link);
        self
    }

    pub fn with_pinned_message(mut self, pinned_message: Box<Message>) -> Self {
        self.pinned_message = Some(pinned_message);
        self
    }

    pub fn with_permissions(mut self, permissions: ChatPermissions) -> Self {
        self.permissions = Some(permissions);
        self
    }

    pub fn with_slow_mode_delay(mut self, slow_mode_delay: i64) -> Self {
        self.slow_mode_delay = Some(slow_mode_delay);
        self
    }

    pub fn with_message_auto_delete_time(mut self, message_auto_delete_time: i64) -> Self {
        self.message_auto_delete_time = Some(message_auto_delete_time);
        self
    }

    pub fn with_has_protected_content(mut self, has_protected_content: bool) -> Self {
        self.has_protected_content = Some(has_protected_content);
        self
    }

    pub fn with_sticker_set_name(mut self, sticker_set_name: String) -> Self {
        self.sticker_set_name = Some(sticker_set_name);
        self
    }

    pub fn with_can_set_sticker_set(mut self, can_set_sticker_set: bool) -> Self {
        self.can_set_sticker_set = Some(can_set_sticker_set);
        self
    }

    pub fn with_linked_chat_id(mut self, linked_chat_id: i64) -> Self {
        self.linked_chat_id = Some(linked_chat_id);
        self
    }

    pub fn with_location(mut self, location: ChatLocation) -> Self {
        self.location = Some(location);
        self
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
        form = self.video_chat_scheduled.serialize(format!("{}[video_chat_scheduled]", key), form);
        form = self.video_chat_started.serialize(format!("{}[video_chat_started]", key), form);
        form = self.video_chat_ended.serialize(format!("{}[video_chat_ended]", key), form);
        form = self.video_chat_participants_invited.serialize(format!("{}[video_chat_participants_invited]", key), form);
        form = self.web_app_data.serialize(format!("{}[web_app_data]", key), form);
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
            video_chat_scheduled: None,
            video_chat_started: None,
            video_chat_ended: None,
            video_chat_participants_invited: None,
            web_app_data: None,
            reply_markup: None,
        }
    }
}

impl Message {
    pub fn with_from(mut self, from: User) -> Self {
        self.from = Some(from);
        self
    }

    pub fn with_sender_chat(mut self, sender_chat: Chat) -> Self {
        self.sender_chat = Some(sender_chat);
        self
    }

    pub fn with_forward_from(mut self, forward_from: User) -> Self {
        self.forward_from = Some(forward_from);
        self
    }

    pub fn with_forward_from_chat(mut self, forward_from_chat: Chat) -> Self {
        self.forward_from_chat = Some(forward_from_chat);
        self
    }

    pub fn with_forward_from_message_id(mut self, forward_from_message_id: i64) -> Self {
        self.forward_from_message_id = Some(forward_from_message_id);
        self
    }

    pub fn with_forward_signature(mut self, forward_signature: String) -> Self {
        self.forward_signature = Some(forward_signature);
        self
    }

    pub fn with_forward_sender_name(mut self, forward_sender_name: String) -> Self {
        self.forward_sender_name = Some(forward_sender_name);
        self
    }

    pub fn with_forward_date(mut self, forward_date: i64) -> Self {
        self.forward_date = Some(forward_date);
        self
    }

    pub fn with_is_automatic_forward(mut self, is_automatic_forward: bool) -> Self {
        self.is_automatic_forward = Some(is_automatic_forward);
        self
    }

    pub fn with_reply_to_message(mut self, reply_to_message: Box<Message>) -> Self {
        self.reply_to_message = Some(reply_to_message);
        self
    }

    pub fn with_via_bot(mut self, via_bot: User) -> Self {
        self.via_bot = Some(via_bot);
        self
    }

    pub fn with_edit_date(mut self, edit_date: i64) -> Self {
        self.edit_date = Some(edit_date);
        self
    }

    pub fn with_has_protected_content(mut self, has_protected_content: bool) -> Self {
        self.has_protected_content = Some(has_protected_content);
        self
    }

    pub fn with_media_group_id(mut self, media_group_id: String) -> Self {
        self.media_group_id = Some(media_group_id);
        self
    }

    pub fn with_author_signature(mut self, author_signature: String) -> Self {
        self.author_signature = Some(author_signature);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn with_entities(mut self, entities: Vec<MessageEntity>) -> Self {
        self.entities = Some(entities);
        self
    }

    pub fn with_animation(mut self, animation: Animation) -> Self {
        self.animation = Some(animation);
        self
    }

    pub fn with_audio(mut self, audio: Audio) -> Self {
        self.audio = Some(audio);
        self
    }

    pub fn with_document(mut self, document: Document) -> Self {
        self.document = Some(document);
        self
    }

    pub fn with_photo(mut self, photo: Vec<PhotoSize>) -> Self {
        self.photo = Some(photo);
        self
    }

    pub fn with_sticker(mut self, sticker: Sticker) -> Self {
        self.sticker = Some(sticker);
        self
    }

    pub fn with_video(mut self, video: Video) -> Self {
        self.video = Some(video);
        self
    }

    pub fn with_video_note(mut self, video_note: VideoNote) -> Self {
        self.video_note = Some(video_note);
        self
    }

    pub fn with_voice(mut self, voice: Voice) -> Self {
        self.voice = Some(voice);
        self
    }

    pub fn with_caption(mut self, caption: String) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn with_caption_entities(mut self, caption_entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }

    pub fn with_contact(mut self, contact: Contact) -> Self {
        self.contact = Some(contact);
        self
    }

    pub fn with_dice(mut self, dice: Dice) -> Self {
        self.dice = Some(dice);
        self
    }

    pub fn with_game(mut self, game: Game) -> Self {
        self.game = Some(game);
        self
    }

    pub fn with_poll(mut self, poll: Poll) -> Self {
        self.poll = Some(poll);
        self
    }

    pub fn with_venue(mut self, venue: Venue) -> Self {
        self.venue = Some(venue);
        self
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_new_chat_members(mut self, new_chat_members: Vec<User>) -> Self {
        self.new_chat_members = Some(new_chat_members);
        self
    }

    pub fn with_left_chat_member(mut self, left_chat_member: User) -> Self {
        self.left_chat_member = Some(left_chat_member);
        self
    }

    pub fn with_new_chat_title(mut self, new_chat_title: String) -> Self {
        self.new_chat_title = Some(new_chat_title);
        self
    }

    pub fn with_new_chat_photo(mut self, new_chat_photo: Vec<PhotoSize>) -> Self {
        self.new_chat_photo = Some(new_chat_photo);
        self
    }

    pub fn with_delete_chat_photo(mut self, delete_chat_photo: bool) -> Self {
        self.delete_chat_photo = Some(delete_chat_photo);
        self
    }

    pub fn with_group_chat_created(mut self, group_chat_created: bool) -> Self {
        self.group_chat_created = Some(group_chat_created);
        self
    }

    pub fn with_supergroup_chat_created(mut self, supergroup_chat_created: bool) -> Self {
        self.supergroup_chat_created = Some(supergroup_chat_created);
        self
    }

    pub fn with_channel_chat_created(mut self, channel_chat_created: bool) -> Self {
        self.channel_chat_created = Some(channel_chat_created);
        self
    }

    pub fn with_message_auto_delete_timer_changed(mut self, message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged) -> Self {
        self.message_auto_delete_timer_changed = Some(message_auto_delete_timer_changed);
        self
    }

    pub fn with_migrate_to_chat_id(mut self, migrate_to_chat_id: i64) -> Self {
        self.migrate_to_chat_id = Some(migrate_to_chat_id);
        self
    }

    pub fn with_migrate_from_chat_id(mut self, migrate_from_chat_id: i64) -> Self {
        self.migrate_from_chat_id = Some(migrate_from_chat_id);
        self
    }

    pub fn with_pinned_message(mut self, pinned_message: Box<Message>) -> Self {
        self.pinned_message = Some(pinned_message);
        self
    }

    pub fn with_invoice(mut self, invoice: Invoice) -> Self {
        self.invoice = Some(invoice);
        self
    }

    pub fn with_successful_payment(mut self, successful_payment: SuccessfulPayment) -> Self {
        self.successful_payment = Some(successful_payment);
        self
    }

    pub fn with_connected_website(mut self, connected_website: String) -> Self {
        self.connected_website = Some(connected_website);
        self
    }

    pub fn with_passport_data(mut self, passport_data: PassportData) -> Self {
        self.passport_data = Some(passport_data);
        self
    }

    pub fn with_proximity_alert_triggered(mut self, proximity_alert_triggered: ProximityAlertTriggered) -> Self {
        self.proximity_alert_triggered = Some(proximity_alert_triggered);
        self
    }

    pub fn with_video_chat_scheduled(mut self, video_chat_scheduled: VideoChatScheduled) -> Self {
        self.video_chat_scheduled = Some(video_chat_scheduled);
        self
    }

    pub fn with_video_chat_started(mut self, video_chat_started: VideoChatStarted) -> Self {
        self.video_chat_started = Some(video_chat_started);
        self
    }

    pub fn with_video_chat_ended(mut self, video_chat_ended: VideoChatEnded) -> Self {
        self.video_chat_ended = Some(video_chat_ended);
        self
    }

    pub fn with_video_chat_participants_invited(mut self, video_chat_participants_invited: VideoChatParticipantsInvited) -> Self {
        self.video_chat_participants_invited = Some(video_chat_participants_invited);
        self
    }

    pub fn with_web_app_data(mut self, web_app_data: WebAppData) -> Self {
        self.web_app_data = Some(web_app_data);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
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

impl MessageEntity {
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn with_user(mut self, user: User) -> Self {
        self.user = Some(user);
        self
    }

    pub fn with_language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
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

impl PhotoSize {
    pub fn with_file_size(mut self, file_size: i64) -> Self {
        self.file_size = Some(file_size);
        self
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

impl Animation {
    pub fn with_thumb(mut self, thumb: PhotoSize) -> Self {
        self.thumb = Some(thumb);
        self
    }

    pub fn with_file_name(mut self, file_name: String) -> Self {
        self.file_name = Some(file_name);
        self
    }

    pub fn with_mime_type(mut self, mime_type: String) -> Self {
        self.mime_type = Some(mime_type);
        self
    }

    pub fn with_file_size(mut self, file_size: i64) -> Self {
        self.file_size = Some(file_size);
        self
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

impl Audio {
    pub fn with_performer(mut self, performer: String) -> Self {
        self.performer = Some(performer);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_file_name(mut self, file_name: String) -> Self {
        self.file_name = Some(file_name);
        self
    }

    pub fn with_mime_type(mut self, mime_type: String) -> Self {
        self.mime_type = Some(mime_type);
        self
    }

    pub fn with_file_size(mut self, file_size: i64) -> Self {
        self.file_size = Some(file_size);
        self
    }

    pub fn with_thumb(mut self, thumb: PhotoSize) -> Self {
        self.thumb = Some(thumb);
        self
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

impl Document {
    pub fn with_thumb(mut self, thumb: PhotoSize) -> Self {
        self.thumb = Some(thumb);
        self
    }

    pub fn with_file_name(mut self, file_name: String) -> Self {
        self.file_name = Some(file_name);
        self
    }

    pub fn with_mime_type(mut self, mime_type: String) -> Self {
        self.mime_type = Some(mime_type);
        self
    }

    pub fn with_file_size(mut self, file_size: i64) -> Self {
        self.file_size = Some(file_size);
        self
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

impl Video {
    pub fn with_thumb(mut self, thumb: PhotoSize) -> Self {
        self.thumb = Some(thumb);
        self
    }

    pub fn with_file_name(mut self, file_name: String) -> Self {
        self.file_name = Some(file_name);
        self
    }

    pub fn with_mime_type(mut self, mime_type: String) -> Self {
        self.mime_type = Some(mime_type);
        self
    }

    pub fn with_file_size(mut self, file_size: i64) -> Self {
        self.file_size = Some(file_size);
        self
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

impl VideoNote {
    pub fn with_thumb(mut self, thumb: PhotoSize) -> Self {
        self.thumb = Some(thumb);
        self
    }

    pub fn with_file_size(mut self, file_size: i64) -> Self {
        self.file_size = Some(file_size);
        self
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

impl Voice {
    pub fn with_mime_type(mut self, mime_type: String) -> Self {
        self.mime_type = Some(mime_type);
        self
    }

    pub fn with_file_size(mut self, file_size: i64) -> Self {
        self.file_size = Some(file_size);
        self
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

impl Contact {
    pub fn with_last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn with_user_id(mut self, user_id: i64) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_vcard(mut self, vcard: String) -> Self {
        self.vcard = Some(vcard);
        self
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

impl Poll {
    pub fn with_correct_option_id(mut self, correct_option_id: i64) -> Self {
        self.correct_option_id = Some(correct_option_id);
        self
    }

    pub fn with_explanation(mut self, explanation: String) -> Self {
        self.explanation = Some(explanation);
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

impl Location {
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

impl Venue {
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

}

impl FormSer for WebAppData {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.data.serialize(format!("{}[data]", key), form);
        form = self.button_text.serialize(format!("{}[button_text]", key), form);
        form
    }
}

impl WebAppData {
    pub fn new(data: String, button_text: String, ) -> Self {
        Self {
            data,
            button_text,
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

impl FormSer for VideoChatScheduled {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.start_date.serialize(format!("{}[start_date]", key), form);
        form
    }
}

impl VideoChatScheduled {
    pub fn new(start_date: i64, ) -> Self {
        Self {
            start_date,
        }
    }
}

impl FormSer for VideoChatStarted {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form
    }
}

impl FormSer for VideoChatEnded {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.duration.serialize(format!("{}[duration]", key), form);
        form
    }
}

impl VideoChatEnded {
    pub fn new(duration: i64, ) -> Self {
        Self {
            duration,
        }
    }
}

impl FormSer for VideoChatParticipantsInvited {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.users.serialize(format!("{}[users]", key), form);
        form
    }
}

impl VideoChatParticipantsInvited {
    pub fn new(users: Vec<User>, ) -> Self {
        Self {
            users,
        }
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

impl File {
    pub fn with_file_size(mut self, file_size: i64) -> Self {
        self.file_size = Some(file_size);
        self
    }

    pub fn with_file_path(mut self, file_path: String) -> Self {
        self.file_path = Some(file_path);
        self
    }

}

impl FormSer for WebAppInfo {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.url.serialize(format!("{}[url]", key), form);
        form
    }
}

impl WebAppInfo {
    pub fn new(url: String, ) -> Self {
        Self {
            url,
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

impl ReplyKeyboardMarkup {
    pub fn with_resize_keyboard(mut self, resize_keyboard: bool) -> Self {
        self.resize_keyboard = Some(resize_keyboard);
        self
    }

    pub fn with_one_time_keyboard(mut self, one_time_keyboard: bool) -> Self {
        self.one_time_keyboard = Some(one_time_keyboard);
        self
    }

    pub fn with_input_field_placeholder(mut self, input_field_placeholder: String) -> Self {
        self.input_field_placeholder = Some(input_field_placeholder);
        self
    }

    pub fn with_selective(mut self, selective: bool) -> Self {
        self.selective = Some(selective);
        self
    }

}

impl FormSer for KeyboardButton {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.text.serialize(format!("{}[text]", key), form);
        form = self.request_contact.serialize(format!("{}[request_contact]", key), form);
        form = self.request_location.serialize(format!("{}[request_location]", key), form);
        form = self.request_poll.serialize(format!("{}[request_poll]", key), form);
        form = self.web_app.serialize(format!("{}[web_app]", key), form);
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
            web_app: None,
        }
    }
}

impl KeyboardButton {
    pub fn with_request_contact(mut self, request_contact: bool) -> Self {
        self.request_contact = Some(request_contact);
        self
    }

    pub fn with_request_location(mut self, request_location: bool) -> Self {
        self.request_location = Some(request_location);
        self
    }

    pub fn with_request_poll(mut self, request_poll: KeyboardButtonPollType) -> Self {
        self.request_poll = Some(request_poll);
        self
    }

    pub fn with_web_app(mut self, web_app: WebAppInfo) -> Self {
        self.web_app = Some(web_app);
        self
    }

}

impl FormSer for KeyboardButtonPollType {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form
    }
}

impl KeyboardButtonPollType {
    pub fn with_type_(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
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

impl ReplyKeyboardRemove {
    pub fn with_selective(mut self, selective: bool) -> Self {
        self.selective = Some(selective);
        self
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
        form = self.callback_data.serialize(format!("{}[callback_data]", key), form);
        form = self.web_app.serialize(format!("{}[web_app]", key), form);
        form = self.login_url.serialize(format!("{}[login_url]", key), form);
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
            callback_data: None,
            web_app: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }
    }
}

impl InlineKeyboardButton {
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn with_callback_data(mut self, callback_data: String) -> Self {
        self.callback_data = Some(callback_data);
        self
    }

    pub fn with_web_app(mut self, web_app: WebAppInfo) -> Self {
        self.web_app = Some(web_app);
        self
    }

    pub fn with_login_url(mut self, login_url: LoginUrl) -> Self {
        self.login_url = Some(login_url);
        self
    }

    pub fn with_switch_inline_query(mut self, switch_inline_query: String) -> Self {
        self.switch_inline_query = Some(switch_inline_query);
        self
    }

    pub fn with_switch_inline_query_current_chat(mut self, switch_inline_query_current_chat: String) -> Self {
        self.switch_inline_query_current_chat = Some(switch_inline_query_current_chat);
        self
    }

    pub fn with_callback_game(mut self, callback_game: CallbackGame) -> Self {
        self.callback_game = Some(callback_game);
        self
    }

    pub fn with_pay(mut self, pay: bool) -> Self {
        self.pay = Some(pay);
        self
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

impl LoginUrl {
    pub fn with_forward_text(mut self, forward_text: String) -> Self {
        self.forward_text = Some(forward_text);
        self
    }

    pub fn with_bot_username(mut self, bot_username: String) -> Self {
        self.bot_username = Some(bot_username);
        self
    }

    pub fn with_request_write_access(mut self, request_write_access: bool) -> Self {
        self.request_write_access = Some(request_write_access);
        self
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

impl CallbackQuery {
    pub fn with_message(mut self, message: Box<Message>) -> Self {
        self.message = Some(message);
        self
    }

    pub fn with_inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

    pub fn with_data(mut self, data: String) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_game_short_name(mut self, game_short_name: String) -> Self {
        self.game_short_name = Some(game_short_name);
        self
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

impl ForceReply {
    pub fn with_input_field_placeholder(mut self, input_field_placeholder: String) -> Self {
        self.input_field_placeholder = Some(input_field_placeholder);
        self
    }

    pub fn with_selective(mut self, selective: bool) -> Self {
        self.selective = Some(selective);
        self
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

impl ChatInviteLink {
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

    pub fn with_pending_join_request_count(mut self, pending_join_request_count: i64) -> Self {
        self.pending_join_request_count = Some(pending_join_request_count);
        self
    }

}

impl FormSer for ChatAdministratorRights {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.is_anonymous.serialize(format!("{}[is_anonymous]", key), form);
        form = self.can_manage_chat.serialize(format!("{}[can_manage_chat]", key), form);
        form = self.can_delete_messages.serialize(format!("{}[can_delete_messages]", key), form);
        form = self.can_manage_video_chats.serialize(format!("{}[can_manage_video_chats]", key), form);
        form = self.can_restrict_members.serialize(format!("{}[can_restrict_members]", key), form);
        form = self.can_promote_members.serialize(format!("{}[can_promote_members]", key), form);
        form = self.can_change_info.serialize(format!("{}[can_change_info]", key), form);
        form = self.can_invite_users.serialize(format!("{}[can_invite_users]", key), form);
        form = self.can_post_messages.serialize(format!("{}[can_post_messages]", key), form);
        form = self.can_edit_messages.serialize(format!("{}[can_edit_messages]", key), form);
        form = self.can_pin_messages.serialize(format!("{}[can_pin_messages]", key), form);
        form
    }
}

impl ChatAdministratorRights {
    pub fn new(is_anonymous: bool, can_manage_chat: bool, can_delete_messages: bool, can_manage_video_chats: bool, can_restrict_members: bool, can_promote_members: bool, can_change_info: bool, can_invite_users: bool, ) -> Self {
        Self {
            is_anonymous,
            can_manage_chat,
            can_delete_messages,
            can_manage_video_chats,
            can_restrict_members,
            can_promote_members,
            can_change_info,
            can_invite_users,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
        }
    }
}

impl ChatAdministratorRights {
    pub fn with_can_post_messages(mut self, can_post_messages: bool) -> Self {
        self.can_post_messages = Some(can_post_messages);
        self
    }

    pub fn with_can_edit_messages(mut self, can_edit_messages: bool) -> Self {
        self.can_edit_messages = Some(can_edit_messages);
        self
    }

    pub fn with_can_pin_messages(mut self, can_pin_messages: bool) -> Self {
        self.can_pin_messages = Some(can_pin_messages);
        self
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

impl ChatMemberOwner {
    pub fn with_custom_title(mut self, custom_title: String) -> Self {
        self.custom_title = Some(custom_title);
        self
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
        form = self.can_manage_video_chats.serialize(format!("{}[can_manage_video_chats]", key), form);
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
    pub fn new(status: String, user: User, can_be_edited: bool, is_anonymous: bool, can_manage_chat: bool, can_delete_messages: bool, can_manage_video_chats: bool, can_restrict_members: bool, can_promote_members: bool, can_change_info: bool, can_invite_users: bool, ) -> Self {
        Self {
            status,
            user,
            can_be_edited,
            is_anonymous,
            can_manage_chat,
            can_delete_messages,
            can_manage_video_chats,
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

impl ChatMemberAdministrator {
    pub fn with_can_post_messages(mut self, can_post_messages: bool) -> Self {
        self.can_post_messages = Some(can_post_messages);
        self
    }

    pub fn with_can_edit_messages(mut self, can_edit_messages: bool) -> Self {
        self.can_edit_messages = Some(can_edit_messages);
        self
    }

    pub fn with_can_pin_messages(mut self, can_pin_messages: bool) -> Self {
        self.can_pin_messages = Some(can_pin_messages);
        self
    }

    pub fn with_custom_title(mut self, custom_title: String) -> Self {
        self.custom_title = Some(custom_title);
        self
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

impl ChatMemberUpdated {
    pub fn with_invite_link(mut self, invite_link: ChatInviteLink) -> Self {
        self.invite_link = Some(invite_link);
        self
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

impl ChatJoinRequest {
    pub fn with_bio(mut self, bio: String) -> Self {
        self.bio = Some(bio);
        self
    }

    pub fn with_invite_link(mut self, invite_link: ChatInviteLink) -> Self {
        self.invite_link = Some(invite_link);
        self
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

impl ChatPermissions {
    pub fn with_can_send_messages(mut self, can_send_messages: bool) -> Self {
        self.can_send_messages = Some(can_send_messages);
        self
    }

    pub fn with_can_send_media_messages(mut self, can_send_media_messages: bool) -> Self {
        self.can_send_media_messages = Some(can_send_media_messages);
        self
    }

    pub fn with_can_send_polls(mut self, can_send_polls: bool) -> Self {
        self.can_send_polls = Some(can_send_polls);
        self
    }

    pub fn with_can_send_other_messages(mut self, can_send_other_messages: bool) -> Self {
        self.can_send_other_messages = Some(can_send_other_messages);
        self
    }

    pub fn with_can_add_web_page_previews(mut self, can_add_web_page_previews: bool) -> Self {
        self.can_add_web_page_previews = Some(can_add_web_page_previews);
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

impl FormSer for MenuButtonCommands {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form
    }
}

impl MenuButtonCommands {
    pub fn new(type_: String, ) -> Self {
        Self {
            type_,
        }
    }
}

impl FormSer for MenuButtonWebApp {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form = self.text.serialize(format!("{}[text]", key), form);
        form = self.web_app.serialize(format!("{}[web_app]", key), form);
        form
    }
}

impl MenuButtonWebApp {
    pub fn new(type_: String, text: String, web_app: WebAppInfo, ) -> Self {
        Self {
            type_,
            text,
            web_app,
        }
    }
}

impl FormSer for MenuButtonDefault {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.type_.serialize(format!("{}[type]", key), form);
        form
    }
}

impl MenuButtonDefault {
    pub fn new(type_: String, ) -> Self {
        Self {
            type_,
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

impl ResponseParameters {
    pub fn with_migrate_to_chat_id(mut self, migrate_to_chat_id: i64) -> Self {
        self.migrate_to_chat_id = Some(migrate_to_chat_id);
        self
    }

    pub fn with_retry_after(mut self, retry_after: i64) -> Self {
        self.retry_after = Some(retry_after);
        self
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

impl InputMediaPhoto {
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

impl InputMediaVideo {
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

    pub fn with_width(mut self, width: i64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: i64) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_supports_streaming(mut self, supports_streaming: bool) -> Self {
        self.supports_streaming = Some(supports_streaming);
        self
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

impl InputMediaAnimation {
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

    pub fn with_width(mut self, width: i64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: i64) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
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

impl InputMediaAudio {
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

impl InputMediaDocument {
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

}

impl FormSer for Sticker {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.file_id.serialize(format!("{}[file_id]", key), form);
        form = self.file_unique_id.serialize(format!("{}[file_unique_id]", key), form);
        form = self.width.serialize(format!("{}[width]", key), form);
        form = self.height.serialize(format!("{}[height]", key), form);
        form = self.is_animated.serialize(format!("{}[is_animated]", key), form);
        form = self.is_video.serialize(format!("{}[is_video]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form = self.emoji.serialize(format!("{}[emoji]", key), form);
        form = self.set_name.serialize(format!("{}[set_name]", key), form);
        form = self.premium_animation.serialize(format!("{}[premium_animation]", key), form);
        form = self.mask_position.serialize(format!("{}[mask_position]", key), form);
        form = self.file_size.serialize(format!("{}[file_size]", key), form);
        form
    }
}

impl Sticker {
    pub fn new(file_id: String, file_unique_id: String, width: i64, height: i64, is_animated: bool, is_video: bool, ) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            is_animated,
            is_video,
            thumb: None,
            emoji: None,
            set_name: None,
            premium_animation: None,
            mask_position: None,
            file_size: None,
        }
    }
}

impl Sticker {
    pub fn with_thumb(mut self, thumb: PhotoSize) -> Self {
        self.thumb = Some(thumb);
        self
    }

    pub fn with_emoji(mut self, emoji: String) -> Self {
        self.emoji = Some(emoji);
        self
    }

    pub fn with_set_name(mut self, set_name: String) -> Self {
        self.set_name = Some(set_name);
        self
    }

    pub fn with_premium_animation(mut self, premium_animation: File) -> Self {
        self.premium_animation = Some(premium_animation);
        self
    }

    pub fn with_mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }

    pub fn with_file_size(mut self, file_size: i64) -> Self {
        self.file_size = Some(file_size);
        self
    }

}

impl FormSer for StickerSet {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.name.serialize(format!("{}[name]", key), form);
        form = self.title.serialize(format!("{}[title]", key), form);
        form = self.is_animated.serialize(format!("{}[is_animated]", key), form);
        form = self.is_video.serialize(format!("{}[is_video]", key), form);
        form = self.contains_masks.serialize(format!("{}[contains_masks]", key), form);
        form = self.stickers.serialize(format!("{}[stickers]", key), form);
        form = self.thumb.serialize(format!("{}[thumb]", key), form);
        form
    }
}

impl StickerSet {
    pub fn new(name: String, title: String, is_animated: bool, is_video: bool, contains_masks: bool, stickers: Vec<Sticker>, ) -> Self {
        Self {
            name,
            title,
            is_animated,
            is_video,
            contains_masks,
            stickers,
            thumb: None,
        }
    }
}

impl StickerSet {
    pub fn with_thumb(mut self, thumb: PhotoSize) -> Self {
        self.thumb = Some(thumb);
        self
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

impl InlineQuery {
    pub fn with_chat_type(mut self, chat_type: String) -> Self {
        self.chat_type = Some(chat_type);
        self
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
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

impl InputTextMessageContent {
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

impl InputLocationMessageContent {
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

impl InputVenueMessageContent {
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

impl InputContactMessageContent {
    pub fn with_last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn with_vcard(mut self, vcard: String) -> Self {
        self.vcard = Some(vcard);
        self
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

impl InputInvoiceMessageContent {
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

impl ChosenInlineResult {
    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

}

impl FormSer for SentWebAppMessage {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        form = self.inline_message_id.serialize(format!("{}[inline_message_id]", key), form);
        form
    }
}

impl SentWebAppMessage {
    pub fn with_inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
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

impl OrderInfo {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_phone_number(mut self, phone_number: String) -> Self {
        self.phone_number = Some(phone_number);
        self
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn with_shipping_address(mut self, shipping_address: ShippingAddress) -> Self {
        self.shipping_address = Some(shipping_address);
        self
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

impl SuccessfulPayment {
    pub fn with_shipping_option_id(mut self, shipping_option_id: String) -> Self {
        self.shipping_option_id = Some(shipping_option_id);
        self
    }

    pub fn with_order_info(mut self, order_info: OrderInfo) -> Self {
        self.order_info = Some(order_info);
        self
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

impl PreCheckoutQuery {
    pub fn with_shipping_option_id(mut self, shipping_option_id: String) -> Self {
        self.shipping_option_id = Some(shipping_option_id);
        self
    }

    pub fn with_order_info(mut self, order_info: OrderInfo) -> Self {
        self.order_info = Some(order_info);
        self
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

impl EncryptedPassportElement {
    pub fn with_data(mut self, data: String) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_phone_number(mut self, phone_number: String) -> Self {
        self.phone_number = Some(phone_number);
        self
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn with_files(mut self, files: Vec<PassportFile>) -> Self {
        self.files = Some(files);
        self
    }

    pub fn with_front_side(mut self, front_side: PassportFile) -> Self {
        self.front_side = Some(front_side);
        self
    }

    pub fn with_reverse_side(mut self, reverse_side: PassportFile) -> Self {
        self.reverse_side = Some(reverse_side);
        self
    }

    pub fn with_selfie(mut self, selfie: PassportFile) -> Self {
        self.selfie = Some(selfie);
        self
    }

    pub fn with_translation(mut self, translation: Vec<PassportFile>) -> Self {
        self.translation = Some(translation);
        self
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

impl Game {
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn with_text_entities(mut self, text_entities: Vec<MessageEntity>) -> Self {
        self.text_entities = Some(text_entities);
        self
    }

    pub fn with_animation(mut self, animation: Animation) -> Self {
        self.animation = Some(animation);
        self
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

