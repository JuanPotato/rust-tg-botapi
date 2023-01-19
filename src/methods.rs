use crate::types::*;
use crate::helpers::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to receive incoming updates using long polling (wiki). Returns an Array of
/// Update objects.
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifier of the first update to be returned. Must be greater by one than the highest
    /// among the identifiers of previously received updates. By default, updates starting with the
    /// earliest unconfirmed update are returned. An update is considered confirmed as soon as
    /// getUpdates is called with an offset higher than its update_id. The negative offset can be
    /// specified to retrieve updates starting from -offset update from the end of the updates
    /// queue. All previous updates will forgotten.
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults
    /// to 100.
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be
    /// positive, short polling should be used for testing purposes only.
    pub timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of the update types you want your bot to receive. For example,
    /// specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of
    /// these types. See Update for a complete list of available update types. Specify an empty
    /// list to receive all update types except chat_member (default). If not specified, the
    /// previous setting will be used.Please note that this parameter doesn't affect updates
    /// created before the call to the getUpdates, so unwanted updates may be received for a short
    /// period of time.
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever
/// there is an update for the bot, we will send an HTTPS POST request to the specified URL,
/// containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after
/// a reasonable amount of attempts. Returns True on success. If you'd like to make sure that the
/// webhook was set by you, you can specify secret data in the parameter secret_token. If
/// specified, the request will contain a header “X-Telegram-Bot-Api-Secret-Token” with the secret
/// token as content.
pub struct SetWebhook {
    /// HTTPS URL to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Upload your public key certificate so that the root certificate in use can be checked. See
    /// our self-signed guide for details.
    pub certificate: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The fixed IP address which will be used to send webhook requests instead of the IP address
    /// resolved through DNS
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update
    /// delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot's server,
    /// and higher values to increase your bot's throughput.
    pub max_connections: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of the update types you want your bot to receive. For example,
    /// specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of
    /// these types. See Update for a complete list of available update types. Specify an empty
    /// list to receive all update types except chat_member (default). If not specified, the
    /// previous setting will be used.Please note that this parameter doesn't affect updates
    /// created before the call to the setWebhook, so unwanted updates may be received for a short
    /// period of time.
    pub allowed_updates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True to drop all pending updates
    pub drop_pending_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A secret token to be sent in a header “X-Telegram-Bot-Api-Secret-Token” in every webhook
    /// request, 1-256 characters. Only characters A-Z, a-z, 0-9, _ and - are allowed. The header
    /// is useful to ensure that the request comes from a webhook set by you.
    pub secret_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to remove webhook integration if you decide to switch back to getUpdates.
/// Returns True on success.
pub struct DeleteWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True to drop all pending updates
    pub drop_pending_updates: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get current webhook status. Requires no parameters. On success, returns a
/// WebhookInfo object. If the bot is using getUpdates, will return an object with the url field
/// empty.
pub struct GetWebhookInfo {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A simple method for testing your bot's authentication token. Requires no parameters. Returns
/// basic information about the bot in form of a User object.
pub struct GetMe {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to log out from the cloud Bot API server before launching the bot locally. You
/// must log out the bot before running it locally, otherwise there is no guarantee that the bot
/// will receive updates. After a successful call, you can immediately log in on a local server,
/// but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns True on
/// success. Requires no parameters.
pub struct LogOut {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to close the bot instance before moving it from one local server to another.
/// You need to delete the webhook before calling this method to ensure that the bot isn't launched
/// again after server restart. The method will return error 429 in the first 10 minutes after the
/// bot is launched. Returns True on success. Requires no parameters.
pub struct Close {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send text messages. On success, the sent Message is returned.
pub struct SendMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Text of the message to be sent, 1-4096 characters after entities parsing
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the message text. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of special entities that appear in message text, which can be
    /// specified instead of parse_mode
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disables link previews for links in this message
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to forward messages of any kind. Service messages can't be forwarded. On
/// success, the sent Message is returned.
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Unique identifier for the chat where the original message was sent (or channel username in
    /// the format @channelusername)
    pub from_chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the forwarded message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to copy messages of any kind. Service messages and invoice messages can't be
/// copied. A quiz poll can be copied only if the value of the field correct_option_id is known to
/// the bot. The method is analogous to the method forwardMessage, but the copied message doesn't
/// have a link to the original message. Returns the MessageId of the sent message on success.
pub struct CopyMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Unique identifier for the chat where the original message was sent (or channel username in
    /// the format @channelusername)
    pub from_chat_id: ChatId,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// New caption for media, 0-1024 characters after entities parsing. If not specified, the
    /// original caption is kept
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the new caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of special entities that appear in the new caption, which can be
    /// specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send photos. On success, the sent Message is returned.
pub struct SendPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers
    /// (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet,
    /// or upload a new photo using multipart/form-data. The photo must be at most 10 MB in size.
    /// The photo's width and height must not exceed 10000 in total. Width and height ratio must be
    /// at most 20. More information on Sending Files »
    pub photo: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Photo caption (may also be used when resending photos by file_id), 0-1024 characters after
    /// entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of special entities that appear in the caption, which can be
    /// specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the photo needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send audio files, if you want Telegram clients to display them in the music
/// player. Your audio must be in the .MP3 or .M4A format. On success, the sent Message is
/// returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed
/// in the future. For sending voice messages, use the sendVoice method instead.
pub struct SendAudio {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Audio file to send. Pass a file_id as String to send an audio file that exists on the
    /// Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio
    /// file from the Internet, or upload a new one using multipart/form-data. More information on
    /// Sending Files »
    pub audio: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Audio caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the audio caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of special entities that appear in the caption, which can be
    /// specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration of the audio in seconds
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Performer
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Track name
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
    /// supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new
    /// file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More information on Sending Files »
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send general files. On success, the sent Message is returned. Bots can
/// currently send files of any type of up to 50 MB in size, this limit may be changed in the
/// future.
pub struct SendDocument {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// File to send. Pass a file_id as String to send a file that exists on the Telegram servers
    /// (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet,
    /// or upload a new one using multipart/form-data. More information on Sending Files »
    pub document: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
    /// supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new
    /// file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More information on Sending Files »
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Document caption (may also be used when resending documents by file_id), 0-1024 characters
    /// after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the document caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of special entities that appear in the caption, which can be
    /// specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disables automatic server-side content type detection for files uploaded using
    /// multipart/form-data
    pub disable_content_type_detection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may
/// be sent as Document). On success, the sent Message is returned. Bots can currently send video
/// files of up to 50 MB in size, this limit may be changed in the future.
pub struct SendVideo {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Video to send. Pass a file_id as String to send a video that exists on the Telegram servers
    /// (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet,
    /// or upload a new video using multipart/form-data. More information on Sending Files »
    pub video: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration of sent video in seconds
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video width
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video height
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
    /// supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new
    /// file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More information on Sending Files »
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video caption (may also be used when resending videos by file_id), 0-1024 characters after
    /// entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the video caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of special entities that appear in the caption, which can be
    /// specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the video needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On
/// success, the sent Message is returned. Bots can currently send animation files of up to 50 MB
/// in size, this limit may be changed in the future.
pub struct SendAnimation {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Animation to send. Pass a file_id as String to send an animation that exists on the
    /// Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an
    /// animation from the Internet, or upload a new animation using multipart/form-data. More
    /// information on Sending Files »
    pub animation: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration of sent animation in seconds
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Animation width
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Animation height
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
    /// supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new
    /// file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More information on Sending Files »
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Animation caption (may also be used when resending animation by file_id), 0-1024 characters
    /// after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the animation caption. See formatting options for more
    /// details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of special entities that appear in the caption, which can be
    /// specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the animation needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send audio files, if you want Telegram clients to display the file as a
/// playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS
/// (other formats may be sent as Audio or Document). On success, the sent Message is returned.
/// Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the
/// future.
pub struct SendVoice {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Audio file to send. Pass a file_id as String to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the
    /// Internet, or upload a new one using multipart/form-data. More information on Sending Files
    /// »
    pub voice: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Voice message caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the voice message caption. See formatting options for more
    /// details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of special entities that appear in the caption, which can be
    /// specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration of the voice message in seconds
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// As of v.4.0, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use
/// this method to send video messages. On success, the sent Message is returned.
pub struct SendVideoNote {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Video note to send. Pass a file_id as String to send a video note that exists on the
    /// Telegram servers (recommended) or upload a new video using multipart/form-data. More
    /// information on Sending Files ». Sending video notes by a URL is currently unsupported
    pub video_note: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration of sent video in seconds
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Video width and height, i.e. diameter of the video message
    pub length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
    /// supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new
    /// file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using
    /// multipart/form-data under <file_attach_name>. More information on Sending Files »
    pub thumb: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send a group of photos, videos, documents or audios as an album. Documents
/// and audio files can be only grouped in an album with messages of the same type. On success, an
/// array of Messages that were sent is returned.
pub struct SendMediaGroup {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// A JSON-serialized array describing messages to be sent, must include 2-10 items
    pub media: Vec<InputMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends messages silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent messages from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the messages are a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send point on the map. On success, the sent Message is returned.
pub struct SendLocation {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Latitude of the location
    pub latitude: f64,
    /// Longitude of the location
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Period in seconds for which the location will be updated (see Live Locations, should be
    /// between 60 and 86400.
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
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to edit live location messages. A location can be edited until its live_period
/// expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if
/// the edited message is not an inline message, the edited Message is returned, otherwise True is
/// returned.
pub struct EditMessageLiveLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// Latitude of new location
    pub latitude: f64,
    /// Longitude of new location
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum distance for proximity alerts about approaching another chat member, in meters.
    /// Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for a new inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to stop updating a live location message before live_period expires. On
/// success, if the message is not an inline message, the edited Message is returned, otherwise
/// True is returned.
pub struct StopMessageLiveLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the message with live
    /// location to stop
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for a new inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send information about a venue. On success, the sent Message is returned.
pub struct SendVenue {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Latitude of the venue
    pub latitude: f64,
    /// Longitude of the venue
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Foursquare identifier of the venue
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
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send phone contacts. On success, the sent Message is returned.
pub struct SendContact {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
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
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send a native poll. On success, the sent Message is returned.
pub struct SendPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Poll question, 1-300 characters
    pub question: String,
    /// A JSON-serialized list of answer options, 2-10 strings 1-100 characters each
    pub options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the poll needs to be anonymous, defaults to True
    pub is_anonymous: Option<bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Poll type, “quiz” or “regular”, defaults to “regular”
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to
    /// False
    pub allows_multiple_answers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    pub correct_option_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a
    /// quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the explanation. See formatting options for more details.
    pub explanation_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of special entities that appear in the poll explanation, which can
    /// be specified instead of parse_mode
    pub explanation_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used
    /// together with close_date.
    pub open_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least
    /// 5 and no more than 600 seconds in the future. Can't be used together with open_period.
    pub close_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the poll needs to be immediately closed. This can be useful for poll preview.
    pub is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send an animated emoji that will display a random value. On success, the
/// sent Message is returned.
pub struct SendDice {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Emoji on which the dice throw animation is based. Currently, must be one of “”, “”, “”, “”,
    /// “”, or “”. Dice can have values 1-6 for “”, “” and “”, values 1-5 for “” and “”, and values
    /// 1-64 for “”. Defaults to “”
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method when you need to tell the user that something is happening on the bot's side.
/// The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients
/// clear its typing status). Returns True on success. We only recommend using this method when a
/// response from the bot will take a noticeable amount of time to arrive.
pub struct SendChatAction {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread; supergroups only
    pub message_thread_id: Option<i64>,
    /// Type of action to broadcast. Choose one, depending on what the user is about to receive:
    /// typing for text messages, upload_photo for photos, record_video or upload_video for videos,
    /// record_voice or upload_voice for voice notes, upload_document for general files,
    /// choose_sticker for stickers, find_location for location data, record_video_note or
    /// upload_video_note for video notes.
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos
/// object.
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to
    /// 100.
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get basic information about a file and prepare it for downloading. For the
/// moment, bots can download files of up to 20MB in size. On success, a File object is returned.
/// The file can then be downloaded via the link
/// https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the
/// response. It is guaranteed that the link will be valid for at least 1 hour. When the link
/// expires, a new one can be requested by calling getFile again.
pub struct GetFile {
    /// File identifier to get information about
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups
/// and channels, the user will not be able to return to the chat on their own using invite links,
/// etc., unless unbanned first. The bot must be an administrator in the chat for this to work and
/// must have the appropriate administrator rights. Returns True on success.
pub struct BanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date when the user will be unbanned, unix time. If user is banned for more than 366 days or
    /// less than 30 seconds from the current time they are considered to be banned forever.
    /// Applied for supergroups and channels only.
    pub until_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True to delete all messages from the chat for the user that is being removed. If
    /// False, the user will be able to see messages in the group that were sent before the user
    /// was removed. Always True for supergroups and channels.
    pub revoke_messages: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to unban a previously banned user in a supergroup or channel. The user will not
/// return to the group or channel automatically, but will be able to join via link, etc. The bot
/// must be an administrator for this to work. By default, this method guarantees that after the
/// call the user is not a member of the chat, but will be able to join it. So if the user is a
/// member of the chat they will also be removed from the chat. If you don't want this, use the
/// parameter only_if_banned. Returns True on success.
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Do nothing if the user is not banned
    pub only_if_banned: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the
/// supergroup for this to work and must have the appropriate administrator rights. Pass True for
/// all permissions to lift restrictions from a user. Returns True on success.
pub struct RestrictChatMember {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date when restrictions will be lifted for the user, unix time. If user is restricted for
    /// more than 366 days or less than 30 seconds from the current time, they are considered to be
    /// restricted forever
    pub until_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an
/// administrator in the chat for this to work and must have the appropriate administrator rights.
/// Pass False for all boolean parameters to demote a user. Returns True on success.
pub struct PromoteChatMember {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the administrator's presence in the chat is hidden
    pub is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the administrator can access the chat event log, chat statistics, message
    /// statistics in channels, see channel members, see anonymous administrators in supergroups
    /// and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the administrator can create channel posts, channels only
    pub can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the administrator can edit messages of other users and can pin messages,
    /// channels only
    pub can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the administrator can delete messages of other users
    pub can_delete_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the administrator can manage video chats
    pub can_manage_video_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the administrator can add new administrators with a subset of their own
    /// privileges or demote administrators that they have promoted, directly or indirectly
    /// (promoted by administrators that were appointed by him)
    pub can_promote_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the administrator can change chat title, photo and other settings
    pub can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the administrator can invite new users to the chat
    pub can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the administrator can pin messages, supergroups only
    pub can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the user is allowed to create, rename, close, and reopen forum topics,
    /// supergroups only
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot.
/// Returns True on success.
pub struct SetChatAdministratorCustomTitle {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is unbanned,
/// the owner of the banned chat won't be able to send messages on behalf of any of their channels.
/// The bot must be an administrator in the supergroup or channel for this to work and must have
/// the appropriate administrator rights. Returns True on success.
pub struct BanChatSenderChat {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot
/// must be an administrator for this to work and must have the appropriate administrator rights.
/// Returns True on success.
pub struct UnbanChatSenderChat {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to set default chat permissions for all members. The bot must be an
/// administrator in the group or a supergroup for this to work and must have the
/// can_restrict_members administrator rights. Returns True on success.
pub struct SetChatPermissions {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
    /// A JSON-serialized object for new default chat permissions
    pub permissions: ChatPermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to generate a new primary invite link for a chat; any previously generated
/// primary link is revoked. The bot must be an administrator in the chat for this to work and must
/// have the appropriate administrator rights. Returns the new invite link as String on success.
pub struct ExportChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to create an additional invite link for a chat. The bot must be an
/// administrator in the chat for this to work and must have the appropriate administrator rights.
/// The link can be revoked using the method revokeChatInviteLink. Returns the new invite link as
/// ChatInviteLink object.
pub struct CreateChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Invite link name; 0-32 characters
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Point in time (Unix timestamp) when the link will expire
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum number of users that can be members of the chat simultaneously after joining
    /// the chat via this invite link; 1-99999
    pub member_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if users joining the chat via the link need to be approved by chat administrators. If
    /// True, member_limit can't be specified
    pub creates_join_request: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to edit a non-primary invite link created by the bot. The bot must be an
/// administrator in the chat for this to work and must have the appropriate administrator rights.
/// Returns the edited invite link as a ChatInviteLink object.
pub struct EditChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    /// The invite link to edit
    pub invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Invite link name; 0-32 characters
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Point in time (Unix timestamp) when the link will expire
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum number of users that can be members of the chat simultaneously after joining
    /// the chat via this invite link; 1-99999
    pub member_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// True, if users joining the chat via the link need to be approved by chat administrators. If
    /// True, member_limit can't be specified
    pub creates_join_request: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a
/// new link is automatically generated. The bot must be an administrator in the chat for this to
/// work and must have the appropriate administrator rights. Returns the revoked invite link as
/// ChatInviteLink object.
pub struct RevokeChatInviteLink {
    /// Unique identifier of the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    /// The invite link to revoke
    pub invite_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to approve a chat join request. The bot must be an administrator in the chat
/// for this to work and must have the can_invite_users administrator right. Returns True on
/// success.
pub struct ApproveChatJoinRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to decline a chat join request. The bot must be an administrator in the chat
/// for this to work and must have the can_invite_users administrator right. Returns True on
/// success.
pub struct DeclineChatJoinRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to set a new profile photo for the chat. Photos can't be changed for private
/// chats. The bot must be an administrator in the chat for this to work and must have the
/// appropriate administrator rights. Returns True on success.
pub struct SetChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    /// New chat photo, uploaded using multipart/form-data
    pub photo: InputFile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must
/// be an administrator in the chat for this to work and must have the appropriate administrator
/// rights. Returns True on success.
pub struct DeleteChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to change the title of a chat. Titles can't be changed for private chats. The
/// bot must be an administrator in the chat for this to work and must have the appropriate
/// administrator rights. Returns True on success.
pub struct SetChatTitle {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    /// New chat title, 1-128 characters
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to change the description of a group, a supergroup or a channel. The bot must
/// be an administrator in the chat for this to work and must have the appropriate administrator
/// rights. Returns True on success.
pub struct SetChatDescription {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// New chat description, 0-255 characters
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a
/// private chat, the bot must be an administrator in the chat for this to work and must have the
/// 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator
/// right in a channel. Returns True on success.
pub struct PinChatMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    /// Identifier of a message to pin
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if it is not necessary to send a notification to all chat members about the new
    /// pinned message. Notifications are always disabled in channels and private chats.
    pub disable_notification: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to remove a message from the list of pinned messages in a chat. If the chat is
/// not a private chat, the bot must be an administrator in the chat for this to work and must have
/// the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator
/// right in a channel. Returns True on success.
pub struct UnpinChatMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifier of a message to unpin. If not specified, the most recent pinned message (by
    /// sending date) will be unpinned.
    pub message_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to clear the list of pinned messages in a chat. If the chat is not a private
/// chat, the bot must be an administrator in the chat for this to work and must have the
/// 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator
/// right in a channel. Returns True on success.
pub struct UnpinAllChatMessages {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
pub struct LeaveChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get up to date information about the chat (current name of the user for one-
/// on-one conversations, current username of a user, group or channel, etc.). Returns a Chat
/// object on success.
pub struct GetChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array
/// of ChatMember objects.
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get the number of members in a chat. Returns Int on success.
pub struct GetChatMemberCount {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get information about a member of a chat. The method is guaranteed to work
/// for other users, only if the bot is an administrator in the chat. Returns a ChatMember object
/// on success.
pub struct GetChatMember {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in
    /// the format @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to set a new group sticker set for a supergroup. The bot must be an
/// administrator in the chat for this to work and must have the appropriate administrator rights.
/// Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot
/// can use this method. Returns True on success.
pub struct SetChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to delete a group sticker set from a supergroup. The bot must be an
/// administrator in the chat for this to work and must have the appropriate administrator rights.
/// Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot
/// can use this method. Returns True on success.
pub struct DeleteChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any
/// user. Requires no parameters. Returns an Array of Sticker objects.
pub struct GetForumTopicIconStickers {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator
/// in the chat for this to work and must have the can_manage_topics administrator rights. Returns
/// information about the created topic as a ForumTopic object.
pub struct CreateForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
    /// Topic name, 1-128 characters
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0),
    /// 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or
    /// 16478047 (0xFB6F5F)
    pub icon_color: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier of the custom emoji shown as the topic icon. Use
    /// getForumTopicIconStickers to get all allowed custom emoji identifiers.
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an
/// administrator in the chat for this to work and must have can_manage_topics administrator
/// rights, unless it is the creator of the topic. Returns True on success.
pub struct EditForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// New topic name, 0-128 characters. If not specified or empty, the current name of the topic
    /// will be kept
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// New unique identifier of the custom emoji shown as the topic icon. Use
    /// getForumTopicIconStickers to get all allowed custom emoji identifiers. Pass an empty string
    /// to remove the icon. If not specified, the current icon will be kept
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to close an open topic in a forum supergroup chat. The bot must be an
/// administrator in the chat for this to work and must have the can_manage_topics administrator
/// rights, unless it is the creator of the topic. Returns True on success.
pub struct CloseForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an
/// administrator in the chat for this to work and must have the can_manage_topics administrator
/// rights, unless it is the creator of the topic. Returns True on success.
pub struct ReopenForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to delete a forum topic along with all its messages in a forum supergroup chat.
/// The bot must be an administrator in the chat for this to work and must have the
/// can_delete_messages administrator rights. Returns True on success.
pub struct DeleteForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to clear the list of pinned messages in a forum topic. The bot must be an
/// administrator in the chat for this to work and must have the can_pin_messages administrator
/// right in the supergroup. Returns True on success.
pub struct UnpinAllForumTopicMessages {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot
/// must be an administrator in the chat for this to work and must have can_manage_topics
/// administrator rights. Returns True on success.
pub struct EditGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
    /// New topic name, 1-128 characters
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an
/// administrator in the chat for this to work and must have the can_manage_topics administrator
/// rights. Returns True on success.
pub struct CloseGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be
/// an administrator in the chat for this to work and must have the can_manage_topics administrator
/// rights. The topic will be automatically unhidden if it was hidden. Returns True on success.
pub struct ReopenGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an
/// administrator in the chat for this to work and must have the can_manage_topics administrator
/// rights. The topic will be automatically closed if it was open. Returns True on success.
pub struct HideGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an
/// administrator in the chat for this to work and must have the can_manage_topics administrator
/// rights. Returns True on success.
pub struct UnhideGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format
    /// @supergroupusername)
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send answers to callback queries sent from inline keyboards. The answer will
/// be displayed to the user as a notification at the top of the chat screen or as an alert. On
/// success, True is returned.
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered
    pub callback_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200
    /// characters
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If True, an alert will be shown by the client instead of a notification at the top of the
    /// chat screen. Defaults to false.
    pub show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// URL that will be opened by the user's client. If you have created a Game and accepted the
    /// conditions via @BotFather, specify the URL that opens your game - note that this will only
    /// work if the query comes from a callback_game button.Otherwise, you may use links like
    /// t.me/your_bot?start=XXXX that open your bot with a parameter.
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum amount of time in seconds that the result of the callback query may be cached
    /// client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    pub cache_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to change the list of the bot's commands. See this manual for more details
/// about bot commands. Returns True on success.
pub struct SetMyCommands {
    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most
    /// 100 commands can be specified.
    pub commands: Vec<BotCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object, describing scope of users for which the commands are relevant.
    /// Defaults to BotCommandScopeDefault.
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from
    /// the given scope, for whose language there are no dedicated commands
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to delete the list of the bot's commands for the given scope and user language.
/// After deletion, higher level commands will be shown to affected users. Returns True on success.
pub struct DeleteMyCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object, describing scope of users for which the commands are relevant.
    /// Defaults to BotCommandScopeDefault.
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from
    /// the given scope, for whose language there are no dedicated commands
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get the current list of the bot's commands for the given scope and user
/// language. Returns an Array of BotCommand objects. If commands aren't set, an empty list is
/// returned.
pub struct GetMyCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object, describing scope of users. Defaults to BotCommandScopeDefault.
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A two-letter ISO 639-1 language code or an empty string
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to change the bot's menu button in a private chat, or the default menu button.
/// Returns True on success.
pub struct SetChatMenuButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target private chat. If not specified, default bot's menu button
    /// will be changed
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for the bot's new menu button. Defaults to MenuButtonDefault
    pub menu_button: Option<MenuButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get the current value of the bot's menu button in a private chat, or the
/// default menu button. Returns MenuButton on success.
pub struct GetChatMenuButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target private chat. If not specified, default bot's menu button
    /// will be returned
    pub chat_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to change the default administrator rights requested by the bot when it's added
/// as an administrator to groups or channels. These rights will be suggested to users, but they
/// are are free to modify the list before adding the bot. Returns True on success.
pub struct SetMyDefaultAdministratorRights {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object describing new default administrator rights. If not specified, the
    /// default administrator rights will be cleared.
    pub rights: Option<ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True to change the default administrator rights of the bot in channels. Otherwise, the
    /// default administrator rights of the bot for groups and supergroups will be changed.
    pub for_channels: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get the current default administrator rights of the bot. Returns
/// ChatAdministratorRights on success.
pub struct GetMyDefaultAdministratorRights {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True to get default administrator rights of the bot in channels. Otherwise, default
    /// administrator rights of the bot for groups and supergroups will be returned.
    pub for_channels: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to edit text and game messages. On success, if the edited message is not an
/// inline message, the edited Message is returned, otherwise True is returned.
pub struct EditMessageText {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// New text of the message, 1-4096 characters after entities parsing
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the message text. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of special entities that appear in message text, which can be
    /// specified instead of parse_mode
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disables link previews for links in this message
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for an inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to edit captions of messages. On success, if the edited message is not an
/// inline message, the edited Message is returned, otherwise True is returned.
pub struct EditMessageCaption {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// New caption of the message, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode for parsing entities in the message caption. See formatting options for more details.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized list of special entities that appear in the caption, which can be
    /// specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for an inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to edit animation, audio, document, photo, or video messages. If a message is
/// part of a message album, then it can be edited only to an audio for audio albums, only to a
/// document for document albums and to a photo or a video otherwise. When an inline message is
/// edited, a new file can't be uploaded; use a previously uploaded file via its file_id or specify
/// a URL. On success, if the edited message is not an inline message, the edited Message is
/// returned, otherwise True is returned.
pub struct EditMessageMedia {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new media content of the message
    pub media: InputMedia,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for a new inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to edit only the reply markup of messages. On success, if the edited message is
/// not an inline message, the edited Message is returned, otherwise True is returned.
pub struct EditMessageReplyMarkup {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for an inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll is
/// returned.
pub struct StopPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    /// Identifier of the original message with the poll
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for a new message inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to delete a message, including service messages, with the following
/// limitations:- A message can only be deleted if it was sent less than 48 hours ago.- Service
/// messages about a supergroup, channel, or forum topic creation can't be deleted.- A dice message
/// in a private chat can only be deleted if it was sent more than 24 hours ago.- Bots can delete
/// outgoing messages in private chats, groups, and supergroups.- Bots can delete incoming messages
/// in private chats.- Bots granted can_post_messages permissions can delete outgoing messages in
/// channels.- If the bot is an administrator of a group, it can delete any message there.- If the
/// bot has can_delete_messages permission in a supergroup or a channel, it can delete any message
/// there.Returns True on success.
pub struct DeleteMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    /// Identifier of the message to delete
    pub message_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send static .WEBP, animated .TGS, or video .WEBM stickers. On success, the
/// sent Message is returned.
pub struct SendSticker {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Sticker to send. Pass a file_id as String to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP file from
    /// the Internet, or upload a new one using multipart/form-data. More information on Sending
    /// Files »
    pub sticker: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply
    /// keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get a sticker set. On success, a StickerSet object is returned.
pub struct GetStickerSet {
    /// Name of the sticker set
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get information about custom emoji stickers by their identifiers. Returns an
/// Array of Sticker objects.
pub struct GetCustomEmojiStickers {
    /// List of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.
    pub custom_emoji_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to upload a .PNG file with a sticker for later use in createNewStickerSet and
/// addStickerToSet methods (can be used multiple times). Returns the uploaded File on success.
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    pub user_id: i64,
    /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed
    /// 512px, and either width or height must be exactly 512px. More information on Sending Files
    /// »
    pub png_sticker: InputFile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to create a new sticker set owned by a user. The bot will be able to edit the
/// sticker set thus created. You must use exactly one of the fields png_sticker, tgs_sticker, or
/// webm_sticker. Returns True on success.
pub struct CreateNewStickerSet {
    /// User identifier of created sticker set owner
    pub user_id: i64,
    /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals). Can
    /// contain only English letters, digits and underscores. Must begin with a letter, can't
    /// contain consecutive underscores and must end in "_by_<bot_username>". <bot_username> is
    /// case insensitive. 1-64 characters.
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed
    /// 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send
    /// a file that already exists on the Telegram servers, pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using multipart/form-data.
    /// More information on Sending Files »
    pub png_sticker: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// TGS animation with the sticker, uploaded using multipart/form-data. See
    /// https://core.telegram.org/stickers#animated-sticker-requirements for technical requirements
    pub tgs_sticker: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// WEBM video with the sticker, uploaded using multipart/form-data. See
    /// https://core.telegram.org/stickers#video-sticker-requirements for technical requirements
    pub webm_sticker: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Type of stickers in the set, pass “regular” or “mask”. Custom emoji sticker sets can't be
    /// created via the Bot API at the moment. By default, a regular sticker set is created.
    pub sticker_type: Option<String>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for position where the mask should be placed on faces
    pub mask_position: Option<MaskPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to add a new sticker to a set created by the bot. You must use exactly one of
/// the fields png_sticker, tgs_sticker, or webm_sticker. Animated stickers can be added to
/// animated sticker sets and only to them. Animated sticker sets can have up to 50 stickers.
/// Static sticker sets can have up to 120 stickers. Returns True on success.
pub struct AddStickerToSet {
    /// User identifier of sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed
    /// 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send
    /// a file that already exists on the Telegram servers, pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using multipart/form-data.
    /// More information on Sending Files »
    pub png_sticker: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// TGS animation with the sticker, uploaded using multipart/form-data. See
    /// https://core.telegram.org/stickers#animated-sticker-requirements for technical requirements
    pub tgs_sticker: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// WEBM video with the sticker, uploaded using multipart/form-data. See
    /// https://core.telegram.org/stickers#video-sticker-requirements for technical requirements
    pub webm_sticker: Option<InputFile>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for position where the mask should be placed on faces
    pub mask_position: Option<MaskPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to move a sticker in a set created by the bot to a specific position. Returns
/// True on success.
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    pub sticker: String,
    /// New sticker position in the set, zero-based
    pub position: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to delete a sticker from a set created by the bot. Returns True on success.
pub struct DeleteStickerFromSet {
    /// File identifier of the sticker
    pub sticker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to set the thumbnail of a sticker set. Animated thumbnails can be set for
/// animated sticker sets only. Video thumbnails can be set only for video sticker sets only.
/// Returns True on success.
pub struct SetStickerSetThumb {
    /// Sticker set name
    pub name: String,
    /// User identifier of the sticker set owner
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A PNG image with the thumbnail, must be up to 128 kilobytes in size and have width and
    /// height exactly 100px, or a TGS animation with the thumbnail up to 32 kilobytes in size; see
    /// https://core.telegram.org/stickers#animated-sticker-requirements for animated sticker
    /// technical requirements, or a WEBM video with the thumbnail up to 32 kilobytes in size; see
    /// https://core.telegram.org/stickers#video-sticker-requirements for video sticker technical
    /// requirements. Pass a file_id as a String to send a file that already exists on the Telegram
    /// servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or
    /// upload a new one using multipart/form-data. More information on Sending Files ». Animated
    /// sticker set thumbnails can't be uploaded via HTTP URL.
    pub thumb: Option<InputFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send answers to an inline query. On success, True is returned.No more than
/// 50 results per query are allowed.
pub struct AnswerInlineQuery {
    /// Unique identifier for the answered query
    pub inline_query_id: String,
    /// A JSON-serialized array of results for the inline query
    pub results: Vec<InlineQueryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum amount of time in seconds that the result of the inline query may be cached on
    /// the server. Defaults to 300.
    pub cache_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if results may be cached on the server side only for the user that sent the
    /// query. By default, results may be returned to any user who sends the same query
    pub is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass the offset that a client should send in the next query with the same text to receive
    /// more results. Pass an empty string if there are no more results or if you don't support
    /// pagination. Offset length can't exceed 64 bytes.
    pub next_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If passed, clients will display a button with specified text that switches the user to a
    /// private chat with the bot and sends the bot a start message with the parameter
    /// switch_pm_parameter
    pub switch_pm_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Deep-linking parameter for the /start message sent to the bot when user presses the switch
    /// button. 1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed.Example: An inline bot
    /// that sends YouTube videos can ask the user to connect the bot to their YouTube account to
    /// adapt search results accordingly. To do this, it displays a 'Connect your YouTube account'
    /// button above the results, or even before showing any. The user presses the button, switches
    /// to a private chat with the bot and, in doing so, passes a start parameter that instructs
    /// the bot to return an OAuth link. Once done, the bot can offer a switch_inline button so
    /// that the user can easily return to the chat where they wanted to use the bot's inline
    /// capabilities.
    pub switch_pm_parameter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to set the result of an interaction with a Web App and send a corresponding
/// message on behalf of the user to the chat from which the query originated. On success, a
/// SentWebAppMessage object is returned.
pub struct AnswerWebAppQuery {
    /// Unique identifier for the query to be answered
    pub web_app_query_id: String,
    /// A JSON-serialized object describing the message to be sent
    pub result: InlineQueryResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send invoices. On success, the sent Message is returned.
pub struct SendInvoice {
    /// Unique identifier for the target chat or username of the target channel (in the format
    /// @channelusername)
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
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
    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency
    /// (integer, not float/double). At most 4 suggested tip amounts can be specified. The
    /// suggested tip amounts must be positive, passed in a strictly increased order and must not
    /// exceed max_tip_amount.
    pub suggested_tip_amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique deep-linking parameter. If left empty, forwarded copies of the sent message will
    /// have a Pay button, allowing multiple users to pay directly from the forwarded message,
    /// using the same invoice. If non-empty, forwarded copies of the sent message will have a URL
    /// button with a deep link to the bot (instead of a Pay button), with the value used as the
    /// start parameter
    pub start_parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A
    /// detailed description of required fields should be provided by the payment provider.
    pub provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image
    /// for a service. People like it better when they see what they are paying for.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button
    /// will be shown. If not empty, the first button must be a Pay button.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to create a link for an invoice. Returns the created invoice link as String on
/// success.
pub struct CreateInvoiceLink {
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for
    /// your internal processes.
    pub payload: String,
    /// Payment provider token, obtained via BotFather
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
    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency
    /// (integer, not float/double). At most 4 suggested tip amounts can be specified. The
    /// suggested tip amounts must be positive, passed in a strictly increased order and must not
    /// exceed max_tip_amount.
    pub suggested_tip_amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A
    /// detailed description of required fields should be provided by the payment provider.
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
    /// Pass True if the user's phone number should be sent to the provider
    pub send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the user's email address should be sent to the provider
    pub send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the final price depends on the shipping method
    pub is_flexible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// If you sent an invoice requesting a shipping address and the parameter is_flexible was
/// specified, the Bot API will send an Update with a shipping_query field to the bot. Use this
/// method to reply to shipping queries. On success, True is returned.
pub struct AnswerShippingQuery {
    /// Unique identifier for the query to be answered
    pub shipping_query_id: String,
    /// Pass True if delivery to the specified address is possible and False if there are any
    /// problems (for example, if delivery to the specified address is not possible)
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if ok is True. A JSON-serialized array of available shipping options.
    pub shipping_options: Option<Vec<ShippingOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if ok is False. Error message in human readable form that explains why it is
    /// impossible to complete the order (e.g. "Sorry, delivery to your desired address is
    /// unavailable'). Telegram will display this message to the user.
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Once the user has confirmed their payment and shipping details, the Bot API sends the final
/// confirmation in the form of an Update with the field pre_checkout_query. Use this method to
/// respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must
/// receive an answer within 10 seconds after the pre-checkout query was sent.
pub struct AnswerPreCheckoutQuery {
    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,
    /// Specify True if everything is alright (goods are available, etc.) and the bot is ready to
    /// proceed with the order. Use False if there are any problems.
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if ok is False. Error message in human readable form that explains the reason for
    /// failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our
    /// amazing black T-shirts while you were busy filling out your payment details. Please choose
    /// a different color or garment!"). Telegram will display this message to the user.
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Informs a user that some of the Telegram Passport elements they provided contains errors. The
/// user will not be able to re-submit their Passport to you until the errors are fixed (the
/// contents of the field for which you returned the error must change). Returns True on success.
/// Use this if the data submitted by the user doesn't satisfy the standards your service requires
/// for any reason. For example, if a birthday date seems invalid, a submitted document is blurry,
/// a scan shows evidence of tampering, etc. Supply some details in the error message to make sure
/// the user knows how to correct the issues.
pub struct SetPassportDataErrors {
    /// User identifier
    pub user_id: i64,
    /// A JSON-serialized array describing the errors
    pub errors: Vec<PassportElementError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to send a game. On success, the sent Message is returned.
pub struct SendGame {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups
    /// only
    pub message_thread_id: Option<i64>,
    /// Short name of the game, serves as the unique identifier for the game. Set up your games via
    /// @BotFather.
    pub game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the message should be sent even if the specified replied-to message is not
    /// found
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Play game_title' button
    /// will be shown. If not empty, the first button must launch the game.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to set the score of the specified user in a game message. On success, if the
/// message is not an inline message, the Message is returned, otherwise True is returned. Returns
/// an error, if the new score is not greater than the user's current score in the chat and force
/// is False.
pub struct SetGameScore {
    /// User identifier
    pub user_id: i64,
    /// New score, must be non-negative
    pub score: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the high score is allowed to decrease. This can be useful when fixing mistakes
    /// or banning cheaters
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pass True if the game message should not be automatically edited to include the current
    /// scoreboard
    pub disable_edit_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the sent message
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Use this method to get data for high score tables. Will return the score of the specified user
/// and several of their neighbors in a game. Returns an Array of GameHighScore objects.
pub struct GetGameHighScores {
    /// Target user id
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the sent message
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
}

