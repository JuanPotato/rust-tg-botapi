extern crate tg_botapi;

use tg_botapi::{args, BotApi, BotError, BotResult, types};
use types::Message;

use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

fn main() {
    let token = &env::var("TOKEN")
        .expect("No bot token provided, please set the environment variable TOKEN");
    let bot = Arc::new(BotApi::new_debug(token));

    let me_irl = bot.get_me().expect("Could not establish a connection :\\");

    let mut update_args = args::GetUpdatesBuilder::default()
        .timeout(600)
        .offset(0)
        .limit(None)
        .allowed_updates(None)
        .build()
        .unwrap();

    'update_loop: loop {
        let updates = bot.get_updates(&update_args).unwrap();

        for update in updates {
            update_args.offset = Some(update.update_id + 1);

            if let Some(message) = update.message {
                // let from = message.from.unwrap();

                let message_text = message.text.as_ref().map_or(String::new(), |s| s.clone());
                let mut split_text = message_text.split_whitespace();

                if let Some(cmd) = split_text.next() {
                    let chat_id = ChatId::Id(message.chat.id);
                    let msg_id = message.message_id;

                    match cmd {
                        "/exit" => {
                            let _ = cmd::exit(&bot, &message);
                            break 'update_loop;
                        }
                        "/start" | "/help" => {
                            let _ = cmd::help(&bot, &message);
                        }
                        "/photo" => {
                            let _ = cmd::photo(&bot, &message);
                        }
                        "/edit" => {
                            let _ = cmd::edit(&bot, &message, split_text.next().map(String::from));
                        }
                        "/thread" | "/threads" => {
                            let bot1 = bot.clone();
                            let bot2 = bot.clone();
                            let bot3 = bot.clone();

                            thread::spawn(move || {
                                let args = args::SendMessageBuilder::default()
                                    .text("Thread 1")
                                    .chat_id(chat_id)
                                    .reply_to_message_id(msg_id)
                                    .parse_mode(None)
                                    .disable_web_page_preview(None)
                                    .disable_notification(None)
                                    .reply_markup(None)
                                    .build()
                                    .unwrap();
                                let _ = bot1.send_message(&args);
                            });
                            thread::spawn(move || {
                                let args = args::SendMessageBuilder::default()
                                    .text("Thread 2")
                                    .chat_id(chat_id)
                                    .reply_to_message_id(msg_id)
                                    .parse_mode(None)
                                    .disable_web_page_preview(None)
                                    .disable_notification(None)
                                    .reply_markup(None)
                                    .build()
                                    .unwrap();
                                let _ = bot2.send_message(&args);
                            });
                            thread::spawn(move || {
                                let args = args::SendMessageBuilder::default()
                                    .text("Thread 3")
                                    .chat_id(chat_id)
                                    .reply_to_message_id(msg_id)
                                    .parse_mode(None)
                                    .disable_web_page_preview(None)
                                    .disable_notification(None)
                                    .reply_markup(None)
                                    .build()
                                    .unwrap();
                                let _ = bot3.send_message(&args);
                            });
                        }
                        "/button" => {
                            let _ = cmd::button(&bot, &message);
                        }
                        "/inline" => {
                            let _ = cmd::inline(&bot, &message);
                        }
                        "/clear" | "No" => {
                            let _ = cmd::clear(&bot, &message);
                        }
                        _ => {}
                    }
                }

                if let Some(new_chat_member) = message.new_chat_member {
                    if new_chat_member.id == me_irl.id {
                        let chat_id = ChatId::Id(message.chat.id);
                        let text = "Hi, thanks for adding me to this group, but I don't want to \
                                    be here.\nSee ya!";
                        let msg_args = args::SendMessageBuilder::default()
                            .text(text)
                            .chat_id(message.chat.id)
                            .parse_mode(None)
                            .disable_web_page_preview(None)
                            .disable_notification(None)
                            .reply_to_message_id(None)
                            .reply_markup(None)
                            .build()
                            .unwrap();
                        let _ = bot.send_message(&msg_args);
                        let leave_args = args::LeaveChat::new(message.chat.id.into());
                        let _ = bot.leave_chat(&leave_args);
                    }
                }
            }

            if let Some(inline_query) = update.inline_query {
                let lenny_txt = format!("{} {}", inline_query.query, "( ͡° ͜ʖ ͡°)");
                let shrug_txt = format!("{} {}", inline_query.query, "¯\\_(ツ)_/¯");
                let lenny = types::InputTextMessageContent::new(lenny_txt.clone());
                let shrug = types::InputTextMessageContent::new(shrug_txt.clone());
                let results = vec![types::InlineQueryResultArticleBuilder::default()
                                       .id("lenny")
                                       .title(lenny_txt)
                                       .input_message_content(lenny)
                                       .reply_markup(None)
                                       .url(None)
                                       .hide_url(None)
                                       .description(None)
                                       .thumb_url(None)
                                       .thumb_width(None)
                                       .thumb_height(None)
                                       .build()
                                       .unwrap()
                                       .into(),
                                   types::InlineQueryResultArticleBuilder::default()
                                       .id("shrug")
                                       .title(shrug_txt)
                                       .input_message_content(shrug)
                                       .reply_markup(None)
                                       .url(None)
                                       .hide_url(None)
                                       .description(None)
                                       .thumb_url(None)
                                       .thumb_width(None)
                                       .thumb_height(None)
                                       .build()
                                       .unwrap()
                                       .into()];

                let _ =
                    bot.answer_inline_query(&args::AnswerInlineQuery::new(inline_query.id,
                                                                          results));
            }
        }
    }
    update_args.limit = Some(0);
    update_args.timeout = Some(0);
    let _ = bot.get_updates(&update_args);
    // Alright, so if you ever decide to have a function that terminates
    // your bot, make sure you have a check at the beginning of the loop
    // that makes sure you aren't processing old messages. You could also
    // just make a getUpdates at the end of the execution that just uses
    // the latest offset. This prevents you from having to reread any
    // updates that were in the update array you got that had the terminate
    // command. Because telegram only will stop sending you the update
    // after you have used an offset greater than its. So if you never make
    // another getUpdates, you will boot up, and terminate, forever. :)
}

