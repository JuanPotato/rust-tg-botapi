use crate::helpers::*;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ChatMember {
    Owner(ChatMemberOwner),
    Administrator(ChatMemberAdministrator),
    Member(ChatMemberMember),
    Restricted(ChatMemberRestricted),
    Left(ChatMemberLeft),
    Banned(ChatMemberBanned),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BotCommandScope {
    #[serde(rename = "default")]
    Default(BotCommandScopeDefault),
    #[serde(rename = "all_private_chats")]
    AllPrivateChats(BotCommandScopeAllPrivateChats),
    #[serde(rename = "all_group_chats")]
    AllGroupChats(BotCommandScopeAllGroupChats),
    #[serde(rename = "all_chat_administrators")]
    AllChatAdministrators(BotCommandScopeAllChatAdministrators),
    #[serde(rename = "chat")]
    Chat(BotCommandScopeChat),
    #[serde(rename = "chat_administrators")]
    ChatAdministrators(BotCommandScopeChatAdministrators),
    #[serde(rename = "chat_member")]
    ChatMember(BotCommandScopeChatMember),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MenuButton {
    #[serde(rename = "commands")]
    Commands(MenuButtonCommands),
    #[serde(rename = "web_app")]
    WebApp(MenuButtonWebApp),
    #[serde(rename = "default")]
    Default(MenuButtonDefault),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputMedia {
    #[serde(rename = "animation")]
    Animation(InputMediaAnimation),
    #[serde(rename = "document")]
    Document(InputMediaDocument),
    #[serde(rename = "audio")]
    Audio(InputMediaAudio),
    #[serde(rename = "photo")]
    Photo(InputMediaPhoto),
    #[serde(rename = "video")]
    Video(InputMediaVideo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InlineQueryResult {
    #[serde(rename = "audio")]
    CachedAudio(InlineQueryResultCachedAudio),
    #[serde(rename = "document")]
    CachedDocument(InlineQueryResultCachedDocument),
    #[serde(rename = "gif")]
    CachedGif(InlineQueryResultCachedGif),
    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    #[serde(rename = "photo")]
    CachedPhoto(InlineQueryResultCachedPhoto),
    #[serde(rename = "sticker")]
    CachedSticker(InlineQueryResultCachedSticker),
    #[serde(rename = "video")]
    CachedVideo(InlineQueryResultCachedVideo),
    #[serde(rename = "voice")]
    CachedVoice(InlineQueryResultCachedVoice),
    #[serde(rename = "article")]
    Article(InlineQueryResultArticle),
    #[serde(rename = "audio")]
    Audio(InlineQueryResultAudio),
    #[serde(rename = "contact")]
    Contact(InlineQueryResultContact),
    #[serde(rename = "game")]
    Game(InlineQueryResultGame),
    #[serde(rename = "document")]
    Document(InlineQueryResultDocument),
    #[serde(rename = "gif")]
    Gif(InlineQueryResultGif),
    #[serde(rename = "location")]
    Location(InlineQueryResultLocation),
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    #[serde(rename = "photo")]
    Photo(InlineQueryResultPhoto),
    #[serde(rename = "venue")]
    Venue(InlineQueryResultVenue),
    #[serde(rename = "video")]
    Video(InlineQueryResultVideo),
    #[serde(rename = "voice")]
    Voice(InlineQueryResultVoice),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
    Invoice(InputInvoiceMessageContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum PassportElementError {
    #[serde(rename = "data")]
    DataField(PassportElementErrorDataField),
    #[serde(rename = "front_side")]
    FrontSide(PassportElementErrorFrontSide),
    #[serde(rename = "reverse_side")]
    ReverseSide(PassportElementErrorReverseSide),
    #[serde(rename = "selfie")]
    Selfie(PassportElementErrorSelfie),
    #[serde(rename = "file")]
    File(PassportElementErrorFile),
    #[serde(rename = "files")]
    Files(PassportElementErrorFiles),
    #[serde(rename = "translation_file")]
    TranslationFile(PassportElementErrorTranslationFile),
    #[serde(rename = "translation_files")]
    TranslationFiles(PassportElementErrorTranslationFiles),
    #[serde(rename = "unspecified")]
    Unspecified(PassportElementErrorUnspecified),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    Keyboard(ReplyKeyboardMarkup),
    KeyboardRemove(ReplyKeyboardRemove),
    Force(ForceReply),
}

#[derive(Debug, Clone, Deserialize)]
/// Describes the current status of a webhook.
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// True, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: i64,
    /// Currently used webhook IP address
    pub ip_address: Option<String>,
    /// Unix time for the most recent error that happened when trying to deliver an update via
    /// webhook
    pub last_error_date: Option<i64>,
    /// Error message in human-readable format for the most recent error that happened when trying
    /// to deliver an update via webhook
    pub last_error_message: Option<String>,
    /// Unix time of the most recent error that happened when trying to synchronize available
    /// updates with Telegram datacenters
    pub last_synchronization_error_date: Option<i64>,
    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update
    /// delivery
    pub max_connections: Option<i64>,
    /// A list of update types the bot is subscribed to. Defaults to all update types except
    /// chat_member
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This object represents a Telegram user or bot.
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits
    /// and some programming languages may have difficulty/silent defects in interpreting it. But
    /// it has at most 52 significant bits, so a 64-bit integer or double-precision float type are
    /// safe for storing this identifier.
    pub id: i64,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User's or bot's last name
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User's or bot's username
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IETF language tag of the user's language
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if this user is a Telegram Premium user
    pub is_premium: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if this user added the bot to the attachment menu
    pub added_to_attachment_menu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the bot can be invited to groups. Returned only in getMe.
    pub can_join_groups: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if privacy mode is disabled for the bot. Returned only in getMe.
    pub can_read_all_group_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the bot supports inline queries. Returned only in getMe.
    pub supports_inline_queries: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a chat.
pub struct Chat {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and
    /// some programming languages may have difficulty/silent defects in interpreting it. But it
    /// has at most 52 significant bits, so a signed 64-bit integer or double-precision float type
    /// are safe for storing this identifier.
    pub id: i64,
    #[serde(rename = "type")]
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    pub type_: String,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// True, if the supergroup chat is a forum (has topics enabled)
    pub is_forum: Option<bool>,
    /// Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    /// If non-empty, the list of all active chat usernames; for private chats, supergroups and
    /// channels. Returned only in getChat.
    pub active_usernames: Option<Vec<String>>,
    /// Custom emoji identifier of emoji status of the other party in a private chat. Returned only
    /// in getChat.
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Bio of the other party in a private chat. Returned only in getChat.
    pub bio: Option<String>,
    /// True, if privacy settings of the other party in the private chat allows to use
    /// tg://user?id=<user_id> links only in chats with the user. Returned only in getChat.
    pub has_private_forwards: Option<bool>,
    /// True, if the privacy settings of the other party restrict sending voice and video note
    /// messages in the private chat. Returned only in getChat.
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// True, if users need to join the supergroup before they can send messages. Returned only in
    /// getChat.
    pub join_to_send_messages: Option<bool>,
    /// True, if all users directly joining the supergroup need to be approved by supergroup
    /// administrators. Returned only in getChat.
    pub join_by_request: Option<bool>,
    /// Description, for groups, supergroups and channel chats. Returned only in getChat.
    pub description: Option<String>,
    /// Primary invite link, for groups, supergroups and channel chats. Returned only in getChat.
    pub invite_link: Option<String>,
    /// The most recent pinned message (by sending date). Returned only in getChat.
    pub pinned_message: Option<Box<Message>>,
    /// Default chat member permissions, for groups and supergroups. Returned only in getChat.
    pub permissions: Option<ChatPermissions>,
    /// For supergroups, the minimum allowed delay between consecutive messages sent by each
    /// unpriviledged user; in seconds. Returned only in getChat.
    pub slow_mode_delay: Option<i64>,
    /// The time after which all messages sent to the chat will be automatically deleted; in
    /// seconds. Returned only in getChat.
    pub message_auto_delete_time: Option<i64>,
    /// True, if aggressive anti-spam checks are enabled in the supergroup. The field is only
    /// available to chat administrators. Returned only in getChat.
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    /// True, if non-administrators can only get the list of bots and administrators in the chat.
    /// Returned only in getChat.
    pub has_hidden_members: Option<bool>,
    /// True, if messages from the chat can't be forwarded to other chats. Returned only in
    /// getChat.
    pub has_protected_content: Option<bool>,
    /// For supergroups, name of group sticker set. Returned only in getChat.
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set. Returned only in getChat.
    pub can_set_sticker_set: Option<bool>,
    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel
    /// and vice versa; for supergroups and channel chats. This identifier may be greater than 32
    /// bits and some programming languages may have difficulty/silent defects in interpreting it.
    /// But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type
    /// are safe for storing this identifier. Returned only in getChat.
    pub linked_chat_id: Option<i64>,
    /// For supergroups, the location to which the supergroup is connected. Returned only in
    /// getChat.
    pub location: Option<ChatLocation>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a message.
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    pub message_thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the
    /// field contains a fake sender user in non-channel chats, if the message was sent on behalf
    /// of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for
    /// channel posts, the supergroup itself for messages from anonymous group administrators, the
    /// linked channel for messages automatically forwarded to the discussion group. For backward
    /// compatibility, the field from contains a fake sender user in non-channel chats, if the
    /// message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// For forwarded messages, sender of the original message
    pub forward_from: Option<User>,
    /// For messages forwarded from channels or from anonymous administrators, information about
    /// the original sender chat
    pub forward_from_chat: Option<Chat>,
    /// For messages forwarded from channels, identifier of the original message in the channel
    pub forward_from_message_id: Option<i64>,
    /// For forwarded messages that were originally sent in channels or by an anonymous chat
    /// administrator, signature of the message sender if present
    pub forward_signature: Option<String>,
    /// Sender's name for messages forwarded from users who disallow adding a link to their account
    /// in forwarded messages
    pub forward_sender_name: Option<String>,
    /// For forwarded messages, date the original message was sent in Unix time
    pub forward_date: Option<i64>,
    /// True, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// True, if the message is a channel post that was automatically forwarded to the connected
    /// discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the Message object in this field will not
    /// contain further reply_to_message fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// True, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous
    /// group administrator
    pub author_signature: Option<String>,
    /// For text messages, the actual UTF-8 text of the message
    pub text: Option<String>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in
    /// the text
    pub entities: Option<Vec<MessageEntity>>,
    /// Message is an animation, information about the animation. For backward compatibility, when
    /// this field is set, the document field will also be set
    pub animation: Option<Animation>,
    /// Message is an audio file, information about the file
    pub audio: Option<Audio>,
    /// Message is a general file, information about the file
    pub document: Option<Document>,
    /// Message is a photo, available sizes of the photo
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker
    pub sticker: Option<Sticker>,
    /// Message is a video, information about the video
    pub video: Option<Video>,
    /// Message is a video note, information about the video message
    pub video_note: Option<VideoNote>,
    /// Message is a voice message, information about the file
    pub voice: Option<Voice>,
    /// Caption for the animation, audio, document, photo, video or voice
    pub caption: Option<String>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that
    /// appear in the caption
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// True, if the message media is covered by a spoiler animation
    pub has_media_spoiler: Option<bool>,
    /// Message is a shared contact, information about the contact
    pub contact: Option<Contact>,
    /// Message is a dice with random value
    pub dice: Option<Dice>,
    /// Message is a game, information about the game. More about games »
    pub game: Option<Game>,
    /// Message is a native poll, information about the poll
    pub poll: Option<Poll>,
    /// Message is a venue, information about the venue. For backward compatibility, when this
    /// field is set, the location field will also be set
    pub venue: Option<Venue>,
    /// Message is a shared location, information about the location
    pub location: Option<Location>,
    /// New members that were added to the group or supergroup and information about them (the bot
    /// itself may be one of these members)
    pub new_chat_members: Option<Vec<User>>,
    /// A member was removed from the group, information about them (this member may be the bot
    /// itself)
    pub left_chat_member: Option<User>,
    /// A chat title was changed to this value
    pub new_chat_title: Option<String>,
    /// A chat photo was change to this value
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Service message: the chat photo was deleted
    pub delete_chat_photo: Option<bool>,
    /// Service message: the group has been created
    pub group_chat_created: Option<bool>,
    /// Service message: the supergroup has been created. This field can't be received in a message
    /// coming through updates, because bot can't be a member of a supergroup when it is created.
    /// It can only be found in reply_to_message if someone replies to a very first message in a
    /// directly created supergroup.
    pub supergroup_chat_created: Option<bool>,
    /// Service message: the channel has been created. This field can't be received in a message
    /// coming through updates, because bot can't be a member of a channel when it is created. It
    /// can only be found in reply_to_message if someone replies to a very first message in a
    /// channel.
    pub channel_chat_created: Option<bool>,
    /// Service message: auto-delete timer settings changed in the chat
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    /// The group has been migrated to a supergroup with the specified identifier. This number may
    /// have more than 32 significant bits and some programming languages may have
    /// difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a
    /// signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_to_chat_id: Option<i64>,
    /// The supergroup has been migrated from a group with the specified identifier. This number
    /// may have more than 32 significant bits and some programming languages may have
    /// difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a
    /// signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_from_chat_id: Option<i64>,
    /// Specified message was pinned. Note that the Message object in this field will not contain
    /// further reply_to_message fields even if it is itself a reply.
    pub pinned_message: Option<Box<Message>>,
    /// Message is an invoice for a payment, information about the invoice. More about payments »
    pub invoice: Option<Invoice>,
    /// Message is a service message about a successful payment, information about the payment.
    /// More about payments »
    pub successful_payment: Option<SuccessfulPayment>,
    /// The domain name of the website on which the user has logged in. More about Telegram Login »
    pub connected_website: Option<String>,
    /// Service message: the user allowed the bot added to the attachment menu to write messages
    pub write_access_allowed: Option<WriteAccessAllowed>,
    /// Telegram Passport data
    pub passport_data: Option<PassportData>,
    /// Service message. A user in the chat triggered another user's proximity alert while sharing
    /// Live Location.
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    /// Service message: forum topic created
    pub forum_topic_created: Option<ForumTopicCreated>,
    /// Service message: forum topic edited
    pub forum_topic_edited: Option<ForumTopicEdited>,
    /// Service message: forum topic closed
    pub forum_topic_closed: Option<ForumTopicClosed>,
    /// Service message: forum topic reopened
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    /// Service message: the 'General' forum topic hidden
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    /// Service message: the 'General' forum topic unhidden
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    /// Service message: video chat scheduled
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    /// Service message: video chat started
    pub video_chat_started: Option<VideoChatStarted>,
    /// Service message: video chat ended
    pub video_chat_ended: Option<VideoChatEnded>,
    /// Service message: new participants invited to a video chat
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    /// Service message: data sent by a Web App
    pub web_app_data: Option<WebAppData>,
    /// Inline keyboard attached to the message. login_url buttons are represented as ordinary url
    /// buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a unique message identifier.
pub struct MessageId {
    /// Unique message identifier
    pub message_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This object represents one special entity in a text message. For example, hashtags, usernames,
/// URLs, etc.
pub struct MessageEntity {
    #[serde(rename = "type")]
    /// Type of the entity. Currently, can be “mention” (@username), “hashtag” (#hashtag),
    /// “cashtag” ($USD), “bot_command” (/start@jobs_bot), “url” (https://telegram.org), “email”
    /// (do-not-reply@telegram.org), “phone_number” (+1-212-555-0123), “bold” (bold text), “italic”
    /// (italic text), “underline” (underlined text), “strikethrough” (strikethrough text),
    /// “spoiler” (spoiler message), “code” (monowidth string), “pre” (monowidth block),
    /// “text_link” (for clickable text URLs), “text_mention” (for users without usernames),
    /// “custom_emoji” (for inline custom emoji stickers)
    pub type_: String,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// For “text_link” only, URL that will be opened after user taps on the text
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// For “text_mention” only, the mentioned user
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// For “pre” only, the programming language of the entity text
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// For “custom_emoji” only, unique identifier of the custom emoji. Use getCustomEmojiStickers
    /// to get full information about the sticker
    pub custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents one size of a photo or a file / sticker thumbnail.
pub struct PhotoSize {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for
    /// different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Photo width
    pub width: i64,
    /// Photo height
    pub height: i64,
    /// File size in bytes
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
pub struct Animation {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for
    /// different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: i64,
    /// Video height as defined by sender
    pub height: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Animation thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original animation filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have
    /// difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a
    /// signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents an audio file to be treated as music by the Telegram clients.
pub struct Audio {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for
    /// different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i64,
    /// Performer of the audio as defined by sender or by audio tags
    pub performer: Option<String>,
    /// Title of the audio as defined by sender or by audio tags
    pub title: Option<String>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have
    /// difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a
    /// signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
    /// Thumbnail of the album cover to which the music file belongs
    pub thumb: Option<PhotoSize>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a general file (as opposed to photos, voice messages and audio files).
pub struct Document {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for
    /// different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have
    /// difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a
    /// signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a video file.
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for
    /// different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: i64,
    /// Video height as defined by sender
    pub height: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have
    /// difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a
    /// signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a video message (available in Telegram apps as of v.4.0).
pub struct VideoNote {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for
    /// different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width and height (diameter of the video message) as defined by sender
    pub length: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// File size in bytes
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a voice note.
pub struct Voice {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for
    /// different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i64,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have
    /// difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a
    /// signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a phone contact.
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Contact's user identifier in Telegram. This number may have more than 32 significant bits
    /// and some programming languages may have difficulty/silent defects in interpreting it. But
    /// it has at most 52 significant bits, so a 64-bit integer or double-precision float type are
    /// safe for storing this identifier.
    pub user_id: Option<i64>,
    /// Additional data about the contact in the form of a vCard
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents an animated emoji that displays a random value.
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// Value of the dice, 1-6 for “”, “” and “” base emoji, 1-5 for “” and “” base emoji, 1-64 for
    /// “” base emoji
    pub value: i64,
}

#[derive(Debug, Clone, Deserialize)]
/// This object contains information about one answer option in a poll.
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Number of users that voted for this option
    pub voter_count: i64,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents an answer of a user in a non-anonymous poll.
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,
    /// The user, who changed the answer to the poll
    pub user: User,
    /// 0-based identifiers of answer options, chosen by the user. May be empty if the user
    /// retracted their vote.
    pub option_ids: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object contains information about a poll.
pub struct Poll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-300 characters
    pub question: String,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll
    pub total_voter_count: i64,
    /// True, if the poll is closed
    pub is_closed: bool,
    /// True, if the poll is anonymous
    pub is_anonymous: bool,
    #[serde(rename = "type")]
    /// Poll type, currently can be “regular” or “quiz”
    pub type_: String,
    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// 0-based identifier of the correct answer option. Available only for polls in the quiz mode,
    /// which are closed, or was sent (not forwarded) by the bot or to the private chat with the
    /// bot.
    pub correct_option_id: Option<i64>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a
    /// quiz-style poll, 0-200 characters
    pub explanation: Option<String>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the explanation
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation
    pub open_period: Option<i64>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed
    pub close_date: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a point on the map.
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: f64,
    /// Latitude as defined by sender
    pub latitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// Time relative to the message sending date, during which the location can be updated; in
    /// seconds. For active live locations only.
    pub live_period: Option<i64>,
    /// The direction in which user is moving, in degrees; 1-360. For active live locations only.
    pub heading: Option<i64>,
    /// The maximum distance for proximity alerts about approaching another chat member, in meters.
    /// For sent live locations only.
    pub proximity_alert_radius: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a venue.
pub struct Venue {
    /// Venue location. Can't be a live location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue. (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See supported types.)
    pub google_place_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// Describes data sent from a Web App to the bot.
pub struct WebAppData {
    /// The data. Be aware that a bad client can send arbitrary data in this field.
    pub data: String,
    /// Text of the web_app keyboard button from which the Web App was opened. Be aware that a bad
    /// client can send arbitrary data in this field.
    pub button_text: String,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents the content of a service message, sent whenever a user in the chat
/// triggers a proximity alert set by another user.
pub struct ProximityAlertTriggered {
    /// User that triggered the alert
    pub traveler: User,
    /// User that set the alert
    pub watcher: User,
    /// The distance between the users
    pub distance: i64,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a service message about a change in auto-delete timer settings.
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat; in seconds
    pub message_auto_delete_time: i64,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a service message about a new forum topic created in the chat.
pub struct ForumTopicCreated {
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: i64,
    /// Unique identifier of the custom emoji shown as the topic icon
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a service message about a forum topic closed in the chat. Currently
/// holds no information.
pub struct ForumTopicClosed {
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a service message about an edited forum topic.
pub struct ForumTopicEdited {
    /// New name of the topic, if it was edited
    pub name: Option<String>,
    /// New identifier of the custom emoji shown as the topic icon, if it was edited; an empty
    /// string if the icon was removed
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a service message about a forum topic reopened in the chat. Currently
/// holds no information.
pub struct ForumTopicReopened {
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a service message about General forum topic hidden in the chat.
/// Currently holds no information.
pub struct GeneralForumTopicHidden {
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a service message about General forum topic unhidden in the chat.
/// Currently holds no information.
pub struct GeneralForumTopicUnhidden {
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a service message about a user allowing a bot added to the attachment
/// menu to write messages. Currently holds no information.
pub struct WriteAccessAllowed {
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a service message about a video chat scheduled in the chat.
pub struct VideoChatScheduled {
    /// Point in time (Unix timestamp) when the video chat is supposed to be started by a chat
    /// administrator
    pub start_date: i64,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a service message about a video chat started in the chat. Currently
/// holds no information.
pub struct VideoChatStarted {
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a service message about a video chat ended in the chat.
pub struct VideoChatEnded {
    /// Video chat duration in seconds
    pub duration: i64,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a service message about new members invited to a video chat.
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the video chat
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represent a user's profile pictures.
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i64,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a file ready to be downloaded. The file can be downloaded via the link
/// https://api.telegram.org/file/bot<token>/<file_path>. It is guaranteed that the link will be
/// valid for at least 1 hour. When the link expires, a new one can be requested by calling
/// getFile.
pub struct File {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for
    /// different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have
    /// difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a
    /// signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
    /// File path. Use https://api.telegram.org/file/bot<token>/<file_path> to get the file.
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Describes a Web App.
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified in Initializing
    /// Web Apps
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This object represents a custom keyboard with reply options (see Introduction to bots for
/// details and examples).
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of KeyboardButton objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults
    /// to false, in which case the custom keyboard can be hidden and opened with a keyboard icon.
    pub is_persistent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard
    /// smaller if there are just two rows of buttons). Defaults to false, in which case the custom
    /// keyboard is always of the same height as the app's standard keyboard.
    pub resize_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be
    /// available, but clients will automatically display the usual letter-keyboard in the chat -
    /// the user can press a special button in the input field to see the custom keyboard again.
    /// Defaults to false.
    pub one_time_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    pub input_field_placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Use this parameter if you want to show the keyboard to specific users only. Targets: 1)
    /// users that are @mentioned in the text of the Message object; 2) if the bot's message is a
    /// reply (has reply_to_message_id), sender of the original message.Example: A user requests to
    /// change the bot's language, bot replies to the request with a keyboard to select the new
    /// language. Other users in the group don't see the keyboard.
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This object represents one button of the reply keyboard. For simple text buttons String can be
/// used instead of this object to specify text of the button. Optional fields web_app,
/// request_contact, request_location, and request_poll are mutually exclusive.
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent as a message
    /// when the button is pressed
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If True, the user's phone number will be sent as a contact when the button is pressed.
    /// Available in private chats only.
    pub request_contact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If True, the user's current location will be sent when the button is pressed. Available in
    /// private chats only.
    pub request_location: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If specified, the user will be asked to create a poll and send it to the bot when the
    /// button is pressed. Available in private chats only.
    pub request_poll: Option<KeyboardButtonPollType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If specified, the described Web App will be launched when the button is pressed. The Web
    /// App will be able to send a “web_app_data” service message. Available in private chats only.
    pub web_app: Option<WebAppInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This object represents type of a poll, which is allowed to be created and sent when the
/// corresponding button is pressed.
pub struct KeyboardButtonPollType {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If quiz is passed, the user will be allowed to create only polls in the quiz mode. If
    /// regular is passed, only regular polls will be allowed. Otherwise, the user will be allowed
    /// to create a poll of any type.
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Upon receiving a message with this object, Telegram clients will remove the current custom
/// keyboard and display the default letter-keyboard. By default, custom keyboards are displayed
/// until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are
/// hidden immediately after the user presses a button (see ReplyKeyboardMarkup).
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard (user will not be able to summon this
    /// keyboard; if you want to hide the keyboard from sight but keep it accessible, use
    /// one_time_keyboard in ReplyKeyboardMarkup)
    pub remove_keyboard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Use this parameter if you want to remove the keyboard for specific users only. Targets: 1)
    /// users that are @mentioned in the text of the Message object; 2) if the bot's message is a
    /// reply (has reply_to_message_id), sender of the original message.Example: A user votes in a
    /// poll, bot returns confirmation message in reply to the vote and removes the keyboard for
    /// that user, while still showing the keyboard with poll options to users who haven't voted
    /// yet.
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This object represents an inline keyboard that appears right next to the message it belongs to.
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of InlineKeyboardButton objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This object represents one button of an inline keyboard. You must use exactly one of the
/// optional fields.
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HTTP or tg:// URL to be opened when the button is pressed. Links tg://user?id=<user_id> can
    /// be used to mention a user by their ID without using a username, if this is allowed by their
    /// privacy settings.
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Data to be sent in a callback query to the bot when button is pressed, 1-64 bytes
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Description of the Web App that will be launched when the user presses the button. The Web
    /// App will be able to send an arbitrary message on behalf of the user using the method
    /// answerWebAppQuery. Available only in private chats between a user and the bot.
    pub web_app: Option<WebAppInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the
    /// Telegram Login Widget.
    pub login_url: Option<LoginUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If set, pressing the button will prompt the user to select one of their chats, open that
    /// chat and insert the bot's username and the specified inline query in the input field. May
    /// be empty, in which case just the bot's username will be inserted.Note: This offers an easy
    /// way for users to start using your bot in inline mode when they are currently in a private
    /// chat with it. Especially useful when combined with switch_pm… actions - in this case the
    /// user will be automatically returned to the chat they switched from, skipping the chat
    /// selection screen.
    pub switch_inline_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If set, pressing the button will insert the bot's username and the specified inline query
    /// in the current chat's input field. May be empty, in which case only the bot's username will
    /// be inserted.This offers a quick way for the user to open your bot in inline mode in the
    /// same chat - good for selecting something from multiple options.
    pub switch_inline_query_current_chat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Description of the game that will be launched when the user presses the button.NOTE: This
    /// type of button must always be the first button in the first row.
    pub callback_game: Option<CallbackGame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specify True, to send a Pay button.NOTE: This type of button must always be the first
    /// button in the first row and can only be used in invoice messages.
    pub pay: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This object represents a parameter of the inline keyboard button used to automatically
/// authorize a user. Serves as a great replacement for the Telegram Login Widget when the user is
/// coming from Telegram. All the user needs to do is tap/click a button and confirm that they want
/// to log in: Telegram apps support these buttons as of version 5.7.
pub struct LoginUrl {
    /// An HTTPS URL to be opened with user authorization data added to the query string when the
    /// button is pressed. If the user refuses to provide authorization data, the original URL
    /// without information about the user will be opened. The data added is the same as described
    /// in Receiving authorization data.NOTE: You must always check the hash of the received data
    /// to verify the authentication and the integrity of the data as described in Checking
    /// authorization.
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// New text of the button in forwarded messages.
    pub forward_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Username of a bot, which will be used for user authorization. See Setting up a bot for more
    /// details. If not specified, the current bot's username will be assumed. The url's domain
    /// must be the same as the domain linked with the bot. See Linking your domain to the bot for
    /// more details.
    pub bot_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True to request the permission for your bot to send messages to the user.
    pub request_write_access: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents an incoming callback query from a callback button in an inline keyboard.
/// If the button that originated the query was attached to a message sent by the bot, the field
/// message will be present. If the button was attached to a message sent via the bot (in inline
/// mode), the field inline_message_id will be present. Exactly one of the fields data or
/// game_short_name will be present.
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Message with the callback button that originated the query. Note that message content and
    /// message date will not be available if the message is too old
    pub message: Option<Box<Message>>,
    /// Identifier of the message sent via the bot in inline mode, that originated the query.
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the
    /// callback button was sent. Useful for high scores in games.
    pub chat_instance: String,
    /// Data associated with the callback button. Be aware that the message originated the query
    /// can contain no callback buttons with this data.
    pub data: Option<String>,
    /// Short name of a Game to be returned, serves as the unique identifier for the game
    pub game_short_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Upon receiving a message with this object, Telegram clients will display a reply interface to
/// the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be
/// extremely useful if you want to create user-friendly step-by-step interfaces without having to
/// sacrifice privacy mode.
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot's message and
    /// tapped 'Reply'
    pub force_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The placeholder to be shown in the input field when the reply is active; 1-64 characters
    pub input_field_placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Use this parameter if you want to force reply from specific users only. Targets: 1) users
    /// that are @mentioned in the text of the Message object; 2) if the bot's message is a reply
    /// (has reply_to_message_id), sender of the original message.
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a chat photo.
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo. This file_id can be used only for photo
    /// download and only for as long as the photo is not changed.
    pub small_file_id: String,
    /// Unique file identifier of small (160x160) chat photo, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the file.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo. This file_id can be used only for photo
    /// download and only for as long as the photo is not changed.
    pub big_file_id: String,
    /// Unique file identifier of big (640x640) chat photo, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the file.
    pub big_file_unique_id: String,
}

#[derive(Debug, Clone, Deserialize)]
/// Represents an invite link for a chat.
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator, then the second
    /// part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// True, if users joining the chat via the link need to be approved by chat administrators
    pub creates_join_request: bool,
    /// True, if the link is primary
    pub is_primary: bool,
    /// True, if the link is revoked
    pub is_revoked: bool,
    /// Invite link name
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire or has been expired
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining
    /// the chat via this invite link; 1-99999
    pub member_limit: Option<i64>,
    /// Number of pending join requests created using this link
    pub pending_join_request_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the rights of an administrator in a chat.
pub struct ChatAdministratorRights {
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// True, if the administrator can access the chat event log, chat statistics, message
    /// statistics in channels, see channel members, see anonymous administrators in supergroups
    /// and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: bool,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,
    /// True, if the administrator can add new administrators with a subset of their own privileges
    /// or demote administrators that they have promoted, directly or indirectly (promoted by
    /// administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the administrator can post in the channel; channels only
    pub can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the administrator can edit messages of other users and can pin messages; channels
    /// only
    pub can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the user is allowed to pin messages; groups and supergroups only
    pub can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the user is allowed to create, rename, close, and reopen forum topics; supergroups
    /// only
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
/// Represents a chat member that owns the chat and has all administrator privileges.
pub struct ChatMemberOwner {
    /// The member's status in the chat, always “creator”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// Custom title for this user
    pub custom_title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// Represents a chat member that has some additional privileges.
pub struct ChatMemberAdministrator {
    /// The member's status in the chat, always “administrator”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// True, if the bot is allowed to edit administrator privileges of that user
    pub can_be_edited: bool,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// True, if the administrator can access the chat event log, chat statistics, message
    /// statistics in channels, see channel members, see anonymous administrators in supergroups
    /// and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: bool,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,
    /// True, if the administrator can add new administrators with a subset of their own privileges
    /// or demote administrators that they have promoted, directly or indirectly (promoted by
    /// administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the administrator can post in the channel; channels only
    pub can_post_messages: Option<bool>,
    /// True, if the administrator can edit messages of other users and can pin messages; channels
    /// only
    pub can_edit_messages: Option<bool>,
    /// True, if the user is allowed to pin messages; groups and supergroups only
    pub can_pin_messages: Option<bool>,
    /// True, if the user is allowed to create, rename, close, and reopen forum topics; supergroups
    /// only
    pub can_manage_topics: Option<bool>,
    /// Custom title for this user
    pub custom_title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// Represents a chat member that has no additional privileges or restrictions.
pub struct ChatMemberMember {
    /// The member's status in the chat, always “member”
    pub status: String,
    /// Information about the user
    pub user: User,
}

#[derive(Debug, Clone, Deserialize)]
/// Represents a chat member that is under certain restrictions in the chat. Supergroups only.
pub struct ChatMemberRestricted {
    /// The member's status in the chat, always “restricted”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// True, if the user is a member of the chat at the moment of the request
    pub is_member: bool,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the user is allowed to pin messages
    pub can_pin_messages: bool,
    /// True, if the user is allowed to create forum topics
    pub can_manage_topics: bool,
    /// True, if the user is allowed to send text messages, contacts, locations and venues
    pub can_send_messages: bool,
    /// True, if the user is allowed to send audios, documents, photos, videos, video notes and
    /// voice notes
    pub can_send_media_messages: bool,
    /// True, if the user is allowed to send polls
    pub can_send_polls: bool,
    /// True, if the user is allowed to send animations, games, stickers and use inline bots
    pub can_send_other_messages: bool,
    /// True, if the user is allowed to add web page previews to their messages
    pub can_add_web_page_previews: bool,
    /// Date when restrictions will be lifted for this user; unix time. If 0, then the user is
    /// restricted forever
    pub until_date: i64,
}

#[derive(Debug, Clone, Deserialize)]
/// Represents a chat member that isn't currently a member of the chat, but may join it themselves.
pub struct ChatMemberLeft {
    /// The member's status in the chat, always “left”
    pub status: String,
    /// Information about the user
    pub user: User,
}

#[derive(Debug, Clone, Deserialize)]
/// Represents a chat member that was banned in the chat and can't return to the chat or view chat
/// messages.
pub struct ChatMemberBanned {
    /// The member's status in the chat, always “kicked”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; unix time. If 0, then the user is
    /// banned forever
    pub until_date: i64,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents changes in the status of a chat member.
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    /// Date the change was done in Unix time
    pub date: i64,
    /// Previous information about the chat member
    pub old_chat_member: ChatMember,
    /// New information about the chat member
    pub new_chat_member: ChatMember,
    /// Chat invite link, which was used by the user to join the chat; for joining by invite link
    /// events only.
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Deserialize)]
/// Represents a join request sent to a chat.
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Date the request was sent in Unix time
    pub date: i64,
    /// Bio of the user.
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Describes actions that a non-administrator user is allowed to take in a chat.
pub struct ChatPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the user is allowed to send text messages, contacts, locations and venues
    pub can_send_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the user is allowed to send audios, documents, photos, videos, video notes and
    /// voice notes, implies can_send_messages
    pub can_send_media_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the user is allowed to send polls, implies can_send_messages
    pub can_send_polls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the user is allowed to send animations, games, stickers and use inline bots,
    /// implies can_send_media_messages
    pub can_send_other_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the user is allowed to add web page previews to their messages, implies
    /// can_send_media_messages
    pub can_add_web_page_previews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the user is allowed to change the chat title, photo and other settings. Ignored in
    /// public supergroups
    pub can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the user is allowed to pin messages. Ignored in public supergroups
    pub can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the user is allowed to create forum topics. If omitted defaults to the value of
    /// can_pin_messages
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
/// Represents a location to which a chat is connected.
pub struct ChatLocation {
    /// The location to which the supergroup is connected. Can't be a live location.
    pub location: Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: String,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a forum topic.
pub struct ForumTopic {
    /// Unique identifier of the forum topic
    pub message_thread_id: i64,
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: i64,
    /// Unique identifier of the custom emoji shown as the topic icon
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This object represents a bot command.
pub struct BotCommand {
    /// Text of the command; 1-32 characters. Can contain only lowercase English letters, digits
    /// and underscores.
    pub command: String,
    /// Description of the command; 1-256 characters.
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the default scope of bot commands. Default commands are used if no commands with a
/// narrower scope are specified for the user.
pub struct BotCommandScopeDefault {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the scope of bot commands, covering all private chats.
pub struct BotCommandScopeAllPrivateChats {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the scope of bot commands, covering all group and supergroup chats.
pub struct BotCommandScopeAllGroupChats {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the scope of bot commands, covering all group and supergroup chat administrators.
pub struct BotCommandScopeAllChatAdministrators {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the scope of bot commands, covering a specific chat.
pub struct BotCommandScopeChat {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the scope of bot commands, covering all administrators of a specific group or
/// supergroup chat.
pub struct BotCommandScopeChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the scope of bot commands, covering a specific member of a group or supergroup chat.
pub struct BotCommandScopeChatMember {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a menu button, which opens the bot's list of commands.
pub struct MenuButtonCommands {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a menu button, which launches a Web App.
pub struct MenuButtonWebApp {
    /// Text on the button
    pub text: String,
    /// Description of the Web App that will be launched when the user presses the button. The Web
    /// App will be able to send an arbitrary message on behalf of the user using the method
    /// answerWebAppQuery.
    pub web_app: WebAppInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Describes that no specific value for the menu button was set.
pub struct MenuButtonDefault {
}

#[derive(Debug, Clone, Deserialize)]
/// Describes why a request was unsuccessful.
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier. This number may
    /// have more than 32 significant bits and some programming languages may have
    /// difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a
    /// signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_to_chat_id: Option<i64>,
    /// In case of exceeding flood control, the number of seconds left to wait before the request
    /// can be repeated
    pub retry_after: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a photo to be sent.
pub struct InputMediaPhoto {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers
    /// (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass
    /// “attach://<file_attach_name>” to upload a new one using multipart/form-data under
    /// <file_attach_name> name. More information on Sending Files »
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the photo to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the photo needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a video to be sent.
pub struct InputMediaVideo {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers
    /// (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass
    /// “attach://<file_attach_name>” to upload a new one using multipart/form-data under
    /// <file_attach_name> name. More information on Sending Files »
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
    /// supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new
    /// file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More information on Sending Files »
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the video caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video width
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video height
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video duration in seconds
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the video needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
pub struct InputMediaAnimation {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers
    /// (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass
    /// “attach://<file_attach_name>” to upload a new one using multipart/form-data under
    /// <file_attach_name> name. More information on Sending Files »
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
    /// supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new
    /// file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More information on Sending Files »
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the animation to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the animation caption. See formatting options for more
    /// details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Animation width
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Animation height
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Animation duration in seconds
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the animation needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an audio file to be treated as music to be sent.
pub struct InputMediaAudio {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers
    /// (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass
    /// “attach://<file_attach_name>” to upload a new one using multipart/form-data under
    /// <file_attach_name> name. More information on Sending Files »
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
    /// supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new
    /// file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More information on Sending Files »
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the audio to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the audio caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration of the audio in seconds
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Performer of the audio
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Title of the audio
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a general file to be sent.
pub struct InputMediaDocument {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers
    /// (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass
    /// “attach://<file_attach_name>” to upload a new one using multipart/form-data under
    /// <file_attach_name> name. More information on Sending Files »
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
    /// supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new
    /// file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More information on Sending Files »
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the document to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the document caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disables automatic server-side content type detection for files uploaded using
    /// multipart/form-data. Always True, if the document is sent as part of an album.
    pub disable_content_type_detection: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a sticker.
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for
    /// different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    #[serde(rename = "type")]
    /// Type of the sticker, currently one of “regular”, “mask”, “custom_emoji”. The type of the
    /// sticker is independent from its format, which is determined by the fields is_animated and
    /// is_video.
    pub type_: String,
    /// Sticker width
    pub width: i64,
    /// Sticker height
    pub height: i64,
    /// True, if the sticker is animated
    pub is_animated: bool,
    /// True, if the sticker is a video sticker
    pub is_video: bool,
    /// Sticker thumbnail in the .WEBP or .JPG format
    pub thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    /// For premium regular stickers, premium animation for the sticker
    pub premium_animation: Option<File>,
    /// For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
    /// For custom emoji stickers, unique identifier of the custom emoji
    pub custom_emoji_id: Option<String>,
    /// File size in bytes
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a sticker set.
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// Type of stickers in the set, currently one of “regular”, “mask”, “custom_emoji”
    pub sticker_type: String,
    /// True, if the sticker set contains animated stickers
    pub is_animated: bool,
    /// True, if the sticker set contains video stickers
    pub is_video: bool,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    pub thumb: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This object describes the position on faces where a mask should be placed by default.
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One of “forehead”,
    /// “eyes”, “mouth”, or “chin”.
    pub point: String,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right.
    /// For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: f64,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to
    /// bottom. For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: f64,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f64,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents an incoming inline query. When the user sends an empty query, your bot
/// could return some default or trending results.
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Text of the query (up to 256 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
    /// Type of the chat from which the inline query was sent. Can be either “sender” for a private
    /// chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat
    /// type should be always known for requests sent from official clients and most third-party
    /// clients, unless the request was sent from a secret chat
    pub chat_type: Option<String>,
    /// Sender location, only for bots that request user location
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to an article or web page.
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// URL of the result
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if you don't want the URL to be shown in the message
    pub hide_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Short description of the result
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail width
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail height
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to a photo. By default, this photo will be sent by the user with optional
/// caption. Alternatively, you can use input_message_content to send a message with the specified
/// content instead of the photo.
pub struct InlineQueryResultPhoto {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL of the photo. Photo must be in JPEG format. Photo size must not exceed 5MB
    pub photo_url: String,
    /// URL of the thumbnail for the photo
    pub thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Width of the photo
    pub photo_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Height of the photo
    pub photo_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Title for the result
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Short description of the result
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the photo to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by
/// the user with optional caption. Alternatively, you can use input_message_content to send a
/// message with the specified content instead of the animation.
pub struct InlineQueryResultGif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Width of the GIF
    pub gif_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Height of the GIF
    pub gif_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration of the GIF in seconds
    pub gif_duration: Option<i64>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”.
    /// Defaults to “image/jpeg”
    pub thumb_mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Title for the result
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this
/// animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use
/// input_message_content to send a message with the specified content instead of the animation.
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the MPEG4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video width
    pub mpeg4_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video height
    pub mpeg4_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video duration in seconds
    pub mpeg4_duration: Option<i64>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”.
    /// Defaults to “image/jpeg”
    pub thumb_mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Title for the result
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to a page containing an embedded video player or a video file. By default,
/// this video file will be sent by the user with an optional caption. Alternatively, you can use
/// input_message_content to send a message with the specified content instead of the video.
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// MIME type of the content of the video URL, “text/html” or “video/mp4”
    pub mime_type: String,
    /// URL of the thumbnail (JPEG only) for the video
    pub thumb_url: String,
    /// Title for the result
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the video caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video width
    pub video_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video height
    pub video_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video duration in seconds
    pub video_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Short description of the result
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the video. This field is required if
    /// InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to an MP3 audio file. By default, this audio file will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the audio.
pub struct InlineQueryResultAudio {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the audio file
    pub audio_url: String,
    /// Title
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the audio caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Performer
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Audio duration in seconds
    pub audio_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to a voice recording in an .OGG container encoded with OPUS. By default, this
/// voice recording will be sent by the user. Alternatively, you can use input_message_content to
/// send a message with the specified content instead of the the voice message.
pub struct InlineQueryResultVoice {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the voice recording
    pub voice_url: String,
    /// Recording title
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the voice message caption. See formatting options for more
    /// details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Recording duration in seconds
    pub voice_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the voice recording
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to a file. By default, this file will be sent by the user with an optional
/// caption. Alternatively, you can use input_message_content to send a message with the specified
/// content instead of the file. Currently, only .PDF and .ZIP files can be sent using this method.
pub struct InlineQueryResultDocument {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the document to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the document caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A valid URL for the file
    pub document_url: String,
    /// MIME type of the content of the file, either “application/pdf” or “application/zip”
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Short description of the result
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// URL of the thumbnail (JPEG only) for the file
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail width
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail height
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a location on a map. By default, the location will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the location.
pub struct InlineQueryResultLocation {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Location title
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1
    /// and 360 if specified.
    pub heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// For live locations, a maximum distance for proximity alerts about approaching another chat
    /// member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the location
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail width
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail height
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use
/// input_message_content to send a message with the specified content instead of the venue.
pub struct InlineQueryResultVenue {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Latitude of the venue location in degrees
    pub latitude: f64,
    /// Longitude of the venue location in degrees
    pub longitude: f64,
    /// Title of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Foursquare identifier of the venue if known
    pub foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Google Places type of the venue. (See supported types.)
    pub google_place_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the venue
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail width
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail height
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a contact with a phone number. By default, this contact will be sent by the user.
/// Alternatively, you can use input_message_content to send a message with the specified content
/// instead of the contact.
pub struct InlineQueryResultContact {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Contact's last name
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub vcard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the contact
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail width
    pub thumb_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail height
    pub thumb_height: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a Game.
pub struct InlineQueryResultGame {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to a photo stored on the Telegram servers. By default, this photo will be
/// sent by the user with an optional caption. Alternatively, you can use input_message_content to
/// send a message with the specified content instead of the photo.
pub struct InlineQueryResultCachedPhoto {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the photo
    pub photo_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Title for the result
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Short description of the result
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the photo to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to an animated GIF file stored on the Telegram servers. By default, this
/// animated GIF file will be sent by the user with an optional caption. Alternatively, you can use
/// input_message_content to send a message with specified content instead of the animation.
pub struct InlineQueryResultCachedGif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the GIF file
    pub gif_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Title for the result
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the
/// Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an
/// optional caption. Alternatively, you can use input_message_content to send a message with the
/// specified content instead of the animation.
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the MPEG4 file
    pub mpeg4_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Title for the result
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be
/// sent by the user. Alternatively, you can use input_message_content to send a message with the
/// specified content instead of the sticker.
pub struct InlineQueryResultCachedSticker {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the sticker
    pub sticker_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the sticker
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to a file stored on the Telegram servers. By default, this file will be sent
/// by the user with an optional caption. Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the file.
pub struct InlineQueryResultCachedDocument {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// A valid file identifier for the file
    pub document_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Short description of the result
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the document to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the document caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to a video file stored on the Telegram servers. By default, this video file
/// will be sent by the user with an optional caption. Alternatively, you can use
/// input_message_content to send a message with the specified content instead of the video.
pub struct InlineQueryResultCachedVideo {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the video file
    pub video_file_id: String,
    /// Title for the result
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Short description of the result
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the video caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the video
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to a voice message stored on the Telegram servers. By default, this voice
/// message will be sent by the user. Alternatively, you can use input_message_content to send a
/// message with the specified content instead of the voice message.
pub struct InlineQueryResultCachedVoice {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the voice message
    pub voice_file_id: String,
    /// Voice message title
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the voice message caption. See formatting options for more
    /// details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the voice message
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a link to an MP3 audio file stored on the Telegram servers. By default, this audio
/// file will be sent by the user. Alternatively, you can use input_message_content to send a
/// message with the specified content instead of the audio.
pub struct InlineQueryResultCachedAudio {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the audio file
    pub audio_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the audio caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in the caption, which can be specified instead of
    /// parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the content of a text message to be sent as the result of an inline query.
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the message text. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of special entities that appear in message text, which can be specified instead of
    /// parse_mode
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disables link previews for links in the sent message
    pub disable_web_page_preview: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the content of a location message to be sent as the result of an inline query.
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: f64,
    /// Longitude of the location in degrees
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1
    /// and 360 if specified.
    pub heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// For live locations, a maximum distance for proximity alerts about approaching another chat
    /// member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the content of a venue message to be sent as the result of an inline query.
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: f64,
    /// Longitude of the venue in degrees
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Foursquare identifier of the venue, if known
    pub foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Google Places type of the venue. (See supported types.)
    pub google_place_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the content of a contact message to be sent as the result of an inline query.
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Contact's last name
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the content of an invoice message to be sent as the result of an inline query.
pub struct InputInvoiceMessageContent {
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for
    /// your internal processes.
    pub payload: String,
    /// Payment provider token, obtained via @BotFather
    pub provider_token: String,
    /// Three-letter ISO 4217 currency code, see more on currencies
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount,
    /// delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not
    /// float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See
    /// the exp parameter in currencies.json, it shows the number of digits past the decimal point
    /// for each currency (2 for the majority of currencies). Defaults to 0
    pub max_tip_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized array of suggested amounts of tip in the smallest units of the currency
    /// (integer, not float/double). At most 4 suggested tip amounts can be specified. The
    /// suggested tip amounts must be positive, passed in a strictly increased order and must not
    /// exceed max_tip_amount.
    pub suggested_tip_amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for data about the invoice, which will be shared with the payment
    /// provider. A detailed description of the required fields should be provided by the payment
    /// provider.
    pub provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image
    /// for a service.
    pub photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Photo size in bytes
    pub photo_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Photo width
    pub photo_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Photo height
    pub photo_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if you require the user's full name to complete the order
    pub need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if you require the user's phone number to complete the order
    pub need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if you require the user's email address to complete the order
    pub need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if you require the user's shipping address to complete the order
    pub need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the user's phone number should be sent to provider
    pub send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the user's email address should be sent to provider
    pub send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the final price depends on the shipping method
    pub is_flexible: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
/// Represents a result of an inline query that was chosen by the user and sent to their chat
/// partner.
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// Identifier of the sent inline message. Available only if there is an inline keyboard
    /// attached to the message. Will be also received in callback queries and can be used to edit
    /// the message.
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}

#[derive(Debug, Clone, Deserialize)]
/// Describes an inline message sent by a Web App on behalf of a user.
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message. Available only if there is an inline keyboard
    /// attached to the message.
    pub inline_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This object represents a portion of the price for goods or services.
pub struct LabeledPrice {
    /// Portion label
    pub label: String,
    /// Price of the product in the smallest units of the currency (integer, not float/double). For
    /// example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in
    /// currencies.json, it shows the number of digits past the decimal point for each currency (2
    /// for the majority of currencies).
    pub amount: i64,
}

#[derive(Debug, Clone, Deserialize)]
/// This object contains basic information about an invoice.
pub struct Invoice {
    /// Product name
    pub title: String,
    /// Product description
    pub description: String,
    /// Unique bot deep-linking parameter that can be used to generate this invoice
    pub start_parameter: String,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example,
    /// for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it
    /// shows the number of digits past the decimal point for each currency (2 for the majority of
    /// currencies).
    pub total_amount: i64,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a shipping address.
pub struct ShippingAddress {
    /// Two-letter ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable
    pub state: String,
    /// City
    pub city: String,
    /// First line for the address
    pub street_line1: String,
    /// Second line for the address
    pub street_line2: String,
    /// Address post code
    pub post_code: String,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents information about an order.
pub struct OrderInfo {
    /// User name
    pub name: Option<String>,
    /// User's phone number
    pub phone_number: Option<String>,
    /// User email
    pub email: Option<String>,
    /// User shipping address
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// This object represents one shipping option.
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Clone, Deserialize)]
/// This object contains basic information about a successful payment.
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example,
    /// for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it
    /// shows the number of digits past the decimal point for each currency (2 for the majority of
    /// currencies).
    pub total_amount: i64,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Order information provided by the user
    pub order_info: Option<OrderInfo>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}

#[derive(Debug, Clone, Deserialize)]
/// This object contains information about an incoming shipping query.
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}

#[derive(Debug, Clone, Deserialize)]
/// This object contains information about an incoming pre-checkout query.
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example,
    /// for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it
    /// shows the number of digits past the decimal point for each currency (2 for the majority of
    /// currencies).
    pub total_amount: i64,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Order information provided by the user
    pub order_info: Option<OrderInfo>,
}

#[derive(Debug, Clone, Deserialize)]
/// Describes Telegram Passport data shared with the bot by the user.
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport elements that was shared
    /// with the bot
    pub data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport
/// files are in JPEG format when decrypted and don't exceed 10MB.
pub struct PassportFile {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for
    /// different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size in bytes
    pub file_size: i64,
    /// Unix time when the file was uploaded
    pub file_date: i64,
}

#[derive(Debug, Clone, Deserialize)]
/// Describes documents or other Telegram Passport elements shared with the bot by the user.
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    /// Element type. One of “personal_details”, “passport”, “driver_license”, “identity_card”,
    /// “internal_passport”, “address”, “utility_bill”, “bank_statement”, “rental_agreement”,
    /// “passport_registration”, “temporary_registration”, “phone_number”, “email”.
    pub type_: String,
    /// Base64-encoded encrypted Telegram Passport element data provided by the user, available for
    /// “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport” and
    /// “address” types. Can be decrypted and verified using the accompanying EncryptedCredentials.
    pub data: Option<String>,
    /// User's verified phone number, available only for “phone_number” type
    pub phone_number: Option<String>,
    /// User's verified email address, available only for “email” type
    pub email: Option<String>,
    /// Array of encrypted files with documents provided by the user, available for “utility_bill”,
    /// “bank_statement”, “rental_agreement”, “passport_registration” and “temporary_registration”
    /// types. Files can be decrypted and verified using the accompanying EncryptedCredentials.
    pub files: Option<Vec<PassportFile>>,
    /// Encrypted file with the front side of the document, provided by the user. Available for
    /// “passport”, “driver_license”, “identity_card” and “internal_passport”. The file can be
    /// decrypted and verified using the accompanying EncryptedCredentials.
    pub front_side: Option<PassportFile>,
    /// Encrypted file with the reverse side of the document, provided by the user. Available for
    /// “driver_license” and “identity_card”. The file can be decrypted and verified using the
    /// accompanying EncryptedCredentials.
    pub reverse_side: Option<PassportFile>,
    /// Encrypted file with the selfie of the user holding a document, provided by the user;
    /// available for “passport”, “driver_license”, “identity_card” and “internal_passport”. The
    /// file can be decrypted and verified using the accompanying EncryptedCredentials.
    pub selfie: Option<PassportFile>,
    /// Array of encrypted files with translated versions of documents provided by the user.
    /// Available if requested for “passport”, “driver_license”, “identity_card”,
    /// “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”,
    /// “passport_registration” and “temporary_registration” types. Files can be decrypted and
    /// verified using the accompanying EncryptedCredentials.
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for using in PassportElementErrorUnspecified
    pub hash: String,
}

#[derive(Debug, Clone, Deserialize)]
/// Describes data required for decrypting and authenticating EncryptedPassportElement. See the
/// Telegram Passport Documentation for a complete description of the data decryption and
/// authentication processes.
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and
    /// secrets required for EncryptedPassportElement decryption and authentication
    pub data: String,
    /// Base64-encoded data hash for data authentication
    pub hash: String,
    /// Base64-encoded secret, encrypted with the bot's public RSA key, required for data
    /// decryption
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an issue in one of the data fields that was provided by the user. The error is
/// considered resolved when the field's value changes.
pub struct PassportElementErrorDataField {
    #[serde(rename = "type")]
    /// The section of the user's Telegram Passport which has the error, one of “personal_details”,
    /// “passport”, “driver_license”, “identity_card”, “internal_passport”, “address”
    pub type_: String,
    /// Name of the data field which has the error
    pub field_name: String,
    /// Base64-encoded data hash
    pub data_hash: String,
    /// Error message
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an issue with the front side of a document. The error is considered resolved when
/// the file with the front side of the document changes.
pub struct PassportElementErrorFrontSide {
    #[serde(rename = "type")]
    /// The section of the user's Telegram Passport which has the issue, one of “passport”,
    /// “driver_license”, “identity_card”, “internal_passport”
    pub type_: String,
    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an issue with the reverse side of a document. The error is considered resolved when
/// the file with reverse side of the document changes.
pub struct PassportElementErrorReverseSide {
    #[serde(rename = "type")]
    /// The section of the user's Telegram Passport which has the issue, one of “driver_license”,
    /// “identity_card”
    pub type_: String,
    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an issue with the selfie with a document. The error is considered resolved when the
/// file with the selfie changes.
pub struct PassportElementErrorSelfie {
    #[serde(rename = "type")]
    /// The section of the user's Telegram Passport which has the issue, one of “passport”,
    /// “driver_license”, “identity_card”, “internal_passport”
    pub type_: String,
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: String,
    /// Error message
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an issue with a document scan. The error is considered resolved when the file with
/// the document scan changes.
pub struct PassportElementErrorFile {
    #[serde(rename = "type")]
    /// The section of the user's Telegram Passport which has the issue, one of “utility_bill”,
    /// “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    pub type_: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an issue with a list of scans. The error is considered resolved when the list of
/// files containing the scans changes.
pub struct PassportElementErrorFiles {
    #[serde(rename = "type")]
    /// The section of the user's Telegram Passport which has the issue, one of “utility_bill”,
    /// “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    pub type_: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an issue with one of the files that constitute the translation of a document. The
/// error is considered resolved when the file changes.
pub struct PassportElementErrorTranslationFile {
    #[serde(rename = "type")]
    /// Type of element of the user's Telegram Passport which has the issue, one of “passport”,
    /// “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”,
    /// “rental_agreement”, “passport_registration”, “temporary_registration”
    pub type_: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an issue with the translated version of a document. The error is considered resolved
/// when a file with the document translation change.
pub struct PassportElementErrorTranslationFiles {
    #[serde(rename = "type")]
    /// Type of element of the user's Telegram Passport which has the issue, one of “passport”,
    /// “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”,
    /// “rental_agreement”, “passport_registration”, “temporary_registration”
    pub type_: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an issue in an unspecified place. The error is considered resolved when new data is
/// added.
pub struct PassportElementErrorUnspecified {
    #[serde(rename = "type")]
    /// Type of element of the user's Telegram Passport which has the issue
    pub type_: String,
    /// Base64-encoded element hash
    pub element_hash: String,
    /// Error message
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents a game. Use BotFather to create and edit games, their short names will
/// act as unique identifiers.
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// Brief description of the game or high scores included in the game message. Can be
    /// automatically edited to include current high scores for the game when the bot calls
    /// setGameScore, or manually edited using editMessageText. 0-4096 characters.
    pub text: Option<String>,
    /// Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Animation that will be displayed in the game message in chats. Upload via BotFather
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A placeholder, currently holds no information. Use BotFather to set up your game.
pub struct CallbackGame {
}

#[derive(Debug, Clone, Deserialize)]
/// This object represents one row of the high scores table for a game.
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: i64,
    /// User
    pub user: User,
    /// Score
    pub score: i64,
}

