use crate::types::*;
use crate::helpers::*;

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

impl crate::TgObject for InputMedia {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        match self {
            Self::Animation(e) => e.add_file(form),
            Self::Document(e) => e.add_file(form),
            Self::Audio(e) => e.add_file(form),
            Self::Video(e) => e.add_file(form),
            _ => form,
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

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(o: InlineKeyboardMarkup) -> Self {
        Self::InlineKeyboardMarkup(o)
    }
}

impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(o: ReplyKeyboardMarkup) -> Self {
        Self::Keyboard(o)
    }
}

impl From<ReplyKeyboardRemove> for ReplyMarkup {
    fn from(o: ReplyKeyboardRemove) -> Self {
        Self::KeyboardRemove(o)
    }
}

impl From<ForceReply> for ReplyMarkup {
    fn from(o: ForceReply) -> Self {
        Self::Force(o)
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

impl MessageEntity {
    pub fn new(type_: String, offset: i64, length: i64, ) -> Self {
        Self {
            type_,
            offset,
            length,
            url: None,
            user: None,
            language: None,
            custom_emoji_id: None,
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

    pub fn with_custom_emoji_id(mut self, custom_emoji_id: String) -> Self {
        self.custom_emoji_id = Some(custom_emoji_id);
        self
    }

}

impl WebAppInfo {
    pub fn new(url: String, ) -> Self {
        Self {
            url,
        }
    }
}

impl ReplyKeyboardMarkup {
    pub fn new(keyboard: Vec<Vec<KeyboardButton>>, ) -> Self {
        Self {
            keyboard,
            is_persistent: None,
            resize_keyboard: None,
            one_time_keyboard: None,
            input_field_placeholder: None,
            selective: None,
        }
    }
}

impl ReplyKeyboardMarkup {
    pub fn with_is_persistent(mut self, is_persistent: bool) -> Self {
        self.is_persistent = Some(is_persistent);
        self
    }

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

impl KeyboardButtonPollType {
    pub fn with_type(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
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

impl InlineKeyboardMarkup {
    pub fn new(inline_keyboard: Vec<Vec<InlineKeyboardButton>>, ) -> Self {
        Self {
            inline_keyboard,
        }
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
            can_manage_topics: None,
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

    pub fn with_can_manage_topics(mut self, can_manage_topics: bool) -> Self {
        self.can_manage_topics = Some(can_manage_topics);
        self
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

    pub fn with_can_manage_topics(mut self, can_manage_topics: bool) -> Self {
        self.can_manage_topics = Some(can_manage_topics);
        self
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

impl BotCommandScopeDefault {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl BotCommandScopeAllPrivateChats {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl BotCommandScopeAllGroupChats {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl BotCommandScopeAllChatAdministrators {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl BotCommandScopeChat {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl BotCommandScopeChatAdministrators {
    pub fn new(chat_id: ChatId, ) -> Self {
        Self {
            chat_id,
        }
    }
}

impl BotCommandScopeChatMember {
    pub fn new(chat_id: ChatId, user_id: i64, ) -> Self {
        Self {
            chat_id,
            user_id,
        }
    }
}

impl MenuButtonCommands {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl MenuButtonWebApp {
    pub fn new(text: String, web_app: WebAppInfo, ) -> Self {
        Self {
            text,
            web_app,
        }
    }
}

impl MenuButtonDefault {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl InputMediaPhoto {
    pub fn new(media: String, ) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            has_spoiler: None,
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

    pub fn with_has_spoiler(mut self, has_spoiler: bool) -> Self {
        self.has_spoiler = Some(has_spoiler);
        self
    }

}

impl InputMediaVideo {
    pub fn new(media: String, ) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
            has_spoiler: None,
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

    pub fn with_has_spoiler(mut self, has_spoiler: bool) -> Self {
        self.has_spoiler = Some(has_spoiler);
        self
    }

}

impl crate::TgObject for InputMediaVideo {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.thumb.add_file(form);
        form
    }
}

impl InputMediaAnimation {
    pub fn new(media: String, ) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
            has_spoiler: None,
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

    pub fn with_has_spoiler(mut self, has_spoiler: bool) -> Self {
        self.has_spoiler = Some(has_spoiler);
        self
    }

}

impl crate::TgObject for InputMediaAnimation {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.thumb.add_file(form);
        form
    }
}

impl InputMediaAudio {
    pub fn new(media: String, ) -> Self {
        Self {
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

impl crate::TgObject for InputMediaAudio {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.thumb.add_file(form);
        form
    }
}

impl InputMediaDocument {
    pub fn new(media: String, ) -> Self {
        Self {
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

impl crate::TgObject for InputMediaDocument {
    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        form = self.thumb.add_file(form);
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

impl InlineQueryResultArticle {
    pub fn new(id: String, title: String, input_message_content: InputMessageContent, ) -> Self {
        Self {
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

impl InlineQueryResultArticle {
    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn with_hide_url(mut self, hide_url: bool) -> Self {
        self.hide_url = Some(hide_url);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_thumb_url(mut self, thumb_url: String) -> Self {
        self.thumb_url = Some(thumb_url);
        self
    }

    pub fn with_thumb_width(mut self, thumb_width: i64) -> Self {
        self.thumb_width = Some(thumb_width);
        self
    }

    pub fn with_thumb_height(mut self, thumb_height: i64) -> Self {
        self.thumb_height = Some(thumb_height);
        self
    }

}

impl InlineQueryResultPhoto {
    pub fn new(id: String, photo_url: String, thumb_url: String, ) -> Self {
        Self {
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

impl InlineQueryResultPhoto {
    pub fn with_photo_width(mut self, photo_width: i64) -> Self {
        self.photo_width = Some(photo_width);
        self
    }

    pub fn with_photo_height(mut self, photo_height: i64) -> Self {
        self.photo_height = Some(photo_height);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
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

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultGif {
    pub fn new(id: String, gif_url: String, thumb_url: String, ) -> Self {
        Self {
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

impl InlineQueryResultGif {
    pub fn with_gif_width(mut self, gif_width: i64) -> Self {
        self.gif_width = Some(gif_width);
        self
    }

    pub fn with_gif_height(mut self, gif_height: i64) -> Self {
        self.gif_height = Some(gif_height);
        self
    }

    pub fn with_gif_duration(mut self, gif_duration: i64) -> Self {
        self.gif_duration = Some(gif_duration);
        self
    }

    pub fn with_thumb_mime_type(mut self, thumb_mime_type: String) -> Self {
        self.thumb_mime_type = Some(thumb_mime_type);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
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

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultMpeg4Gif {
    pub fn new(id: String, mpeg4_url: String, thumb_url: String, ) -> Self {
        Self {
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

impl InlineQueryResultMpeg4Gif {
    pub fn with_mpeg4_width(mut self, mpeg4_width: i64) -> Self {
        self.mpeg4_width = Some(mpeg4_width);
        self
    }

    pub fn with_mpeg4_height(mut self, mpeg4_height: i64) -> Self {
        self.mpeg4_height = Some(mpeg4_height);
        self
    }

    pub fn with_mpeg4_duration(mut self, mpeg4_duration: i64) -> Self {
        self.mpeg4_duration = Some(mpeg4_duration);
        self
    }

    pub fn with_thumb_mime_type(mut self, thumb_mime_type: String) -> Self {
        self.thumb_mime_type = Some(thumb_mime_type);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
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

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultVideo {
    pub fn new(id: String, video_url: String, mime_type: String, thumb_url: String, title: String, ) -> Self {
        Self {
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

impl InlineQueryResultVideo {
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

    pub fn with_video_width(mut self, video_width: i64) -> Self {
        self.video_width = Some(video_width);
        self
    }

    pub fn with_video_height(mut self, video_height: i64) -> Self {
        self.video_height = Some(video_height);
        self
    }

    pub fn with_video_duration(mut self, video_duration: i64) -> Self {
        self.video_duration = Some(video_duration);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultAudio {
    pub fn new(id: String, audio_url: String, title: String, ) -> Self {
        Self {
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

impl InlineQueryResultAudio {
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

    pub fn with_performer(mut self, performer: String) -> Self {
        self.performer = Some(performer);
        self
    }

    pub fn with_audio_duration(mut self, audio_duration: i64) -> Self {
        self.audio_duration = Some(audio_duration);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultVoice {
    pub fn new(id: String, voice_url: String, title: String, ) -> Self {
        Self {
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

impl InlineQueryResultVoice {
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

    pub fn with_voice_duration(mut self, voice_duration: i64) -> Self {
        self.voice_duration = Some(voice_duration);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultDocument {
    pub fn new(id: String, title: String, document_url: String, mime_type: String, ) -> Self {
        Self {
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

impl InlineQueryResultDocument {
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

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

    pub fn with_thumb_url(mut self, thumb_url: String) -> Self {
        self.thumb_url = Some(thumb_url);
        self
    }

    pub fn with_thumb_width(mut self, thumb_width: i64) -> Self {
        self.thumb_width = Some(thumb_width);
        self
    }

    pub fn with_thumb_height(mut self, thumb_height: i64) -> Self {
        self.thumb_height = Some(thumb_height);
        self
    }

}

impl InlineQueryResultLocation {
    pub fn new(id: String, latitude: f64, longitude: f64, title: String, ) -> Self {
        Self {
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

impl InlineQueryResultLocation {
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

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

    pub fn with_thumb_url(mut self, thumb_url: String) -> Self {
        self.thumb_url = Some(thumb_url);
        self
    }

    pub fn with_thumb_width(mut self, thumb_width: i64) -> Self {
        self.thumb_width = Some(thumb_width);
        self
    }

    pub fn with_thumb_height(mut self, thumb_height: i64) -> Self {
        self.thumb_height = Some(thumb_height);
        self
    }

}

impl InlineQueryResultVenue {
    pub fn new(id: String, latitude: f64, longitude: f64, title: String, address: String, ) -> Self {
        Self {
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

impl InlineQueryResultVenue {
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

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

    pub fn with_thumb_url(mut self, thumb_url: String) -> Self {
        self.thumb_url = Some(thumb_url);
        self
    }

    pub fn with_thumb_width(mut self, thumb_width: i64) -> Self {
        self.thumb_width = Some(thumb_width);
        self
    }

    pub fn with_thumb_height(mut self, thumb_height: i64) -> Self {
        self.thumb_height = Some(thumb_height);
        self
    }

}

impl InlineQueryResultContact {
    pub fn new(id: String, phone_number: String, first_name: String, ) -> Self {
        Self {
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

impl InlineQueryResultContact {
    pub fn with_last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn with_vcard(mut self, vcard: String) -> Self {
        self.vcard = Some(vcard);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

    pub fn with_thumb_url(mut self, thumb_url: String) -> Self {
        self.thumb_url = Some(thumb_url);
        self
    }

    pub fn with_thumb_width(mut self, thumb_width: i64) -> Self {
        self.thumb_width = Some(thumb_width);
        self
    }

    pub fn with_thumb_height(mut self, thumb_height: i64) -> Self {
        self.thumb_height = Some(thumb_height);
        self
    }

}

impl InlineQueryResultGame {
    pub fn new(id: String, game_short_name: String, ) -> Self {
        Self {
            id,
            game_short_name,
            reply_markup: None,
        }
    }
}

impl InlineQueryResultGame {
    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

}

impl InlineQueryResultCachedPhoto {
    pub fn new(id: String, photo_file_id: String, ) -> Self {
        Self {
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

impl InlineQueryResultCachedPhoto {
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
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

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultCachedGif {
    pub fn new(id: String, gif_file_id: String, ) -> Self {
        Self {
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

impl InlineQueryResultCachedGif {
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
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

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultCachedMpeg4Gif {
    pub fn new(id: String, mpeg4_file_id: String, ) -> Self {
        Self {
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

impl InlineQueryResultCachedMpeg4Gif {
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
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

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultCachedSticker {
    pub fn new(id: String, sticker_file_id: String, ) -> Self {
        Self {
            id,
            sticker_file_id,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

impl InlineQueryResultCachedSticker {
    pub fn with_reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultCachedDocument {
    pub fn new(id: String, title: String, document_file_id: String, ) -> Self {
        Self {
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

impl InlineQueryResultCachedDocument {
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
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

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultCachedVideo {
    pub fn new(id: String, video_file_id: String, title: String, ) -> Self {
        Self {
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

impl InlineQueryResultCachedVideo {
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
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

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultCachedVoice {
    pub fn new(id: String, voice_file_id: String, title: String, ) -> Self {
        Self {
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

impl InlineQueryResultCachedVoice {
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

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

}

impl InlineQueryResultCachedAudio {
    pub fn new(id: String, audio_file_id: String, ) -> Self {
        Self {
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

impl InlineQueryResultCachedAudio {
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

    pub fn with_input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
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

impl LabeledPrice {
    pub fn new(label: String, amount: i64, ) -> Self {
        Self {
            label,
            amount,
        }
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

impl PassportElementErrorDataField {
    pub fn new(type_: String, field_name: String, data_hash: String, message: String, ) -> Self {
        Self {
            type_,
            field_name,
            data_hash,
            message,
        }
    }
}

impl PassportElementErrorFrontSide {
    pub fn new(type_: String, file_hash: String, message: String, ) -> Self {
        Self {
            type_,
            file_hash,
            message,
        }
    }
}

impl PassportElementErrorReverseSide {
    pub fn new(type_: String, file_hash: String, message: String, ) -> Self {
        Self {
            type_,
            file_hash,
            message,
        }
    }
}

impl PassportElementErrorSelfie {
    pub fn new(type_: String, file_hash: String, message: String, ) -> Self {
        Self {
            type_,
            file_hash,
            message,
        }
    }
}

impl PassportElementErrorFile {
    pub fn new(type_: String, file_hash: String, message: String, ) -> Self {
        Self {
            type_,
            file_hash,
            message,
        }
    }
}

impl PassportElementErrorFiles {
    pub fn new(type_: String, file_hashes: Vec<String>, message: String, ) -> Self {
        Self {
            type_,
            file_hashes,
            message,
        }
    }
}

impl PassportElementErrorTranslationFile {
    pub fn new(type_: String, file_hash: String, message: String, ) -> Self {
        Self {
            type_,
            file_hash,
            message,
        }
    }
}

impl PassportElementErrorTranslationFiles {
    pub fn new(type_: String, file_hashes: Vec<String>, message: String, ) -> Self {
        Self {
            type_,
            file_hashes,
            message,
        }
    }
}

impl PassportElementErrorUnspecified {
    pub fn new(type_: String, element_hash: String, message: String, ) -> Self {
        Self {
            type_,
            element_hash,
            message,
        }
    }
}