mod cmd {
    use super::*;

    pub fn exit(bot: &BotApi, message: &Message) -> BotResult {
        let args = args::SendMessageBuilder::default()
            .text("Goodbye!")
            .chat_id(message.chat.id)
            .reply_to_message_id(message.message_id)
            .parse_mode(None)
            .disable_web_page_preview(None)
            .disable_notification(None)
            .reply_markup(None)
            .build()
            .unwrap();
        bot.send_message(&args)
    }

    pub fn help(bot: &BotApi, message: &Message) -> BotResult {
        let args = args::SendMessageBuilder::default()
            .text("Hi, I'm a bot!")
            .chat_id(message.chat.id)
            .reply_to_message_id(message.message_id)
            .parse_mode(None)
            .disable_web_page_preview(None)
            .disable_notification(None)
            .reply_markup(None)
            .build()
            .unwrap();
        bot.send_message(&args)
    }

    pub fn photo(bot: &BotApi, message: &Message) -> BotResult {
        let args = args::SendPhotoBuilder::default()
            .chat_id(message.chat.id)
            .reply_to_message_id(message.message_id)
            .photo(PathBuf::from("../multi_photo.png"))
            .file_id(None)
            .caption(None)
            .disable_notification(None)
            .reply_markup(None)
            .build()
            .unwrap();
        bot.send_photo(&args)
    }

