extern crate tg_botapi;

use tg_botapi::{args, types, BotApi, BotError};
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
                    let chat_id = message.chat.id;
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
                                    .build()
                                    .unwrap();
                                let _ = bot1.send_message(&args);
                            });

                            thread::spawn(move || {
                                let args = args::SendMessageBuilder::default()
                                    .text("Thread 2")
                                    .chat_id(chat_id)
                                    .reply_to_message_id(msg_id)
                                    .build()
                                    .unwrap();
                                let _ = bot2.send_message(&args);
                            });

                            thread::spawn(move || {
                                let args = args::SendMessageBuilder::default()
                                    .text("Thread 3")
                                    .chat_id(chat_id)
                                    .reply_to_message_id(msg_id)
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

                if let Some(new_chat_members) = message.new_chat_members {
                    for new_chat_member in new_chat_members {
                        if new_chat_member.id == me_irl.id {
                            let text = "Hi, thanks for adding me to this group, \
                                        but I don't want to be here.\nSee ya!";

                            let msg_args = args::SendMessageBuilder::default()
                                .text(text)
                                .chat_id(message.chat.id)
                                .build()
                                .unwrap();
                            let _ = bot.send_message(&msg_args);
                            let leave_args = args::LeaveChatBuilder::default()
                                .chat_id(message.chat.id)
                                .build()
                                .unwrap();

                            let _ = bot.leave_chat(&leave_args);
                        }
                    }
                }
            }

            if let Some(inline_query) = update.inline_query {
                let lenny_txt = format!("{} {}", inline_query.query, "( ͡° ͜ʖ ͡°)");
                let shrug_txt = format!("{} {}", inline_query.query, "¯\\_(ツ)_/¯");
                let lenny = types::InputTextMessageContent::new(lenny_txt.clone());
                let shrug = types::InputTextMessageContent::new(shrug_txt.clone());
                let results = vec![
                    types::InlineQueryResultArticleBuilder::default()
                        .id("lenny")
                        .title(lenny_txt)
                        .input_message_content(lenny)
                        .build()
                        .unwrap()
                        .into(),
                    types::InlineQueryResultArticleBuilder::default()
                        .id("shrug")
                        .title(shrug_txt)
                        .input_message_content(shrug)
                        .build()
                        .unwrap()
                        .into(),
                ];

                let _ = bot.answer_inline_query(&args::AnswerInlineQueryBuilder::default()
                    .inline_query_id(inline_query.id)
                    .results(results)
                    .build()
                    .unwrap());
            }

            if let Some(channel_post) = update.channel_post {
                bot.leave_chat(&args::LeaveChatBuilder::default()
                    .chat_id(channel_post.chat.id)
                    .build()
                    .unwrap());
            }

            if let Some(edited_channel_post) = update.edited_channel_post {
                bot.leave_chat(&args::LeaveChatBuilder::default()
                    .chat_id(edited_channel_post.chat.id)
                    .build()
                    .unwrap());
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

    pub fn exit(bot: &BotApi, message: &Message) -> Result<Message, BotError> {
        let args = args::SendMessageBuilder::default()
            .text("Goodbye!")
            .chat_id(message.chat.id)
            .reply_to_message_id(message.message_id)
            .build()
            .unwrap();

        bot.send_message(&args)
    }

    pub fn help(bot: &BotApi, message: &Message) -> Result<Message, BotError> {
        let args = args::SendMessageBuilder::default()
            .text(
                r#"Hi, I'm a bot!
I was built using Rust and a library by @JuanPotato

[Check out the libary!](https://github.com/JuanPotato/rust-tg-botapi)

Commands:
 - /help `Send this help message`
 - /photo `An example of a photo being sent`
 - /edit `An example of editing a message`
 - /threads `An example of messages being sent from multiple threads`
 - /button `Displays buttons`
 - /inline `Displays inline buttons`
 - /clear `Clears any buttons`
"#,
            )
            .chat_id(message.chat.id)
            .parse_mode(String::from("Markdown"))
            .reply_to_message_id(message.message_id)
            .build()
            .unwrap();

        bot.send_message(&args)
    }

    pub fn photo(bot: &BotApi, message: &Message) -> Result<Message, BotError> {
        let args = args::SendPhotoBuilder::default()
            .chat_id(message.chat.id)
            .reply_to_message_id(message.message_id)
            .photo(PathBuf::from("./image.jpg"))
            .build()
            .unwrap();

        bot.send_photo(&args)
    }

    pub fn edit(
        bot: &BotApi,
        message: &Message,
        edit_text: Option<String>,
    ) -> Result<types::MessageOrBool, BotError> {
        let args = args::SendMessageBuilder::default()
            .text("Editing")
            .chat_id(message.chat.id)
            .reply_to_message_id(message.message_id)
            .build()
            .unwrap();

        bot.send_message(&args).and_then(move |sent_message| {
            let mut edit_args = args::EditMessageTextBuilder::default()
                .text("Edited")
                .chat_id(message.chat.id)
                .message_id(sent_message.message_id)
                .parse_mode(String::from("Markdown"))
                .build()
                .unwrap();
            if let Some(arg) = edit_text {
                edit_args.text = arg;
            }
            bot.edit_message_text(&edit_args)
        })
    }

    macro_rules! button {
        ($text:expr) => (
            types::KeyboardButtonBuilder::default().text(String::from($text)).build().unwrap()
        )
    }

    pub fn button(bot: &BotApi, message: &Message) -> Result<Message, BotError> {
        let keyboard = types::ReplyKeyboardMarkupBuilder::default()
            .keyboard(vec![
                vec![button!("Yes"), button!("No")],
                vec![button!("Eh"), button!("He")],
            ])
            .build()
            .unwrap();

        let args = args::SendMessageBuilder::default()
            .text("Yes or No?")
            .chat_id(message.chat.id)
            .reply_markup(types::ReplyMarkup::from(keyboard))
            .build()
            .unwrap();

        bot.send_message(&args)
    }


    pub fn inline(bot: &BotApi, message: &Message) -> Result<Message, BotError> {
        let keyboard = types::InlineKeyboardMarkupBuilder::default()
            .inline_keyboard(vec![
                vec![
                    types::InlineKeyboardButtonBuilder::default()
                        .text("Some")
                        .url(String::from("https://www.youtube.com/watch?v=L_jWHffIx5E"))
                        .build()
                        .unwrap(),
                    types::InlineKeyboardButtonBuilder::default()
                        .text("Body")
                        .url(String::from("https://www.youtube.com/watch?v=rlYys58hsCU"))
                        .build()
                        .unwrap(),
                ],
                vec![
                    types::InlineKeyboardButtonBuilder::default()
                        .text("Once")
                        .url(String::from("https://www.youtube.com/watch?v=Q-MizNywQ94"))
                        .build()
                        .unwrap(),
                    types::InlineKeyboardButtonBuilder::default()
                        .text("Told")
                        .url(String::from("https://www.youtube.com/watch?v=J48dqyz_C6s"))
                        .build()
                        .unwrap(),
                ],
            ])
            .build()
            .unwrap();

        let args = args::SendMessageBuilder::default()
            .text("Me")
            .chat_id(message.chat.id)
            .reply_markup(Some(keyboard.into()))
            .build()
            .unwrap();

        bot.send_message(&args)
    }

    pub fn clear(bot: &BotApi, message: &Message) -> Result<Message, BotError> {
        let args = args::SendMessageBuilder::default()
            .text("Me too")
            .chat_id(message.chat.id)
            .reply_markup(Some(
                types::ReplyKeyboardRemoveMarkupBuilder::default()
                    .remove_keyboard(true)
                    .build()
                    .unwrap()
                    .into(),
            ))
            .build()
            .unwrap();
        bot.send_message(&args)
    }
}
