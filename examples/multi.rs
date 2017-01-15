extern crate tg_botapi;

use tg_botapi::args;
use tg_botapi::BotApi;

use std::sync::Arc;
use std::thread;
use std::env;

fn main() {
    let token = &env::var("TOKEN").expect("No bot token provided, please set the environment variable TOKEN");
    let bot = Arc::new(BotApi::new(token));

    let mut update_args = args::GetUpdates {
        offset: Some(0),
        limit: None,
        timeout: Some(600),
        allowed_updates: None,
    };
            
    'update_loop: loop {
        let updates = bot.get_updates(&update_args).unwrap();

        for update in updates {
            update_args.offset = Some(update.update_id + 1);

            if let Some(message) = update.message {
                // let from = message.from.unwrap();

                let message_text = message.text.unwrap_or(String::new());
                let mut split_text = message_text.split_whitespace();

                if let Some(cmd) = split_text.next() {
                    match cmd {
                        "/exit" => {
                            let _ = bot.send_message(&args::SendMessage {
                                chat_id: Some(message.chat.id),
                                chat_username: None,
                                text: "Goodbye!",
                                parse_mode: Some("Markdown"),
                                disable_web_page_preview: None,
                                disable_notification: None,
                                reply_to_message_id: Some(message.message_id),
                                reply_markup: None,
                            });
                            break 'update_loop;
                        }
                        "/start" | "/help" => {
                            let _ = bot.send_message(&args::SendMessage {
                                chat_id: Some(message.chat.id),
                                chat_username: None,
                                text: "Hi, I'm a bot!",
                                parse_mode: Some("Markdown"),
                                disable_web_page_preview: None,
                                disable_notification: None,
                                reply_to_message_id: Some(message.message_id),
                                reply_markup: None,
                            });
                        }
                        "/photo" => {
                            let _ = bot.send_photo(&args::SendPhoto {
                                chat_id: Some(message.chat.id),
                                chat_username: None,
                                photo: Some("/home/juan/Documents/JuanPotato.png"),
                                file_id: None,
                                caption: Some("Yeahboi"),
                                disable_notification: None,
                                reply_to_message_id: Some(message.message_id),
                                reply_markup: None,
                            });
                        }
                        "/edit" => {
                            let sent = bot.send_message(&args::SendMessage {
                                chat_id: Some(message.chat.id),
                                chat_username: None,
                                text: "Editing...",
                                parse_mode: Some("Markdown"),
                                disable_web_page_preview: None,
                                disable_notification: None,
                                reply_to_message_id: Some(message.message_id),
                                reply_markup: None,
                            });

                            match sent {
                                Ok(sent_message) => {
                                    let mut edit_args = args::EditMessageText {
                                        chat_id: Some(message.chat.id),
                                        chat_username: None,
                                        message_id: Some(sent_message.message_id),
                                        inline_message_id: None,
                                        text: "Edited",
                                        parse_mode: None,
                                        disable_web_page_preview: None,
                                        reply_markup: None,
                                    };

                                    if let Some(arg) = split_text.next() {
                                        edit_args.text = &arg;
                                    }

                                    let _ = bot.edit_message_text(&edit_args);
                                }
                                Err(_) => {}
                            }
                        }
                        "/thread" | "/threads" => {
                            let bot1 = bot.clone();
                            let bot2 = bot.clone();
                            let bot3 = bot.clone();

                            let chat_id = message.chat.id;
                            let msg_id = message.message_id;

                            thread::spawn(move || {
                                let _ = bot1.send_message(&args::SendMessage {
                                    chat_id: Some(chat_id),
                                    chat_username: None,
                                    text: "Thread 1",
                                    parse_mode: Some("Markdown"),
                                    disable_web_page_preview: None,
                                    disable_notification: None,
                                    reply_to_message_id: Some(msg_id),
                                    reply_markup: None,
                                });
                            });
                            thread::spawn(move || {
                                let _ = bot2.send_message(&args::SendMessage {
                                    chat_id: Some(chat_id),
                                    chat_username: None,
                                    text: "Thread 2",
                                    parse_mode: Some("Markdown"),
                                    disable_web_page_preview: None,
                                    disable_notification: None,
                                    reply_to_message_id: Some(msg_id),
                                    reply_markup: None,
                                });
                            });
                            thread::spawn(move || {
                                let _ = bot3.send_message(&args::SendMessage {
                                    chat_id: Some(chat_id),
                                    chat_username: None,
                                    text: "Thread 3",
                                    parse_mode: Some("Markdown"),
                                    disable_web_page_preview: None,
                                    disable_notification: None,
                                    reply_to_message_id: Some(msg_id),
                                    reply_markup: None,
                                });
                            });
                        }
                        _ => {}
                    }
                }
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