    pub fn edit(bot: &BotApi,
                message: &Message,
                edit_text: Option<String>)
                -> Result<types::MessageOrBool, BotError> {
        let args = args::SendMessageBuilder::default()
            .text("Editing")
            .chat_id(message.chat.id)
            .reply_to_message_id(message.message_id)
            .parse_mode(None)
            .disable_web_page_preview(None)
            .disable_notification(None)
            .reply_markup(None)
            .build()
            .unwrap();
        bot.send_message(&args).and_then(move |sent_message| {
            let mut edit_args = args::EditMessageTextBuilder::default()
                .text("Edited")
                .chat_id(message.chat.id)
                .message_id(sent_message.message_id)
                .parse_mode(String::from("Markdown"))
                .inline_message_id(None)
                .disable_web_page_preview(None)
                .reply_markup(None)
                .build()
                .unwrap();
            if let Some(arg) = edit_text {
                edit_args.text = arg;
            }
            bot.edit_message_text(&edit_args)
        })
    }

    pub fn button(bot: &BotApi, message: &Message) -> BotResult {
        let keyboard =
            types::ReplyKeyboardMarkup::new(vec![vec![types::KeyboardButton::new("Yes".into()),
                                                      types::KeyboardButton::new("No".into())],
                                                 vec![types::KeyboardButton::new("Eh".into()),
                                                      types::KeyboardButton::new("He".into())]]);

        let args = args::SendMessageBuilder::default()
            .text("Yes or No?")
            .chat_id(message.chat.id)
            .reply_markup(types::ReplyMarkup::from(keyboard))
            .parse_mode(None)
            .disable_web_page_preview(None)
            .disable_notification(None)
            .reply_to_message_id(None)
            .build()
            .unwrap();

        bot.send_message(&args)
    }

    pub fn inline(bot: &BotApi, message: &Message) -> BotResult {
        let keyboard = types::InlineKeyboardMarkup::new(vec![vec![
                types::InlineKeyboardButtonBuilder::default()
                    .text("Some")
                    .url(String::from("https://www.youtube.com/watch?v=L_jWHffIx5E"))
                    .callback_data(None)
                    .switch_inline_query(None)
                    .switch_inline_query_current_chat(None)
                    .callback_game(None)
                    .build()
                    .unwrap(),
                types::InlineKeyboardButtonBuilder::default()
                    .text("Body")
                    .url(String::from("https://www.youtube.com/watch?v=rlYys58hsCU"))
                    .callback_data(None)
                    .switch_inline_query(None)
                    .switch_inline_query_current_chat(None)
                    .callback_game(None)
                    .build()
                    .unwrap()
            ],
                                                             vec![
                types::InlineKeyboardButtonBuilder::default()
                    .text("Once")
                    .url(String::from("https://www.youtube.com/watch?v=Q-MizNywQ94"))
                    .callback_data(None)
                    .switch_inline_query(None)
                    .switch_inline_query_current_chat(None)
                    .callback_game(None)
                    .build()
                    .unwrap(),
                types::InlineKeyboardButtonBuilder::default()
                    .text("Told")
                    .url(String::from("https://www.youtube.com/watch?v=J48dqyz_C6s"))
                    .callback_data(None)
                    .switch_inline_query(None)
                    .switch_inline_query_current_chat(None)
                    .callback_game(None)
                    .build()
                    .unwrap()
            ]]);
        let args = args::SendMessageBuilder::default()
            .text("Me")
            .chat_id(message.chat.id)
            .reply_markup(Some(keyboard.into()))
            .parse_mode(None)
            .disable_web_page_preview(None)
            .disable_notification(None)
            .reply_to_message_id(None)
            .build()
            .unwrap();

        bot.send_message(&args)
    }

    pub fn clear(bot: &BotApi, message: &Message) -> BotResult {
        let args = args::SendMessageBuilder::default()
            .text("Me too")
            .chat_id(message.chat.id)
            .reply_markup(Some(types::ReplyKeyboardRemoveMarkup::new(true).into()))
            .parse_mode(None)
            .disable_web_page_preview(None)
            .disable_notification(None)
            .reply_to_message_id(None)
            .build()
            .unwrap();
        bot.send_message(&args)
    }
}
